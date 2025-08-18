#![no_main]
#![no_std]

use core::fmt::Write;

use iis2dlpc_rs::{prelude::*, I2CAddress, Iis2dlpc, PROPERTY_ENABLE};

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

    // Set Full scale
    sensor.full_scale_set(Fs::_2g).unwrap();
    // Configure filtering chain
    // Accelerometer - filter path / bandwidth
    sensor.filter_path_set(Fds::LpfOnOut).unwrap();
    sensor.filter_bandwidth_set(BwFilt::OdrDiv4).unwrap();
    // Configure power mode
    sensor
        .power_mode_set(Mode::ContLowPwrLowNoise12bit)
        .unwrap();
    // Set wake-up duration
    // Wake up duration event 1Lsb = 1 / ODR
    sensor.wkup_dur_set(2).unwrap();
    // Set sleep duration
    // Duration to go in sleep mode (1 = Lsb 512 / ODR)
    sensor.act_sleep_dur_set(2).unwrap();
    // Set Activity wake-up threshold
    // Threshold for wake-up 1 LSB = FS_XL / 64
    sensor.wkup_threshold_set(2).unwrap();
    // Data sent to wake-up interrupt function
    sensor.wkup_feed_data_set(UsrOffOnWu::HpFeed).unwrap();
    // Config activity / inactivity of stationary / motion detection
    sensor.act_mode_set(SleepOn::ActInact).unwrap();
    // Enable activiy detection interrupt
    let mut int_route = sensor.pin_int1_route_get().unwrap();
    int_route.set_int1_wu(PROPERTY_ENABLE);
    sensor.pin_int1_route_set(&int_route).unwrap();
    // Set Output Data Rate
    sensor.data_rate_set(Odr::_200hz).unwrap();

    // Wait Events
    loop {
        // Read status register
        let all_sources = sensor.all_sources_get().unwrap();

        // Check if Activity/Inactivity events
        if all_sources.wake_up_src.sleep_state_ia() == 1 {
            writeln!(tx, "Inactivity Detected").unwrap();
        }

        if all_sources.wake_up_src.wu_ia() == 1 {
            writeln!(tx, "Activity Detected").unwrap();
        }
    }
}
