//! Data types for representing the ClinVar information.

use std::collections::HashMap;

use chrono::NaiveDate;
use shrinkwraprs::Shrinkwrap;

/// Represent clinical significance.
#[derive(Debug, Default)]
pub struct ClinicalSignificance {
    /// Date of last evaluation.
    pub date_evaluated: NaiveDate,
    /// Significance review status.
    pub review_status: String,
    /// Significance description.
    pub description: Option<String>,
    /// Comments.
    pub comments: Vec<String>,
}

/// Relevant information from `ObservedData/Attribute[@Type="Description"]`.
#[derive(Debug, Default)]
pub struct ObservedDataDescription {
    /// Optional description text.
    pub description: Option<String>,
    /// PubMed IDs.
    pub pubmed_ids: Vec<u32>,
    /// OMIM IDs.
    pub omim_ids: Vec<u32>,
}

/// Relevant part of `ObservedIn`.
#[derive(Debug, Default)]
pub struct ObservedIn {
    /// The origin of the sample.
    pub origin: String,
    /// The species of the sample.
    pub species: String,
    /// Affected state.
    pub affected_status: String,
    /// Optional observation info.
    pub observed_data_description: Option<ObservedDataDescription>,
    /// Comments
    pub comments: Vec<String>,
}

/// Sequence location information.
#[derive(Debug, Default)]
pub struct SequenceLocation {
    pub assembly: String,
    pub chrom: String,
    pub chrom_acc: String,
    pub start: Option<u32>,
    pub stop: Option<u32>,
    pub outer_start: Option<u32>,
    pub outer_stop: Option<u32>,
    pub inner_start: Option<u32>,
    pub inner_stop: Option<u32>,
    pub reference: Option<String>,
    pub alternative: Option<String>,
}

/// Represent the relevant information from a Measure.
#[derive(Debug, Default)]
pub struct Measure {
    pub measure_type: String,
    pub symbols: Vec<String>,
    pub hgnc_ids: Vec<String>,
    pub sequence_locations: HashMap<String, SequenceLocation>,
    pub comments: Vec<String>,
}

/// Represent the relevant information from a Trait.
#[derive(Debug, Default)]
pub struct Trait {
    pub preferred_name: Option<String>,
    pub alternate_names: Vec<String>,
}

/// Represent the relevant information from a TraitSet.
#[derive(Debug, Default)]
pub struct TraitSet {
    /// Value of the "Type" attribute.
    pub set_type: String,
    /// Numeric `id_no` for the ClinVarSet.
    pub id_no: Option<u32>,
    /// The traits in the set.
    pub traits: Vec<Trait>,
}

/// Represent the relevant information from a MeasureSet.
#[derive(Debug, Default)]
pub struct MeasureSet {
    pub set_type: String,
    pub accession: String,
    pub measures: Vec<Measure>,
}

/// Represents a genotype observation in ClinVar.
///
/// NB: we introduce dummy sets even for non-compound variants.
#[derive(Debug, Default)]
pub struct GenotypeSet {
    pub set_type: String,
    pub accession: String,
    pub measure_sets: Vec<MeasureSet>,
}

/// Represent the relevant parts of a `ReferenceClinVarAssertion`
#[derive(Debug, Default)]
pub struct ReferenceClinVarAssertion {
    /// Numeric id_no for the ClinVarSet.
    pub id_no: String,
    /// Record status
    pub record_status: String,
    /// Date of creation.
    pub date_created: NaiveDate,
    /// Date of last update.
    pub date_updated: NaiveDate,

    /// The accession number.
    pub clinvar_accession: String,
    /// The version of the record.
    pub version_no: u32,
    /// Description where the variant was observed.
    pub observed_in: Option<ObservedIn>,
    /// Genotype sets
    pub genotype_sets: Vec<GenotypeSet>,
    /// Trait sets.
    pub trait_sets: Vec<TraitSet>,
    /// Clinical significance entries.
    pub clin_sigs: Vec<ClinicalSignificance>,

    /// Number of gold stars shown on ClinVar.
    pub gold_stars: u32,
    /// Review status
    pub review_status: String,
    /// Assertion of pathogenicity.
    pub pathogenicity: String,
}

/// Represent the relevant parts of a `ClinVarAssertion`.
#[derive(Debug, Default)]
pub struct ClinVarAssertion {
    /// Numeric id_no for the ClinVarSet.
    pub id_no: u32,
    /// Record status
    pub record_status: String,
    /// Date of submission.
    pub submitter_date: Option<NaiveDate>,

    /// The accession number.
    pub clinvar_accession: String,
    /// The version of the record.
    pub version_no: u32,
    /// Description where the variant was observed.
    pub observed_in: Option<ObservedIn>,
    /// Genotype sets
    pub genotype_sets: Vec<GenotypeSet>,
    /// Trait sets.
    pub trait_sets: Vec<TraitSet>,
    /// Clinical significance entries.
    pub clin_sigs: Vec<ClinicalSignificance>,

    /// Review status
    pub review_status: String,
    /// Assertion of pathogenicity.
    pub pathogenicity: String,
}

/// Represent the relevant parts of a ClinVarSet
#[derive(Debug, Default)]
pub struct ClinVarSet {
    /// Numeric id_no for the ClinVarSet.
    pub id_no: u32,
    /// Record status
    pub record_status: String,
    /// Record title
    pub title: String,
    /// The ReferenceClinVarAssertion, if any.
    pub ref_cv_assertion: Option<ReferenceClinVarAssertion>,
    /// The ClinVarAssertion objects, if any.
    pub cv_assertions: Vec<ClinVarAssertion>,
}

/// Root tag representation.
#[derive(PartialEq, PartialOrd, Eq, Debug, Default)]
pub struct ReleaseSet {
    pub release_date: NaiveDate,
}

/// Pathogenicity.
#[derive(PartialEq, PartialOrd, Eq, Hash, Copy, Clone, Debug, Default)]
pub enum Pathogenicity {
    Benign,
    LikelyBenign,
    #[default]
    Uncertain,
    LikelyPathogenic,
    Pathogenic,
}

/// Review status.
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, Default)]
pub enum ReviewStatus {
    ConflictingInterpretations,
    CriteriaProvided,
    MultipleSubmitters,
    #[default]
    NoAssertionCriteriaProvided,
    NoAssertionProvided,
    NoConflicts,
    PracticeGuideline,
    ExpertPanel,
    SingleSubmitter,
}

/// Gold star representation.
#[derive(Shrinkwrap, PartialEq, Eq, Debug, Copy, Clone, Default)]
pub struct GoldStars(u32);

impl GoldStars {
    pub fn new(val: u32) -> Self {
        GoldStars(val)
    }
}
