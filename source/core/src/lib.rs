//! Pure parsing, tag construction, and export for IntegritySCADA signal trees.
//! File access and command-line argument handling belong to the CLI crate.

mod error;
mod export;
mod generator;
mod parser;
mod signal;
mod tag;
mod opc_config;

pub use error::IntegritreeError;
pub use export::export;
pub use generator::{generate, OpcConfig};
pub use parser::parse_gvl;
pub use signal::{Signal, SignalKind};
pub use tag::{Tag};
pub use opc_config::{parse_opc_connection};
