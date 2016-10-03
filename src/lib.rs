//! The unicorn library.
//!
//! `unicorn`'s purpose is to dissolve fragmentation of the internet;
//! by making it possible to bridge together different types of
//! networks. It aims to be a data-agnostic communications technology
//! platform that can connect any number and combination of clients
//! (humans or machines).

#![cfg_attr(feature="nightly", feature(custom_derive, plugin))]
#![cfg_attr(feature="nightly", plugin(serde_macros))]

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate log;

extern crate ws;
extern crate jsonrpc_core;

pub mod api;
pub mod config;
pub mod datastore;
pub mod kernel;
pub mod logger;
pub mod network;

/// Defines schemas (datatypes) used by API
#[cfg(feature = "nightly")]
pub mod schema;

/// Defines schemas (datatypes) used by API
#[cfg(not(feature = "nightly"))]
pub mod schema {
    include!(concat!(env!("OUT_DIR"), "/schema.rs"));
}

/// unicorn version
pub const VERSION: [i32; 3] = [0, 0, 1];

/// Return version as a formatted string in semver format
pub fn get_version() -> String { format!("{:?}.{:?}.{:?}", VERSION[0], VERSION[1], VERSION[2]) }
