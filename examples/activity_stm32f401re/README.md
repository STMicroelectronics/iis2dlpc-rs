# IIS2DLPC Accelerometer Activity/Inactivity Detection on STM32F401RE Nucleo-64

This example demonstrates how to configure and use the **IIS2DLPC** accelerometer sensor on an **STM32F401RE** microcontroller to detect activity and inactivity events. The sensor is set up to generate interrupts on wake-up (activity) and sleep (inactivity) states, which are reported via UART.

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

- The microcontroller peripherals and clocks are initialized, setting the system clock to use an 8 MHz external crystal.
- GPIO pins PB8 and PB9 are configured for I2C1 communication.
- GPIO pin PA2 is configured for USART2 TX for serial output.
- I2C1 is configured in standard mode at 100 kHz.
- USART2 is configured for 115200 baud, 8 data bits, no parity.
- A delay abstraction is created using the system timer.

### Sensor Setup

- The IIS2DLPC sensor is initialized over I2C with the high I2C address.
- The device ID is read and verified; if mismatched, the program panics.
- The sensor is reset to default configuration and the code waits until reset completes.
- The full scale is set to ±2g.
- The accelerometer filtering chain is configured with low-pass filter on output and bandwidth set to ODR/4.
- Power mode is set to continuous low power with low noise and 12-bit resolution.
- Wake-up duration and sleep duration are configured to define timing for activity/inactivity detection.
- Activity wake-up threshold is set to a low value to detect motion.
- Wake-up feed data is configured to use high-pass filtered data.
- Activity mode is set to detect both activity and inactivity.
- Interrupt routing is configured to enable wake-up interrupt on INT1 pin.
- Output data rate is set to 200 Hz.

### Event Loop

- The program enters an infinite loop where it reads the sensor's all-sources status register.
- It checks the wake-up source flags for sleep state (inactivity) and wake-up (activity) events.
- When inactivity is detected, it prints "Inactivity Detected" over UART.
- When activity is detected, it prints "Activity Detected" over UART.

---

## Usage

1. Connect the IIS2DLPC sensor to the STM32F401RE Nucleo board via I2C1 (PB8/SCL, PB9/SDA).
2. Build and flash the firmware onto the STM32F401RE board.
3. Open a serial terminal at 115200 baud on the USART2 TX line.
4. Move or keep the sensor still to trigger activity and inactivity events.
5. Observe the corresponding messages printed over UART.

---

## Notes

- The example uses polling to check for activity/inactivity events by reading sensor status registers.
- The sensor's internal wake-up and sleep detection features are used to generate events.
- UART output uses blocking writes.
- The environment is `#![no_std]` and `#![no_main]` for embedded Rust applications.
- Panic behavior is set to halt on panic using `panic_itm`.
- The wake-up threshold and durations can be tuned to adjust sensitivity and timing.

---

## References

- [STM32F401RE Nucleo-64 Board](https://www.st.com/en/evaluation-tools/nucleo-f401re.html)
- [IIS2DLPC Datasheet](https://www.st.com/resource/en/datasheet/iis2dlpc.pdf)
- [stm32f4xx-hal Rust crate](https://docs.rs/stm32f4xx-hal)

---

*This README explains the embedded Rust program for detecting activity and inactivity events using the IIS2DLPC accelerometer sensor on STM32F401RE.*
