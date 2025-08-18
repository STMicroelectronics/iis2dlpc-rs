# IIS2DLPC Free-Fall Detection on STM32F401RE Nucleo-64 Using Embassy Framework

This example demonstrates how to configure the **IIS2DLPC** ultra-low-power accelerometer for **free-fall detection** on an **STM32F401RE Nucleo-64** board. The sensor is set up to generate an interrupt when a free-fall event is detected, and the event is reported over UART.

The code is written in Rust using the [Embassy](https://embassy.dev/) async runtime, the `embassy-stm32` hardware abstraction layer, and the `iis2dlpc` sensor driver crate. It showcases sensor initialization, configuration for free-fall detection, and event reporting via UART.

---

## Hardware Setup

- **Microcontroller Board:** STM32F401RE Nucleo-64
- **Sensor:** IIS2DLPC Accelerometer (I2C interface)
- **Communication Interface:** I2C1 at 100 kHz Standard Mode
- **UART:** USART2 for serial output at 115200 baud
- **Interrupt Pin:** INT1 (optional, not used in this polling example)

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
- Power mode is set to high-performance, low-noise mode.
- Output Data Rate (ODR) is set to 200 Hz for responsive detection.
- Full scale is set to ±2g for acceleration measurements.
- Free-fall duration and threshold are configured for sensitivity.
- Free-fall interrupt is enabled and routed to INT1 (though this example uses polling).
- Interrupts are set to latched mode for reliable event capture.

### Event Polling Loop

- The main loop continuously polls the sensor’s status registers.
- If a free-fall event is detected, a message "Free fall detected" is sent over UART.
- UART writes are blocking for simplicity.

---

## Usage

1. Connect the IIS2DLPC sensor to the STM32F401RE Nucleo board via I2C1 (PB8/SCL, PB9/SDA).
2. (Optional) Connect the sensor’s INT1 pin to a GPIO if you wish to use interrupts.
3. Build and flash the firmware onto the STM32F401RE board.
4. Open a serial terminal at 115200 baud on the USART2 TX line (PA2).
5. Drop or shake the sensor to trigger a free-fall event.
6. Observe "Free fall detected" messages printed over UART.

---

## Notes

- This example uses polling to check for free-fall events. For lower power consumption and faster response, consider using GPIO interrupts.
- The free-fall duration and threshold can be tuned for your specific application and sensitivity requirements.
- The environment is `#![no_std]` and `#![no_main]` for embedded Rust applications using the Embassy runtime.
- Panic behavior is set to use `panic_probe` and `defmt` for debugging.

---

## References

- [STM32F401RE Nucleo-64 Board](https://www.st.com/en/evaluation-tools/nucleo-f401re.html)
- [IIS2DLPC Datasheet](https://www.st.com/resource/en/datasheet/iis2dlpc.pdf)
- [Embassy STM32 HAL](https://github.com/embassy-rs/embassy)

---

*This README provides a detailed explanation of the embedded Rust program for free-fall detection using the IIS2DLPC sensor on STM32F401RE with UART output, leveraging the Embassy async framework.*
