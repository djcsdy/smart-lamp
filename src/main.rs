#![no_std]
#![no_main]

mod light;
mod net;

use crate::light::pwm_lights;
use defmt::*;
#[allow(unused_imports)]
use defmt_rtt as _;
use embassy_executor::Spawner;
#[allow(unused_imports)]
use panic_probe as _;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    let (_net_device, _cyw43_control) = net::cyw43::start_cyw43(
        &spawner,
        peripherals.PIN_23,
        peripherals.PIN_25,
        peripherals.PIO0,
        peripherals.PIN_24,
        peripherals.PIN_29,
        peripherals.DMA_CH0,
    )
    .await;

    unwrap!(spawner.spawn(pwm_lights(
        peripherals.PWM_SLICE1,
        peripherals.PIN_2,
        peripherals.PIN_3
    )));
}
