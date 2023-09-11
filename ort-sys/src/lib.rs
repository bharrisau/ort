#![allow(dead_code)]

mod ctypes;
#[allow(dead_code, non_upper_case_globals, non_camel_case_types, non_snake_case, deref_nullptr)]
#[allow(clippy::all)]
mod sys;

mod api;
mod environment;

pub use api::Api;
