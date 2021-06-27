use crate::riscv::*;

const CLINT_BASE: usize = 0x2000000;
const TIMER_INTERVAL: i32 = 10000000;

extern "C" {
    fn printf(s: *const u8, ...);
}

static mut _tick: u32 = 0;

pub fn timer_init() {
    timer_load(TIMER_INTERVAL);
    w_mie(r_mie() | (1 << 7));
    w_mstatus(r_mstatus() | (1 << 3));
}

fn timer_load(interval: i32) {
    let id = r_mhartid();
    unsafe {
        *(clint_mtimecmp(id as usize) as *mut i32) = *(clint_mtime() as *const i32) + interval;
    }
}

pub fn timer_handler() {
    unsafe {
        _tick += 1;
	    printf(b"tick: %d\n" as *const u8, _tick);
    }

	timer_load(TIMER_INTERVAL);
}

// CLINT_MTIMECMP(hartid) (CLINT_BASE + 0x4000 + 8 * (hartid))
#[inline(always)]
fn clint_mtimecmp(hartid: usize) -> usize {
    CLINT_BASE + 0x4000 + 8 * hartid
}

// CLINT_MTIME (CLINT_BASE + 0xBFF8)
#[inline(always)]
fn clint_mtime() -> usize {
    CLINT_BASE + 0xBFF8
}