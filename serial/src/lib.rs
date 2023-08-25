// Translated from https://isu-rathnayaka.medium.com/develop-your-own-x86-operating-system-os-4-e8479e150451

#![no_std]

pub const COM1_BASE: u16 = 0x03f8;

pub struct SerialPort {
    base: u16,
}

impl SerialPort {
    pub fn new(base: u16) -> Self {
        Self { base }
    }

    fn configure_baud_rate(&self, divisor: u16) {
        unsafe {
            outb(self.line_command_port(), LINE_ENABLE_DLAB);
            outb(self.data_port(), ((divisor >> 8) & 0x00FF) as _);
            outb(self.data_port(), (divisor & 0x00FF) as _);
        }
    }

    fn configure_line(&self) {
        /* Bit:     | 7 | 6 | 5 4 3 | 2 | 1 0 |
         * Content: | d | b | prty  | s | dl  |
         * Value:   | 0 | 0 | 0 0 0 | 0 | 1 1 | = 0x03
         */
        unsafe {
            outb(self.line_command_port(), 0x03);
        }
    }

    fn configure_fifo_buffer(&self) {
        /* Bit:     | 7 6 | 5  | 4 | 3   | 2   | 1   | 0 |
         * Content: | lvl | bs | r | dma | clt | clr | e |
         * Value:   | 1 1 | 0  | 0 | 0   | 1   | 1   | 1 | = 0xC7
         */
        unsafe {
            outb(self.fifo_command_port(), 0xC7);
        }
    }

    fn configure_modem(&self) {
        /* Bit:     | 7 | 6 | 5  | 4  | 3   | 2   | 1   | 0   |
         * Content: | r | r | af | lb | ao2 | ao1 | rts | dtr |
         * Value:   | 0 | 0 | 0  | 0  | 0   | 0   | 1   | 1 | = 0x03
         */
        unsafe {
            outb(self.modem_command_port(), 0x03);
        }
    }

    fn is_transmit_fifo_empty(&self) -> bool {
        /* 0x20 = 0010 0000 */
        unsafe { inb(self.line_status_port()) & 0x20 != 0 }
    }

    pub fn serial_write(&mut self, data: &[u8]) {
        for b in data {
            if self.is_transmit_fifo_empty() {
                self.serial_write_byte(*b);
            }
        }
    }

    fn serial_write_byte(&mut self, byte: u8) {
        unsafe {
            outb(self.data_port(), byte);
        }
    }

    pub fn configure(&self, baud_rate: u16) {
        self.configure_baud_rate(baud_rate);
        self.configure_line();
        self.configure_fifo_buffer();
        self.configure_modem();
    }

    fn data_port(&self) -> u16 {
        self.base
    }

    fn fifo_command_port(&self) -> u16 {
        self.base + 2
    }

    fn line_command_port(&self) -> u16 {
        self.base + 3
    }

    fn modem_command_port(&self) -> u16 {
        self.base + 4
    }

    fn line_status_port(&self) -> u16 {
        self.base + 5
    }
}

const LINE_ENABLE_DLAB: u8 = 0x80;

/// Write byte to port
unsafe fn outb(port: u16, data: u8) {
    core::arch::asm!("mov al, {}; mov dx, {:x}; out dx, al",
      in(reg_byte) data,
      in(reg_abcd) port
    );
}

unsafe fn inb(port: u16) -> u8 {
    // Trust me bro
    #[allow(unused_assignments)]
    let mut data = 0;
    core::arch::asm!("mov dx, {:x}; in {}, dx",
      in(reg_abcd) port,
      out(reg_byte) data
    );
    data
}

impl core::fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.serial_write(s.as_bytes());
        Ok(())
    }
}
