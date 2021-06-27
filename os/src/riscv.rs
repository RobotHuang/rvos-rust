use crate::config::*;

#[inline(always)]
pub fn w_mscratch(x: RegT) {
    unsafe {
        llvm_asm!("csrw mscratch, $0"
            :
            : "r" (x)
            :
            : "volatile"
        );
    }
}

#[inline(always)]
pub fn w_mtvec(x: RegT) {
    unsafe {
        llvm_asm!("csrw mtvec, $0"
            :
            : "r"(x)
            :
            : "volatile"
        )
    }
}

#[inline(always)]
pub fn r_tp() -> RegT {
    let x: RegT;
    unsafe {
        llvm_asm!("mv $0, tp"
            : "=r"(x)
            :
            :
            : "volatile"
        )
    }
    x
}

#[inline(always)]
pub fn w_mie(x: RegT) {
    unsafe {
        llvm_asm!("csrw mie, $0"
            :
            : "r"(x)
            :
            : "volatile"
        )
    }
}

#[inline(always)]
pub fn r_mie() -> RegT {
    let x: RegT;
    unsafe {
        llvm_asm!("csrr $0, mie" : "=r" (x) : : : "volatile");
    }
    return x;
}

#[inline(always)]
pub fn w_mstatus(x: RegT) {
    unsafe {
        llvm_asm!("csrw mstatus, $0" : : "r" (x));
    }
}

#[inline(always)]
pub fn r_mstatus() -> RegT {
    let x: RegT;
    unsafe {
        llvm_asm!("csrr $0, mstatus" : "=r" (x) );
    }
    x
}

#[inline(always)]
pub fn r_mhartid() -> RegT {
    let x: RegT;
    unsafe {
        llvm_asm!("csrr $0, mhartid": "=r" (x));
    }
    x
}