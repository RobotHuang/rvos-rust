use crate::riscv::*;
use crate::config::*;
use crate::uart::*;

extern "C" {
    fn trap_vector();
    fn printf(s: *const u8, ...);
    fn panic(s: *const u8);
}

pub fn trap_init() {
    w_mtvec(trap_vector as RegT);
}

#[no_mangle]
pub extern "C" fn trap_handler(epc: RegT, cause: RegT) -> RegT {
    let return_pc = epc;
    let cause_code = cause & 0xfff;

    if cause & 0x80000000 > 0 {
        match cause {
            3 => {
                uart_puts(b"software interruption!\n");
            },
            7 => {
                uart_puts(b"timer interruption!\n");
            },
            11 => {
                uart_puts(b"external interruption!\n");
            }
            _ => {
                uart_puts(b"unknown async exception!\n");
            }
        }
    } else {
        unsafe {
            printf(b"Sync exceptions!, code = %d\n\0" as *const u8, cause_code);
		    panic(b"OOPS! What can I do!\0" as *const u8);
        }
    }

    return return_pc;
}

pub fn trap_test() {
    let a = 0x00000000 as *mut u8;
    unsafe {
        *a = 100;
    }
    uart_puts(b"Yeah! I'm return back from trap!\n");
}
