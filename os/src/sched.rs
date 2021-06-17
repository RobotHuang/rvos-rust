#![allow(unused)]
#![feature(llvm_asm)]

use crate::config::STACK_SIZE;

extern "C" {
    fn switch_to(next: *const context);
}

type RegT = u32;

#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
/* task management */
pub struct context {
    /* ignore x0 */
    ra: RegT,
    sp: RegT,
    gp: RegT,
    tp: RegT,
    t0: RegT,
    t1: RegT,
    t2: RegT,
    s0: RegT,
    s1: RegT,
    a0: RegT,
    a1: RegT,
    a2: RegT,
    a3: RegT,
    a4: RegT,
    a5: RegT,
    a6: RegT,
    a7: RegT,
    s2: RegT,
    s3: RegT,
    s4: RegT,
    s5: RegT,
    s6: RegT,
    s7: RegT,
    s8: RegT,
    s9: RegT,
    s10: RegT,
    s11: RegT,
    t3: RegT,
    t4: RegT,
    t5: RegT,
    t6: RegT,
}

impl context {
    pub fn new() -> context {
        context {
            ra: 0,
            sp: 0,
            gp: 0,
            tp: 0,
            t0: 0,
            t1: 0,
            t2: 0,
            s0: 0,
            s1: 0,
            a0: 0,
            a1: 0,
            a2: 0,
            a3: 0,
            a4: 0,
            a5: 0,
            a6: 0,
            a7: 0,
            s2: 0,
            s3: 0,
            s4: 0,
            s5: 0,
            s6: 0,
            s7: 0,
            s8: 0,
            s9: 0,
            s10: 0,
            s11: 0,
            t3: 0,
            t4: 0,
            t5: 0,
            t6: 0,
        }
    }
}

fn w_mscratch(x: RegT) {
    unsafe {
        llvm_asm!("csrw mscratch, $0"
            : 
            : "r" (x)
            :
            : "volatile"
        );
    }
}

pub fn sched_init(ctx_task: &mut context, task_stack: &mut [u8]) {
    w_mscratch(0);

    ctx_task.sp = task_stack[STACK_SIZE - 1] as RegT;
    user_task0();
    ctx_task.sp = user_task0 as fn() as RegT;
}

pub fn schedule(ctx_task: &context) {
    let next = ctx_task as *const context;
    unsafe {
        switch_to(next);
    }
}

fn task_delay(mut count: i32) {
    count = count * 5000;
    while count > 0 {
        count -= 1;
    }
}

fn user_task0() {
    use crate::uart::uart_puts;
    uart_puts(b"Task 0: Created!\n");
    loop {
        uart_puts(b"Task 0: Running...\n");
        task_delay(100000000);
    }
}