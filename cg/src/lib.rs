#![feature(range_contains)]
#![feature(rustc_private)]
#![feature(const_vec_new)]
#![feature(integer_atomics)]
#![feature(custom_derive,asm,box_syntax,box_patterns)]
#![feature(core_intrinsics)]
#![feature(generators, generator_trait)]
#![feature(associated_type_defaults)]
#![feature(exclusive_range_pattern)]
#![feature(box_into_raw_non_null)]
#![feature(trait_alias)]
#![feature(const_fn)]
#![feature(nll)]
#![feature(fnbox)]
#[warn(unreachable_patterns)]

extern crate core;

extern crate cgmath;
extern crate collision;
extern crate slab;


pub mod color;
pub mod octree;

pub use cgmath::*;
pub use collision::*;