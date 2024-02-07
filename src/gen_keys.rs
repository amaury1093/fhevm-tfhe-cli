// BSD 3-Clause Clear License
//
// Copyright © 2023 ZAMA.
// All rights reserved.

use tfhe::{generate_keys, ClientKey, CompactPublicKey, ConfigBuilder, ServerKey};

pub fn gen_keys() -> (ClientKey, ServerKey, CompactPublicKey) {
    let config = ConfigBuilder::default().build();
    let (cks, sks) = generate_keys(config);
    let pks = CompactPublicKey::new(&cks);
    (cks, sks, pks)
}
