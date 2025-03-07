use core::convert::Infallible;
use core::num::TryFromIntError;
use defmt::unwrap;
use embassy_rp::peripherals::{PIN_2, PIN_3, PWM_SLICE1};
use embassy_rp::pwm::Pwm;
use embassy_rp::{clocks, pwm};
use embassy_time::{Duration, Ticker};
use fixed::traits::{FromFixed, LosslessTryFrom, LosslessTryInto, LossyFrom, LossyInto};
use fixed::types::extra::U4;
use fixed::{FixedU16, FixedU32, FixedU64};

#[embassy_executor::task]
pub async fn pwm_lights(pwm_slice_1: PWM_SLICE1, pin_2: PIN_2, pin_3: PIN_3) {
    let mut pwm_config = pwm::Config::default();
    let pwm_frequency_hz = 100;
    let system_clock_cycles_per_pwm_cycle =
        FixedU64::<U4>::from(clocks::clk_sys_freq()) / pwm_frequency_hz;
    if let Ok(system_clock_cycles_per_pwm_cycle_u16) =
        u16::try_from(u64::lossy_from(system_clock_cycles_per_pwm_cycle))
    {
        pwm_config.top = system_clock_cycles_per_pwm_cycle_u16 - 1;
    } else {
        let divider: FixedU64<U4> =
            system_clock_cycles_per_pwm_cycle / u64::from(u16::MAX) + FixedU64::<U4>::from_bits(1);
        pwm_config.divider = FixedU16::<U4>::unwrapped_from_fixed(divider);
        pwm_config.top =
            u16::try_from(u64::lossy_from(system_clock_cycles_per_pwm_cycle / divider) - 1)
                .unwrap();
    }

    let mut pwm = Pwm::new_output_ab(pwm_slice_1, pin_2, pin_3, pwm_config.clone());

    let mut ticker = Ticker::every(Duration::from_secs(1));

    pwm_config.compare_a = 0;
    pwm_config.compare_b = 0;
    loop {
        pwm_config.compare_a = (pwm_config.top + 1) / 16;
        pwm_config.compare_b = pwm_config.compare_a;
        pwm.set_config(&pwm_config);
        ticker.next().await;

        pwm_config.compare_a = (pwm_config.top + 1) / 8;
        pwm_config.compare_b = pwm_config.compare_a;
        pwm.set_config(&pwm_config);
        ticker.next().await;

        pwm_config.compare_a = (pwm_config.top + 1) / 4;
        pwm_config.compare_b = pwm_config.compare_a;
        pwm.set_config(&pwm_config);
        ticker.next().await;

        pwm_config.compare_a = (pwm_config.top + 1) / 2;
        pwm_config.compare_b = pwm_config.compare_a;
        pwm.set_config(&pwm_config);
        ticker.next().await;

        pwm_config.compare_a = pwm_config.top + 1;
        pwm_config.compare_b = pwm_config.compare_a;
        pwm.set_config(&pwm_config);
        ticker.next().await;
    }
}
