//! 一个线程中关于内存空间的所有信息 [`MemorySet`]
//!

use super::*;
use alloc::{vec, vec::Vec};
use xmas_elf::{
    program::{SegmentData, Type},
    ElfFile,
};

/// 一个进程所有关于内存空间管理的信息
#[derive(Clone,PartialEq)]
pub struct MemorySet {
    /// 维护页表和映射关系
    pub mapping: Mapping,
    /// 每个字段
    pub segments: Vec<Segment>,
}

impl MemorySet {
    /// 创建内核重映射
    pub fn new_kernel() -> MemoryResult<MemorySet> {
        
        // 在 linker.ld 里面标记的各个字段的起始点，均为 4K 对齐
        extern "C" {
            fn text_start();
            fn rodata_start();
            fn data_start();
            fn bss_start();
        }

        // 建立字段
        let segments = vec![
            // .text 段，r-x
            Segment {
                map_type: MapType::Linear,
                range: Range::from((text_start as usize)..(rodata_start as usize)),
                flags: Flags::READABLE | Flags::EXECUTABLE,
            },
            // .rodata 段，r--
            Segment {
                map_type: MapType::Linear,
                range: Range::from((rodata_start as usize)..(data_start as usize)),
                flags: Flags::READABLE,
            },
            // .data 段，rw-
            Segment {
                map_type: MapType::Linear,
                range: Range::from((data_start as usize)..(bss_start as usize)),
                flags: Flags::READABLE | Flags::WRITABLE,
            },
            // .bss 段，rw-
            Segment {
                map_type: MapType::Linear,
                range: Range::from(VirtualAddress::from(bss_start as usize)..*KERNEL_END_ADDRESS),
                flags: Flags::READABLE | Flags::WRITABLE,
            },
            // 剩余内存空间，rw-
            Segment {
                map_type: MapType::Linear,
                range: Range::from(*KERNEL_END_ADDRESS..VirtualAddress::from(MEMORY_END_ADDRESS)),
                flags: Flags::READABLE | Flags::WRITABLE,
            },
        ];
        let mut mapping = Mapping::new()?;
        
        // 每个字段在页表中进行映射
        for segment in segments.iter() {
            mapping.map(segment, None)?;
        }
        Ok(MemorySet {
            mapping,
            segments,
        })
    }
    
    pub fn from_elf(elf_data: &[u8], is_user: bool) -> (Self, usize) {
        // 建立带有内核映射的 MemorySet
        let mut memory_set = MemorySet::new_kernel().unwrap();

        let elf = ElfFile::new(elf_data).unwrap();

        // 遍历 elf 文件的所有部分
        for program_header in elf.program_iter() {
            if program_header.get_type() != Ok(Type::Load) {
                continue;
            }
            // 从每个字段读取「起始地址」「大小」和「数据」
            let start = VirtualAddress(program_header.virtual_addr() as usize);
            let size = program_header.mem_size() as usize;
            let data: &[u8] =
                if let SegmentData::Undefined(data) = program_header.get_data(&elf).unwrap() {
                    data
                } else {
                    panic!("unsupported elf format");
                };

            // 将每一部分作为 Segment 进行映射
            let segment = Segment {
                map_type: MapType::Framed,
                range: Range::from(start..(start + size)),
                flags: Flags::user(is_user)
                    | Flags::readable(program_header.flags().is_read())
                    | Flags::writable(program_header.flags().is_write())
                    | Flags::executable(program_header.flags().is_execute()),
            };

            // 建立映射并复制数据
            memory_set.add_segment(segment, Some(data)).unwrap();
        }

        (memory_set, elf.header.pt2.entry_point() as usize)
    }
    /// 替换 `satp` 以激活页表
    /// 如果当前页表就是自身，则不会替换，但仍然会刷新 TLB。
    pub fn activate(&self) {
        self.mapping.activate();
    }

    /// 添加一个 [`Segment`] 的内存映射
    pub fn add_segment(&mut self, segment: Segment, init_data: Option<&[u8]>) -> MemoryResult<()> {
        // 检测 segment 没有重合
        assert!(!self.overlap_with(segment.page_range()));
        // 映射
        self.mapping.map(&segment, init_data)?;
        self.segments.push(segment);
        Ok(())
    }

    /// 移除一个 [`Segment`] 的内存映射
    ///
    /// `segment` 必须已经映射
    pub fn remove_segment(&mut self, segment: &Segment) -> MemoryResult<()> {
        // 找到对应的 segment
        let segment_index = self
            .segments
            .iter()
            .position(|s| s == segment)
            .expect("segment to remove cannot be found");
        self.segments.remove(segment_index);
        // 移除映射
        self.mapping.unmap(segment);
        Ok(())
    }

    /// 检测一段内存区域和已有的是否存在重叠区域
    pub fn overlap_with(&self, range: Range<VirtualPageNumber>) -> bool {
        for seg in self.segments.iter() {
            if range.overlap_with(&seg.page_range()) {
                return true;
            }
        }
        false
    }

    pub fn copy_memory_set(current_memory_set: &MemorySet) -> MemorySet {
        let mut memory_set = MemorySet::new_kernel().unwrap();
        for segment in current_memory_set.segments.iter() {
            if segment.map_type == MapType::Framed {
                let new_segment = Segment {
                    map_type: segment.map_type,
                    range: segment.range.clone(),
                    flags: segment.flags,
                };
                memory_set.add_segment(new_segment, None).unwrap();
                let mut vpn_range = Vec::new();
                let mut temp: usize = 0;
                for vp in segment.range.iter() {
                    let vpn = vp.0 / PAGE_SIZE;
                    if vpn != temp {
                        temp = vpn;
                        vpn_range.push(vp);
                    }
                }

                //println!("vpn {}", vpn_range.len());

                // for vpn in segment.range.iter() {
                //     current_memory_set.activate();
                //     let parent_ppn = Mapping::lookup(vpn).unwrap();
                //     memory_set.activate();
                //     let children_ppn = Mapping::lookup(vpn).unwrap();
                //     *children_ppn.deref_kernel::<u8>() = *parent_ppn.deref_kernel::<u8>();
                // }

                
                
                for vpn in vpn_range {
                    //println!("{}", vpn);
                    current_memory_set.activate();
                    let parent_ppn = Mapping::lookup(vpn).unwrap();
                    memory_set.activate();
                    let children_ppn = Mapping::lookup(vpn).unwrap();
                    //println!("parent_ppn {}, children_ppn {}", parent_ppn, children_ppn);
                    //children_ppn.get_bytes_array().copy_from_slice(parent_ppn.get_bytes_array());
                    for i in 0..PAGE_SIZE {
                        *(children_ppn + i).deref_kernel::<u8>() = *(parent_ppn + i).deref_kernel::<u8>();
                    }
                }
            }
        }
        current_memory_set.activate();
        memory_set
    }

}