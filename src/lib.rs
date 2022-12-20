pub struct ScoreGrid {
    total_score: usize,
}

impl ScoreGrid {
    pub fn new() -> Self {
        Self { total_score: 0 }
    }

    pub fn score(&self) -> usize {
        self.total_score
    }

    pub fn roll(&mut self, arg: usize) {
        self.total_score += arg;
    }
}

impl Default for ScoreGrid {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_score_should_be_0() {
        let score_grid = ScoreGrid::default();

        assert_eq!(0, score_grid.score());
    }

    #[test]
    fn five_pin_roll_should_score_five_points() {
        let mut score_grid = ScoreGrid::default();
        score_grid.roll(5);

        assert_eq!(score_grid.score(), 5)
    }

    #[test]
    fn two_rolls_of_five_pins_should_score_ten() {
        let mut score_grid = ScoreGrid::default();

        score_grid.roll(5);
        score_grid.roll(5);

        assert_eq!(score_grid.total_score, 10);
    }
}
