use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

use super::general::KILO_BYTE;
use crate::println;

pub struct MemoryAllocator;

const DISK_SIZE: usize = KILO_BYTE * 1024;
static mut MEM: [u8; DISK_SIZE] = [0; DISK_SIZE];
const MAX_ALLOCS: usize = 128;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Alloc {
    ptr: *mut u8,
    size: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MemoryLayout {
    allocs: [core::option::Option<Alloc>; MAX_ALLOCS],
    nb_allocs: usize,
}

static mut MEMLAYOUT: MemoryLayout = MemoryLayout {
    allocs: [None; MAX_ALLOCS],
    nb_allocs: 0,
};

// TODO : make better
unsafe impl GlobalAlloc for MemoryAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut ptr = MEM.as_ptr() as *mut u8;
        for t_alloc in MEMLAYOUT.allocs {
            let alloc = t_alloc.unwrap_or( Alloc {
                ptr: 0 as *mut u8,
                size: 0,
            });
            if alloc.ptr >= ptr {
                ptr = alloc.ptr.wrapping_add(alloc.size);
            }
        }

        let new_alloc = Alloc {
            ptr: ptr,
            size: layout.size(),
        };

        if MEMLAYOUT.nb_allocs >= MAX_ALLOCS {
            panic!("Too much stuff allocated !");
        }

        MEMLAYOUT.allocs[MEMLAYOUT.nb_allocs] = Some(new_alloc);
        MEMLAYOUT.nb_allocs += 1;

        ptr
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // panic!("dealloc should be never called")
    }
}

#[global_allocator]
static ALLOCATOR: MemoryAllocator = MemoryAllocator;
