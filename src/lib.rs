extern crate serde_derive;
extern crate bincode;

pub mod frontend;

pub mod vm;

pub mod core;

pub mod prelude {
    pub use super::frontend::prelude::*;
    pub use super::vm::prelude::*;
    pub use super::core::prelude::*;
    pub use bincode::*;
}