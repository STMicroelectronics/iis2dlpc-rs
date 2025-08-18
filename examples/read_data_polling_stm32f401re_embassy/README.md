# IIS2DLPC Acceleration Data Acquisition on STM32F401RE Nucleo-64 Using Embassy Framework

This example demonstrates how to configure the **IIS2DLPC** ultra-low-power accelerometer for **continuous acceleration data acquisition** on an **STM32F401RE Nucleo-64** board. The sensor is set up to output acceleration data in polling mode, and the results are reported over UART in milli-g (mg) units.

The code is written in Rust using the [Embassy](https://embassy.dev/) async runtime, the `embassy-stm32` hardware abstraction layer, and the `iis2dlpc` sensor driver crate. It showcases sensor initialization, configuration for continuous data acquisition, and real-time data reporting via UART.

---

## Hardware Setup

- **Microcontroller Board:** STM32F401RE Nucleo-64
- **Sensor:** IIS2DLPC Accelerometer (I2C interface)
- **Communication Interface:** I2C1 at 100 kHz Standard Mode
- **UART:** USART2 for serial output at 115200 baud

### Default Pin Configuration

| Signal       | STM32F401RE Pin | Description                    |
|--------------|-----------------|-------------------------------|
| I2C1_SCL     | PB8             | I2C clock line (open-drain)   |
| I2C1_SDA     | PB9             | I2C data line (open-drain)    |
| USART2_TX    | PA2             | UART transmit for debug output|

The IIS2DLPC sensor is connected to the STM32F401RE via I2C1 on PB8 (SCL) and PB9 (SDA). UART output is routed through PA2.

---

## Code Description

### Initialization

- The Embassy STM32 HAL initializes microcontroller peripherals, including clocks, GPIOs, I2C, and UART.
- USART2 is configured for 115200 baud, 8 data bits, no parity, using DMA for efficient transmission.
- I2C1 is set up at 100 kHz Standard Mode with DMA and interrupt support.
- A delay provider is used for sensor startup and timing.

### Sensor Configuration

- The IIS2DLPC sensor is initialized over I2C with the high I2C address.
- The device ID is read and verified to ensure correct communication.
- The sensor is reset to its default configuration and waits for reset completion.
- Block Data Update (BDU) is enabled to ensure data consistency during reads.
- Full scale is set to ±8g for acceleration measurements.
- The accelerometer filtering chain is configured for low-pass filtering (LPF on output, ODR/4 bandwidth).
- Power mode is set to continuous low-power, low-noise 12-bit mode.
- Output Data Rate (ODR) is set to 25 Hz for periodic sampling.

### Data Acquisition Loop

- The main loop continuously polls the sensor’s data-ready flag.
- When new acceleration data is available, the raw data is read, converted to mg units, and formatted.
- Acceleration values for X, Y, and Z axes are printed over UART.
- UART writes are performed asynchronously.

---

## Usage

1. Connect the IIS2DLPC sensor to the STM32F401RE Nucleo board via I2C1 (PB8/SCL, PB9/SDA).
2. Build and flash the firmware onto the STM32F401RE board.
3. Open a serial terminal at 115200 baud on the USART2 TX line (PA2).
4. Observe acceleration readings (in mg) for X, Y, and Z axes printed continuously.

---

## Notes

- This example uses polling to check for new acceleration data. For lower power consumption and faster response, consider using GPIO interrupts.
- The full scale, output data rate, and filter settings can be tuned for your specific application and sensitivity requirements.
- The environment is `#![no_std]` and `#![no_main]` for embedded Rust applications using the Embassy runtime.
- Panic behavior is set to use `panic_probe` and `defmt` for debugging.

---

## References

- [STM32F401RE Nucleo-64 Board](https://www.st.com/en/evaluation-tools/nucleo-f401re.html)
- [IIS2DLPC Datasheet](https://www.st.com/resource/en/datasheet/iis2dlpc.pdf)
- [Embassy STM32 HAL](https://github.com/embassy-rs/embassy)

---

*This README provides a detailed explanation of the embedded Rust program for continuous acceleration data acquisition using the IIS2DLPC sensor on STM32F401RE with UART output, leveraging the Embassy async framework.*
