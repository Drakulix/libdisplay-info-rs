#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/gen.rs"));

#[link(name = "display-info")]
extern "C" {}
