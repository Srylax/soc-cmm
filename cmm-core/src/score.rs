use std::{
    fmt::Display,
    iter::{Zip, zip},
};

use strum::VariantArray;

use crate::{
    cid::Domain,
    control::Control,
    data::SOCData,
    schema::{ControlSchema, Schema},
};

pub struct Stats {
    data: SOCData,
    schema: Schema,
}

impl Stats {
    pub fn new(data: SOCData, schema: Schema) -> Self {
        Self { data, schema }
    }

    pub fn score_overall(&self) -> Score {
        let mut score = 0.0;
        for &domain in Domain::VARIANTS {
            score += self.maturity_by_domain(&domain).score();
            let domain_capability = self.capability_by_domain(&domain).score();
            if domain_capability.is_normal() {
                score += domain_capability;
            }
        }
        Score::new(
            score,
            Domain::VARIANTS.len() as f64 * 5.0 + Domain::VARIANTS.len() as f64 * 2.0,
        )
    }

    pub fn capability_by_domain(&self, domain: &Domain) -> Score {
        Stats::capability_score(self.controls_by_domain(domain))
    }

    pub fn maturity_by_domain(&self, domain: &Domain) -> Score {
        Stats::maturity_score(self.controls_by_domain(domain))
    }

    pub fn capability_by_aspect(&self, domain: &Domain, aspect_id: u8) -> Score {
        Stats::capability_score(self.controls_by_aspect(domain, aspect_id))
    }

    pub fn maturity_by_aspect(&self, domain: &Domain, aspect_id: u8) -> Score {
        Stats::maturity_score(self.controls_by_aspect(domain, aspect_id))
    }

    fn controls_by_domain(
        &self,
        domain: &Domain,
    ) -> Zip<impl Iterator<Item = &Control>, impl Iterator<Item = &ControlSchema>> {
        zip(
            self.data
                .controls_by_domain(domain)
                .map(|(_cid, control)| control),
            self.schema
                .controls_by_domain(domain)
                .map(|(_cid, schema)| schema),
        )
    }

    fn controls_by_aspect(
        &self,
        domain: &Domain,
        aspect_id: u8,
    ) -> Zip<impl Iterator<Item = &Control>, impl Iterator<Item = &ControlSchema>> {
        zip(
            self.data
                .controls_by_aspect(domain, aspect_id)
                .map(|(_cid, control)| control),
            self.schema
                .controls_by_aspect(domain, aspect_id)
                .map(|(_cid, schema)| schema),
        )
    }

    fn capability_score<'a, T: IntoIterator<Item = (&'a Control, &'a ControlSchema)>>(
        controls: T,
    ) -> Score {
        let controls_in_scope: Vec<&'a Control> = controls
            .into_iter()
            .filter(|(data, schema)| data.answer().capability_in_scope() && !schema.nist_only())
            .map(|(data, _schema)| data)
            .collect();

        let count = controls_in_scope.len() as f64;

        let total_score = controls_in_scope
            .iter()
            .flat_map(|cap| cap.answer().capability_score())
            .sum::<u32>() as f64;
        let max_score = controls_in_scope
            .iter()
            .flat_map(|cap| cap.answer().max_score())
            .sum::<u32>() as f64;

        Score::new(3.0 * ((total_score - count) / (max_score - count)), 3.0)
    }

    fn maturity_score<'a, T: IntoIterator<Item = (&'a Control, &'a ControlSchema)>>(
        controls: T,
    ) -> Score {
        let controls_in_scope: Vec<&'a Control> = controls
            .into_iter()
            .filter(|(data, schema)| data.answer().maturity_in_scope() && !schema.nist_only())
            .map(|(data, _schema)| data)
            .collect();

        let count = controls_in_scope.len() as f64;

        let total_score = controls_in_scope
            .iter()
            .flat_map(|cap| cap.answer().maturity_score())
            .sum::<u32>() as f64;
        let max_score = controls_in_scope
            .iter()
            .flat_map(|cap| cap.answer().max_score())
            .sum::<u32>() as f64;

        Score::new(5.0 * ((total_score - count) / (max_score - count)), 5.0)
    }
}

/// Score can be either over an Range of controls (0-5max / 0-3max)
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

impl Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", &self.score().round(), &self.max().round())
    }
}

#[cfg(test)]
mod tests {
    use crate::answer::{Answer, DetailedOptional, Satisfaction};

    use super::*;

    fn stats_from_controls(controls: Vec<Control>) -> Stats {
        let data = controls
            .clone()
            .into_iter()
            .enumerate()
            .map(|(cid, control)| (format!("Business.{}", cid + 1).parse().unwrap(), control))
            .collect();
        let data = SOCData::new(data, None);
        let schema = controls
            .into_iter()
            .enumerate()
            .map(|(cid, _control)| {
                (
                    format!("Business.{}", cid + 1).parse().unwrap(),
                    ControlSchema::default(),
                )
            })
            .collect();
        let schema = Schema::new(schema);
        Stats { data, schema }
    }

    #[test]
    fn test_capability_score() {
        // 3 out of 3 so score is == max == 100% == 5
        let controls = vec![
            Control::new(Answer::DetailedOptional(DetailedOptional::Fully), None),
            Control::new(Answer::DetailedOptional(DetailedOptional::Fully), None),
            Control::new(Answer::DetailedOptional(DetailedOptional::Fully), None),
        ];
        let stats = stats_from_controls(controls);

        let score = stats.capability_by_domain(&Domain::Business);
        assert_eq!(score.score(), 3.0);
        assert_eq!(score.max(), 3.0);
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

        let stats = stats_from_controls(controls);

        let score = stats.capability_by_domain(&Domain::Business);

        assert_eq!(score.score(), 3.0);
        assert_eq!(score.max(), 3.0);
        assert_eq!(score.as_percentage(), 100.0);

        let score = stats.maturity_by_domain(&Domain::Business);
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

        let stats = stats_from_controls(controls);

        let score = stats.capability_by_domain(&Domain::Business);
        assert_eq!(score.score(), 3.0);
        assert_eq!(score.max(), 3.0);
        assert_eq!(score.as_percentage(), 100.0);
    }
}
