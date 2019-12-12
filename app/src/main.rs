#![no_std]
#![no_main]

//use core::fmt::Write;
use cortex_m::interrupt;
use cortex_m_semihosting::{
	debug,
	hio::{self, HStdout}};

use rt::entry;
use log::{global_logger, log, GlobalLog};
//use log::{error, warn, Log};
//use log::{log, Log};

/*
struct Logger {
	hstdout: HStdout,
}
impl Log for Logger {
	type Error = ();
	fn log(&mut self, address: u8) -> Result<(), ()> {
		self.hstdout.write_all(&[address])
	}
}
*/

struct Logger;
impl GlobalLog for Logger {
	fn log(&self, address: u8) {
		interrupt::free(|_| unsafe {
            static mut HSTDOUT: Option<HStdout> = None;

            // lazy initialization
            if HSTDOUT.is_none() {
                HSTDOUT = Some(hio::hstdout()?);
            }

            let hstdout = HSTDOUT.as_mut().unwrap();

            hstdout.write_all(&[address])
        }).ok();
	}
}

global_logger!(Logger);

entry!(main);

static RODATA: &[u8] = b"Hello, world!";
static mut BSS: u8 = 0;
static mut DATA: u16 = 1;

fn main() -> ! {
    let _x = RODATA;
    let _y = unsafe { &BSS };
	let _z = unsafe { &DATA };

	log!("Hello, world!");

    log!("Goodbye");
	/*
	let hstdout = hio::hstdout().unwrap();
	let mut logger = Logger { hstdout };
	
    let _ = log!(logger, "Hello, world!");

	let _ = log!(logger, "Goodbye");
	*/

	/*
	let _ = warn!(logger, "Warn");

	let _ = error!(logger, "Error");
	*/

	/*
	#[export_name = "Hello, World!"]
	#[link_section = ".log"]
	static A: u8 = 0;
	
	//let _ = writeln!(hstdout, "{:#x}", &A as *const u8 as usize);
	let address = &A as *const u8 as usize as u8;
    hstdout.write_all(&[address]).unwrap();
	
	#[export_name = "Goodbye"]
	#[link_section = ".log"]
	static B: u8 = 0;
	
	//let _ = writeln!(hstdout, "{:#x}", &B as *const u8 as usize);
	let address = &B as *const u8 as usize as u8;
    hstdout.write_all(&[address]).unwrap();
	*/

	debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub fn HardFault(_ef: *const u32) -> ! {
    loop {}
}
