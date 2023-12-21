#![no_std]

mod macros;

use avr_device::interrupt::Mutex;
use core::cell::RefCell;
use void::ResultVoidExt;

use nb;

pub use platform_serial::PlatformSerial;

type Usart = arduino_hal::hal::usart::Usart0<arduino_hal::DefaultClock>;
static GLOBAL_SERIAL: Mutex<RefCell<Option<Usart>>> = Mutex::new(RefCell::new(None));

pub struct ArduinoNanoSerial;

impl ArduinoNanoSerial {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn init_serial(usart: Usart) {
    avr_device::interrupt::free(|cs| {
        GLOBAL_SERIAL.borrow(cs).replace(Some(usart));
    });
}

impl embedded_hal::serial::Read<u8> for ArduinoNanoSerial {
    type Error = void::Void;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        let mut result = nb::Result::Err(nb::Error::WouldBlock);

        ::avr_device::interrupt::free(|critical_secion| {
            if let Some(serial) = &mut *GLOBAL_SERIAL.borrow(critical_secion).borrow_mut() {
                result = serial.read()
            }
        });
        result
    }
}

impl embedded_hal::serial::Write<u8> for ArduinoNanoSerial {
    type Error = void::Void;

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        ::avr_device::interrupt::free(|critical_secion| {
            if let Some(serial) = &mut *GLOBAL_SERIAL.borrow(critical_secion).borrow_mut() {
                serial.write_byte(word);
            }
        });
        return nb::Result::Ok(());
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        ::avr_device::interrupt::free(|critical_secion| {
            if let Some(serial) = &mut *GLOBAL_SERIAL.borrow(critical_secion).borrow_mut() {
                serial.flush();
            }
        });
        return nb::Result::Ok(());
    }
}

impl ufmt::uWrite for ArduinoNanoSerial {
    type Error = nb::Error<void::Void>;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        for b in s.as_bytes().iter() {
            nb::block!(<ArduinoNanoSerial as embedded_hal::serial::Write<u8>>::write(self, *b))
                .void_unwrap();
        }
        Ok(())
    }

    // fn write_char is implemented by default trait uWrite
}

impl PlatformSerial<u8> for ArduinoNanoSerial {}
