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

mod page;
mod sched;
mod uart;
mod config;

#[no_mangle]
pub extern "C" fn start_kernel() -> ! {
    use config::STACK_SIZE;
    uart::uart_init();
    uart::uart_puts(b"Hello RVOS!\n");
    page::page_init();
    page::page_test();
    let mut ctx_task = sched::context::new();
    let mut task_stack: [u8; STACK_SIZE] = [0; STACK_SIZE];
    sched::sched_init(&mut ctx_task, &mut task_stack);
    loop {}
}
