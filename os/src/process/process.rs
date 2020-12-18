use crate::memory::*;
use crate::process::*;

pub type ProcessID = isize;

static mut PROCESS_COUNTER: ProcessID = 0;

#[derive(PartialEq)]
pub struct Process {
    pub id: ProcessID,
    pub stack: Range<VirtualAddress>,
    pub is_user: bool,
    pub context_ptr: usize,
    pub memory_set: MemorySet,
    pub state: ProcessStatus,
}

impl Process {
    pub fn new(elf_data: &[u8]) -> Self {
        let (mut memory_set, enter_point) = MemorySet::from_elf(elf_data, true);
        let stack = Process::alloc_page_range(&mut memory_set, USER_STACK_SIZE, Flags::READABLE | Flags::WRITABLE);
        let context = Context::new(stack.end.into(), enter_point, true);
        let mut context_ptr: usize = 0;
        unsafe {
            context_ptr = KERNEL_STACK[PROCESS_COUNTER as usize].push_context(context) as * const _ as usize;
        }
        Process {
            id: unsafe {
                PROCESS_COUNTER += 1;
                PROCESS_COUNTER
            },
            stack,
            is_user: true,
            context_ptr,
            memory_set,
            state: ProcessStatus::Ready,
        }
    }

    pub fn prepare(&self) -> usize {
        self.memory_set.activate();
        self.context_ptr
    }

    pub fn set_state(&mut self, state: ProcessStatus) {
        self.state = state;
    }


    /// 分配一定数量的连续虚拟空间
    ///
    /// 从 `memory_set` 中找到一段给定长度的未占用虚拟地址空间，分配物理页面并建立映射。返回对应的页面区间。
    ///
    /// `flags` 只需包括 rwx 权限，user 位会根据进程而定。
    pub fn alloc_page_range(
        memory_set: &mut MemorySet,
        size: usize,
        flags: Flags,
    ) -> Range<VirtualAddress> {
        // memory_set 只能按页分配，所以让 size 向上取整页
        let alloc_size = (size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
        // 从 memory_set 中找一段不会发生重叠的空间
        let mut range = Range::<VirtualAddress>::from(0x1000000..0x1000000 + alloc_size);
        while memory_set.overlap_with(range.into()) {
            range.start += alloc_size;
            range.end += alloc_size;
        }
        // 分配物理页面，建立映射
        memory_set.add_segment(
            Segment {
                map_type: MapType::Framed,
                range,
                flags: flags | Flags::user(true),
            },
            None,
        );
        // 返回地址区间（使用参数 size，而非向上取整的 alloc_size）
        Range::from(range.start..(range.start + size))
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum ProcessStatus {
    Running,
    Waiting,
    Ready,
    Exited,
}