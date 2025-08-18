#![no_std]
#![no_main]

use core::fmt::Write;
use core::ops::RangeInclusive;

use embassy_executor::Spawner;
use embassy_stm32::i2c::{self, I2c};
use embassy_stm32::time::khz;
use embassy_stm32::usart::{self, BufferedInterruptHandler, DataBits, Parity, UartTx};
use embassy_stm32::{bind_interrupts, peripherals, peripherals::USART2};
use embassy_time::Delay;
use embedded_hal::delay::DelayNs;
use heapless::String;
use iis2dlpc_rs::{I2CAddress, Iis2dlpc, PROPERTY_ENABLE, from_fs4_to_mg, prelude::*};

use st_mems_bus::BusOperation;

use {defmt_rtt as _, panic_probe as _};

#[defmt::panic_handler]
fn panic() -> ! {
    core::panic!("panic via `defmt::panic!")
}

bind_interrupts!(struct Irqs {
    USART2 => BufferedInterruptHandler<USART2>;
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

const ST_RANGE_POS: RangeInclusive<f32> = 70.0..=1500.0;
const SELF_TEST_SAMPLES: usize = 5;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let mut delay = Delay;
    delay.delay_ms(10);

    let mut usart_cfg = usart::Config::default();
    usart_cfg.baudrate = 115200;
    usart_cfg.data_bits = DataBits::DataBits8;
    usart_cfg.parity = Parity::ParityNone;

    let mut tx = UartTx::new(p.USART2, p.PA2, p.DMA1_CH6, usart_cfg).unwrap();

    let i2c = I2c::new(
        p.I2C1,
        p.PB8,
        p.PB9,
        Irqs,
        p.DMA1_CH7,
        p.DMA1_CH5,
        khz(100),
        Default::default(),
    );

    let mut msg = String::<64>::new();

    let mut sensor = Iis2dlpc::new_i2c(i2c, I2CAddress::I2cAddH, delay.clone());

    match sensor.device_id_get() {
        Ok(value) => {
            if value != iis2dlpc_rs::ID {
                panic!("Invalid sensor ID")
            }
        }
        Err(e) => {
            writeln!(&mut msg, "An error occured while reading sensor ID: {e:?}").unwrap();
            tx.blocking_write(msg.as_bytes()).unwrap();
            msg.clear();
        }
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

                (0..3).for_each(|j| media[j] += acceleration_mg[j]);

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

                (0..3).for_each(|j| media_st[j] += acceleration_mg[j]);
                i += 1;
            }
            if i >= SELF_TEST_SAMPLES {
                break;
            }
        }
        (0..3).for_each(|i| media_st[i] /= SELF_TEST_SAMPLES as f32);

        // Check for all axis self test value range
        let mut st_dev = [0_f32; 3];
        (0..3).for_each(|i| st_dev[i] = (media_st[i] - media[i]).abs());

        st_dev.iter().enumerate().for_each(|(i, dev)| {
            msg.clear();
            writeln!(
                &mut msg,
                "{i}: |{}| <= |{}| <= |{}|",
                ST_RANGE_POS.start(),
                dev,
                ST_RANGE_POS.end(),
            )
            .unwrap();
            tx.blocking_write(msg.as_bytes()).unwrap();
        });

        msg.clear();
        if st_dev.iter().all(|dev| ST_RANGE_POS.contains(dev)) {
            writeln!(&mut msg, "PASSED").unwrap();
        } else {
            writeln!(&mut msg, "FAILED").unwrap();
        }
        tx.blocking_write(msg.as_bytes()).unwrap();

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
