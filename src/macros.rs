#[macro_export]
macro_rules! serial_println {
    ($($arg:tt)*) => {
            ::avr_device::interrupt::free(|cs| {
            if let Some(serial) = &mut *crate::mesh_lib::serial::GLOBAL_SERIAL.borrow(cs).borrow_mut() {
                ::ufmt::uwriteln!(serial, $($arg)*).unwrap()  // TODO: Review this unwrap
            }
        })
    }
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        ::avr_device::interrupt::free(|cs| {
            if let Some(serial) = &mut *crate::mesh_lib::serial::GLOBAL_SERIAL.borrow(cs).borrow_mut() {
                ::ufmt::uwrite!(serial, $($arg)*)
            } else {
                Ok(())
            }
        })
    }
}

#[macro_export]
macro_rules! serial_write_byte {
    ($stream:expr, $arg:expr) => {
        $stream.write($arg)
    };
}

#[macro_export]
macro_rules! serial_try_read_byte {
    ($mutexed_celled_option_byte:expr) => {
        ::avr_device::interrupt::free(|cs| {
            if let Some(serial) = &mut *crate::mesh_lib::serial::GLOBAL_SERIAL
                .borrow(cs)
                .borrow_mut()
            {
                match serial.read() {
                    Ok(byte) => $mutexed_celled_option_byte.get_mut().replace(Some(byte)),
                    Err(_) => $mutexed_celled_option_byte.get_mut().replace(None),
                };
            } else {
                $mutexed_celled_option_byte.get_mut().replace(None);
            }
        })
    };
}

#[macro_export]
#[cfg(feature = "serial_debug")]
macro_rules! serial_debug {
    ($($arg:tt)*) => {
            ::avr_device::interrupt::free(|cs| {
            if let Some(serial) = &mut *crate::mesh_lib::serial::GLOBAL_SERIAL.borrow(cs).borrow_mut() {
                ::ufmt::uwriteln!(serial, $($arg)*).unwrap()  // TODO: Review this unwrap
            }
        })
    }
}

#[macro_export]
#[cfg(not(feature = "serial_debug"))]
macro_rules! serial_debug {
    ($($arg:tt)*) => {
        ()
    };
}
