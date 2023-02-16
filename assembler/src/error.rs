use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompError {
    #[error("Size not Implemented")]
    InvalidSize,
    #[error("Instruction not found")]
    InstructionAccess,
}
