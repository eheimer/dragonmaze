mod maze;

use crossterm::{
    cursor,
    event::{Event, KeyCode, KeyEvent, read},
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use maze::DragonMaze;
use std::io::{self, Write};
use rand::Rng;

fn print_instructions() {
    println!("WELCOME TO THE DRAGON'S MAZE!");
    println!("YOU MAY WATCH WHILE I BUILD A MAZE,");
    println!("BUT WHEN IT'S COMPLETE, I'LL ERASE");
    println!("THE PICTURE. THEN YOU'LL ONLY SEE THE WA");
    println!("LLS AS YOU BUMP INTO THEM.");
    println!("TO MOVE, YOU HIT 'D' FOR RIGHT,");
    println!("'A' FOR LEFT, 'W' FOR UP, AND");
    println!("'S' FOR DOWN. DO NOT HIT RETURN!");
    println!();
    println!("THE OBJECT IS FOR YOU (THE GREEN DOT");
    println!("TO GET TO THE DOOR ON THE RIGHT SIDE");
    println!("BEFORE THE DRAGON (THE RED DOT) EATS");
    println!("YOU.");
    println!("BEWARE!!!!!!!!! SOMETIMES THE DRAGON");
    println!("GETS REAL MAD, AND CLIMBS OVER A WALL.");
    println!("BUT MOST OF THE TIME, HE CAN'T GO OVER");
    println!("AND HAS TO GO AROUND.");
    println!();
    println!("(HINT: YOU CAN OFTEN TELL WHERE A WALL");
    println!("IS, EVEN BEFORE YOU CAN SEE IT, BY");
    println!("THE FACT THAT THE DRAGON CAN'T GET");
    println!("THROUGH IT!)");
    println!();
}

fn main() {
    let autoplay = true;
    execute!(io::stdout(), EnterAlternateScreen, Clear(ClearType::All)).unwrap();
    print_instructions();

    if autoplay {
        let mut game = DragonMaze::new();
    
        enable_raw_mode().unwrap();
        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut rng = rand::rng();
        loop {
            let (dx, dy) = directions[rng.random_range(0..directions.len())];
            game.move_player(dx, dy);
            if game.autoplay {
                std::thread::sleep(std::time::Duration::from_millis(game.AUTOPLAY_DELAY));
            }
            game.move_dragon();
            if game.player == game.exit {
                game.win();
                break;
            } else if game.player == game.dragon {
                game.lose();
                break;
            }
        }
        disable_raw_mode().unwrap();
        execute!(io::stdout(), cursor::MoveTo(0, 24)).unwrap();
        return;
    }

    // Wait for the user to type "go" to start the game
    print!("TYPE 'GO' TO BEGIN ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input.trim().to_lowercase() != "go" {
        return;
    }
    let mut game = DragonMaze::new();
    

    enable_raw_mode().unwrap(); // Enable raw mode to suppress character output
    if game.autoplay {
        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut rng = rand::rng();
        loop {
            let (dx, dy) = directions[rng.random_range(0..directions.len())];
            game.move_player(dx, dy);
            if game.autoplay {
                std::thread::sleep(std::time::Duration::from_millis(game.AUTOPLAY_DELAY));
            }
            game.move_dragon();
            if game.player == game.exit {
                game.win();
                break;
            } else if game.player == game.dragon {
                game.lose();
                break;
            }
        }
    } else {
        loop {
            if let Event::Key(KeyEvent { code, .. }) = read().unwrap() {
                match code {
                    KeyCode::Char('d') | KeyCode::Right => game.move_player(1, 0),
                    KeyCode::Char('a') | KeyCode::Left => game.move_player(-1, 0),
                    KeyCode::Char('w') | KeyCode::Up => game.move_player(0, -1),
                    KeyCode::Char('s') | KeyCode::Down => game.move_player(0, 1),
                    KeyCode::Esc => break, // Allow exiting the loop with the Esc key
                    _ => continue,
                }
                game.move_dragon();
                if game.player == game.exit {
                    game.win();
                    break;
                } else if game.player == game.dragon {
                    game.lose();
                    break;
                }
            }
        }
    }
    disable_raw_mode().unwrap(); // Disable raw mode when done
    execute!(io::stdout(), cursor::MoveTo(0, 24)).unwrap();
}
