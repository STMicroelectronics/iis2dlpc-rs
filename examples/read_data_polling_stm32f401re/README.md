# IIS2DLPC Accelerometer Basic Data Polling on STM32F401RE Nucleo-64

This example demonstrates how to initialize and configure the **IIS2DLPC** 3-axis accelerometer sensor on an **STM32F401RE Nucleo-64** board using I2C communication. The program continuously polls the sensor for acceleration data and outputs the readings over UART.

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

The IIS2DLPC sensor is connected to the STM32F401RE via I2C1 on pins PB8 (SCL) and PB9 (SDA). UART output is routed through PA2.

---

## Code Description

### Initialization

- The microcontroller peripherals and system clocks are initialized, using an 8 MHz external crystal.
- GPIO ports B and A are split to configure I2C pins (PB8, PB9) and UART TX pin (PA2).
- I2C1 is configured in standard mode at 100 kHz.
- USART2 is configured for 115200 baud, 8 data bits, no parity.
- The IIS2DLPC sensor is initialized over I2C with the high I2C address.
- The sensor device ID is read and verified; if mismatched, the program panics.
- The sensor is reset to its default configuration and waits for the reset to complete.

### Sensor Configuration

- **Block Data Update (BDU)** is enabled to ensure data consistency during reads.
- **Full scale** is set to ±8g for the accelerometer.
- The **filtering chain** is configured:
  - Output low-pass filter enabled (`LpfOnOut`)
  - Bandwidth set to ODR/4
- **Power mode** is set to continuous low-power, low-noise 12-bit mode.
- **Output Data Rate (ODR)** is set to 25 Hz.

### Data Polling Loop

- The main loop continuously checks the data-ready flag.
- When new acceleration data is available, raw data is read, converted to milli-g (mg) units, and printed over UART in a tab-separated format.

---

## Usage

1. Connect the IIS2DLPC sensor to the STM32F401RE Nucleo board via I2C1 (PB8/SCL, PB9/SDA).
2. Build and flash the firmware onto the STM32F401RE board.
3. Open a serial terminal at 115200 baud on the USART2 TX line.
4. Observe acceleration readings (in mg) printed continuously for X, Y, and Z axes.

---

## Notes

- This example uses polling to read sensor data, which is simple but may be less power efficient than interrupt-driven approaches.
- Block Data Update (BDU) is enabled to prevent reading partially updated sensor data.
- Output Data Rate and full scale settings can be adjusted to suit application requirements.
- The filtering chain configuration improves signal quality by applying a low-pass filter.

---

## References

- [STM32F401RE Nucleo-64 Board](https://www.st.com/en/evaluation-tools/nucleo-f401re.html)
- [IIS2DLPC Datasheet](https://www.st.com/resource/en/datasheet/iis2dlpc.pdf)
- [stm32f4xx-hal Rust crate](https://docs.rs/stm32f4xx-hal)

---

*This README explains how to initialize and poll the IIS2DLPC accelerometer on an STM32F401RE board and output acceleration data via UART.*
