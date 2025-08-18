#![no_std]
#![no_main]

use core::fmt::Write;

use embassy_executor::Spawner;
use embassy_stm32::i2c::{self, I2c};
use embassy_stm32::time::khz;
use embassy_stm32::usart::{self, BufferedInterruptHandler, DataBits, Parity, UartTx};
use embassy_stm32::{bind_interrupts, peripherals, peripherals::USART2};
use embassy_time::Delay;
use embedded_hal::delay::DelayNs;
use heapless::String;
use iis2dlpc_rs::{I2CAddress, Iis2dlpc};
use iis2dlpc_rs::{PROPERTY_ENABLE, from_fs8_to_mg, prelude::*};

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

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

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

    let mut delay = Delay;
    let mut msg = String::<64>::new();

    delay.delay_ms(10);

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

    // Restore default configuration
    sensor.reset_set().unwrap();
    while sensor.reset_get().unwrap() == 1 {}

    // Enable Block Data Update
    sensor.block_data_update_set(PROPERTY_ENABLE).unwrap();
    // Set Full scale
    sensor.full_scale_set(Fs::_8g).unwrap();

    // Configure filtering chain

    // Accelerometer - filter path / bandwidth
    sensor.filter_path_set(Fds::LpfOnOut).unwrap();
    sensor.filter_bandwidth_set(BwFilt::OdrDiv4).unwrap();

    // Configure power mode
    sensor
        .power_mode_set(Mode::ContLowPwrLowNoise12bit)
        .unwrap();
    // Set Output Data Rate
    sensor.data_rate_set(Odr::_25hz).unwrap();

    // Read samples in polling mode (no int)
    loop {
        if sensor.flag_data_ready_get().unwrap() == 1 {
            let acceleration_mg = sensor.acceleration_raw_get().unwrap().map(from_fs8_to_mg);

            msg.clear();
            writeln!(
                &mut msg,
                "Acceleration [mg]: {:4.2}\t{:4.2}\t{:4.2}",
                acceleration_mg[0], acceleration_mg[1], acceleration_mg[2]
            )
            .unwrap();
            tx.write(msg.as_bytes()).await.unwrap();
        }
    }
}
