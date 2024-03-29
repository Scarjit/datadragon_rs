#![allow(
    clippy::needless_return,
    clippy::multiple_crate_versions,
    clippy::missing_docs_in_private_items,
    clippy::missing_inline_in_public_items,
    clippy::non_ascii_literal,
    clippy::implicit_return,
    clippy::module_name_repetitions,
    clippy::pub_enum_variant_names,
)]
#[macro_use]
extern crate cached;

pub mod api;
pub mod tools;