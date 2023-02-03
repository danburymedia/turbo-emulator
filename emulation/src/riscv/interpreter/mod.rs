pub mod main;
pub mod core;
mod decode;
pub mod arith;
pub mod branch;
pub mod loadstore;
pub mod floating;
mod atomic;
// mod vector_insns;
pub mod crypto;
pub mod defs;
mod bitmanip;
// mod vector_arith;
mod decode16;
pub mod consts;
mod floating_helpers;
#[cfg(test)]
mod tests;
pub mod system;

use arith::*;
use branch::*;
use loadstore::*;