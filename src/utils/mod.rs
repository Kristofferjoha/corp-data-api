// Validation trait, mostly just to show interface usage

pub trait Validate {
    fn validate(&self) -> Result<(), String>;
}