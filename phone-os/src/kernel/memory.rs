//! Simple memory allocator for the kernel
//! 
//! This module provides a basic bump allocator for early boot memory allocation.
//! In a production system, this would be replaced with a more sophisticated allocator.

use core::ptr;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Default heap size (16MB)
const HEAP_SIZE: usize = 16 * 1024 * 1024;
/// Default heap base address (will be set during initialization)
static mut HEAP_BASE: usize = 0x90000000;

/// Atomic bump allocator state
static ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static mut HEAP_END: usize = 0;

/// Initialize the heap allocator
/// 
/// # Arguments
/// * `base` - Base address of the heap region
/// * `size` - Size of the heap region in bytes
/// 
/// # Safety
/// This function must be called before any allocations and only once
pub unsafe fn init_heap(base: usize, size: usize) {
    HEAP_BASE = base;
    HEAP_END = base + size;
    ALLOCATED.store(0, Ordering::Relaxed);
}

/// Allocate memory from the heap
/// 
/// # Arguments
/// * `size` - Number of bytes to allocate
/// * `align` - Alignment requirement (must be power of 2)
/// 
/// # Returns
/// Pointer to allocated memory, or null if allocation failed
/// 
/// # Safety
/// This function is unsafe because it returns a raw pointer
pub unsafe fn allocate(size: usize, align: usize) -> *mut u8 {
    // Ensure alignment is at least pointer size
    let align = align.max(core::mem::size_of::<usize>());
    
    // Get current allocation offset
    let mut current = ALLOCATED.load(Ordering::Acquire);
    
    loop {
        // Calculate aligned address
        let start = HEAP_BASE + current;
        let aligned_start = (start + align - 1) & !(align - 1);
        let padding = aligned_start - start;
        let new_allocated = current + padding + size;
        
        // Check if we have enough space
        if HEAP_BASE + new_allocated > HEAP_END {
            return ptr::null_mut(); // Out of memory
        }
        
        // Try to claim this space atomically
        match ALLOCATED.compare_exchange_weak(
            current,
            new_allocated,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => {
                return (aligned_start as *mut u8);
            }
            Err(val) => {
                current = val; // Retry with updated value
            }
        }
    }
}

/// Deallocate memory (no-op for bump allocator)
/// 
/// # Arguments
/// * `ptr` - Pointer to deallocate
/// * `size` - Size of the allocation
/// * `align` - Alignment of the allocation
/// 
/// # Note
/// This bump allocator does not support deallocation.
/// Memory is only freed when the allocator is reset.
pub unsafe fn deallocate(_ptr: *mut u8, _size: usize, _align: usize) {
    // Bump allocators don't support individual deallocations
    // In a real implementation, use a proper allocator like linked-list or slab
}

/// Get the amount of allocated memory
pub fn allocated_bytes() -> usize {
    ALLOCATED.load(Ordering::Relaxed)
}

/// Get the amount of free memory
pub fn free_bytes() -> usize {
    unsafe {
        if HEAP_END == 0 {
            return 0;
        }
        HEAP_END - HEAP_BASE - ALLOCATED.load(Ordering::Relaxed)
    }
}

/// Reset the allocator (frees all memory)
/// 
/// # Safety
/// This invalidates all previously allocated pointers
pub unsafe fn reset() {
    ALLOCATED.store(0, Ordering::Relaxed);
}

/// Test allocation functionality
pub fn test_allocator() -> bool {
    unsafe {
        // Initialize a small test heap
        static mut TEST_HEAP: [u8; 4096] = [0; 4096];
        let heap_base = TEST_HEAP.as_ptr() as usize;
        
        init_heap(heap_base, 4096);
        
        // Test simple allocation
        let ptr1 = allocate(64, 8);
        if ptr1.is_null() {
            return false;
        }
        
        // Test aligned allocation
        let ptr2 = allocate(128, 64);
        if ptr2.is_null() {
            return false;
        }
        
        // Verify alignment
        if (ptr2 as usize) % 64 != 0 {
            return false;
        }
        
        // Reset for normal operation
        reset();
    }
    true
}

/// Global allocator implementation for #[global_allocator]
/// 
/// To use this, add to your main.rs:
/// ```rust
/// #[global_allocator]
/// static GLOBAL: kernel::memory::GlobalAllocator = kernel::memory::GlobalAllocator;
/// ```
pub struct GlobalAllocator;

unsafe impl core::alloc::GlobalAlloc for GlobalAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        allocate(layout.size(), layout.align())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        deallocate(ptr, layout.size(), layout.align());
    }
}
