// Copyright 2024, Horizen Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
#![cfg_attr(not(feature = "std"), no_std)]

use crate::VerifyError;
use sp_runtime_interface::runtime_interface;
#[cfg(feature = "std")]
use valida_verifier::verifier::VerifyError as ValidaError;

// TODO: :)
#[cfg(feature = "std")]
impl From<ValidaError> for VerifyError {
    fn from(value: ValidaError) -> Self {
        match value {
            ValidaError::TooManyFilesInArchive
            | ValidaError::WrongName
            | ValidaError::DirectoryWasSent => VerifyError::InvalidInput,
            _ => VerifyError::VerifyError,
        }
    }
}

#[runtime_interface]
pub trait ValidaVerify {
    // FYI: Passing program here is a temporary measure.
    fn verify(compressed_proof: &[u8], compressed_program: &[u8]) -> Result<(), VerifyError> {
        valida_verifier::verifier::verify(compressed_proof, compressed_program)
            .inspect_err(|e| println!("Cannot verify proof: {:?}", e))
            // TODO: Errors
            .map_err(|e| VerifyError::VerifyError)
            .map(|_| log::trace!("verified"))
    }
}
