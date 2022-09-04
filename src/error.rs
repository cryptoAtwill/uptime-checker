/// All the error from the actor crate
pub enum Error {
    AlreadyNode,
    AlreadyChecker,
    AlreadyVoted,
    CannotDeserialize,
    FVMError(fvm_shared::error::ErrorNumber)
}

impl Error {
    pub fn code(&self) -> u32 {
        match self {
            Error::AlreadyNode => 10000,
            Error::AlreadyChecker => 10001,
            Error::AlreadyVoted => 10002,
            Error::CannotDeserialize => 10003,
            Error::FVMError(e) => *e as u32
        }
    }
}
