// unsized_tuple_coercion requires nightly
#![feature(unsized_tuple_coercion)]
#![feature(unboxed_closures)]
#![feature(register_tool)]
#![register_tool(tarpaulin)]

/// Fork of bidule
/// Adds handle_ids for subscribers
pub mod frp;
pub use frp::*;

#[cfg(test)]
#[tarpaulin::ignore]
mod tests;
