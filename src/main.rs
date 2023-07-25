use std::{ thread, time::Duration };
use rand::Rng;
use clearscreen;

fn main() {
    let mut grid: GridSystem = GridSystem {
        max_row: 50,
        max_col: 110,
        grid: [['⬛'; 110]; 50],
        tmp_grid: [['⬛'; 110]; 50]
    };
    let chr: char = '🟥';

    /*
    grid.grid[10][19] = chr;
    grid.grid[10][20] = chr;
    grid.grid[10][21] = chr;
    grid.grid[11][19] = chr;
    grid.grid[11][21] = chr;
    grid.grid[12][19] = chr;
    grid.grid[12][20] = chr;
    grid.grid[12][21] = chr;

    grid.grid[8][25] = chr;
    grid.grid[8][26] = chr;
    grid.grid[8][27] = chr;
    grid.grid[9][25] = chr;
    grid.grid[9][27] = chr;
    grid.grid[10][25] = chr;
    grid.grid[10][26] = chr;
    grid.grid[10][27] = chr;
    grid.grid[9][55] = chr;
    grid.grid[11][55] = chr;
    grid.grid[12][54] = chr;
    grid.grid[12][53] = chr;
    grid.grid[12][52] = chr;
    grid.grid[12][51] = chr;
    grid.grid[12][50] = chr;
    grid.grid[11][50] = chr;
    grid.grid[10][50] = chr;
    grid.grid[9][51] = chr;
    grid.grid[8][53] = chr;
    **/
    grid.randomize_grid();

    refresh_screen(&grid);
    thread::sleep(Duration::from_secs(1));

    loop {
        thread::sleep(Duration::from_millis(200));

        grid.calculate_cells();

        refresh_screen(&grid);
    }
}

fn refresh_screen(grid: &GridSystem) {
    let mut tmp_str = String::new();
    std::process::Command::new("clear").status().unwrap();

    for row in 0..grid.max_row {
        for col in 0..grid.max_col {
            tmp_str.push(grid.get_value(row, col));
        }
        tmp_str.push_str("\n");
    }

    println!("{}", tmp_str);
}

struct GridSystem {
    max_row: isize,
    max_col: isize,
    grid: [[char; 110]; 50],
    tmp_grid: [[char; 110]; 50] 
}

impl GridSystem {
    fn get_value(&self, row: isize, col: isize) -> char {
        self.grid[row as usize][col as usize]
    }

    fn set_true_value(&mut self) {
        self.grid = self.tmp_grid;
    }

    fn set_temp_value(&mut self, row: isize, col: isize, value: char) {
        self.tmp_grid[row as usize][col as usize] = value;
    }
    fn randomize_grid(&mut self) {
        let mut rng = rand::thread_rng();

        for row in 0..self.max_row {
            for col in 0..self.max_col {
                let chr: char = match rng.gen_range(0..=1) {
                    0 => '⬛',
                    1 => '🟥',
                    _ => 'E',
                };

                self.set_temp_value(row, col, chr);
            }
        }
        self.set_true_value();
    }
    fn number_of_neighbours(&self, row: isize, col: isize) -> u8 {
        let mut neighbours = Vec::new();
        let mut number_of_neighbours: u8 = 0;
        for r in row-1..=row+1 {
            if r<0 || r>=self.max_row { continue; }
            for c in col-1..=col+1 {
                if (c<0 || c>=self.max_col) || (r==row && c==col) { continue; }

                neighbours.push(self.get_value(r, c));
            }
        }

        for chr in neighbours {
            match chr {
                '🟥' => number_of_neighbours += 1,
                _ => number_of_neighbours += 0
            };
        }

        return number_of_neighbours;
    }

    fn calculate_cells(&mut self) {
        for row in 0..self.max_row {
            for col in 0..self.max_col {
                let mut current_cell: char = self.get_value(row, col);
                let cell_alive: bool = match current_cell {
                    '⬛' => false,
                    '🟥' => true,
                    _ => false,
                };
                let alive_neighbours: u8 = self.number_of_neighbours(row as isize, col as isize);

                if alive_neighbours<2 && cell_alive {
                    current_cell = '⬛'
                } else if (alive_neighbours==2 || alive_neighbours==3) && cell_alive {
                    current_cell = '🟥'
                } else if alive_neighbours>3 && cell_alive {
                    current_cell = '⬛'
                } else if alive_neighbours==3 && !cell_alive {
                    current_cell = '🟥'
                }

                self.set_temp_value(row, col, current_cell);
            }
        }
        self.set_true_value();
    }
}
