use crate::control::Control;

pub trait ScoreCalculator {
    fn capability_score(self) -> Score;
    fn maturity_score(self) -> Score;
}

impl<'a, T: IntoIterator<Item = &'a Control>> ScoreCalculator for T {
    fn capability_score(self) -> Score {
        let controls_in_scope: Vec<&'a Control> = self
            .into_iter()
            .filter(|cap| cap.answer().capability_in_scope() && !cap.nist_only())
            .collect();

        let total_score = controls_in_scope
            .iter()
            .flat_map(|cap| cap.answer().capability_score())
            .sum::<u8>() as f64;
        let max_score = controls_in_scope
            .iter()
            .flat_map(|cap| cap.answer().max_score())
            .sum::<u8>() as f64;

        Score::new(5.0 * (total_score / max_score), 5.0)
    }

    fn maturity_score(self) -> Score {
        let controls_in_scope: Vec<&'a Control> = self
            .into_iter()
            .filter(|cap| cap.answer().maturity_in_scope() && !cap.nist_only())
            .collect();

        let total_score = controls_in_scope
            .iter()
            .flat_map(|cap| cap.answer().maturity_score())
            .sum::<u8>() as f64;
        let max_score = controls_in_scope
            .iter()
            .flat_map(|cap| cap.answer().max_score())
            .sum::<u8>() as f64;

        Score::new(5.0 * (total_score / max_score), 5.0)
    }
}

/// Score can be either over an Range of controls (0-5max)
/// Or over the whole soc-cmm itself, each domain calculated seperately
/// 5 domains 0-5 = 0-25max
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Score {
    score: f64,
    max: f64,
}

impl Score {
    pub fn new(score: f64, max: f64) -> Self {
        Self { score, max }
    }

    pub fn as_percentage(&self) -> f64 {
        self.score / self.max * 100.0
    }

    pub fn score(&self) -> f64 {
        self.score
    }

    pub fn max(&self) -> f64 {
        self.max
    }
}

#[cfg(test)]
mod tests {
    use crate::answer::{Answer, DetailedOptional, Satisfaction};

    use super::*;

    #[test]
    fn test_capability_score() {
        // 3 out of 3 so score is == max == 100% == 5
        let controls = vec![
            Control::new(Answer::DetailedOptional(DetailedOptional::Fully), None),
            Control::new(Answer::DetailedOptional(DetailedOptional::Fully), None),
            Control::new(Answer::DetailedOptional(DetailedOptional::Fully), None),
        ];

        let score = controls.capability_score();
        assert_eq!(score.score(), 5.0);
        assert_eq!(score.max(), 5.0);
        assert_eq!(score.as_percentage(), 100.0);
    }

    #[test]
    fn test_mixed_score() {
        // 3 out of 3 so score is == max == 100% == 5
        let controls = vec![
            Control::new(Answer::DetailedOptional(DetailedOptional::Fully), None),
            Control::new(Answer::DetailedOptional(DetailedOptional::Fully), None),
            Control::new(Answer::DetailedOptional(DetailedOptional::Fully), None),
            Control::new(Answer::Satisfaction(Satisfaction::Fully), None),
            Control::new(Answer::Satisfaction(Satisfaction::Fully), None),
            Control::new(Answer::Satisfaction(Satisfaction::Fully), None),
        ];

        let score = controls.capability_score();
        assert_eq!(score.score(), 5.0);
        assert_eq!(score.max(), 5.0);
        assert_eq!(score.as_percentage(), 100.0);

        let score = controls.maturity_score();
        assert_eq!(score.score(), 5.0);
        assert_eq!(score.max(), 5.0);
        assert_eq!(score.as_percentage(), 100.0);
    }

    #[test]
    fn test_capability_score_not_required() {
        // 2 out of 2 so score is == max == 100% == 5
        let controls = vec![
            Control::new(
                Answer::DetailedOptional(DetailedOptional::NotRequired),
                None,
            ),
            Control::new(Answer::DetailedOptional(DetailedOptional::Fully), None),
            Control::new(Answer::DetailedOptional(DetailedOptional::Fully), None),
        ];

        let score = controls.capability_score();
        assert_eq!(score.score(), 5.0);
        assert_eq!(score.max(), 5.0);
        assert_eq!(score.as_percentage(), 100.0);
    }
}
