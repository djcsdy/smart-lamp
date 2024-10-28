#![no_std]
#![no_main]

mod net;

use defmt::*;
#[allow(unused_imports)]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::pwm::Pwm;
use embassy_rp::{clocks, pwm};
use embassy_time::{Duration, Ticker};
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

    let mut pwm_config = pwm::Config::default();
    let pwm_frequency = 25000;
    let pwm_top = unwrap!(u16::try_from(clocks::clk_sys_freq() / pwm_frequency)) - 1;
    pwm_config.top = pwm_top;
    pwm_config.compare_a = 0;
    pwm_config.compare_b = 0;
    let mut pwm = Pwm::new_output_ab(
        peripherals.PWM_SLICE1,
        peripherals.PIN_2,
        peripherals.PIN_3,
        pwm_config.clone(),
    );

    let mut ticker = Ticker::every(Duration::from_secs(1));

    loop {
        pwm_config.compare_a = pwm_top / 128;
        pwm_config.compare_b = pwm_config.compare_a;
        pwm.set_config(&pwm_config);
        ticker.next().await;

        pwm_config.compare_a = pwm_top / 64;
        pwm_config.compare_b = pwm_config.compare_a;
        pwm.set_config(&pwm_config);
        ticker.next().await;

        pwm_config.compare_a = pwm_top / 32;
        pwm_config.compare_b = pwm_config.compare_a;
        pwm.set_config(&pwm_config);
        ticker.next().await;

        pwm_config.compare_a = pwm_top / 16;
        pwm_config.compare_b = pwm_config.compare_a;
        pwm.set_config(&pwm_config);
        ticker.next().await;

        pwm_config.compare_a = pwm_top / 8;
        pwm_config.compare_b = pwm_config.compare_a;
        pwm.set_config(&pwm_config);
        ticker.next().await;

        pwm_config.compare_a = pwm_top / 4;
        pwm_config.compare_b = pwm_config.compare_a;
        pwm.set_config(&pwm_config);
        ticker.next().await;

        pwm_config.compare_a = pwm_top / 2;
        pwm_config.compare_b = pwm_config.compare_a;
        pwm.set_config(&pwm_config);
        ticker.next().await;

        pwm_config.compare_a = pwm_top;
        pwm_config.compare_b = pwm_config.compare_a;
        pwm.set_config(&pwm_config);
        ticker.next().await;
    }
}
