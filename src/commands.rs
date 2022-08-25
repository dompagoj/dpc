use clap::{Parser, Subcommand};

mod find_const_funcs;

pub use find_const_funcs::*;

pub trait CommandRunner<T> {
    fn execute(&self) -> anyhow::Result<T>;
}
