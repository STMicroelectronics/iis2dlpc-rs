# IIS2DLPC Accelerometer Self-Test Example on STM32F401RE Nucleo-64

This example demonstrates how to perform a **self-test** on the **IIS2DLPC** 3-axis accelerometer sensor using an **STM32F401RE Nucleo-64** board. The self-test verifies the sensor's functionality by comparing acceleration measurements before and after enabling the self-test mode, and checks if the measured deviation falls within the datasheet-specified range.

---

## Hardware Setup

- **Microcontroller Board:** STM32F401RE Nucleo-64
- **Sensor:** IIS2DLPC 3-axis Accelerometer
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

- Initializes microcontroller peripherals, system clocks (using 8 MHz HSE), and delay abstraction.
- Configures I2C1 in standard mode (100 kHz) on PB8/PB9.
- Configures USART2 for UART output at 115200 baud on PA2.
- Initializes the IIS2DLPC sensor over I2C with the high I2C address.
- Reads and verifies the sensor device ID; panics if mismatched.
- Waits 25 ms after sensor startup.

### Self-Test Procedure

- **Sensor Reset and Configuration:**
  - Resets the sensor to default configuration and waits for completion.
  - Enables Block Data Update (BDU) for consistent data reads.
  - Sets full scale to ±4g.
  - Sets power mode to High Performance.
  - Sets Output Data Rate (ODR) to 50 Hz.
  - Waits 100 ms for settings to take effect.
- **Sample Acquisition (Normal Mode):**
  - Flushes old samples.
  - Collects 5 acceleration samples in normal mode, averages them for each axis.
- **Sample Acquisition (Self-Test Mode):**
  - Enables positive self-test mode.
  - Waits 100 ms, flushes old samples.
  - Collects 5 acceleration samples in self-test mode, averages them for each axis.
- **Deviation Calculation and Validation:**
  - Computes the absolute difference between self-test and normal averages for each axis.
  - Checks if each deviation falls within the datasheet-specified range (70 mg to 1500 mg).
  - Prints the deviation and the result ("PASSED" or "FAILED") over UART.
- **Cleanup:**
  - Disables self-test mode and sets ODR to Off.

### Helper Function

- `flush_samples`: Reads and discards one sample to ensure only fresh data is used for averaging.

---

## Usage

1. Connect the IIS2DLPC sensor to the STM32F401RE Nucleo board via I2C1 (PB8/SCL, PB9/SDA).
2. Build and flash the firmware onto the STM32F401RE board.
3. Open a serial terminal at 115200 baud on the USART2 TX line.
4. Observe the self-test results printed for each axis and the overall test status.

---

## Notes

- The self-test uses averaging over 5 samples to reduce noise and improve reliability.
- The deviation range (70 mg to 1500 mg) is based on the IIS2DLPC datasheet recommendations for self-test.
- UART output uses blocking writes.
- The environment is `#![no_std]` and `#![no_main]` for embedded Rust applications.
- Panic behavior is set to halt on panic using `panic_itm`.

---

## References

- [STM32F401RE Nucleo-64 Board](https://www.st.com/en/evaluation-tools/nucleo-f401re.html)
- [IIS2DLPC Datasheet](https://www.st.com/resource/en/datasheet/iis2dlpc.pdf)
- [stm32f4xx-hal Rust crate](https://docs.rs/stm32f4xx-hal)

---

*This README explains the embedded Rust program for performing a self-test on the IIS2DLPC accelerometer using STM32F401RE.*
