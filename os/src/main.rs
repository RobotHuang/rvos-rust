#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]

use core::panic::PanicInfo;

global_asm!(include_str!("start.S"));

/// 当 panic 发生时会调用该函数
/// 我们暂时将它的实现为一个死循环
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

mod config;
mod page;
mod plic;
mod riscv;
mod sched;
mod trap;
mod uart;
mod timer;
mod platform;
mod lock;
mod user;

#[no_mangle]
pub extern "C" fn start_kernel() -> ! {
    uart::uart_init();
    uart::uart_puts(b"Hello RVOS!\n");
    page::page_init();
    page::page_test();
    trap::trap_init();
    plic::plic_init();
    timer::timer_init();
    sched::sched_init();
    user::os_main();
    sched::schedule();
    loop {}
}
