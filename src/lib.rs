#![feature(const_vec_new, test)]

#[cfg_attr(test, macro_use)]
#[cfg(test)]
extern crate proptest;
#[cfg(test)]
extern crate test;

pub mod client;
pub mod handlers;
pub mod screen;
pub mod utils;

pub use crate::utils::*;

pub type Result<T> = std::result::Result<T, failure::Error>;
