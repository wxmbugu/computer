use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompError {
    #[error("Invalid header (expected {expected:?}, got {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("Missing attribute: {0}")]
    MissingAttribute(String),
    #[error("Size not Implemented")]
    InvalidSize,
    #[error("Instruction not found")]
    InstructionAccess,
}
