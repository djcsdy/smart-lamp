#![no_std]
#![no_main]

use rp_pico::entry;
use defmt::*;
#[allow(unused_imports)]
use defmt_rtt as _;
#[allow(unused_imports)]
use panic_probe as _;

#[entry]
fn main() -> ! {
    info!("Hello, World!");
    loop {}
}
