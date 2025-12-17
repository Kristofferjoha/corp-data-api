/// Validation trait, mostly just to show interface usage
/// used for entity validation before DB operations

pub trait Validate {
    fn validate(&self) -> Result<(), String>;
}