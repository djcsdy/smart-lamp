use defmt::unwrap;
use embassy_rp::peripherals::{PIN_2, PIN_3, PWM_SLICE1};
use embassy_rp::pwm::Pwm;
use embassy_rp::{clocks, pwm};
use embassy_time::{Duration, Ticker};

#[embassy_executor::task]
pub async fn pwm_lights(pwm_slice_1: PWM_SLICE1, pin_2: PIN_2, pin_3: PIN_3) {
    let mut pwm_config = pwm::Config::default();
    let pwm_frequency = 25000;
    let pwm_top = unwrap!(u16::try_from(clocks::clk_sys_freq() / pwm_frequency)) - 1;
    pwm_config.top = pwm_top;
    pwm_config.compare_a = 0;
    pwm_config.compare_b = 0;
    let mut pwm = Pwm::new_output_ab(pwm_slice_1, pin_2, pin_3, pwm_config.clone());

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
