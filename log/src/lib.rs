#![no_std]

pub trait Log {
    type Error;

    fn log(&mut self, address: u8) -> Result<(), Self::Error>;
}

/*
#[macro_export]
macro_rules! log {
    ($logger:expr, $string:expr) => {{
        #[export_name = $string]
        #[link_section = ".log"]
        static SYMBOL: u8 = 0;

        $crate::Log::log(&mut $logger, &SYMBOL as *const u8 as usize as u8)
    }};
}
*/

#[macro_export]
macro_rules! error {
    ($logger:expr, $string:expr) => {{
        #[export_name = $string]
        #[link_section = ".log.error"]
        static SYMBOL: u8 = 0;

        $crate::Log::log(&mut $logger, &SYMBOL as *const u8 as usize as u8)
    }};
}

#[macro_export]
macro_rules! warn {
    ($logger:expr, $string:expr) => {{
        #[export_name = $string]
        #[link_section = ".log.warning"]
        static SYMBOL: u8 = 0;

        $crate::Log::log(&mut $logger, &SYMBOL as *const u8 as usize as u8)
    }};
}
