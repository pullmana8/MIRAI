// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// A test for tag tracking involving function calls

#![feature(const_generics)]
#![allow(incomplete_features)]

#[macro_use]
extern crate mirai_annotations;

use mirai_annotations::{TagPropagation, TagPropagationSet};

struct SecretTaintKind<const MASK: TagPropagationSet> {}

const SECRET_TAINT: TagPropagationSet = tag_propagation_set!(TagPropagation::BitOr);

type SecretTaint = SecretTaintKind<SECRET_TAINT>;

fn argument_must_be_tainted(value: &i32) {
    precondition!(has_tag!(value, SecretTaint));
}

pub fn main() {
    let secret = 99991;
    add_tag!(&secret, SecretTaint);
    argument_must_be_tainted(&secret);
}
