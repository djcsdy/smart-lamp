#![no_std]
#![no_main]

use defmt::*;
#[allow(unused_imports)]
use defmt_rtt as _;
use embassy_executor::Spawner;
#[allow(unused_imports)]
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    embassy_rp::init(Default::default());
    info!("Hello, World!");
}
