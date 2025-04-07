use crossterm::{
    cursor, execute,
    style::{Color, Stylize},
    terminal::{Clear, ClearType},
};
use rand::{Rng, seq::IndexedRandom};
use std::io::{self, Write};

const MAZE_SIZE: usize = 13;

pub struct DragonMaze {
    pub maze: [[i32; MAZE_SIZE]; MAZE_SIZE],
    pub render_grid: [[bool; MAZE_SIZE * 3 + 1]; MAZE_SIZE * 3 + 1],
    pub player: (usize, usize),
    pub dragon: (usize, usize),
    pub exit: (usize, usize),
    pub setup: bool,
    pub score: i32,
    pub autoplay: bool,
    pub AUTOPLAY_DELAY: u64,
}

// Bitmask constants for the walls
const NORTH: i32 = 1;
const SOUTH: i32 = 2;
const EAST: i32 = 4;
const WEST: i32 = 8;

const PAUSE: u64 = 100;

const WALLS: [char; 4] = [
    ' ', // 0000
    '█', // 2588
    '▄', // 2584
    '▀', // 2580
];

impl DragonMaze {
    pub fn new() -> Self {
        let mut game = DragonMaze {
            maze: [[15; MAZE_SIZE]; MAZE_SIZE],
            render_grid: [[false; MAZE_SIZE * 3 + 1]; MAZE_SIZE * 3 + 1],
            player: (0, rand::rng().random_range(0..MAZE_SIZE)),
            dragon: (MAZE_SIZE - 1, rand::rng().random_range(0..MAZE_SIZE)),
            exit: (MAZE_SIZE - 1, rand::rng().random_range(0..MAZE_SIZE)),
            setup: true,
            score: 0,
            autoplay: true,
            AUTOPLAY_DELAY: 20,
        };

        game.score = 1000;

        //hide the cursor
        execute!(io::stdout(), cursor::Hide).unwrap();

        // draw the full maze grid
        game.init_render_grid();
        execute!(io::stdout(), Clear(ClearType::All)).unwrap();
        for y in 0..MAZE_SIZE * 3 + 1 {
            for x in 0..MAZE_SIZE * 3 + 1 {
                game.draw_pixel(x, y);
            }
        }
        // output credits
        execute!(io::stdout(), cursor::MoveTo(0, 20)).unwrap();
        print!("DRAGON MAZE");
        execute!(io::stdout(), cursor::MoveTo(24, 20)).unwrap();
        print!("GARY  J. SHANNON");

        // generate the maze
        game.generate_maze();

        // clear the maze grid except for the outer border
        for y in 1..MAZE_SIZE * 3 {
            for x in 1..MAZE_SIZE * 3 {
                game.render_grid[x][y] = false;
                game.draw_pixel(x, y);
            }
        }
        // place and draw the exit
        game.maze[game.exit.0][game.exit.1] &= !EAST;
        game.remove_wall(game.exit.0, game.exit.1, EAST);

        game.setup = false;
        execute!(io::stdout(), cursor::MoveTo(0, 21)).unwrap();
        print!("THE MAZE IS READY");
        io::stdout().flush().unwrap();

        // draw the player and dragon
        game.draw_cell(game.player.0, game.player.1);
        game.draw_cell(game.dragon.0, game.dragon.1);

        game
    }

    fn generate_maze(&mut self) {
        let mut rng = rand::rng();
        let mut stack = Vec::new();
        let mut visited = vec![vec![false; MAZE_SIZE]; MAZE_SIZE];

        let start_x = rng.random_range(0..MAZE_SIZE);
        let start_y = rng.random_range(0..MAZE_SIZE);
        stack.push((start_x, start_y));
        visited[start_x][start_y] = true;

        while let Some((x, y)) = stack.pop() {
            let pause_duration = if self.autoplay { 20 } else { PAUSE };
            std::thread::sleep(std::time::Duration::from_millis(pause_duration));
            let mut neighbors = Vec::new();

            if x > 0 && !visited[x - 1][y] {
                neighbors.push((x - 1, y, WEST, EAST));
            }
            if x < MAZE_SIZE - 1 && !visited[x + 1][y] {
                neighbors.push((x + 1, y, EAST, WEST));
            }
            if y > 0 && !visited[x][y - 1] {
                neighbors.push((x, y - 1, NORTH, SOUTH));
            }
            if y < MAZE_SIZE - 1 && !visited[x][y + 1] {
                neighbors.push((x, y + 1, SOUTH, NORTH));
            }

            if !neighbors.is_empty() {
                // I want to pause here for an arbitrary amount of time
                // so the user can see the maze being generated
                std::thread::sleep(std::time::Duration::from_millis(pause_duration));
                stack.push((x, y));
                let &(nx, ny, wall, opposite_wall) = neighbors.choose(&mut rng).unwrap();
                self.maze[x][y] &= !wall; // Remove the wall between the current cell and the neighbor
                self.remove_wall(x, y, wall);
                self.maze[nx][ny] &= !opposite_wall; // Remove the opposite wall in the neighbor cell
                visited[nx][ny] = true;
                stack.push((nx, ny));
            }
        }
    }

    /*
     ** this function turns off pixels in the render_grid based on the
     ** parameters passed.  We must convert the x,y coordinates to the
     ** render_grid coordinates, then turn off the appropriate pixels
     */
    fn remove_wall(&mut self, x: usize, y: usize, direction: i32) {
        // let x = x * 3 + 1;
        // let y = y * 3 + 1;
        match direction {
            NORTH => {
                self.render_grid[x * 3 + 1][y * 3] = false;
                self.render_grid[x * 3 + 2][y * 3] = false;
                self.draw_pixel(x * 3 + 1, y * 3);
                self.draw_pixel(x * 3 + 2, y * 3);
            }
            SOUTH => {
                self.render_grid[x * 3 + 1][y * 3 + 3] = false;
                self.render_grid[x * 3 + 2][y * 3 + 3] = false;
                self.draw_pixel(x * 3 + 1, y * 3 + 3);
                self.draw_pixel(x * 3 + 2, y * 3 + 3);
            }
            EAST => {
                self.render_grid[x * 3 + 3][y * 3 + 1] = false;
                self.render_grid[x * 3 + 3][y * 3 + 2] = false;
                self.draw_pixel(x * 3 + 3, y * 3 + 1);
                self.draw_pixel(x * 3 + 3, y * 3 + 2);
            }
            WEST => {
                self.render_grid[x * 3][y * 3 + 1] = false;
                self.render_grid[x * 3][y * 3 + 2] = false;
                self.draw_pixel(x * 3, y * 3 + 1);
                self.draw_pixel(x * 3, y * 3 + 2);
            }
            _ => {}
        }
    }

    fn init_render_grid(&mut self) {
        // clear the render_grid
        for row in self.render_grid.iter_mut() {
            for cell in row.iter_mut() {
                *cell = false;
            }
        }
        // draw grid lines
        for y in (0..MAZE_SIZE * 3 + 1).step_by(3) {
            self.hlin(0, y, MAZE_SIZE * 3 + 1);
        }
        for x in (0..MAZE_SIZE * 3 + 1).step_by(3) {
            self.vlin(x, 0, MAZE_SIZE * 3 + 1);
        }
        //self.draw_initial_screen();
    }

    // this function has to take x,y from the render_grid and apply the conversion
    fn has_player(&self, x: usize, y: usize) -> bool {
        if self.setup {
            return false;
        }
        self.coords_match(x, y, self.player.0, self.player.1)
    }

    fn has_dragon(&self, x: usize, y: usize) -> bool {
        if self.setup {
            return false;
        }
        self.coords_match(x, y, self.dragon.0, self.dragon.1)
    }

    fn coords_match(&self, render_x: usize, render_y: usize, maze_x: usize, maze_y: usize) -> bool {
        if render_x == maze_x * 3 + 1 || render_x == maze_x * 3 + 2 {
            return render_y == maze_y * 3 + 1 || render_y == maze_y * 3 + 2;
        }
        // if maze_y % 2 == 0 {
        //     return render_y == maze_y / 2 * 3 || render_y == maze_y / 2 * 3 + 1;
        // } else {
        //     return render_y == (3 * maze_y + 1) / 2;
        // }
        false
    }

    fn hlin(&mut self, x: usize, y: usize, len: usize) {
        if len == 0 {
            return;
        }
        //clamp x and y to the render_grid size
        let start_col = x.clamp(0, MAZE_SIZE * 3);
        let row = y.clamp(0, MAZE_SIZE * 3);
        let len = len.clamp(0, MAZE_SIZE * 3 - start_col + 1);

        for column in start_col..start_col + len {
            self.render_grid[column][row] = true;
        }
    }

    fn vlin(&mut self, x: usize, y: usize, len: usize) {
        if len == 0 {
            return;
        }
        //clamp x and y to the render_grid size
        let column = x.clamp(0, MAZE_SIZE * 3);
        let start_row = y.clamp(0, MAZE_SIZE * 3);
        let len = len.clamp(0, MAZE_SIZE * 3 - start_row + 1);

        for row in start_row..start_row + len {
            self.render_grid[column][row] = true;
        }
    }

    fn draw_pixel(&mut self, x: usize, y: usize) {
        // x and y are render_grid coordinates
        // x will be a direct translation to the terminal column
        // but y will need special handling
        let t_col = x;
        let t_row = y / 2;
        //let mut draw = 0; // default to drawing an empty cell
        let cell: String = self.get_character_to_draw(x, y);

        // move cursor to the right position
        execute!(io::stdout(), cursor::MoveTo(t_col as u16, t_row as u16)).unwrap();
        // draw the pixel
        print!("{}", cell);
        //flush the buffer to ensure the text is displayed
        io::stdout().flush().unwrap();
    }

    fn draw_cell(&mut self, x: usize, y: usize) {
        self.draw_pixel(x * 3 + 1, y * 3 + 1);
        self.draw_pixel(x * 3 + 2, y * 3 + 1);
        self.draw_pixel(x * 3 + 1, y * 3 + 2);
        self.draw_pixel(x * 3 + 2, y * 3 + 2);
    }

    fn draw_cell_wall(&mut self, x: usize, y: usize, direction: i32) {
        match direction {
            NORTH => {
                self.render_grid[x * 3][y * 3] = true;
                self.render_grid[x * 3 + 1][y * 3] = true;
                self.render_grid[x * 3 + 2][y * 3] = true;
                self.render_grid[x * 3 + 3][y * 3] = true;
                self.draw_pixel(x * 3, y * 3);
                self.draw_pixel(x * 3 + 1, y * 3);
                self.draw_pixel(x * 3 + 2, y * 3);
                self.draw_pixel(x * 3 + 3, y * 3);
            }
            SOUTH => {
                self.render_grid[x * 3][y * 3 + 3] = true;
                self.render_grid[x * 3 + 1][y * 3 + 3] = true;
                self.render_grid[x * 3 + 2][y * 3 + 3] = true;
                self.render_grid[x * 3 + 3][y * 3 + 3] = true;
                self.draw_pixel(x * 3, y * 3 + 3);
                self.draw_pixel(x * 3 + 1, y * 3 + 3);
                self.draw_pixel(x * 3 + 2, y * 3 + 3);
                self.draw_pixel(x * 3 + 3, y * 3 + 3);
            }
            EAST => {
                self.render_grid[x * 3 + 3][y * 3] = true;
                self.render_grid[x * 3 + 3][y * 3 + 1] = true;
                self.render_grid[x * 3 + 3][y * 3 + 2] = true;
                self.render_grid[x * 3 + 3][y * 3 + 3] = true;
                self.draw_pixel(x * 3 + 3, y * 3);
                self.draw_pixel(x * 3 + 3, y * 3 + 1);
                self.draw_pixel(x * 3 + 3, y * 3 + 2);
                self.draw_pixel(x * 3 + 3, y * 3 + 3);
            }
            WEST => {
                self.render_grid[x * 3][y * 3] = true;
                self.render_grid[x * 3][y * 3 + 1] = true;
                self.render_grid[x * 3][y * 3 + 2] = true;
                self.render_grid[x * 3][y * 3 + 3] = true;
                self.draw_pixel(x * 3, y * 3);
                self.draw_pixel(x * 3, y * 3 + 1);
                self.draw_pixel(x * 3, y * 3 + 2);
                self.draw_pixel(x * 3, y * 3 + 3);
            }
            _ => {}
        }
    }

    fn get_character_to_draw(&self, x: usize, y: usize) -> String {
        let mut background_color = Color::Black;
        let other_pixel: (usize, usize);
        let pixel_on = self.render_grid[x][y];
        let top_pixel;
        if y % 2 == 0 {
            other_pixel = (x, y + 1);
            top_pixel = true;
        } else {
            other_pixel = (x, y - 1);
            top_pixel = false;
        }
        let other_pixel_on = self.render_grid[other_pixel.0][other_pixel.1];

        // easy peasy
        if pixel_on && other_pixel_on {
            return WALLS[1].to_string();
        }

        if pixel_on || other_pixel_on {
            if self.has_player(x, y) || self.has_player(other_pixel.0, other_pixel.1) {
                background_color = Color::Green;
            } else if self.has_dragon(x, y) || self.has_dragon(other_pixel.0, other_pixel.1) {
                background_color = Color::Red;
            }
            if top_pixel {
                if pixel_on {
                    return WALLS[3].to_string().on(background_color).to_string();
                } else {
                    return WALLS[2].to_string().on(background_color).to_string();
                }
            } else if pixel_on {
                return WALLS[2].to_string().on(background_color).to_string();
            } else {
                return WALLS[3].to_string().on(background_color).to_string();
            }
        }

        // the only case left is where neither pixel is on
        let full_player = self.has_player(x, y) && self.has_player(other_pixel.0, other_pixel.1);
        let full_dragon = self.has_dragon(x, y) && self.has_dragon(other_pixel.0, other_pixel.1);

        if full_player {
            return WALLS[1].to_string().with(Color::Green).to_string();
        }
        if full_dragon {
            return WALLS[1].to_string().with(Color::Red).to_string();
        }

        if self.has_player(x, y) {
            if top_pixel {
                return WALLS[3].to_string().with(Color::Green).to_string();
            } else {
                return WALLS[2].to_string().with(Color::Green).to_string();
            }
        } else if self.has_dragon(x, y) {
            if top_pixel {
                return WALLS[3].to_string().with(Color::Red).to_string();
            } else {
                return WALLS[2].to_string().with(Color::Red).to_string();
            }
        } else if self.has_player(other_pixel.0, other_pixel.1) {
            if top_pixel {
                return WALLS[2].to_string().with(Color::Green).to_string();
            } else {
                return WALLS[3].to_string().with(Color::Green).to_string();
            }
        } else if self.has_dragon(other_pixel.0, other_pixel.1) {
            if top_pixel {
                return WALLS[2].to_string().with(Color::Red).to_string();
            } else {
                return WALLS[3].to_string().with(Color::Red).to_string();
            }
        }

        " ".to_string()
    }

    // Function to check if there is a path in the given direction
    fn has_path(&self, x: usize, y: usize, direction: i32) -> bool {
        self.maze[x][y] & direction == 0
    }

    pub fn move_player(&mut self, dx: isize, dy: isize) {
        let old_x = self.player.0;
        let old_y = self.player.1;
        let new_x = (self.player.0 as isize + dx).clamp(0, MAZE_SIZE as isize - 1) as usize;
        let new_y = (self.player.1 as isize + dy).clamp(0, MAZE_SIZE as isize - 1) as usize;

        // let's convert dx and dy to a direction, then test to make sure
        // the direction is valid for the given cell
        let direction = match (dx, dy) {
            (1, 0) => EAST,
            (-1, 0) => WEST,
            (0, 1) => SOUTH,
            (0, -1) => NORTH,
            _ => 0,
        };
        if !self.has_path(old_x, old_y, direction) {
            self.draw_cell_wall(old_x, old_y, direction);
            self.score -= 1;
            return;
        }

        self.player = (new_x, new_y);
        self.draw_cell(old_x, old_y);
        self.draw_cell(new_x, new_y);
        self.score -= 1;
    }

    pub fn move_dragon(&mut self) {
        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut rng = rand::rng();
        let (dx, dy) = directions.choose(&mut rng).unwrap();
        let direction = match (dx, dy) {
            (1, 0) => EAST,
            (-1, 0) => WEST,
            (0, 1) => SOUTH,
            (0, -1) => NORTH,
            _ => 0,
        };
        let old_x = self.dragon.0;
        let old_y = self.dragon.1;
        let new_x = (self.dragon.0 as isize + dx).clamp(0, MAZE_SIZE as isize - 1) as usize;
        let new_y = (self.dragon.1 as isize + dy).clamp(0, MAZE_SIZE as isize - 1) as usize;
        if self.has_path(old_x, old_y, direction) {
            self.dragon = (new_x, new_y);
        } else {
            //there's a chance the dragon will climb over the wall
        }
        self.draw_cell(old_x, old_y);
        self.draw_cell(new_x, new_y);
    }

    pub fn win(&self) {
        execute!(io::stdout(), cursor::MoveTo(0, 21), Clear(ClearType::CurrentLine)).unwrap();
        execute!(io::stdout(), cursor::MoveTo(0, 22)).unwrap();
        println!("YOU WIN!");
        execute!(io::stdout(), cursor::MoveTo(0, 23)).unwrap();
        println!("SCORE: {}", self.score);
    }

    pub fn lose(&self) {
        execute!(io::stdout(), cursor::MoveTo(0, 21), Clear(ClearType::CurrentLine)).unwrap();
        execute!(io::stdout(), cursor::MoveTo(0, 22)).unwrap();
        println!("THE DRAGON GOT YOU!");
    }
}
