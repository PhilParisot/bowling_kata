pub struct ScoreGrid {
    pub total_score: usize,
}

impl ScoreGrid {
    pub fn score(&self) -> usize {
        self.total_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_score_should_be_0() {
        let mut score_grid = ScoreGrid { total_score: 0 };

        score_grid.total_score = 0;

        assert_eq!(0, score_grid.score());
    }
}
