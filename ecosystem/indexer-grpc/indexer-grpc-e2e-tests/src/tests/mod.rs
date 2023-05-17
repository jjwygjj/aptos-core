// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use std::env;

#[test]
fn sanity_check() {
    assert_eq!(2 + 2, 4);
}

const VALIDATOR_IMAGE_REPO: &str = env!("VALIDATOR_IMAGE_REPO", "Missing VALIDATOR_IMAGE_REPO");
const VALIDATOR_IMAGE_TAG: &str = env!("VALIDATOR_IMAGE_TAG", "Missing VALIDATOR_IMAGE_TAG");

mod fullnode_tests;
