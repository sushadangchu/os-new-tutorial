mod handler;
mod context;

pub use context::Context;

pub fn init() {
    handler::init();
    println!("[S] mod interrupt init");
}