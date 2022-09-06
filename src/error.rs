/// All the error from the actor crate
pub enum Error {
    AlreadyNode,
    AlreadyChecker,
    AlreadyVoted,
    CannotDeserialize,
    FVMError(fvm_shared::error::ErrorNumber),
    FVMIpldHamtError(fvm_ipld_hamt::Error),
    AnyhowError(anyhow::Error),
    FVMSharedErrorNum(fvm_shared::error::ErrorNumber),
    FVMSDKNoStateError(fvm_sdk::error::NoStateError),
    FVMEncodingError(fvm_ipld_encoding::Error),
    FVMSharedAddressError(fvm_shared::address::Error),
    NotOwner,
    NotExists,
}

impl Error {
    pub fn code(&self) -> u32 {
        match self {
            Error::AlreadyNode => 10000,
            Error::AlreadyChecker => 10001,
            Error::AlreadyVoted => 10002,
            Error::CannotDeserialize => 10003,
            Error::FVMError(e) => *e as u32,
            Error::FVMIpldHamtError(_) => 10004,
            Error::AnyhowError(_) => 10005,
            Error::FVMSharedErrorNum(e) => *e as u32,
            Error::FVMSDKNoStateError(_) => 10006,
            Error::FVMEncodingError(_) => 10007,
            Error::FVMSharedAddressError(_) => 10008,
            Error::NotOwner => 10009,
            Error::NotExists => 10010,
        }
    }

    pub fn msg(&self) -> String {
        match self {
            Error::FVMSharedAddressError(e) => format!("{:?}", e),
            _ => String::from("")
        }
    }
}

impl From<fvm_ipld_hamt::Error> for Error {
    fn from(e: fvm_ipld_hamt::Error) -> Self {
        Error::FVMIpldHamtError(e)
    }
}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        Error::AnyhowError(e)
    }
}

impl From<fvm_shared::error::ErrorNumber> for Error {
    fn from(e: fvm_shared::error::ErrorNumber) -> Self {
        Error::FVMSharedErrorNum(e)
    }
}

impl From<fvm_sdk::error::NoStateError> for Error {
    fn from(e: fvm_sdk::error::NoStateError) -> Self {
        Error::FVMSDKNoStateError(e)
    }
}

impl From<fvm_ipld_encoding::Error> for Error {
    fn from(e: fvm_ipld_encoding::Error) -> Self {
        Error::FVMEncodingError(e)
    }
}

impl From<fvm_shared::address::Error> for Error {
    fn from(e: fvm_shared::address::Error) -> Self {
        Error::FVMSharedAddressError(e)
    }
}
