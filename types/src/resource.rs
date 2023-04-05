// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;
use move_vm_types::resolver::Resource;
use crate::state_store::state_key::StateKey;
use crate::state_store::state_value::StateValue;

pub trait TransactionWrite {
    // We do not need this anymore!
    fn extract_raw_bytes(&self) -> Option<Vec<u8>>;

    // We do not need this anymore!
    fn as_state_value(&self) -> Option<StateValue>;

    // Should be generic on Key?
    fn as_aptos_resource(&self) -> Option<AptosResource>;
}

pub enum AptosResource {
    Aggregator(u128),
    Standard(Resource),
    Group(BTreeMap<StateKey, Resource>),
}

impl AptosResource {
    pub fn from_blob(blob: Vec<u8>) -> Self {
        AptosResource::Standard(Resource::from_blob(blob))
    }
}