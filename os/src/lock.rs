use crate::riscv::*;

pub fn spin_lock() -> i32 {
    w_mstatus(r_mstatus() & !(1 << 3));
    0
}

pub fn spin_unlock() -> i32 {
    w_mstatus(r_mstatus() | (1 << 3));
    0
}