# IIS2DLPC 6D Orientation Detection on STM32F401RE Nucleo-64

This example demonstrates how to configure the **IIS2DLPC** accelerometer sensor to detect 6D orientation changes using an **STM32F401RE** microcontroller. When the device orientation crosses a 60-degree threshold on any axis, the event is reported over UART, indicating the new orientation.

---

## Hardware Setup

- **Microcontroller Board:** STM32F401RE Nucleo-64
- **Sensor:** IIS2DLPC Accelerometer
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

- The microcontroller peripherals and clocks are initialized, using an 8 MHz external crystal.
- GPIO pins PB8 and PB9 are configured for I2C1 communication.
- GPIO pin PA2 is configured for USART2 TX for serial output.
- I2C1 is set to standard mode at 100 kHz.
- USART2 is set to 115200 baud, 8 data bits, no parity.
- A delay abstraction is created using the system timer.

### Sensor Setup

- The IIS2DLPC sensor is initialized over I2C with the high I2C address.
- The device ID is read and verified; if mismatched, the program panics.
- The sensor is reset to default configuration and the code waits until reset completes.
- Full scale is set to ±2g.
- Power mode is set to continuous low power with low noise and 12-bit resolution.
- The 6D orientation threshold is set to 60 degrees.
- The 6D function is configured to use the LPF2 filtered data for improved noise immunity.
- 6D orientation interrupt is enabled and routed to the INT1 pin.
- Output data rate is set to 200 Hz.

### Event Loop

- The program enters an infinite loop, polling the sensor's status registers.
- When a 6D orientation event is detected (6D_IA flag), the code prints the new orientation over UART, indicating which axis and direction (XH, XL, YH, YL, ZH, ZL) the device has switched to.

---

## Usage

1. Connect the IIS2DLPC sensor to the STM32F401RE Nucleo board via I2C1 (PB8/SCL, PB9/SDA).
2. Build and flash the firmware onto the STM32F401RE board.
3. Open a serial terminal at 115200 baud on the USART2 TX line.
4. Rotate or tilt the sensor to cross the 60-degree threshold on any axis.
5. Observe messages such as "6D or. switched to XH", "6D or. switched to YL", etc., printed over UART, indicating the detected orientation.

---

## Notes

- 6D orientation detection allows the sensor to determine which of its axes are aligned with gravity, useful for applications like screen rotation or device positioning.
- The threshold can be adjusted to make the orientation detection more or less sensitive.
- The LPF2 filter improves detection reliability by reducing the effect of high-frequency noise.
- UART output uses blocking writes.
- The environment is `#![no_std]` and `#![no_main]` for embedded Rust applications.
- Panic behavior is set to halt on panic using `panic_itm`.

---

## References

- [STM32F401RE Nucleo-64 Board](https://www.st.com/en/evaluation-tools/nucleo-f401re.html)
- [IIS2DLPC Datasheet](https://www.st.com/resource/en/datasheet/iis2dlpc.pdf)
- [stm32f4xx-hal Rust crate](https://docs.rs/stm32f4xx-hal)

---

*This README explains the embedded Rust program for 6D orientation detection using the IIS2DLPC accelerometer sensor on STM32F401RE.*
