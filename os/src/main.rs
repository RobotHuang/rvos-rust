#![no_std]
#![no_main]
#![feature(global_asm)]

use core::panic::PanicInfo;

global_asm!(include_str!("start.S"));

/// 当 panic 发生时会调用该函数
/// 我们暂时将它的实现为一个死循环
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

mod uart;

#[no_mangle]
pub extern "C" fn start_kernel() -> ! {
    uart::uart_init();
    let s = b"Hello RVOS!\n";
    let mut i = 0;
    while i != 12 {
        uart::uart_putc(s[i]);
        i += 1;
    }
    loop {}
}
