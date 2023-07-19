use core::fmt;
use std::{thread, time};

#[derive(Clone, Debug)]
enum Cell {
    Alive(char),
    Dead(char),
}

struct Grid {
    width: i32,
    height: i32,
    alive: char,
    dead: char,
    grid: Option<Vec<Vec<Cell>>>,
}


impl Grid {
    fn create(mut self) -> Grid{
        self.grid = Some(vec![vec![Cell::Dead(self.dead); self.height as usize]; self.width as usize]);
        self
    }
    
    fn set_cell(&mut self, x: usize, y: usize, state: Cell) {
        if let Some(ref mut g) = self.grid {g[x - 1][y - 1]=state} 
    }

    fn get_neighbors(&self, x: i32, y: i32) -> i32{
        let modified_x = [x-1, x,  x+1];
        let modified_y = [y-1, y, y+1];
        
        let mut neighbor_count = - 1;

        if modified_x.iter().any(|c| c < &0 || c > &(self.width - 1)) {return neighbor_count};  
        if modified_y.iter().any(|c| c < &0 || c > &(self.height - 1)) {return neighbor_count};

        for x in modified_x {
            for y in modified_y{
                match &self.grid {
                    Some(t) => if let Cell::Alive(_) = &t[x as usize][y as usize] {neighbor_count += 1},
                    None => neighbor_count += 0,
                };
            };
        };
        neighbor_count
    }

    fn update(&mut self){
        let mut current_grid = vec![vec![Cell::Dead(self.dead)]; 1];
        if let Some(g) = &self.grid {current_grid = g.to_vec()};

        let mut updated_grid = current_grid.clone();
        
        for column in 0..self.width as usize{
            for row in 0..self.height as usize{
                match current_grid[column][row] {
                    Cell::Alive(_) => {
                        if self.get_neighbors(column as i32, row as i32) < 2 || self.get_neighbors(column as i32, row as i32) > 3 {
                            updated_grid[column][row] = Cell::Dead(self.dead)
                        }
                    },
                    Cell::Dead(_) => {
                        if self.get_neighbors(column as i32, row as i32) == 2 {
                            updated_grid[column][row] = Cell::Alive(self.alive)
                        }
                    }
                }
            }
        }
        println!("before: {}", self);
        self.grid = Some(updated_grid);
        println!("after: {}", self);
    }      
}

impl Cell {
    fn to_char(&self) -> char{
        match self {
            Cell::Alive(c) | Cell::Dead(c) => *c
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Cell::Alive(c) | Cell::Dead(c) => write!(f, "{}", c)
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.grid {
            Some(g) => {
                let mut canvas = String::new();
                let mut counter = 0i32;
                for row in 0..self.height{
                    for column in g {
                        canvas.push(column[row as usize].to_char());
                        counter += 1;
                        if counter % self.width == 0 {
                            canvas.push_str("\n");
                        }
                    }
                };
                write!(f, "{}", canvas)
            },
            None => write!(f, "no vector")
        }
    }
}

fn main() {
    let mut grid = Grid{width: 60, height: 30, alive: '🟩', dead: '⬛', grid: None}.create();
    grid.set_cell(30, 17, Cell::Alive(grid.alive));
    grid.set_cell(31, 17, Cell::Alive(grid.alive));
    grid.set_cell(32, 17, Cell::Alive(grid.alive));

    loop {
        print!("{}c", 27 as char);
        println!("{}", grid);
        grid.update();
        thread::sleep(time::Duration::from_secs_f64(0.2));
    }
}
