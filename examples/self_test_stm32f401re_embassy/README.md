# IIS2DLPC Accelerometer Self-Test on STM32F401RE Nucleo-64 Using Embassy Framework

This example demonstrates how to perform a **self-test** on the **IIS2DLPC** ultra-low-power accelerometer using an **STM32F401RE Nucleo-64** board. The self-test procedure measures the sensor's response to an internal stimulus and verifies that the response falls within the expected range, reporting the result over UART.

The code is written in Rust using the [Embassy](https://embassy.dev/) async runtime, the `embassy-stm32` hardware abstraction layer, and the `iis2dlpc` sensor driver crate. It showcases sensor initialization, self-test configuration, and result reporting via UART.

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

### Self-Test Procedure

- The IIS2DLPC sensor is initialized over I2C with the high I2C address.
- The device ID is read and verified to ensure correct communication.
- The sensor is reset to its default configuration and waits for reset completion.
- Block Data Update (BDU) is enabled for data consistency.
- Full scale is set to ±4g, power mode to high performance, and output data rate to 50 Hz.
- Old samples are flushed before measurement.
- **Baseline Measurement:** The average acceleration is measured over 5 samples for each axis.
- **Self-Test Activation:** The sensor's self-test mode is enabled, and after a short delay, the average acceleration is measured again over 5 samples.
- The absolute difference between self-test and baseline measurements is calculated for each axis.
- The result for each axis is checked against the expected self-test range (70 mg to 1500 mg).
- The result ("PASSED" or "FAILED") is printed over UART.
- Self-test mode is disabled and the sensor is powered down.

### Helper Function

- `flush_samples` is used to clear any old data from the sensor's output registers before taking new measurements.

---

## Usage

1. Connect the IIS2DLPC sensor to the STM32F401RE Nucleo board via I2C1 (PB8/SCL, PB9/SDA).
2. Build and flash the firmware onto the STM32F401RE board.
3. Open a serial terminal at 115200 baud on the USART2 TX line (PA2).
4. Observe the self-test results for each axis and the overall "PASSED" or "FAILED" message printed over UART.

---

## Notes

- The self-test is performed in a loop; the result is printed repeatedly.
- The self-test range (70 mg to 1500 mg) is based on the IIS2DLPC datasheet recommendations.
- The environment is `#![no_std]` and `#![no_main]` for embedded Rust applications using the Embassy runtime.
- Panic behavior is set to use `panic_probe` and `defmt` for debugging.

---

## References

- [STM32F401RE Nucleo-64 Board](https://www.st.com/en/evaluation-tools/nucleo-f401re.html)
- [IIS2DLPC Datasheet](https://www.st.com/resource/en/datasheet/iis2dlpc.pdf)
- [Embassy STM32 HAL](https://github.com/embassy-rs/embassy)

---

*This README provides a detailed explanation of the embedded Rust program for performing a self-test on the IIS2DLPC accelerometer using the STM32F401RE and reporting results via UART, leveraging the Embassy async framework.*
