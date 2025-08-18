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
use iis2dlpc_rs::{PROPERTY_ENABLE, prelude::*};

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

    // Set Full scale
    sensor.full_scale_set(Fs::_2g).unwrap();
    // Configure power mode
    sensor
        .power_mode_set(Mode::ContLowPwrLowNoise12bit)
        .unwrap();
    // Set Output Data Rate
    sensor.data_rate_set(Odr::_200hz).unwrap();
    // Apply hogh-pass digital filter on Wake-Up function
    // Duration time is set to zero so Wake-Up interrupt signal
    // is generated for each X,Y,Z filtered data exceeding the
    // configured threshold
    sensor.wkup_dur_set(0).unwrap();
    // Set wake-up threshold
    // Set wake-up threshold: 1 Lsb corresponds to FS_XL/2^6
    sensor.wkup_threshold_set(2).unwrap();
    // Enable interrupt generation on Wake-Up INT1 pin
    let mut int_route = sensor.pin_int1_route_get().unwrap();
    int_route.set_int1_wu(PROPERTY_ENABLE);
    sensor.pin_int1_route_set(&int_route).unwrap();

    // Wait Events
    loop {
        // Check Wake-Up events
        let all_sources = sensor.all_sources_get().unwrap();
        if all_sources.wake_up_src.wu_ia() == 1 {
            msg.clear();
            write!(&mut msg, "Wake-Up event on ").unwrap();

            if all_sources.wake_up_src.x_wu() == 1 {
                write!(&mut msg, "X").unwrap();
            }

            if all_sources.wake_up_src.y_wu() == 1 {
                write!(&mut msg, "Y").unwrap();
            }

            if all_sources.wake_up_src.z_wu() == 1 {
                write!(&mut msg, "Z").unwrap();
            }

            writeln!(&mut msg, " direction.").unwrap();
            tx.blocking_write(msg.as_bytes()).unwrap();
        }
    }
}
