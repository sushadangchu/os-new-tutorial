pub mod config;
pub mod heap;
pub mod address;
pub mod frame_allocator;
pub mod frame_tracker;
pub mod range;
pub mod mapping;
pub mod memory_set;
pub mod segment;
pub mod page_table;

pub use frame_allocator::FRAME_ALLOCATOR;
pub use frame_tracker::FrameTracker;

pub use config::*;
pub use heap::*;
pub use address::*;
pub use range::*;
pub use mapping::*;
pub use memory_set::*;
pub use segment::*;
pub use page_table::*;

pub type MemoryResult<T> = Result<T, &'static str>;

pub fn init() {
    heap::init();

    unsafe { riscv::register::sstatus::set_sum() };

    println!("[K] mod memory initialized");
}