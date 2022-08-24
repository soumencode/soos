use core::fmt::{Arguments, Error, Write};

const UART_ADDR: usize = 0x10000000;

pub struct Uart {}

impl Uart {
    pub fn new() -> Self {
        Uart {}
    }

    pub fn send(&self, c: u8) {
        unsafe {
            core::ptr::write_volatile(UART_ADDR as *mut u8, c);
        }
    }

    pub fn receive(&self) -> u8 {
        unsafe { core::ptr::read_volatile(UART_ADDR as *mut u8) }
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        for c in s.bytes() {
            self.send(c);
        }
        Ok(())
    }

    /*
        fn write_fmt(&mut self, args: Arguments) -> Result<(), Error> {
            if let Some(s) = args.as_str() {
                self.write_str(s);
            }
            Ok(())
        }
    */
}
