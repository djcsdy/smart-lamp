#![no_std]
#![no_main]

use cyw43_pio::PioSpi;
use defmt::*;
#[allow(unused_imports)]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_time::{Duration, Timer};
#[allow(unused_imports)]
use panic_probe as _;
use static_cell::StaticCell;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

#[embassy_executor::task]
async fn cyw43_task(
    runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>,
) -> ! {
    info!("CYW43: Task running");
    runner.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());
    let cyw43_firmware = include_bytes!("./cyw43-firmware/43439A0.bin");
    let cyw43_clm = include_bytes!("./cyw43-firmware/43439A0_clm.bin");

    let cyw43_power_control = Output::new(peripherals.PIN_23, Level::Low);
    let cyw43_chip_select = Output::new(peripherals.PIN_25, Level::High);
    let mut pio = Pio::new(peripherals.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        pio.irq0,
        cyw43_chip_select,
        peripherals.PIN_24,
        peripherals.PIN_29,
        peripherals.DMA_CH0,
    );

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (_net_device, mut control, runner) =
        cyw43::new(state, cyw43_power_control, spi, cyw43_firmware).await;
    info!("CYW43: Starting task");
    unwrap!(spawner.spawn(cyw43_task(runner)));

    info!("CYW43: Initializing");
    control.init(cyw43_clm).await;

    info!("CYW43: Setting  power management to PowerSave");
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    let delay = Duration::from_secs(1);

    loop {
        info!("LED: on");
        control.gpio_set(0, true).await;
        Timer::after(delay).await;

        info!("LED: off");
        control.gpio_set(0, false).await;
        Timer::after(delay).await;
    }
}
