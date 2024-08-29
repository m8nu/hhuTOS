/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: bump                                                            ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Imnplementing a basic heap allocator which cannot use           ║
   ║         deallocated memory. Thus it is only for learning and testing.   ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author: Philipp Oppermann                                               ║
   ║         https://os.phil-opp.com/allocator-designs/                      ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
use super::{align_up, Locked};
use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr;


/**
 Description: Metadata of the bump allocator
*/
pub struct BumpAllocator {
   heap_start: usize,
   heap_end: usize,
   next: usize,
   allocations: usize,
}

impl BumpAllocator {
   // Creates a new empty bump allocator.
   pub const fn new() -> Self {

      /* Hier muss Code eingefuegt werden */
      BumpAllocator {
         heap_start: 0,
         heap_end: 0,
         next: 0,
         allocations: 0,
     }

   }

   // Initialize the allocator with the given heap bounds.
   //
   // This function is unsafe because the caller must guarantee that 
   // the given heap bounds are valid. This method must be called only once.
   pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {

      /* Hier muss Code eingefuegt werden */
      self.heap_start = heap_start;
      self.heap_end = heap_start + heap_size;
      self.next = heap_start;

   }

   // Dump free list
   pub fn dump_free_list(&mut self) {

      /* Hier muss Code eingefuegt werden */
      kprintln!("BumpAllocator: heap_start={:#x}, heap_end={:#x}, next={:#x}, allocations={}", self.heap_start, self.heap_end, self.next, self.allocations);
   }
	
   pub unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {

      /* Hier muss Code eingefuegt werden */
      let alloc_start = self.next;
      self.next = alloc_start + layout.size();
      self.allocations += 1;
      alloc_start as *mut u8

   }
   	
   pub unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
      kprintln!("bump-dealloc: size={}, align={}; not supported", layout.size(), layout.align());
   }

}

// Trait required by the Rust runtime for heap allocations
unsafe impl GlobalAlloc for Locked<BumpAllocator> {
	
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.lock().alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.lock().dealloc(ptr, layout);
    }
}
