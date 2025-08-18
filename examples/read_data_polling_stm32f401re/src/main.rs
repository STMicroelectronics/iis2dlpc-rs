#![no_main]
#![no_std]

use core::fmt::Write;

use iis2dlpc_rs::{from_fs8_to_mg, prelude::*, PROPERTY_ENABLE};
use iis2dlpc_rs::{I2CAddress, Iis2dlpc};

use panic_itm as _;

use cortex_m_rt::entry;
use stm32f4xx_hal::{
    hal::delay::DelayNs,
    i2c::{DutyCycle, I2c, Mode as I2cMode},
    pac,
    prelude::*,
    serial::Config,
};

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

            writeln!(
                tx,
                "Acceleration [mg]: {:4.2}\t{:4.2}\t{:4.2}",
                acceleration_mg[0], acceleration_mg[1], acceleration_mg[2]
            )
            .unwrap();
        }
    }
}
