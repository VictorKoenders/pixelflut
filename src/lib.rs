#![feature(const_vec_new)]
#![cfg_attr(test, feature(test))]

#[cfg_attr(test, macro_use)]
#[cfg(test)]
extern crate proptest;
#[cfg(test)]
extern crate test;

pub mod client;
mod handlers;
mod screen;
mod utils;

pub use crate::utils::*;