# IIS2DLPC Tap and Double Tap Detection on STM32F401RE Nucleo-64

This example demonstrates how to configure and use the **IIS2DLPC** accelerometer's embedded tap and double tap detection features on an **STM32F401RE Nucleo-64** board. The program sets up the sensor to detect single and double tap events on all axes and reports them over UART.

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
- Resets the sensor to default configuration and waits for completion.

### Sensor Configuration

- Sets full scale to ±2g.
- Sets power mode to continuous low-power, low-noise 12-bit.
- Sets Output Data Rate (ODR) to 400 Hz.
- Enables tap detection on X, Y, and Z axes.
- Sets tap threshold to 12 on all axes.
- Configures double tap parameters: duration, quiet, and shock.
- Enables both single and double tap detection.
- Routes tap detection interrupt to INT1 pin.

### Event Handling Loop

- Continuously polls the sensor for tap events.
- On **double tap** detection:
  - Reports the event, sign (positive/negative), and axis (X, Y, or Z) over UART.
- On **single tap** detection:
  - Reports the event, sign (positive/negative), and axis (X, Y, or Z) over UART.
- Uses a fixed-size heapless string buffer for UART messages.

---

## Usage

1. Connect the IIS2DLPC sensor to the STM32F401RE Nucleo board via I2C1 (PB8/SCL, PB9/SDA).
2. Build and flash the firmware onto the STM32F401RE board.
3. Open a serial terminal at 115200 baud on the USART2 TX line.
4. Tap or double tap the sensor; observe detection messages printed for each event and axis.

---

## Notes

- Tap thresholds and timing parameters can be tuned for sensitivity and responsiveness.
- The example uses polling for event detection; for lower power or higher efficiency, consider using interrupts.
- UART output uses blocking writes.
- The environment is `#![no_std]` and `#![no_main]` for embedded Rust applications.
- Panic behavior is set to halt on panic using `panic_itm`.

---

## References

- [STM32F401RE Nucleo-64 Board](https://www.st.com/en/evaluation-tools/nucleo-f401re.html)
- [IIS2DLPC Datasheet](https://www.st.com/resource/en/datasheet/iis2dlpc.pdf)
- [stm32f4xx-hal Rust crate](https://docs.rs/stm32f4xx-hal)

---

*This README explains the embedded Rust program for tap and double tap detection using the IIS2DLPC sensor on STM32F401RE.*
