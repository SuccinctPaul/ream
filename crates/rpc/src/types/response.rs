use alloy_primitives::B256;
use serde::{Deserialize, Serialize};

pub const ELECTRA: &str = "electra";
pub const ETH_CONSENSUS_VERSION_HEADER: &str = "Eth-Consensus-Version";
const EXECUTION_OPTIMISTIC: bool = false;
const FINALIZED: bool = false;

#[derive(Serialize, Deserialize)]
pub struct RootResponse {
    pub root: B256,
}

impl RootResponse {
    pub fn new(root: B256) -> Self {
        Self { root }
    }
}

/// A BeaconResponse data struct that can be used to wrap data type
/// used for json rpc responses
///
/// # Example
/// {
///  "data": json!({
///     "execution_optimistic" : bool,
///     "finalized" : bool,
///     "data" : T
/// })
/// }
#[derive(Debug, Serialize)]
pub struct BeaconResponse<T> {
    pub execution_optimistic: bool,
    pub finalized: bool,
    pub data: T,
}
impl<T: Serialize> BeaconResponse<T> {
    pub fn from(data: T) -> Self {
        Self {
            data,
            execution_optimistic: EXECUTION_OPTIMISTIC,
            finalized: FINALIZED,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct BeaconVersionedResponse<T> {
    pub version: String,
    pub execution_optimistic: bool,
    pub finalized: bool,
    pub data: T,
}

impl<T: Serialize> BeaconVersionedResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            version: String::from("electra"),
            data,
            execution_optimistic: EXECUTION_OPTIMISTIC,
            finalized: FINALIZED,
        }
    }
}
