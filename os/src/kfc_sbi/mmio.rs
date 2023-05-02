use crate::mm::{Page, VPRange};

const VIRT_MMIO: VPRange = VPRange {
    start: Page(0x10_0000),
    end: Page(0x10_0000 + 0x2000),
};

const UART_MMIO: VPRange = VPRange {
    start: Page(0x1000_0000),
    end: Page(0x1000_0000 + 0x1000),
};

pub const MMIO: [VPRange; 2] = [VIRT_MMIO, UART_MMIO];
