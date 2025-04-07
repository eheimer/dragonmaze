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
    pub anger: i32,
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

const WALLS: [char; 4] = [
    ' ', // 0000
    '█', // 2588
    '▄', // 2584
    '▀', // 2580
];

impl DragonMaze {
    pub fn new() -> Self {
        let mut game = DragonMaze {
            maze: [[NORTH + EAST + WEST + SOUTH; MAZE_SIZE]; MAZE_SIZE],
            anger: 10,
            render_grid: [[false; MAZE_SIZE * 3 + 1]; MAZE_SIZE * 3 + 1],
            player: (0, rand::rng().random_range(0..MAZE_SIZE)),
            dragon: (MAZE_SIZE - 1, rand::rng().random_range(0..MAZE_SIZE)),
            exit: (MAZE_SIZE - 1, rand::rng().random_range(0..MAZE_SIZE)),
            setup: true,
            score: 0,
            autoplay: false,
            AUTOPLAY_DELAY: 20,
        };

        game.score = 1000;

        // Generate the maze
        game.generate_maze();

        // Initialize the render grid to represent the generated maze
        game.init_render_grid();

        // Place and draw the exit
        game.maze[game.exit.0][game.exit.1] &= !EAST;
        game.remove_wall(game.exit.0, game.exit.1, EAST);

        game.setup = false;

        game
    }

    pub fn draw_maze(&mut self) {
        execute!(io::stdout(), Clear(ClearType::All)).unwrap();
        for y in 0..MAZE_SIZE * 3 + 1 {
            for x in 0..MAZE_SIZE * 3 + 1 {
                self.draw_pixel(x, y);
            }
        }

        // Output credits
        execute!(io::stdout(), cursor::MoveTo(0, 20)).unwrap();
        print!("DRAGON MAZE");
        execute!(io::stdout(), cursor::MoveTo(24, 20)).unwrap();
        print!("GARY  J. SHANNON");
        io::stdout().flush().unwrap();
    }

    pub fn clear_internal_walls(&mut self) {
        for y in 1..MAZE_SIZE * 3 {
            for x in 1..MAZE_SIZE * 3 {
                self.render_grid[x][y] = false;
            }
        }
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
        match direction {
            NORTH => {
                self.render_grid[x * 3 + 1][y * 3] = false;
                self.render_grid[x * 3 + 2][y * 3] = false;
            }
            SOUTH => {
                self.render_grid[x * 3 + 1][y * 3 + 3] = false;
                self.render_grid[x * 3 + 2][y * 3 + 3] = false;
            }
            EAST => {
                self.render_grid[x * 3 + 3][y * 3 + 1] = false;
                self.render_grid[x * 3 + 3][y * 3 + 2] = false;
            }
            WEST => {
                self.render_grid[x * 3][y * 3 + 1] = false;
                self.render_grid[x * 3][y * 3 + 2] = false;
            }
            _ => {}
        }
    }

    fn init_render_grid(&mut self) {
        if !self.setup { return; }
        // Loop through each cell of the maze array and set the walls in the render_grid
        for x in 0..MAZE_SIZE {
            for y in 0..MAZE_SIZE {
                if self.maze[x][y] & NORTH != 0 {
                    self.render_grid[x * 3][y * 3] = true;
                    self.render_grid[x * 3 + 1][y * 3] = true;
                    self.render_grid[x * 3 + 2][y * 3] = true;
                    self.render_grid[x * 3 + 3][y * 3] = true;
                }
                if self.maze[x][y] & SOUTH != 0 {
                    self.render_grid[x * 3][y * 3 + 3] = true;
                    self.render_grid[x * 3 + 1][y * 3 + 3] = true;
                    self.render_grid[x * 3 + 2][y * 3 + 3] = true;
                    self.render_grid[x * 3 + 3][y * 3 + 3] = true;
                }
                if self.maze[x][y] & EAST != 0 {
                    self.render_grid[x * 3 + 3][y * 3] = true;
                    self.render_grid[x * 3 + 3][y * 3 + 1] = true;
                    self.render_grid[x * 3 + 3][y * 3 + 2] = true;
                    self.render_grid[x * 3 + 3][y * 3 + 3] = true;
                }
                if self.maze[x][y] & WEST != 0 {
                    self.render_grid[x * 3][y * 3] = true;
                    self.render_grid[x * 3][y * 3 + 1] = true;
                    self.render_grid[x * 3][y * 3 + 2] = true;
                    self.render_grid[x * 3][y * 3 + 3] = true;
                }
            }
        }
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
        let (player_x, player_y) = self.player;
        let (dragon_x, dragon_y) = self.dragon;

        // Initialize visit count for the dragon's current cell if not already done
        let visit_count = self.maze[dragon_x][dragon_y] >> 4; // Use higher bits for visit count
        let mut new_visit_count = visit_count + 1;

        if new_visit_count > self.anger {
            // Reset visit count and move directly toward the player ignoring walls
            new_visit_count = 0;
            self.maze[dragon_x][dragon_y] = (self.maze[dragon_x][dragon_y] & 0xF) | (new_visit_count << 4);

            let dx = player_x as isize - dragon_x as isize;
            let dy = player_y as isize - dragon_y as isize;

            let (move_x, move_y) = if dx.abs() >= dy.abs() {
                (dx.signum(), 0) // Move along the longest x-axis
            } else {
                (0, dy.signum()) // Move along the longest y-axis
            };

            let new_x = (dragon_x as isize + move_x).clamp(0, MAZE_SIZE as isize - 1) as usize;
            let new_y = (dragon_y as isize + move_y).clamp(0, MAZE_SIZE as isize - 1) as usize;

            self.dragon = (new_x, new_y);
            self.draw_cell(dragon_x, dragon_y);
            self.draw_cell(new_x, new_y);
            return;
        }

        // Update the visit count in the maze
        self.maze[dragon_x][dragon_y] = (self.maze[dragon_x][dragon_y] & 0xF) | (new_visit_count << 4);

        let dx = player_x as isize - dragon_x as isize;
        let dy = player_y as isize - dragon_y as isize;

        let mut moves = vec![
            ((dx.signum(), 0), dx.abs() >= dy.abs()), // 1. Move toward the player along the longest axis
            ((0, dy.signum()), dx.abs() < dy.abs()),  // 2. Move toward the player along the shortest axis
            ((0, -dy.signum()), dx.abs() < dy.abs()), // 3. Move away from the player along the shortest axis
            ((-dx.signum(), 0), dx.abs() >= dy.abs()), // 4. Move away from the player along the longest axis
        ];

        // Sort moves by priority (true first)
        moves.sort_by_key(|&(_, priority)| !priority);

        for ((dx, dy), _) in moves {
            let new_x = (dragon_x as isize + dx).clamp(0, MAZE_SIZE as isize - 1) as usize;
            let new_y = (dragon_y as isize + dy).clamp(0, MAZE_SIZE as isize - 1) as usize;

            let direction = match (dx, dy) {
                (1, 0) => EAST,
                (-1, 0) => WEST,
                (0, 1) => SOUTH,
                (0, -1) => NORTH,
                _ => 0,
            };

            if self.has_path(dragon_x, dragon_y, direction) {
                self.dragon = (new_x, new_y);
                self.draw_cell(dragon_x, dragon_y);
                self.draw_cell(new_x, new_y);
                return;
            }
        }
        // If no valid move is found (shouldn't happen), stay in place
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
