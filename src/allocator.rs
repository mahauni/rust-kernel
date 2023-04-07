use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr
};
// use linked_list_allocator::LockedHeap;
// use bump::BumpAllocator;
// use linked_list::LinkedListAllocator;
use fixed_size_block::FixedSizeBlockAllocator;

pub mod bump;
pub mod linked_list;
pub mod fixed_size_block;

// If we need more heap space in the future just change the HEAP_END
// to a bigger number.
pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush()
        };
    }

    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}

// Imported crate heap allocation with linked_list_allocator
// #[global_allocator]
// static ALLOCATOR: LockedHeap = LockedHeap::empty();

// Bump allocator implementation
// Has a problem with long lived heap values. It can be fixed, but only for a problem and not 
// the whole. So this is a use case specific.
// #[global_allocator]
// static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());

// Linked List Allocator implementation 
// problem with this allocation is without a agrouping of heap, so in a future, the heap will be 
// so fragmented that it will not be able to allocate anything more. The imported crate has a 
// implementation to reagroup and dont have this issue
// #[global_allocator]
// static ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new());

#[global_allocator]
static ALLOCATOR: Locked<FixedSizeBlockAllocator> = Locked::new(FixedSizeBlockAllocator::new());

// A wrapper around spin::Mutex to permit trait implementations
pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

// slower version of align_up
// fn align_up(addr: usize, align: usize) -> usize {
//     let remainder = addr % align;
//     if remainder == 0 {
//         addr
//     } else {
//         addr - remainder + align
//     }
// }

// Align the given address 'addr' upwards to alignment 'align'
//
// Requires that 'align' is a power of two.
fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}
