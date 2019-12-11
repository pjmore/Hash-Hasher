#![warn(unused, missing_copy_implementations, missing_docs)]
#![deny(
    deprecated_in_future,
    future_incompatible,
    macro_use_extern_crate,
    rust_2018_idioms,
    nonstandard_style,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    warnings,
    clippy::all,
    clippy::pedantic
)]
#![forbid(
    const_err,
    duplicate_macro_exports,
    exceeding_bitshifts,
    invalid_type_param_default,
    legacy_constructor_visibility,
    legacy_directory_ownership,
    macro_expanded_macro_exports_accessed_by_absolute_paths,
    missing_fragment_specifier,
    mutable_transmutes,
    no_mangle_const_items,
    order_dependent_trait_objects,
    overflowing_literals,
    parenthesized_params_in_types_and_modules,
    pub_use_of_private_extern_crate,
    safe_extern_statics,
    unknown_crate_types
)]
#![feature(test)]

extern crate test;

mod utils;

use test::Bencher;
use utils::{
    sha1::Digest as Sha1Digest, sha256::Digest as Sha256Digest, sha512::Digest as Sha512Digest,
    sip::Digest as SipDigest,
};

#[bench]
fn hash_sha1s_using_default_hasher(bencher: &mut Bencher) {
    utils::hash_using_default_hasher::<Sha1Digest>(bencher)
}

#[bench]
fn hash_sha1s_using_hash_hasher(bencher: &mut Bencher) {
    utils::hash_using_hash_hasher::<Sha1Digest>(bencher)
}

#[bench]
fn hash_sha1s_using_fnv_hasher(bencher: &mut Bencher) {
    utils::hash_using_fnv_hasher::<Sha1Digest>(bencher)
}

#[bench]
fn insert_sha1s_into_set_using_default_hasher(bencher: &mut Bencher) {
    utils::set_using_default_hasher::<Sha1Digest>(bencher)
}

#[bench]
fn insert_sha1s_into_set_using_hash_hasher(bencher: &mut Bencher) {
    utils::set_using_hash_hasher::<Sha1Digest>(bencher)
}

#[bench]
fn insert_sha1s_into_fnv_set(bencher: &mut Bencher) {
    utils::set_using_fnv_hasher::<Sha1Digest>(bencher)
}

#[bench]
fn hash_sha256s_using_default_hasher(bencher: &mut Bencher) {
    utils::hash_using_default_hasher::<Sha256Digest>(bencher)
}

#[bench]
fn hash_sha256s_using_hash_hasher(bencher: &mut Bencher) {
    utils::hash_using_hash_hasher::<Sha256Digest>(bencher)
}

#[bench]
fn hash_sha256s_using_fnv_hasher(bencher: &mut Bencher) {
    utils::hash_using_fnv_hasher::<Sha256Digest>(bencher)
}

#[bench]
fn insert_sha256s_into_set_using_default_hasher(bencher: &mut Bencher) {
    utils::set_using_default_hasher::<Sha256Digest>(bencher)
}

#[bench]
fn insert_sha256s_into_set_using_hash_hasher(bencher: &mut Bencher) {
    utils::set_using_hash_hasher::<Sha256Digest>(bencher)
}

#[bench]
fn insert_sha256s_into_set_using_fnv_hasher(bencher: &mut Bencher) {
    utils::set_using_fnv_hasher::<Sha256Digest>(bencher)
}

#[bench]
fn hash_sha512s_using_default_hasher(bencher: &mut Bencher) {
    utils::hash_using_default_hasher::<Sha512Digest>(bencher)
}

#[bench]
fn hash_sha512s_using_hash_hasher(bencher: &mut Bencher) {
    utils::hash_using_hash_hasher::<Sha512Digest>(bencher)
}

#[bench]
fn hash_sha512s_using_fnv_hasher(bencher: &mut Bencher) {
    utils::hash_using_fnv_hasher::<Sha512Digest>(bencher)
}

#[bench]
fn insert_sha512s_into_set_using_default_hasher(bencher: &mut Bencher) {
    utils::set_using_default_hasher::<Sha512Digest>(bencher)
}

#[bench]
fn insert_sha512s_into_set_using_hash_hasher(bencher: &mut Bencher) {
    utils::set_using_hash_hasher::<Sha512Digest>(bencher)
}

#[bench]
fn insert_sha512s_into_set_using_fnv_hasher(bencher: &mut Bencher) {
    utils::set_using_fnv_hasher::<Sha512Digest>(bencher)
}

#[bench]
fn hash_sip_hashes_using_default_hasher(bencher: &mut Bencher) {
    utils::hash_using_default_hasher::<SipDigest>(bencher)
}

#[bench]
fn hash_sip_hashes_using_hash_hasher(bencher: &mut Bencher) {
    utils::hash_using_hash_hasher::<SipDigest>(bencher)
}

#[bench]
fn hash_sip_hashes_using_fnv_hasher(bencher: &mut Bencher) {
    utils::hash_using_fnv_hasher::<SipDigest>(bencher)
}

#[bench]
fn insert_sip_hashes_into_set_using_default_hasher(bencher: &mut Bencher) {
    utils::set_using_default_hasher::<SipDigest>(bencher)
}

#[bench]
fn insert_sip_hashes_into_set_using_hash_hasher(bencher: &mut Bencher) {
    utils::set_using_hash_hasher::<SipDigest>(bencher)
}

#[bench]
fn insert_sip_hashes_into_set_using_fnv_hasher(bencher: &mut Bencher) {
    utils::set_using_fnv_hasher::<SipDigest>(bencher)
}