use crate::config::*;

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
