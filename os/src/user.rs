use crate::uart::*;
use crate::sched::*;
use crate::lock::*;

fn user_task0() {
    uart_puts(b"Task 0: Created!\n");
    // task_yield();
    // uart_puts(b"Task 0: I'm back!\n");
    // loop {
    //     uart_puts(b"Task 0: Running...\n");
    //     task_delay(1000);
    // }
    loop {
        spin_lock();
	    uart_puts(b"Task 0: Begin ... \n");
	    for _ in 0..5 {
		    uart_puts(b"Task 0: Running... \n");
		    task_delay(1000);
	    }
	    uart_puts(b"Task 0: End ... \n");
	    spin_unlock();
    }
}

fn user_task1() {
    uart_puts(b"Task 1: Created!\n");
    loop {
        uart_puts(b"Task 1: Running...\n");
        task_delay(1000);
    }
}

pub fn os_main() {
    uart_puts(b"os_main...\n");
    task_create(user_task0);
    task_create(user_task1);
    uart_puts(b"os_main finished...\n");
}
