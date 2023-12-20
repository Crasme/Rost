use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

use super::general::KILO_BYTE;
use crate::println;

pub struct MemoryAllocator;

#[derive(Debug, Copy, Clone, PartialEq)]
enum BlockState {
    Free,
    Used,
}

const BLOCK_SIZE: usize = KILO_BYTE * 4;
const BLOCK_NUMBER: usize = 1024;
const MEM_SIZE: usize = BLOCK_SIZE * BLOCK_NUMBER;
static mut MEMORY: [u8; MEM_SIZE] = [0; MEM_SIZE];
static mut BLOCKS: [BlockState; BLOCK_NUMBER] = [BlockState::Free; BLOCK_NUMBER];

unsafe impl GlobalAlloc for MemoryAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // we want contiguous memory
        let nb_blocks = layout.size() / BLOCK_SIZE + 1;
        let mut start_block = 0;
        let mut nb_free_blocks = 0;
        for (i, block) in BLOCKS.iter().enumerate() {
            nb_free_blocks += 1;
            if *block == BlockState::Free {
                if nb_free_blocks == nb_blocks {
                    start_block = i + 1 - nb_free_blocks;
                    break;
                }
            } else {
                nb_free_blocks = 0;
            }
        }
        if nb_free_blocks == nb_blocks {
            for block in BLOCKS.iter_mut().skip(start_block).take(nb_blocks) {
                *block = BlockState::Used
            }
            return &mut MEMORY[start_block * BLOCK_SIZE];
        }
        null_mut()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // we look for the block corresponding to the ptr
        let mut start_block = 0;
        for (i, _block) in BLOCKS.iter().enumerate() {
            if MEMORY[i * BLOCK_SIZE] == *ptr {
                start_block = i;
                break;
            }
        }
        // we free the size of the layout
        let nb_blocks = layout.size() / BLOCK_SIZE + 1;
        for item in BLOCKS.iter_mut().skip(start_block).take(nb_blocks) {
            *item = BlockState::Free;
        }
    }
}

#[global_allocator]
static ALLOCATOR: MemoryAllocator = MemoryAllocator;
