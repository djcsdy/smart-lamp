use cyw43::{Control, NetDriver, State};
use cyw43_pio::PioSpi;
use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output, Pin};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio, PioPin};
use embassy_rp::{bind_interrupts, Peripheral};
use embassy_time::Timer;
use static_cell::StaticCell;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

#[embassy_executor::task]
async fn cyw43_driver(
    runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>,
) -> ! {
    info!("CYW43: Driver running");
    runner.run().await
}

pub async fn start_cyw43(
    spawner: &Spawner,
    power_control_pin: impl Peripheral<P = impl Pin> + 'static,
    chip_select_pin: impl Peripheral<P = impl Pin> + 'static,
    pio_pin: impl Peripheral<P = PIO0> + 'static,
    spi_dio_pin: impl PioPin,
    spi_clk_pin: impl PioPin,
    spi_dma_channel: DMA_CH0,
) -> (NetDriver<'static>, Control<'static>) {
    info!("CYW43: Starting driver");
    let firmware = include_bytes!("firmware/43439A0.bin");
    let clm = include_bytes!("firmware/43439A0_clm.bin");

    let power_control = Output::new(power_control_pin, Level::Low);
    let chip_select = Output::new(chip_select_pin, Level::High);
    let mut pio = Pio::new(pio_pin, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        pio.irq0,
        chip_select,
        spi_dio_pin,
        spi_clk_pin,
        spi_dma_channel,
    );

    static STATE: StaticCell<State> = StaticCell::new();
    let state = STATE.init_with(|| State::new());
    let (net_driver, mut control, runner) = cyw43::new(state, power_control, spi, firmware).await;
    unwrap!(spawner.spawn(cyw43_driver(runner)));

    info!("CYW43: Initializing");
    control.init(clm).await;
    info!("CYW43: Initialized");

    let mut secrets = include_str!("secrets.txt").lines();
    let ssid = secrets.next();
    let passphrase = secrets.next().filter(|passphrase| *passphrase != "");

    match ssid {
        Some(ssid) => {
            info!("CYW43: Joining {}", ssid);
            loop {
                let result = match passphrase {
                    Some(passphrase) => control.join_wpa2(ssid, passphrase).await,
                    None => control.join_open(ssid).await,
                };
                match result {
                    Ok(_) => break,
                    Err(err) => {
                        info!("CYW43: Failed to join {}: {}", ssid, err.status);
                        Timer::after_secs(2).await;
                    }
                }
            }
            info!("CYW43: Joined {}", ssid);
        }
        None => {
            info!("CYW43: No WiFi SSID specified");
        }
    }

    info!("CYW43: Setting  power management to PowerSave");
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    (net_driver, control)
}
