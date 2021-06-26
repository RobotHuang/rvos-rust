use crate::config::*;
use crate::riscv::*;

const PLIC_BASE: usize = 0x0c000000;

pub fn plic_init() {
    let hart = r_tp();

    let priority_addr = plic_priority(UART0_IRQ) as *mut usize;
    let enable_addr = plic_menable(hart as usize) as *mut usize;
    let threshold_addr = plic_mthreshold(hart as usize) as *mut usize;
    unsafe {
        *priority_addr = 1;
        *enable_addr = 1 << UART0_IRQ;
        *threshold_addr = 0;
    }

    w_mie(r_mie() | (1 << 11));
    w_mstatus(r_mstatus() | (1 << 3));
}

pub fn plic_claim() -> usize {
    let hart = r_tp();
    unsafe { *(plic_mclaim(hart as usize) as *const usize) }
}

pub fn plic_complete(irq: i32) {
    let hart = r_tp();
    unsafe {
        *(plic_mcomplete(hart as usize) as *mut i32) = irq;
    }
}

#[inline(always)]
fn plic_priority(id: usize) -> usize {
    PLIC_BASE + id * 4
}

#[inline(always)]
fn plic_pending(id: usize) -> usize {
    PLIC_BASE + 0x1000 + (id) / 32
}

#[inline(always)]
fn plic_menable(hart: usize) -> usize {
    PLIC_BASE + 0x2000 + hart * 0x80
}

// PLIC_MTHRESHOLD(hart) (PLIC_BASE + 0x200000 + (hart) * 0x1000)
#[inline(always)]
fn plic_mthreshold(hart: usize) -> usize {
    PLIC_BASE + 0x200000 + hart * 0x1000
}

// PLIC_MCLAIM(hart) (PLIC_BASE + 0x200004 + (hart) * 0x1000)
#[inline(always)]
fn plic_mclaim(hart: usize) -> usize {
    PLIC_BASE + 0x200004 + (hart) * 0x1000
}

// PLIC_MCOMPLETE(hart) (PLIC_BASE + 0x200004 + (hart) * 0x1000)
#[inline(always)]
fn plic_mcomplete(hart: usize) -> usize {
    PLIC_BASE + 0x200004 + (hart) * 0x1000
}
