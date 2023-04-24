// 16550 simulated by qemu

#![allow(unused)]
const UART0: *mut u8 = 0x10000000 as *mut u8;
const RHR: usize = 0; // receive holding register
const THR: usize = 0; // transmit holding register
const IER: usize = 1; // interrupt enable register
const IER_RX_ENABLE: usize = 1 << 0; // enable receiver interrupt
const IER_TX_ENABLE: usize = 1 << 1; // enable transmitter interrupt
const FCR: usize = 2; // FIFO control register
const FCR_FIFO_ENABLE: usize = 1 << 0; // enable FIFOs
const FCR_FIFO_CLEAR: usize = 3 << 1; // clear FIFOs
const ISR: usize = 2; // interrupt status register
const LCR: usize = 3; // line control register
const LCR_EIGHT_BITS: usize = 3; // eight bits
const LCR_BAUD_LATCH: usize = 1 << 7; // special mode to set baud rate
const LSR: usize = 5; // line status register
const LSR_RX_READY: usize = 1 << 0; // input is waiting to be read from RHR
const LSR_TX_IDLE: usize = 1 << 5; // THR can accept another character to send

fn read_reg(reg: usize) -> u8 {
    unsafe { UART0.add(reg).read_volatile() }
}

fn write_reg(reg: usize, data: u8) {
    unsafe { UART0.add(reg).write_volatile(data) }
}

pub fn uart_putc_sync(data: u8) {
    while read_reg(LSR) & LSR_TX_IDLE as u8 == 0 {}
    write_reg(THR, data);
}
