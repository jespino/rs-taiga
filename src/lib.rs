#![cfg_attr(feature = "serde_macros", feature(custom_derive, plugin))]
#![cfg_attr(feature = "serde_macros", plugin(serde_macros))]

extern crate serde;
extern crate serde_json;

#[cfg(feature = "serde_macros")]
include!("lib.in.rs");

#[cfg(not(feature = "serde_macros"))]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));
