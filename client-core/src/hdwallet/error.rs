//! # Extended Key for HD-wallet
//! adapted from https://github.com/jjyr/hdwallet (HDWallet)
//! Copyright (c) 2018, Jiang Jinyang (licensed under the MIT License)
//! Modifications Copyright (c) 2018 - 2019, Foris Limited (licensed under the Apache License, Version 2.0)
//!

pub use crate::hdwallet::ChainPathError;

#[derive(Debug, Clone, Eq, PartialEq)]
/// Error code for hdwallet
pub enum Error {
    /// Index is out of range
    KeyIndexOutOfRange,
    /// ChainPathError
    ChainPath(ChainPathError),
    /// secp256k1 errors
    Secp(secp256k1::Error),
}

impl From<ChainPathError> for Error {
    fn from(err: ChainPathError) -> Error {
        Error::ChainPath(err)
    }
}

impl From<secp256k1::Error> for Error {
    fn from(err: secp256k1::Error) -> Error {
        Error::Secp(err)
    }
}