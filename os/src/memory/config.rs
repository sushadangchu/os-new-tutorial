use lazy_static::*;
use super::address::*;

// 8m
pub const KERNEL_HEAP_SIZE: usize = 0x80_0000;

pub const PAGE_SIZE: usize = 4096;

pub const MEMORY_START_ADDRESS: PhysicalAddress = PhysicalAddress(0x8000_0000);

pub const MEMORY_END_ADDRESS: PhysicalAddress = PhysicalAddress(0x8800_0000);

lazy_static! {
    /// 内核代码结束的地址，即可以用来分配的内存起始地址
    /// 
    /// 因为 Rust 语言限制，我们只能将其作为一个运行时求值的 static 变量，而不能作为 const
    pub static ref KERNEL_END_ADDRESS: VirtualAddress = VirtualAddress(kernel_end as usize); 
}

/// 内核使用线性映射的偏移量
pub const KERNEL_MAP_OFFSET: usize = 0xffff_ffff_0000_0000;

extern "C" {
    fn kernel_end();
}