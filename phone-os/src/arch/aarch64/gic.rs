//! ARM Generic Interrupt Controller (GIC) driver
//! 
//! This module provides GICv2/GICv3 initialization and interrupt handling
//! for ARM64 platforms.

use core::arch::asm;
use core::ptr;

/// GIC Distributor base address (typical for many ARM platforms)
const GICD_BASE: u64 = 0x08000000;
/// GIC CPU Interface base address
const GICC_BASE: u64 = 0x08010000;

/// Distributor registers
const GICD_CTLR: u64 = 0x000;  // Control register
const GICD_TYPER: u64 = 0x004; // Interrupt Controller Type register
const GICD_IGROUPR: u64 = 0x080; // Interrupt Group registers
const GICD_ISENABLER: u64 = 0x100; // Interrupt Set-Enable registers
const GICD_ICENABLER: u64 = 0x180; // Interrupt Clear-Enable registers
const GICD_ISPENDR: u64 = 0x200;  // Interrupt Set-Pending registers
const GICD_IPRIORITYR: u64 = 0x400; // Interrupt Priority registers
const GICD_ITARGETSR: u64 = 0x800; // Interrupt Target registers
const GICD_SGIR: u64 = 0xF00;     // Software Generated Interrupt register

/// CPU Interface registers
const GICC_CTLR: u64 = 0x000;    // Control register
const GICC_PMR: u64 = 0x004;     // Priority Mask register
const GICC_BPR: u64 = 0x008;     // Binary Point register
const GICC_IAR: u64 = 0x00C;     // Interrupt Acknowledge register
const GICC_EOIR: u64 = 0x010;    // End Of Interrupt register
const GICC_RPR: u64 = 0x014;     // Running Priority register

/// Interrupt types
#[derive(Debug, Clone, Copy)]
pub enum InterruptType {
    PPI, // Private Peripheral Interrupt
    SPI, // Shared Peripheral Interrupt
    SGI, // Software Generated Interrupt
}

/// Interrupt handler function type
pub type IrqHandler = fn(u32);

/// GIC information structure
pub struct GicInfo {
    pub spi_count: u32,
    pub cpu_count: u32,
    pub gic_version: u32,
}

impl GicInfo {
    pub const fn new() -> Self {
        GicInfo {
            spi_count: 0,
            cpu_count: 1,
            gic_version: 2,
        }
    }
}

/// GIC Driver structure
pub struct GicDriver {
    gicd_base: u64,
    gicc_base: u64,
    info: GicInfo,
}

impl GicDriver {
    /// Create a new GIC driver instance
    pub const fn new(gicd_base: u64, gicc_base: u64) -> Self {
        GicDriver {
            gicd_base,
            gicc_base,
            info: GicInfo::new(),
        }
    }

    /// Initialize the GIC distributor
    /// 
    /// # Safety
    /// This function accesses hardware registers directly
    pub unsafe fn init_distributor(&mut self) {
        let gicd_ptr = self.gicd_base as *mut u32;

        // Disable the distributor first
        ptr::write_volatile(gicd_ptr.add((GICD_CTLR / 4) as usize), 0u32);

        // Read interrupt controller type register
        let typer = ptr::read_volatile(gicd_ptr.add((GICD_TYPER / 4) as usize));
        self.info.spi_count = ((typer & 0x1F) + 1) * 32;
        self.info.cpu_count = ((typer >> 5) & 0x7) + 1;

        // Set all interrupts to Group 0 (secure)
        let num_registers = ((self.info.spi_count + 31) / 32) as usize;
        for i in 0..num_registers {
            ptr::write_volatile(
                gicd_ptr.add(((GICD_IGROUPR + (i as u64) * 4) / 4) as usize),
                0xFFFFFFFF,
            );
        }

        // Disable all SPIs
        for i in 0..num_registers {
            ptr::write_volatile(
                gicd_ptr.add(((GICD_ICENABLER + (i as u64) * 4) / 4) as usize),
                0xFFFFFFFF,
            );
        }

        // Set priority for all interrupts to lowest (0xFF)
        for i in 32..self.info.spi_count {
            ptr::write_volatile(
                gicd_ptr.add(((GICD_IPRIORITYR + (i as u64)) / 4) as usize),
                0xFFFFFFFF,
            );
        }

        // Enable the distributor
        ptr::write_volatile(
            gicd_ptr.add((GICD_CTLR / 4) as usize),
            1u32, // Enable bit
        );
    }

    /// Initialize the GIC CPU interface
    /// 
    /// # Safety
    /// This function accesses hardware registers directly
    pub unsafe fn init_cpu_interface(&self) {
        let gicc_ptr = self.gicc_base as *mut u32;

        // Set priority mask to allow all priorities
        ptr::write_volatile(gicc_ptr.add((GICC_PMR / 4) as usize), 0xFFu32);

        // Set binary point to 0
        ptr::write_volatile(gicc_ptr.add((GICC_BPR / 4) as usize), 0u32);

        // Enable the CPU interface
        ptr::write_volatile(gicc_ptr.add((GICC_CTLR / 4) as usize), 1u32);
    }

    /// Enable a specific interrupt
    /// 
    /// # Arguments
    /// * `irq` - Interrupt number
    /// 
    /// # Safety
    /// Must ensure IRQ number is valid
    pub unsafe fn enable_interrupt(&self, irq: u32) {
        if irq < 32 {
            return; // Don't enable PPIs/SGIs here
        }

        let gicd_ptr = self.gicd_base as *mut u32;
        let reg_index = (irq / 32) as usize;
        let bit_index = irq % 32;

        ptr::write_volatile(
            gicd_ptr.add(((GICD_ISENABLER + (reg_index as u64) * 4) / 4) as usize),
            1u32 << bit_index,
        );
    }

    /// Send a Software Generated Interrupt (SGI)
    /// 
    /// # Arguments
    /// * `sgi_id` - SGI number (0-15)
    /// * `target_list` - Target CPU list
    /// 
    /// # Safety
    /// This function sends interrupts to other CPUs
    pub unsafe fn send_sgi(&self, sgi_id: u32, target_list: u32) {
        let gicd_ptr = self.gicd_base as *mut u32;
        let sgir_value = (sgi_id & 0xF) | ((target_list & 0xFF) << 16);

        ptr::write_volatile(gicd_ptr.add((GICD_SGIR / 4) as usize), sgir_value);
    }

    /// Acknowledge and get the interrupt ID
    /// 
    /// # Returns
    /// Interrupt ID (0-1023) or 1023 if no interrupt pending
    /// 
    /// # Safety
    /// This function accesses hardware registers
    pub unsafe fn acknowledge_interrupt(&self) -> u32 {
        let gicc_ptr = self.gicc_base as *mut u32;
        ptr::read_volatile(gicc_ptr.add((GICC_IAR / 4) as usize)) & 0x3FF
    }

    /// Signal end of interrupt
    /// 
    /// # Arguments
    /// * `irq_id` - Interrupt ID to acknowledge
    /// 
    /// # Safety
    /// This function accesses hardware registers
    pub unsafe fn end_interrupt(&self, irq_id: u32) {
        let gicc_ptr = self.gicc_base as *mut u32;
        ptr::write_volatile(gicc_ptr.add((GICC_EOIR / 4) as usize), irq_id);
    }

    /// Get GIC information
    pub const fn get_info(&self) -> &GicInfo {
        &self.info
    }
}

/// Global GIC driver instance (for use in interrupt handlers)
static mut GIC: Option<GicDriver> = None;

/// Initialize the GIC system
/// 
/// # Safety
/// This function initializes hardware and should only be called once
pub unsafe fn init_gic() -> GicInfo {
    let mut gic = GicDriver::new(GICD_BASE, GICC_BASE);
    
    gic.init_distributor();
    gic.init_cpu_interface();
    
    GIC = Some(gic);
    
    GIC.as_ref().unwrap().info
}

/// Enable interrupts globally
#[inline]
pub unsafe fn enable_interrupts() {
    asm!(
        "msr daifclr, #2",
        options(nomem, nostack)
    );
}

/// Disable interrupts globally
#[inline]
pub unsafe fn disable_interrupts() {
    asm!(
        "msr daifset, #2",
        options(nomem, nostack)
    );
}

/// Check if interrupts are enabled
#[inline]
pub fn interrupts_enabled() -> bool {
    let daif: u64;
    unsafe {
        asm!(
            "mrs {}, daif",
            out(reg) daif,
            options(nomem, nostack)
        );
    }
    (daif & 0x80) == 0
}

/// Default interrupt handler (weak symbol can be overridden)
#[no_mangle]
pub extern "C" fn default_irq_handler(_irq: u32) {
    // Default empty handler
    // In a real implementation, this would log or handle unexpected interrupts
}

/// Exception vector table entry for IRQ
#[no_mangle]
pub extern "C" fn irq_vector_entry() {
    unsafe {
        if let Some(ref gic) = GIC {
            let irq_id = gic.acknowledge_interrupt();
            
            if irq_id < 1023 {
                // Call the appropriate handler (in real impl, use handler table)
                default_irq_handler(irq_id);
                
                // Signal end of interrupt
                gic.end_interrupt(irq_id);
            }
        }
    }
}
