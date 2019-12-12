#![no_std]

pub trait GlobalLog: Sync {
    fn log(&self, address: u8);
}

pub trait Log {
    type Error;

    fn log(&mut self, address: u8) -> Result<(), Self::Error>;
}

#[macro_export]
macro_rules! log {
    ($string:expr) => {
        unsafe {
            extern "Rust" {
                static LOGGER: &'static dyn $crate::GlobalLog;
            }

            #[export_name = $string]
            #[link_section = ".log"]
            static SYMBOL: u8 = 0;

            $crate::GlobalLog::log(LOGGER, &SYMBOL as *const u8 as usize as u8)
        }
    };

    ($logger:expr, $string:expr) => {{
        #[export_name = $string]
        #[link_section = ".log"]
        static SYMBOL: u8 = 0;

        $crate::Log::log(&mut $logger, &SYMBOL as *const u8 as usize as u8)
    }};
}

#[macro_export]
macro_rules! global_logger {
    ($logger:expr) => {
        #[no_mangle]
        pub static LOGGER: &dyn $crate::GlobalLog = &$logger;
    };
}

/*
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
*/