mod handler;
mod context;
mod timer;

pub use context::Context;
pub use timer::*;

pub fn init() {
    handler::init();
    timer::init();
    println!("[K] mod interrupt init");
}