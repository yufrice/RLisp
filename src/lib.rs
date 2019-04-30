#![feature(slice_patterns)]

#[macro_use]
extern crate combine;
#[macro_use]
extern crate auto_enums;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
extern crate env_logger;

pub mod compile;
pub mod engine;
pub mod error;
pub mod parser;
pub mod syntax;
