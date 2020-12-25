use crate::memory::*;
use crate::process::*;
use core::mem::size_of;
use spin::Mutex;
use super::*;
use alloc::sync::Arc;
use alloc::vec::Vec;

pub type ProcessID = isize;

static mut PROCESS_COUNTER: ProcessID = 0;

pub struct Process {
    pub id: ProcessID,
    pub is_user: bool,
    pub inner: Mutex<ProcessInner>,
}

pub struct ProcessInner {
    pub stack: Range<VirtualAddress>,
    pub context_ptr: usize,
    pub memory_set: MemorySet,
    pub state: ProcessStatus,
    pub child: Vec<isize>,
}

impl Process {
    pub fn new(elf_data: &[u8]) -> Self {
        let (mut memory_set, enter_point) = MemorySet::from_elf(elf_data, true);
        let stack = Process::alloc_page_range(&mut memory_set, USER_STACK_SIZE, Flags::READABLE | Flags::WRITABLE);
        let context = Context::new(stack.end.into(), enter_point, true);
        
        Process {
            id: unsafe {
                PROCESS_COUNTER += 1;
                PROCESS_COUNTER
            },
            is_user: true,
            inner: Mutex::new(ProcessInner {
                stack,
                context_ptr: unsafe {
                    KERNEL_STACK[(PROCESS_COUNTER - 1) as usize].push_context(context) as * const _ as usize
                },
                memory_set,
                state: ProcessStatus::Ready,
                child: Vec::new(),
            }),
        }
    }

    pub fn fork(self: &Arc<Process>) -> Arc<Process> {
        let memory_set = MemorySet::copy_memory_set(&self.inner().memory_set);
        let stack = self.inner().stack.clone();
        let index = self.id - 1;
        
        for i in 0..KERNEL_STACK_SIZE {
            unsafe {
                KERNEL_STACK[PROCESS_COUNTER as usize].stack[i] = KERNEL_STACK[index as usize].stack[i];
            }
        }

        let mut context_ptr = 0;

        unsafe {
            let stack_top = &KERNEL_STACK[PROCESS_COUNTER as usize].stack as *const _ as usize + size_of::<KernelStack>();
            
            context_ptr = stack_top - size_of::<Context>();
        }

        unsafe {
            self.inner().child.push(PROCESS_COUNTER + 1);
        }

        Arc::new(Process {
            id: unsafe {
                PROCESS_COUNTER += 1;
                PROCESS_COUNTER
            },
            is_user: true,
            inner: Mutex::new(ProcessInner {
                stack,
                context_ptr,
                memory_set,
                state: ProcessStatus::Ready,
                child: Vec::new(),
            }),
        })
    }

    pub fn exec(&self, elf_data: &[u8]) {
        let (mut memory_set, enter_point) = MemorySet::from_elf(elf_data, true);
        let stack = Process::alloc_page_range(&mut memory_set, USER_STACK_SIZE, Flags::READABLE | Flags::WRITABLE);
        let context = Context::new(stack.end.into(), enter_point, true);
        self.inner().context_ptr = unsafe {
            KERNEL_STACK[(self.id - 1) as usize].push_context(context) as * const _ as usize
        };
        memory_set.activate();
        self.inner().stack = stack;
        self.inner().memory_set = memory_set;
        self.inner().child = Vec::new();
    }

    pub fn get_context(&self) -> *mut Context {
        unsafe {
            self.inner().context_ptr as *mut Context
        }
    }

    pub fn prepare(&self) -> usize {
        self.inner().memory_set.activate();
        self.inner().context_ptr
    }

    pub fn set_state(&mut self, state: ProcessStatus) {
        self.inner().state = state;
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

    pub fn get_pid(&self) -> isize {
        self.id
    }

    pub fn inner(&self) -> spin::MutexGuard<ProcessInner> {
        self.inner.lock()
    }
}

impl PartialEq for Process {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Process {}

#[derive(Copy, Clone, PartialEq)]
pub enum ProcessStatus {
    Running,
    Waiting,
    Ready,
    Exited,
}