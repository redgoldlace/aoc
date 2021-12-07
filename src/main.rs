// I am a being of chaos and destruction
#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(slice_group_by)]

use app::App;
pub mod app;
pub mod solution;
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
