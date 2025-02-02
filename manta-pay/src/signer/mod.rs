// Copyright 2019-2022 Manta Network.
// This file is part of manta-rs.
//
// manta-rs is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// manta-rs is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with manta-rs.  If not, see <http://www.gnu.org/licenses/>.

//! Manta Pay Signer Tools

use manta_accounting::wallet::signer;

#[cfg(feature = "groth16")]
use crate::config::{utxo::Checkpoint, Config};

#[cfg(feature = "serde")]
use manta_util::serde::{Deserialize, Serialize};

pub mod client;

#[cfg(feature = "wallet")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "wallet")))]
pub mod base;

/// Synchronization Request
pub type SyncRequest = signer::SyncRequest<Config, Checkpoint>;

/// Synchronization Response
pub type SyncResponse = signer::SyncResponse<Config, Checkpoint>;

/// Synchronization Error
pub type SyncError = signer::SyncError<Checkpoint>;

/// Synchronization Result
pub type SyncResult = signer::SyncResult<Config, Checkpoint>;

/// Signing Request
pub type SignRequest = signer::SignRequest<Config>;

/// Signing Response
pub type SignResponse = signer::SignResponse<Config>;

/// Signing Error
pub type SignError = signer::SignError<Config>;

/// Signing Result
pub type SignResult = signer::SignResult<Config>;

/// Receiving Key Request
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(crate = "manta_util::serde", deny_unknown_fields)
)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GetRequest {
    /// GET
    #[default]
    Get,
}
