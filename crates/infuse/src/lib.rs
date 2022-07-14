#![feature(const_type_id)]
#![feature(const_trait_impl)]
#![feature(specialization)]
#![feature(const_option)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(const_refs_to_cell)]

pub mod dependencies;
pub mod service;

pub use infuse_macros::*;
