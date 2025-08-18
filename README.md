# iis2dlpc-rs
[![Crates.io][crates-badge]][crates-url]
[![BSD 3-Clause licensed][bsd-badge]][bsd-url]

[crates-badge]: https://img.shields.io/crates/v/iis2dlpc-rs
[crates-url]: https://crates.io/crates/iis2dlpc-rs
[bsd-badge]: https://img.shields.io/crates/l/iis2dlpc-rs
[bsd-url]: https://opensource.org/licenses/BSD-3-Clause

Provides a platform-agnostic, no_std-compatible driver for the ST IIS2DLPC sensor, supporting both I2C and SPI communication interfaces.

## Sensor Overview

The IIS2DLPC is an ultra-low-power high-performance
three-axis linear accelerometer with digital I²C/SPI
output interface which leverages on the robust and
mature manufacturing processes already used for the
production of micromachined accelerometers.

The IIS2DLPC has user-selectable full scales of
±2g/±4g/±8g/±16g and is capable of measuring
accelerations with output data rates from 1.6 Hz to
1600 Hz.

The IIS2DLPC has one high-performance mode and 4
low-power modes which can be changed on the fly,
providing outstanding versatility and adaptability to the
requirements of the application.
The IIS2DLPC has an integrated 32-level first-in, first-out
(FIFO) buffer allowing the user to store data in order to
limit intervention by the host processor.

The embedded self-test capability allows the user to
check the functioning of the sensor in the final
application.

The IIS2DLPC has a dedicated internal engine to
process motion and acceleration detection including
free-fall, wakeup, highly configurable single/double-tap
recognition, activity/inactivity, stationary/motion
detection, portrait/landscape detection and 6D/4D
orientation.

The IIS2DLPC is available in a small thin plastic land
grid array package (LGA) and it is guaranteed to operate
over an extended temperature range from -40 °C to
+85 °C.

For more info, please visit the device page at [https://www.st.com/en/mems-and-sensors/iis2dlpc.html](https://www.st.com/en/mems-and-sensors/iis2dlpc.html)

## Installation

Add the driver to your `Cargo.toml` dependencies:

```toml
[dependencies]
iis2dlpc-rs = "0.1.0"
```

Or, add it directly from the terminal:

```sh
cargo add iis2dlpc-rs
```

## Usage

Include the crate and its prelude
```rust
use iis2dlpc_rs as iis2dlpc;
use iis2dlpc::*;
use iis2dlpc::prelude::*;
```

### Create an instance

Create an instance of the driver with the `new_<bus>` associated function, by passing an I2C (`embedded_hal::i2c::I2c`) instance and I2C address, or an SPI (`embedded_hal::spi::SpiDevice`) instance, along with a timing peripheral.

An example with I2C:

```rust
let mut sensor = Lsm6dsv320x::new_i2c(i2c, I2CAddress::I2cAddL, delay);
```

### Check "Who Am I" Register

This step ensures correct communication with the sensor. It returns a unique ID to verify the sensor's identity.

```rust
let whoami = sensor.device_id_get().unwrap();
if whoami != ID {
    panic!("Invalid sensor ID");
}
```

### Configure

See details in specific examples; the following are common api calls:

```rust
// Restore default configuration
sensor.reset_set().unwrap();
while sensor.reset_get().unwrap() == 1 {}

// Enable Block Data Update
sensor.block_data_update_set(PROPERTY_ENABLE).unwrap();

// Set Full scale
sensor.full_scale_set(Fs::_8g).unwrap();

// Configure power mode
sensor
    .power_mode_set(Mode::ContLowPwrLowNoise12bit)
    .unwrap();

// Set Output Data Rate
sensor.data_rate_set(Odr::_25hz).unwrap();
```

## License

Distributed under the BSD-3 Clause license.

More Information: [http://www.st.com](http://st.com/MEMS).

**Copyright (C) 2025 STMicroelectronics**