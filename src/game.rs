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
