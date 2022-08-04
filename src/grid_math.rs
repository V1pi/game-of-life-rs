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

    pub fn get_based_on_relative_position(
        &self,
        grid: &Vec<u32>,
        row: i32,
        col: i32,
        absolute_row: i32,
        absolute_col: i32,
    ) -> Result<u32, &'static str> {
        let new_row_int = row + absolute_row;
        let new_col_int = col + absolute_col;

        let new_row = match new_row_int.try_into() {
            Ok(new_row) => new_row,
            Err(_) => return Err("Row out of bounds"),
        };

        let new_col = match new_col_int.try_into() {
            Ok(new_col) => new_col,
            Err(_) => return Err("Col out of bounds"),
        };

        if new_row >= self.rows {
            return Err("Row out of bounds");
        }

        if new_col >= self.cols {
            return Err("Col out of bounds");
        }

        Ok(self.get(grid, new_row, new_col))
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
    fn get_based_relative_position() {
        let grid = vec![0, 0, 0, 0, 1, 0, 0, 2, 1];
        let grid_math = GridMath { rows: 3, cols: 3 };
        assert_eq!(
            grid_math.get_based_on_relative_position(&grid, 1, 1, 0, 0),
            Ok(1)
        );

        assert_eq!(
            grid_math.get_based_on_relative_position(&grid, 1, 1, 1, 1),
            Ok(1)
        );

        assert_eq!(
            grid_math.get_based_on_relative_position(&grid, 1, 1, 2, 2),
            Err("Row out of bounds")
        );

        assert_eq!(
            grid_math.get_based_on_relative_position(&grid, 1, 1, 1, 0),
            Ok(2)
        );
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
