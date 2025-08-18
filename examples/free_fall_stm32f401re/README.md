## IIS2DLPC Accelerometer Free-Fall Detection on STM32F401RE Nucleo-64

This example demonstrates how to configure the **IIS2DLPC** accelerometer sensor to detect free-fall events using an **STM32F401RE** microcontroller. When a free-fall condition is detected, the event is reported over UART.

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
- Power mode is set to high performance with low noise.
- Output data rate is set to 200 Hz.
- Full scale is set to ±2g.
- Free-fall detection is configured:
  - **Duration:** Set to 0x06 (number of consecutive samples below threshold to trigger event).
  - **Threshold:** Set to 10 LSB (lowest threshold for free-fall detection).
- Free-fall interrupt is enabled and routed to INT1 pin.
- Interrupt notification is set to latched mode (interrupt remains active until cleared by reading the source register).

### Event Loop

- The program enters an infinite loop, polling the sensor's status registers.
- When a free-fall event is detected (FF_IA flag), "Free fall detected" is printed over UART.

---

## Usage

1. Connect the IIS2DLPC sensor to the STM32F401RE Nucleo board via I2C1 (PB8/SCL, PB9/SDA).
2. Build and flash the firmware onto the STM32F401RE board.
3. Open a serial terminal at 115200 baud on the USART2 TX line.
4. Drop or move the sensor to simulate a free-fall event.
5. Observe "Free fall detected" messages printed over UART when a free-fall is detected.

---

## Notes

- Free-fall detection is based on the sensor's ability to detect when acceleration on all axes falls below a set threshold for a specified duration.
- The threshold and duration can be tuned to adjust sensitivity and response time.
- UART output uses blocking writes.
- The environment is `#![no_std]` and `#![no_main]` for embedded Rust applications.
- Panic behavior is set to halt on panic using `panic_itm`.

---

## References

- [STM32F401RE Nucleo-64 Board](https://www.st.com/en/evaluation-tools/nucleo-f401re.html)
- [IIS2DLPC Datasheet](https://www.st.com/resource/en/datasheet/iis2dlpc.pdf)
- [stm32f4xx-hal Rust crate](https://docs.rs/stm32f4xx-hal)

---

*This README explains the embedded Rust program for free-fall detection using the IIS2DLPC accelerometer sensor on STM32F401RE.*
