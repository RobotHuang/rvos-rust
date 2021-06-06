#![allow(unused)]

const UART0: u64 = 0x10000000;

const RHR: u8 = 0; // Receive Holding Register (read mode)
const THR: u8 = 0; // Transmit Holding Register (write mode)
const DLL: u8 = 0; // LSB of Divisor Latch (write mode)
const IER: u8 = 1; // Interrupt Enable Register (write mode)
const DLM: u8 = 1; // MSB of Divisor Latch (write mode)
const FCR: u8 = 2; // FIFO Control Register (write mode)
const ISR: u8 = 2; // Interrupt Status Register (read mode)
const LCR: u8 = 3; // Line Control Register
const MCR: u8 = 4; // Modem Control Register
const LSR: u8 = 5; // Line Status Register
const MSR: u8 = 6; // Modem Status Register
const SPR: u8 = 7; // ScratchPad Register

const LSR_RX_READY: u8 = 1 << 0;
const LSR_TX_IDLE: u8 = 1 << 5;

fn uart_reg(reg: u8) -> *mut u8 {
    (UART0 + reg as u64) as *mut u8
}

fn uart_read_reg(reg: u8) -> u8 {
    unsafe { *uart_reg(reg) }
}

fn uart_write_reg(reg: u8, v: u8) {
    unsafe {
        *uart_reg(reg) = v;
    }
}

pub fn uart_init() {
    /* disable interrupts. */
    uart_write_reg(IER, 0x00);

    /*
     * Setting baud rate. Just a demo here if we care about the divisor,
     * but for our purpose [QEMU-virt], this doesn't really do anything.
     *
     * Notice that the divisor register DLL (divisor latch least) and DLM (divisor
     * latch most) have the same base address as the receiver/transmitter and the
     * interrupt enable register. To change what the base address points to, we
     * open the "divisor latch" by writing 1 into the Divisor Latch Access Bit
     * (DLAB), which is bit index 7 of the Line Control Register (LCR).
     *
     * Regarding the baud rate value, see [1] "BAUD RATE GENERATOR PROGRAMMING TABLE".
     * We use 38.4K when 1.8432 MHZ crystal, so the corresponding value is 3.
     * And due to the divisor register is two bytes (16 bits), so we need to
     * split the value of 3(0x0003) into two bytes, DLL stores the low byte,
     * DLM stores the high byte.
     */
    let mut lcr = uart_read_reg(LCR);
    uart_write_reg(LCR, lcr | (1 << 7));
    uart_write_reg(DLL, 0x03);
    uart_write_reg(DLM, 0x00);

    /*
     * Continue setting the asynchronous data communication format.
     * - number of the word length: 8 bits
     * - number of stop bits：1 bit when word length is 8 bits
     * - no parity
     * - no break control
     * - disabled baud latch
     */
    lcr = 0;
    uart_write_reg(LCR, lcr | (3 << 0));
}

pub extern "C" fn uart_putc(ch: u8) {
    while (uart_read_reg(LSR) & LSR_TX_IDLE) == 0 {}
    uart_write_reg(THR, ch);
}

pub fn uart_puts(s: &[u8]) {
    for c in s {
        uart_putc(*c);
    }
}
