#![no_main]
#![no_std]

use core::{fmt::Write, ops::RangeInclusive};

use iis2dlpc_rs::{from_fs4_to_mg, prelude::*, PROPERTY_ENABLE};
use iis2dlpc_rs::{I2CAddress, Iis2dlpc};

use panic_itm as _;
use st_mems_bus::BusOperation;

use cortex_m_rt::entry;
use stm32f4xx_hal::{
    hal::delay::DelayNs,
    i2c::{DutyCycle, I2c, Mode as I2cMode},
    pac,
    prelude::*,
    serial::Config,
};

const ST_RANGE_POS: RangeInclusive<f32> = 70.0..=1500.0;
const SELF_TEST_SAMPLES: usize = 5;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).sysclk(48.MHz()).freeze();

    let delay = cp.SYST.delay(&clocks);

    let gpiob = dp.GPIOB.split();
    let gpioa = dp.GPIOA.split();

    let scl = gpiob.pb8;
    let sda = gpiob.pb9;

    let i2c = I2c::new(
        dp.I2C1,
        (scl, sda),
        I2cMode::Fast {
            frequency: 400.kHz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        &clocks,
    );

    let tx_pin = gpioa.pa2.into_alternate();
    let mut tx = dp
        .USART2
        .tx(
            tx_pin,
            Config::default()
                .baudrate(115200.bps())
                .wordlength_8()
                .parity_none(),
            &clocks,
        )
        .unwrap();

    let mut sensor = Iis2dlpc::new_i2c(i2c, I2CAddress::I2cAddH, delay);

    match sensor.device_id_get() {
        Ok(value) => {
            if value != iis2dlpc_rs::ID {
                panic!("Invalid sensor ID")
            }
        }
        Err(e) => writeln!(tx, "An error occured while reading sensor ID: {e:?}").unwrap(),
    }
    sensor.tim.delay_ms(25);

    loop {
        // Restore default configuration
        sensor.reset_set().unwrap();
        while sensor.reset_get().unwrap() == 1 {}

        sensor.block_data_update_set(PROPERTY_ENABLE).unwrap();
        sensor.full_scale_set(Fs::_4g).unwrap();
        sensor.power_mode_set(Mode::HighPerformance).unwrap();
        sensor.data_rate_set(Odr::_50hz).unwrap();
        sensor.tim.delay_ms(100);

        // Flush old samples
        flush_samples(&mut sensor).unwrap();

        let mut media = [0_f32; 3];
        let mut i = 0;
        loop {
            let status = sensor.status_reg_get().unwrap();
            if status.drdy() == 1 {
                let acceleration_mg = sensor.acceleration_raw_get().unwrap().map(from_fs4_to_mg);

                (0..3).for_each(|i| media[i] += acceleration_mg[i]);
                i += 1;
            }
            if i >= SELF_TEST_SAMPLES {
                break;
            }
        }
        (0..3).for_each(|i| media[i] /= SELF_TEST_SAMPLES as f32);

        // Enable self test mode
        sensor.self_test_set(St::Positive).unwrap();
        sensor.tim.delay_ms(100);
        flush_samples(&mut sensor).unwrap();

        let mut media_st = [0_f32; 3];
        let mut i = 0;
        loop {
            let status = sensor.status_reg_get().unwrap();
            if status.drdy() == 1 {
                let acceleration_mg = sensor.acceleration_raw_get().unwrap().map(from_fs4_to_mg);

                (0..3).for_each(|i| media_st[i] += acceleration_mg[i]);
                i += 1;
            }
            if i >= SELF_TEST_SAMPLES {
                break;
            }
        }
        (0..3).for_each(|i| media_st[i] /= SELF_TEST_SAMPLES as f32);

        // Check for all axis self test value range
        let mut st_dev = [0_f32; 3];
        (0..3).for_each(|j| st_dev[j] = (media_st[j] - media[j]).abs());

        st_dev.iter().enumerate().for_each(|(i, dev)| {
            writeln!(
                tx,
                "{i}: |{}| <= |{}| <= |{}|",
                ST_RANGE_POS.start(),
                dev,
                ST_RANGE_POS.end(),
            )
            .unwrap();
        });

        if st_dev.iter().all(|dev| ST_RANGE_POS.contains(dev)) {
            writeln!(tx, "PASSED").unwrap();
        } else {
            writeln!(tx, "FAILED").unwrap();
        }

        // Disable self test mode
        sensor.data_rate_set(Odr::Off).unwrap();
        sensor.self_test_set(St::Disable).unwrap();
    }
}

fn flush_samples<B, T>(sensor: &mut Iis2dlpc<B, T>) -> Result<(), iis2dlpc_rs::Error<B::Error>>
where
    B: BusOperation,
    T: DelayNs,
{
    loop {
        let reg = sensor.status_reg_get()?;

        if reg.drdy() == 1 {
            let _ = sensor.acceleration_raw_get()?;
            return Ok(());
        }
    }
}
