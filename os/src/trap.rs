use crate::config::*;
use crate::plic::*;
use crate::riscv::*;
use crate::uart::*;
use crate::timer::*;
use crate::platform::*;
use crate::sched::*;

extern "C" {
    fn trap_vector();
    fn printf(s: *const u8, ...);
    fn panic(s: *const u8);
}

pub fn trap_init() {
    w_mtvec(trap_vector as RegT);
}

fn external_interrupt_handler() {
    let irq = plic_claim();
    if irq == UART0_IRQ {
        uart_isr();
    } else {
        unsafe {
            printf(b"unexpected interrupt irq = %d\n\0" as *const u8, irq);
        }
    }
    if irq > 0 {
        plic_complete(irq as i32);
    }
}

#[no_mangle]
pub extern "C" fn trap_handler(epc: RegT, cause: RegT) -> RegT {
    let return_pc = epc;
    let cause_code = cause & 0xfff;

    // interrupt or exception
    if cause & 0x80000000 > 0 {
        uart_puts(b"interruption!\n");
        match cause_code {
            3 => {
                uart_puts(b"software interruption!\n");
                let id = r_mhartid();
                unsafe {
                    *(clint_msip(id as usize) as *mut u32) = 0;
                }
                schedule();
            }
            7 => {
                uart_puts(b"timer interruption!\n");
                timer_handler();
            }
            11 => {
                uart_puts(b"external interruption!\n");
                external_interrupt_handler();
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

// pub fn trap_test() {
//     let a = 0x00000000 as *mut u8;
//     unsafe {
//         *a = 100;
//     }
//     uart_puts(b"Yeah! I'm return back from trap!\n");
// }
