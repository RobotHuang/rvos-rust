use crate::riscv::*;
use crate::sched::*;
use crate::platform::*;

const TIMER_INTERVAL: i32 = 10000000;

extern "C" {
    fn printf(s: *const u8, ...);
}

static mut _tick: u32 = 0;

pub fn timer_init() {
    unsafe {
        printf(b"timer init\n\0" as *const u8);
    }
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
	    printf(b"tick: %d\n\0" as *const u8, _tick);
    }

    timer_load(TIMER_INTERVAL);
    
    schedule();
}
