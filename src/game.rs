use crate::grid_math::GridMath;

pub enum GameState {
    IDLE,
    RUNNING,
}

pub struct Game {
    grid: Vec<u32>,
    grid_math: GridMath,
    rows: usize,
    cols: usize,
    state: GameState,
}

impl Game {
    pub fn new(rows: usize, cols: usize) -> Self {
        let grid_math = GridMath { rows, cols };
        Game {
            grid: grid_math.initialize_grid(),
            grid_math,
            rows,
            cols,
            state: GameState::IDLE,
        }
    }

    async fn perform_step(&self, grid: &Vec<u32>) -> Vec<u32> {
        let mut new_grid = grid.clone();
        let directions = vec![
            (0, 1),
            (-1, 1),
            (-1, -1),
            (1, -1),
            (-1, -1),
            (1, 0),
            (0, -1),
            (-1, 0),
        ];

        for (index, _) in grid.iter().enumerate() {
            let (row, col) = self.grid_math.get_pos_from_index(index);
            let mut sum = 0;
            for (absolute_row, absolute_col) in directions.iter() {
                match self.grid_math.get_based_on_relative_position(
                    grid,
                    row as i32,
                    col as i32,
                    *absolute_row,
                    *absolute_col,
                ) {
                    Ok(value) => sum += value,
                    Err(_) => continue,
                }
            }
            if sum >= 2 {
                new_grid[index] = 1;
            } else if sum < 2 {
                new_grid[index] = 0;
            }
        }

        new_grid
    }

    pub async fn step(&mut self) {
        if matches!(self.state, GameState::RUNNING) {
            return;
        }

        self.state = GameState::RUNNING;
        self.grid = self.perform_step(&self.grid).await;
        self.state = GameState::IDLE;
    }

    pub fn get_grid(&self) -> &Vec<u32> {
        &self.grid
    }

    pub fn add_draw(&mut self, row: usize, col: usize, value: u32) {
        match self.state {
            GameState::IDLE => self.grid_math.put(&mut self.grid, row, col, value),
            _ => {}
        }
    }

    pub fn stop(&mut self) {
        self.state = GameState::IDLE;
    }
}

#[cfg(test)]
mod game_tests {
    use crate::game::{Game, GameState};
    use futures::executor::block_on;

    #[test]
    fn new_game() {
        let game = Game::new(3, 3);
        assert_eq!(game.rows, 3);
        assert_eq!(game.cols, 3);
        assert_eq!(game.grid.len(), 9);
    }

    #[test]
    fn add_draw() {
        let mut game = Game::new(3, 3);
        game.add_draw(0, 0, 1);
        assert_eq!(game.grid[0], 1);
    }

    #[test]
    fn step() {
        let mut game = Game::new(3, 3);
        game.add_draw(0, 0, 1);
        game.add_draw(0, 1, 1);
        game.add_draw(1, 0, 1);
        block_on(game.step());
        assert_eq!(game.grid[0], 1);
    }

    #[test]
    fn stop() {
        let mut game = Game::new(3, 3);
        game.stop();
        assert!(matches!(game.state, GameState::IDLE));
    }

    #[test]
    fn get_grid() {
        let game = Game::new(3, 3);
        assert_eq!(game.get_grid()[0], 0);
    }
}
