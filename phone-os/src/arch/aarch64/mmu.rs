//! ARM64 MMU (Memory Management Unit) setup
//! 
//! This module provides page table initialization and MMU configuration
//! for ARM64 architecture.

use core::arch::asm;
use core::ptr;

/// Page size (4KB)
pub const PAGE_SIZE: usize = 4096;
/// Page shift amount
pub const PAGE_SHIFT: usize = 12;

/// Translation Table Base Register 0 (TTBR0_EL1) flags
const TTBR0_ASID_SHIFT: u64 = 48;
const TTBR0_BADDR_SHIFT: u64 = 1;

/// MAIR (Memory Attribute Indirection Register) attributes
const MAIR_DEVICE_NGnRnE: u64 = 0x00;
const MAIR_NORMAL_WT_RA_WA: u64 = 0xAA;
const MAIR_NORMAL_WB_RA_WA: u64 = 0xFF;

/// Page table entry flags
pub const PTE_VALID: u64 = 1 << 0;
pub const PTE_TABLE: u64 = 1 << 1;
pub const PTE_PAGE: u64 = 1 << 1;
pub const PTE_BLOCK: u64 = 0 << 1;
pub const PTE_USER: u64 = 1 << 6;
pub const PTE_RDONLY: u64 = 1 << 7;
pub const PTE_AF: u64 = 1 << 10; // Access flag
pub const PTE_SH_INNER: u64 = 3 << 8; // Inner shareable
pub const PTE_SH_OUTER: u64 = 2 << 8; // Outer shareable
pub const PTE_XN: u64 = 1 << 54; // Execute never
pub const PTE_PXN: u64 = 1 << 53; // Privileged execute never

/// Memory attribute indices
const ATTR_INDEX_DEVICE: u64 = 0;
const ATTR_INDEX_NORMAL: u64 = 1;

/// Number of entries per level
const ENTRY_COUNT: usize = 512;

/// Page table level
#[repr(C, align(4096))]
pub struct PageTable {
    entries: [u64; ENTRY_COUNT],
}

impl PageTable {
    /// Create a new empty page table
    pub const fn new() -> Self {
        PageTable {
            entries: [0; ENTRY_COUNT],
        }
    }

    /// Get a mutable reference to an entry
    #[inline]
    pub fn get_entry(&self, index: usize) -> u64 {
        if index < ENTRY_COUNT {
            self.entries[index]
        } else {
            0
        }
    }

    /// Set an entry
    #[inline]
    pub fn set_entry(&mut self, index: usize, value: u64) {
        if index < ENTRY_COUNT {
            self.entries[index] = value;
        }
    }

    /// Map a physical address to a virtual address
    /// 
    /// # Arguments
    /// * `virt_addr` - Virtual address to map
    /// * `phys_addr` - Physical address to map to
    /// * `flags` - Page table entry flags
    pub fn map_block(&mut self, virt_addr: u64, phys_addr: u64, flags: u64) {
        let index = ((virt_addr >> 39) & 0x1FF) as usize; // Level 1 index for 1GB blocks
        let entry = (phys_addr & 0x0000_FFFF_FFC0_0000) | flags | PTE_VALID;
        self.set_entry(index, entry);
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        for i in 0..ENTRY_COUNT {
            self.entries[i] = 0;
        }
    }
}

/// Initialize the MMU
/// 
/// # Arguments
/// * `dtb_ptr` - Pointer to device tree blob (optional)
/// 
/// # Safety
/// This function modifies system registers and should only be called once
pub unsafe fn init_mmu(_dtb_ptr: Option<*const u8>) {
    // Create identity mapping for kernel space
    let mut l1_table = PageTable::new();
    
    // Map first 1GB as normal memory (kernel code/data)
    // Attributes: Normal, Write-Back, Read/Write, Execute allowed
    let kernel_flags = PTE_BLOCK | PTE_AF | PTE_SH_INNER | 
                       ((ATTR_INDEX_NORMAL & 0x7) << 2) | PTE_XN;
    
    for i in 0..2 {
        let virt_addr = (i * 0x40000000u64) as u64; // 1GB blocks
        let phys_addr = virt_addr; // Identity mapping
        l1_table.map_block(virt_addr, phys_addr, kernel_flags);
    }
    
    // Map device memory (MMIO regions)
    let device_flags = PTE_BLOCK | PTE_AF | PTE_SH_OUTER | 
                       ((ATTR_INDEX_DEVICE & 0x7) << 2) | PTE_XN;
    
    // Map typical device regions (will be refined with DT parsing)
    l1_table.map_block(0x10000000, 0x10000000, device_flags);
    l1_table.map_block(0x20000000, 0x20000000, device_flags);

    // Set up MAIR (Memory Attribute Indirection Register)
    let mair_value = (MAIR_DEVICE_NGnRnE << (ATTR_INDEX_DEVICE * 8)) |
                     (MAIR_NORMAL_WB_RA_WA << (ATTR_INDEX_NORMAL * 8));
    
    asm!(
        "msr mair_el1, {}",
        in(reg) mair_value,
        options(nomem, nostack)
    );

    // Set up TCR (Translation Control Register)
    // 48-bit IPA, 4KB pages, 1-level translation
    let tcr_value = (48u64 << 32) | // IPS: 48-bit physical address
                    (16u64 << 16) | // TG0: 4KB granule
                    (1u64 << 14) |  // SH0: Inner shareable
                    (3u64 << 12) |  // ORGN0: Normal memory, Write-Back
                    (3u64 << 10) |  // IRGN0: Normal memory, Write-Back
                    (0u64 << 8) |   // EPD0: Use TTBR0
                    (35u64 << 0);   // T0SZ: 48-bit address space (64-35=29 bits offset)
    
    asm!(
        "msr tcr_el1, {}",
        in(reg) tcr_value,
        options(nomem, nostack)
    );

    // Load TTBR0_EL1 with our page table
    let ttbr0_value = ((&l1_table as *const PageTable as u64) & !0xFFF) | 
                      ((0u64 & 0xFFFF) << TTBR0_ASID_SHIFT);
    
    asm!(
        "msr ttbr0_el1, {}",
        in(reg) ttbr0_value,
        options(nomem, nostack)
    );

    // Invalidate TLB
    asm!(
        "tlbi vmalle1is",
        "dsb sy",
        "isb",
        options(nostack)
    );

    // Enable MMU
    let sctlr_value: u64;
    asm!(
        "mrs {}, sctlr_el1",
        out(reg) sctlr_value,
        options(nomem, nostack)
    );
    
    let sctlr_value = sctlr_value | (1 << 0) | (1 << 2) | (1 << 12); // M, C, I bits
    
    asm!(
        "msr sctlr_el1, {}",
        in(reg) sctlr_value,
        options(nostack, nomem)
    );
    
    // Instruction synchronization barrier
    asm!("isb", options(nomem, nostack));
}

/// Flush the TLB
#[inline]
pub unsafe fn flush_tlb() {
    asm!(
        "tlbi vmalle1is",
        "dsb sy",
        "isb",
        options(nostack)
    );
}

/// Invalidate instruction cache
#[inline]
pub unsafe fn invalidate_icache() {
    asm!(
        "ic iallu",
        "dsb sy",
        "isb",
        options(nostack)
    );
}
