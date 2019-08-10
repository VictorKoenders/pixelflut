#![feature(const_vec_new)]
#![cfg_attr(test, feature(test))]

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
