pub mod main;
pub mod core;
mod decode;
pub mod arith;
pub mod branch;
pub mod loadstore;
pub mod floating;
mod atomic;
pub mod crypto;
pub mod defs;
mod bitmanip;
mod decode16;
pub mod consts;
mod floating_helpers;
#[cfg(test)]
mod tests;
pub mod system;

use arith::*;
use branch::*;
use loadstore::*;