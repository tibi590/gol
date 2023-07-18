#[derive(Clone)]
enum Cell {
    Alive(char),
    Dead(char),
}

struct Grid {
    width: u32,
    height: u32,
    grid: Option<Vec<Vec<Cell>>>,
}


impl Grid {
    fn create(mut self) -> Grid{
        self.grid = Some(vec![vec![Cell::Dead('-'); self.height as usize]; self.width as usize]);
        self
    }

    fn get_neighbors(&self, x: i32, y: i32) -> i32{
        let mut max_x = 0usize;
        if let Some(t) = &self.grid {max_x = t.len()};
        let mut max_y = 0usize; 
        if let Some(t) = &self.grid {max_y = t[0].len()};
        
        let modified_x = [x-1, x+1];
        let modified_y = [y-1, y+1];
        
        let mut neighbor_count = 0;

        if modified_x.iter().any(|c| c < &0 || c > &(max_x as i32)) {return neighbor_count};  
        if modified_y.iter().any(|c| c < &0 || c > &(max_y as i32)) {return neighbor_count};

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
    
}

fn main() {
    let grid = Grid{width: 20, height: 20, grid: None}.create();
    let hal = grid.get_neighbors(3, 5);
    println!("{}", hal)
}
