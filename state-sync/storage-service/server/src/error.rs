// Copyright © Diem Foundation
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Deserialize, Error, PartialEq, Eq, Serialize)]
pub enum Error {
    #[error("Invalid request received: {0}")]
    InvalidRequest(String),
    #[error("Storage error encountered: {0}")]
    StorageErrorEncountered(String),
    #[error("Too many invalid requests: {0}")]
    TooManyInvalidRequests(String),
    #[error("Unexpected error encountered: {0}")]
    UnexpectedErrorEncountered(String),
}

impl Error {
    /// Returns a summary label for the error type
    pub fn get_label(&self) -> &'static str {
        match self {
            Error::InvalidRequest(_) => "invalid_request",
            Error::StorageErrorEncountered(_) => "storage_error",
            Error::TooManyInvalidRequests(_) => "too_many_invalid_requests",
            Error::UnexpectedErrorEncountered(_) => "unexpected_error",
        }
    }
}

impl From<diem_storage_service_types::responses::Error> for Error {
    fn from(error: diem_storage_service_types::responses::Error) -> Self {
        Error::UnexpectedErrorEncountered(error.to_string())
    }
}
