//! Code for encoding/decoding strings as data structures.

use std::collections::HashMap;

use tracing::warn;

use crate::records::{GoldStars, Pathogenicity, ReviewStatus};

lazy_static::lazy_static! {
    static ref PATHOGENICITY_LABELS: HashMap<&'static str, Pathogenicity> = {
        let mut m = HashMap::new();
        m.insert("benign", Pathogenicity::Benign);
        m.insert("no known pathogenicity", Pathogenicity::Benign);
        m.insert("non-pathogenic", Pathogenicity::Benign);
        m.insert("poly", Pathogenicity::Benign);

        m.insert("likely benign", Pathogenicity::LikelyBenign);
        m.insert("probable-non-pathogenic", Pathogenicity::LikelyBenign);
        m.insert("probably not pathogenic", Pathogenicity::LikelyBenign);
        m.insert("protective", Pathogenicity::LikelyBenign);
        m.insert("suspected benign", Pathogenicity::LikelyBenign);

        m.insert("uncertain significance", Pathogenicity::Uncertain);
        m.insert("association", Pathogenicity::Uncertain);
        m.insert("association not found", Pathogenicity::Uncertain);
        m.insert("cancer", Pathogenicity::Uncertain);
        m.insert("confers sensitivity", Pathogenicity::Uncertain);
        m.insert("drug response", Pathogenicity::Uncertain);
        m.insert("drug-response", Pathogenicity::Uncertain);
        m.insert("histocompatibility", Pathogenicity::Uncertain);
        m.insert("not provided", Pathogenicity::Uncertain);
        m.insert("other", Pathogenicity::Uncertain);
        m.insert("protective", Pathogenicity::Uncertain);
        m.insert("risk factor", Pathogenicity::Uncertain);
        m.insert("uncertain", Pathogenicity::Uncertain);
        m.insert("unknown", Pathogenicity::Uncertain);
        m.insert("untested", Pathogenicity::Uncertain);
        m.insert("variant of unknown significance", Pathogenicity::Uncertain);
        m.insert("associated with leiomyomas", Pathogenicity::Uncertain);

        m.insert( "likely pathogenic", Pathogenicity::LikelyPathogenic);
        m.insert("affects", Pathogenicity::LikelyPathogenic);
        m.insert("association", Pathogenicity::LikelyPathogenic);
        m.insert("confers sensitivity", Pathogenicity::LikelyPathogenic);
        m.insert("conflicting interpretations of pathogenicity", Pathogenicity::LikelyPathogenic);
        m.insert("probable-pathogenic", Pathogenicity::LikelyPathogenic);
        m.insert("probably pathogenic", Pathogenicity::LikelyPathogenic);
        m.insert("risk factor", Pathogenicity::LikelyPathogenic);
        m.insert("suspected pathogenic", Pathogenicity::LikelyPathogenic);

        m.insert("pathogenic", Pathogenicity::Pathogenic);
        m.insert("moderate", Pathogenicity::Pathogenic);
        m.insert("mut", Pathogenicity::Pathogenic);
        m.insert("pathologic", Pathogenicity::Pathogenic);

        m
    };
}
impl Pathogenicity {
    pub fn from_label(label: &str) -> Result<Self, anyhow::Error> {
        if let Some(pathogenicity) = PATHOGENICITY_LABELS.get(label) {
            Ok(*pathogenicity)
        } else {
            warn!("Cannot decode pathogenicity from {}", label);
            Ok(Pathogenicity::Uncertain)
        }
    }
}

lazy_static::lazy_static! {
    static ref REVIEW_STATUS_LABELS: HashMap<&'static str, ReviewStatus> = {
        let mut m = HashMap::new();
        m.insert("conflicting interpretations", ReviewStatus::ConflictingInterpretations);
        m.insert("conflicting interpretations of pathogenicity", ReviewStatus::ConflictingInterpretations);
        m.insert("criteria provided", ReviewStatus::CriteriaProvided);
        m.insert("multiple submitters", ReviewStatus::MultipleSubmitters);
        m.insert("no assertion criteria provided", ReviewStatus::NoAssertionCriteriaProvided);
        m.insert("no assertion provided", ReviewStatus::NoAssertionProvided);
        m.insert("no conflicts", ReviewStatus::NoConflicts);
        m.insert("practice guideline", ReviewStatus::PracticeGuideline);
        m.insert("reviewed by expert panel", ReviewStatus::ExpertPanel);
        m.insert("single submitter", ReviewStatus::SingleSubmitter);
        m
    };
}

impl ReviewStatus {
    pub fn from_label(label: &str) -> Result<Self, anyhow::Error> {
        if let Some(review_status) = REVIEW_STATUS_LABELS.get(label) {
            Ok(*review_status)
        } else {
            Err(anyhow::anyhow!("Unknown review status {}", label))
        }
    }
}

lazy_static::lazy_static! {
    static ref GOLD_STAR_MAP: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();
        m.insert("no assertion provided", 0);
        m.insert("no assertion criteria provided", 0);
        m.insert("criteria provided, single submitter", 1);
        m.insert("criteria provided, multiple submitters, no conflicts", 2);
        m.insert("criteria provided, conflicting interpretations", 1);
        m.insert("reviewed by expert panel", 3);
        m.insert("practice guideline", 4);
        m
    };
}

impl GoldStars {
    pub fn from_review_status(review_status: &str) -> Result<GoldStars, anyhow::Error> {
        if let Some(gold_stars) = GOLD_STAR_MAP.get(review_status) {
            Ok(GoldStars::new(*gold_stars))
        } else {
            Err(anyhow::anyhow!("Unknown review status {}", review_status))
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::records::{GoldStars, Pathogenicity, ReviewStatus};

    #[test]
    fn pathogenicity_from_label() -> Result<(), anyhow::Error> {
        assert_eq!(Pathogenicity::from_label("benign")?, Pathogenicity::Benign);
        assert!(Pathogenicity::from_label("xxx").is_err());

        Ok(())
    }

    #[test]
    fn review_status_from_label() -> Result<(), anyhow::Error> {
        assert_eq!(
            ReviewStatus::from_label("single submitter")?,
            ReviewStatus::SingleSubmitter
        );
        assert!(ReviewStatus::from_label("xxx").is_err());

        Ok(())
    }

    #[test]
    fn gold_stars_from_review_status() -> Result<(), anyhow::Error> {
        assert_eq!(
            GoldStars::from_review_status("practice guideline")?,
            GoldStars::new(4)
        );
        assert!(GoldStars::from_review_status("xxx").is_err());

        Ok(())
    }
}
