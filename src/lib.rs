extern crate serde_derive;
extern crate bincode;
#[macro_use] extern crate enum_primitive_derive;
extern crate num_traits;

pub mod frontend;

pub mod vm;

pub mod core;

pub mod codegen;

pub mod prelude {
    pub use super::frontend::prelude::*;
    pub use super::vm::prelude::*;
    pub use super::core::prelude::*;
}