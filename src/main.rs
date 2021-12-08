// I am a being of chaos and destruction
#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(slice_group_by)]
#![feature(custom_inner_attributes)]
#![feature(proc_macro_hygiene)]

// This is desirable since it implicitly places `aoc` in macro scope and means we don't need to use full paths.
#[macro_use]
extern crate codegen;

// This is desirable for the same reason.
#[macro_use]
pub mod solution;
use app::App;
pub mod app;
mod solutions;

pub mod prelude {
    pub use super::solution::{Day, Part, Solution};
}

#[tokio::main]
async fn main() {
    let result = App::run().await;

    if let Err(ref err) = result {
        eprintln!("{}", err);
    }

    std::process::exit(result.is_err() as i32)
}
