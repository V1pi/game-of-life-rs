pub struct GridMath {
    pub rows: usize,
    pub cols: usize,
}

impl GridMath {
    pub fn initialize_grid(&self) -> Vec<u32> {
        vec![0; (self.rows * self.cols) as usize]
    }

    pub fn put(&self, grid: &mut Vec<u32>, row: usize, col: usize, value: u32) {
        grid[(row * self.cols + col) as usize] = value;
    }

    pub fn get(&self, grid: &Vec<u32>, row: usize, col: usize) -> u32 {
        grid[(row * self.cols + col) as usize]
    }

    pub fn merge_grids(&self, original: &Vec<u32>, grids: &Vec<Vec<u32>>) -> Vec<u32> {
        let mut merged = original.clone();
        for (index, value) in original.iter().enumerate() {
            let mut new_value = *value;
            for grid in grids.iter() {
                if *value != grid[index] {
                    new_value = grid[index];
                }
            }
            merged[index] = new_value;
        }
        merged
    }
}

#[cfg(test)]
mod grid_tests {
    use crate::grid_math::GridMath;

    #[test]
    fn instantiate_grid() {
        let grid_math = GridMath { rows: 3, cols: 3 };
        let grid = grid_math.initialize_grid();
        assert_eq!(grid, vec![0; 9]);
    }

    #[test]
    fn put_value_in_grid() {
        let mut grid = vec![0; 9];
        let grid_math = GridMath { rows: 3, cols: 3 };
        grid_math.put(&mut grid, 1, 1, 1);
        assert_eq!(grid, vec![0, 0, 0, 0, 1, 0, 0, 0, 0]);
    }

    #[test]
    fn get_value_from_grid() {
        let grid = vec![0, 0, 0, 0, 1, 0, 0, 0, 0];
        let grid_math = GridMath { rows: 3, cols: 3 };
        assert_eq!(grid_math.get(&grid, 1, 1), 1);
    }

    #[test]
    fn merge_grids() {
        let original = vec![0, 0, 0, 0, 1, 0, 0, 1, 0];
        let grid1 = vec![0, 0, 0, 0, 0, 1, 1, 0, 0];
        let grid2 = vec![0, 0, 1, 0, 0, 0, 0, 1, 0];
        let grid_math = GridMath { rows: 3, cols: 3 };
        let merged = grid_math.merge_grids(&original, &vec![grid1, grid2]);
        assert_eq!(merged, vec![0, 0, 1, 0, 0, 1, 1, 0, 0]);
    }
}
