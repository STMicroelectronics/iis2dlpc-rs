#![no_std]
#![doc = include_str!("../README.md")]

use core::fmt::Debug;
use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::{I2c, SevenBitAddress};
use embedded_hal::spi::SpiDevice;
use st_mems_bus::BusOperation;

use prelude::*;

pub mod prelude;
pub mod register;

/// The Iis2dlpc generic driver struct.
pub struct Iis2dlpc<B, T> {
    /// The bus driver.
    pub bus: B,
    pub tim: T,
}

/// Driver errors.
#[derive(Debug)]
pub enum Error<B> {
    Bus(B),          // Error at the bus level
    WhoAmIError(u8), // Incorrect Iis2dlpc identifier
    UnexpectedValue, // Unexpected value read from a register
}

impl<P, T> Iis2dlpc<st_mems_bus::i2c::I2cBus<P>, T>
where
    P: I2c,
    T: DelayNs,
{
    /// Constructor method for using the I2C bus.
    ///
    /// # Arguments
    ///
    /// * `i2c`: The I2C peripheral.
    /// * `address`: The I2C address of the Iis2dlpc sensor.
    ///
    /// # Returns
    ///
    /// * `Result`
    ///     * `Self`: Returns an instance of `Iis2dlpc`.
    ///     * `Err`: Returns an error if the initialization fails.
    pub fn new_i2c(i2c: P, address: I2CAddress, tim: T) -> Self {
        // Initialize the I2C bus with the Iis2dlpc address
        let bus = st_mems_bus::i2c::I2cBus::new(i2c, address as SevenBitAddress);
        Self { bus, tim }
    }
}

impl<P, T> Iis2dlpc<st_mems_bus::spi::SpiBus<P>, T>
where
    P: SpiDevice,
    T: DelayNs,
{
    /// Constructor method for using the SPI bus.
    ///
    /// # Arguments
    ///
    /// * `spi`: The SPI peripheral.
    ///
    /// # Returns
    ///
    /// * `Result`
    ///     * `Self`: Returns an instance of `Iis2dlpc`.
    ///     * `Err`: Returns an error if the initialization fails.
    pub fn new_spi(spi: P, tim: T) -> Self {
        // Initialize the SPI bus
        let bus = st_mems_bus::spi::SpiBus::new(spi);
        Self { bus, tim }
    }
}

impl<B: BusOperation, T: DelayNs> Iis2dlpc<B, T> {
    /// # Arguments
    ///
    /// * `bus`: The bus that implements BusOperation.
    /// * `tim`: The timer of the COMPONENT sensor.
    ///
    /// # Returns
    ///
    /// * `Self`: Returns an instance of `Iis2mdc`.
    #[inline]
    pub fn from_bus(bus: B, tim: T) -> Self {
        Self { bus, tim }
    }

    #[inline]
    pub fn read_from_register(&mut self, reg: u8, buf: &mut [u8]) -> Result<(), Error<B::Error>> {
        self.bus.read_from_register(reg, buf).map_err(Error::Bus)
    }

    #[inline]
    pub fn write_to_register(&mut self, reg: u8, buf: &[u8]) -> Result<(), Error<B::Error>> {
        self.bus.write_to_register(reg, buf).map_err(Error::Bus)
    }

    /// Set the accelerometer operating mode.
    ///
    /// This function configures the accelerometer's operating mode by updating the `mode` and `lp_mode` fields in the `CTRL1` register,
    /// and the `low_noise` field in the `CTRL6` register.
    ///
    /// ### Arguments
    /// - `val`: A [`Mode`] value representing the desired operating mode. This includes settings for:
    ///   - `mode`: Operating mode.
    ///   - `lp_mode`: Low-power mode configuration.
    ///   - `low_noise`: Low-noise mode configuration.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn power_mode_set(&mut self, val: Mode) -> Result<(), Error<B::Error>> {
        let mut ctrl1 = Ctrl1::read(self)?;
        ctrl1.set_mode(val.mode());
        ctrl1.set_lp_mode(val.lp_mode());
        ctrl1.write(self)?;

        let mut ctrl6 = Ctrl6::read(self)?;
        ctrl6.set_low_noise(val.low_noise());
        ctrl6.write(self)
    }

    /// Get the accelerometer operating mode.
    ///
    /// This function retrieves the current operating mode of the accelerometer by reading the `mode` and `lp_mode` fields from the `CTRL1` register,
    /// and the `low_noise` field from the `CTRL6` register.
    ///
    /// ### Returns
    /// - `Ok(Mode)`: The current operating mode, represented as a [`Mode`] value.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn power_mode_get(&mut self) -> Result<Mode, Error<B::Error>> {
        let ctrl1 = Ctrl1::read(self)?;
        let ctrl6 = Ctrl6::read(self)?;

        Ok(Mode::new(ctrl1.mode(), ctrl1.lp_mode(), ctrl6.low_noise()))
    }

    /// Set the accelerometer data rate.
    ///
    /// This function configures the accelerometer's data rate by updating the `odr` field in the `CTRL1` register,
    /// and the `slp_mode` field in the `CTRL3` register.
    ///
    /// ### Arguments
    /// - `val`: A [`Odr`] value representing the desired data rate and sleep mode configuration.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn data_rate_set(&mut self, val: Odr) -> Result<(), Error<B::Error>> {
        let mut ctrl1 = Ctrl1::read(self)?;
        ctrl1.set_odr(val.odr());
        ctrl1.write(self)?;

        let mut ctrl3 = Ctrl3::read(self)?;
        ctrl3.set_slp_mode(val.slp_mode());
        ctrl3.write(self)
    }

    /// Get the accelerometer data rate.
    ///
    /// This function retrieves the current data rate of the accelerometer by reading the `odr` field from the `CTRL1` register,
    /// and the `slp_mode` field from the `CTRL3` register.
    ///
    /// ### Returns
    /// - `Ok(Odr)`: The current data rate, represented as an [`Odr`] value.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn data_rate_get(&mut self) -> Result<Odr, Error<B::Error>> {
        let ctrl1 = Ctrl1::read(self)?;
        let ctrl3 = Ctrl3::read(self)?;

        Ok(Odr::new(ctrl1.odr(), ctrl3.slp_mode()))
    }

    /// Set the block data update (BDU) configuration.
    ///
    /// This function configures the block data update (BDU) setting by updating the `bdu` field in the `CTRL2` register.
    /// When BDU is enabled, the output registers are not updated until both the high and low parts are read, ensuring data consistency.
    ///
    /// ### Arguments
    /// - `val`: The desired BDU value:
    ///   - `0`: Continuous update.
    ///   - `1`: Output registers not updated until MSB and LSB are read.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn block_data_update_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut ctrl2 = Ctrl2::read(self)?;
        ctrl2.set_bdu(val);
        ctrl2.write(self)
    }

    /// Get the block data update (BDU) configuration.
    ///
    /// This function retrieves the current block data update (BDU) setting from the `CTRL2` register.
    ///
    /// ### Returns
    /// - `Ok(u8)`: The current BDU value:
    ///   - `0`: Continuous update.
    ///   - `1`: Output registers not updated until MSB and LSB are read.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn block_data_update_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(Ctrl2::read(self)?.bdu())
    }

    /// Set the accelerometer full-scale selection.
    ///
    /// This function configures the full-scale range of the accelerometer by updating the `fs` field in the `CTRL6` register.
    /// The full-scale range determines the maximum measurable acceleration.
    ///
    /// ### Arguments
    /// - `val`: A [`Fs`] value representing the desired full-scale range:
    ///   - `Fs2g`: ±2g (default).
    ///   - `Fs4g`: ±4g.
    ///   - `Fs8g`: ±8g.
    ///   - `Fs16g`: ±16g.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn full_scale_set(&mut self, val: Fs) -> Result<(), Error<B::Error>> {
        let mut ctrl6 = Ctrl6::read(self)?;
        ctrl6.set_fs(val as u8);
        ctrl6.write(self)
    }

    /// Get the accelerometer full-scale selection.
    ///
    /// This function retrieves the current full-scale range of the accelerometer from the `CTRL6` register.
    ///
    /// ### Returns
    /// - `Ok(Fs)`: The current full-scale range as a [`Fs`] value.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn full_scale_get(&mut self) -> Result<Fs, Error<B::Error>> {
        Ok(Fs::try_from(Ctrl6::read(self)?.fs()).unwrap_or_default())
    }

    /// Get the status register.
    ///
    /// This function retrieves the current status of the device by reading the `STATUS` register.
    /// The `STATUS` register provides information about various events, such as data-ready, free-fall detection, and tap detection.
    ///
    /// ### Returns
    /// - `Ok(Status)`: The current status as a [`Status`] struct, which represents the union of registers from `STATUS`.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn status_reg_get(&mut self) -> Result<Status, Error<B::Error>> {
        Status::read(self)
    }

    /// Get the accelerometer new data availability flag.
    ///
    /// This function checks whether new accelerometer data is available by reading the `drdy` field in the `STATUS` register.
    ///
    /// ### Returns
    /// - `Ok(u8)`: The value of the `drdy` field:
    ///   - `0`: No new data available.
    ///   - `1`: New data is available.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation..
    pub fn flag_data_ready_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(self.status_reg_get()?.drdy())
    }

    /// Get all interrupt and status flags of the device.
    ///
    /// This function retrieves the status of all interrupt and status flags by reading the following registers:
    /// - `STATUS_DUP`
    /// - `WAKE_UP_SRC`
    /// - `TAP_SRC`
    /// - `SIXD_SRC`
    /// - `ALL_INT_SRC`
    ///
    /// ### Returns
    /// - `Ok(AllSources)`: A struct containing the values of all the above registers.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn all_sources_get(&mut self) -> Result<AllSources, Error<B::Error>> {
        Ok(AllSources {
            status_dup: StatusDup::read(self)?,
            wake_up_src: WakeUpSrc::read(self)?,
            tap_src: TapSrc::read(self)?,
            sixd_src: SixdSrc::read(self)?,
            all_int_src: AllIntSrc::read(self)?,
        })
    }

    /// Set the X-axis user offset correction.
    ///
    /// This function configures the X-axis user offset correction value in the `X_OFS_USR` register.
    /// The value's weight depends on the `USR_OFF_W` bit in the `CTRL7` register.
    ///
    /// ### Arguments
    /// - `val`: The X-axis user offset correction value to set.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the write operation.
    pub fn usr_offset_x_set(&mut self, val: i8) -> Result<(), Error<B::Error>> {
        XOfsUsr::from_bits(val.cast_unsigned()).write(self)
    }

    /// Get the X-axis user offset correction.
    ///
    /// This function retrieves the X-axis user offset correction value from the `X_OFS_USR` register.
    /// The value's weight depends on the `USR_OFF_W` bit in the `CTRL7` register.
    ///
    /// ### Returns
    /// - `Ok(i8)`: The X-axis user offset correction value.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn usr_offset_x_get(&mut self) -> Result<i8, Error<B::Error>> {
        Ok(XOfsUsr::read(self)?.x_ofs_usr())
    }

    /// Set the Y-axis user offset correction.
    ///
    /// This function configures the Y-axis user offset correction value in the `Y_OFS_USR` register.
    /// The value's weight depends on the `USR_OFF_W` bit in the `CTRL7` register.
    ///
    /// ### Arguments
    /// - `val`: The Y-axis user offset correction value to set.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the write operation.
    pub fn usr_offset_y_set(&mut self, val: i8) -> Result<(), Error<B::Error>> {
        YOfsUsr::from_bits(val.cast_unsigned()).write(self)
    }

    /// Get the Y-axis user offset correction.
    ///
    /// This function retrieves the Y-axis user offset correction value from the `Y_OFS_USR` register.
    /// The value's weight depends on the `USR_OFF_W` bit in the `CTRL7` register.
    ///
    /// ### Returns
    /// - `Ok(i8)`: The Y-axis user offset correction value.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn usr_offset_y_get(&mut self) -> Result<i8, Error<B::Error>> {
        Ok(YOfsUsr::read(self)?.y_ofs_usr())
    }

    /// Set the Z-axis user offset correction.
    ///
    /// This function configures the Z-axis user offset correction value in the `Z_OFS_USR` register.
    /// The value's weight depends on the `USR_OFF_W` bit in the `CTRL7` register.
    ///
    /// ### Arguments
    /// - `val`: The Z-axis user offset correction value to set.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the write operation.
    pub fn usr_offset_z_set(&mut self, val: i8) -> Result<(), Error<B::Error>> {
        ZOfsUsr::from_bits(val.cast_unsigned()).write(self)
    }

    /// Get the Z-axis user offset correction.
    ///
    /// This function retrieves the Z-axis user offset correction value from the `Z_OFS_USR` register.
    /// The value's weight depends on the `USR_OFF_W` bit in the `CTRL7` register.
    ///
    /// ### Returns
    /// - `Ok(i8)`: The Z-axis user offset correction value.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn usr_offset_z_get(&mut self) -> Result<i8, Error<B::Error>> {
        Ok(ZOfsUsr::read(self)?.z_ofs_usr())
    }

    /// Set the weight of XL user offset bits.
    ///
    /// This function configures the weight of the user offset bits in the `X_OFS_USR`, `Y_OFS_USR`, and `Z_OFS_USR` registers by updating the `usr_off_w` field in the `CTRL7` register.
    ///
    /// ### Arguments
    /// - `val`: A [`UsrOffW`] value representing the desired weight:
    ///   - `Lsb977ug`: 977 μg/LSB (default).
    ///   - `Lsb15mg6`: 15.6 mg/LSB.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation..
    pub fn offset_weight_set(&mut self, val: UsrOffW) -> Result<(), Error<B::Error>> {
        let mut ctrl7 = Ctrl7::read(self)?;
        ctrl7.set_usr_off_w(val as u8);
        ctrl7.write(self)
    }

    /// Get the weight of XL user offset bits.
    ///
    /// This function retrieves the weight of the user offset bits from the `usr_off_w` field in the `CTRL7` register.
    ///
    /// ### Returns
    /// - `Ok(UsrOffW)`: The current weight of the user offset bits:
    ///   - `Lsb977ug`: 977 μg/LSB (default).
    ///   - `Lsb15mg6`: 15.6 mg/LSB.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation..
    pub fn offset_weight_get(&mut self) -> Result<UsrOffW, Error<B::Error>> {
        Ok(UsrOffW::try_from(Ctrl7::read(self)?.usr_off_w()).unwrap_or_default())
    }

    /// Get the raw temperature data.
    ///
    /// This function retrieves the raw temperature data from the `OUT_T_L` and `OUT_T_H` registers.
    /// The value is expressed as a 16-bit word in two's complement format.
    ///
    /// ### Returns
    /// - `Ok(i16)`: The raw temperature data.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn temperature_raw_get(&mut self) -> Result<i16, Error<B::Error>> {
        Ok(OutT::read(self)?.temp())
    }

    /// Get the raw acceleration data.
    ///
    /// This function retrieves the raw acceleration data for the X, Y, and Z axes from the `OUT_X_L`, `OUT_X_H`, `OUT_Y_L`, `OUT_Y_H`, `OUT_Z_L`, and `OUT_Z_H` registers.
    /// The values are expressed as 16-bit words in two's complement format.
    ///
    /// ### Returns
    /// - `Ok([i16; 3])`: An array containing the raw acceleration data for the X, Y, and Z axes.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn acceleration_raw_get(&mut self) -> Result<[i16; 3], Error<B::Error>> {
        Ok([
            OutX::read(self)?.x(),
            OutY::read(self)?.y(),
            OutZ::read(self)?.z(),
        ])
    }

    /// Get the device ID.
    ///
    /// This function retrieves the device ID from the `WHO_AM_I` register.
    /// The device ID is a fixed value that identifies the IIS2DLPC sensor.
    ///
    /// ### Returns
    /// - `Ok(u8)`: The device ID (expected value: `0x44`).
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation..
    pub fn device_id_get(&mut self) -> Result<u8, Error<B::Error>> {
        let mut buff: [u8; 1] = [0];
        self.read_from_register(Reg::WhoAmI as u8, &mut buff)?;
        Ok(buff[0])
    }

    /// Enable or disable automatic register address increment.
    ///
    /// This function configures the automatic register address increment feature by updating the `if_add_inc` field in the `CTRL2` register.
    /// When enabled, the register address is automatically incremented during multiple-byte access.
    ///
    /// ### Arguments
    /// - `val`: The desired value for the `if_add_inc` field:
    ///   - `0`: Disable automatic increment.
    ///   - `1`: Enable automatic increment.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn auto_increment_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut ctrl2 = Ctrl2::read(self)?;
        ctrl2.set_if_add_inc(val);
        ctrl2.write(self)
    }

    /// Get the automatic register address increment configuration.
    ///
    /// This function retrieves the current value of the `if_add_inc` field from the `CTRL2` register.
    ///
    /// ### Returns
    /// - `Ok(u8)`: The current value of the `if_add_inc` field:
    ///   - `0`: Automatic increment is disabled.
    ///   - `1`: Automatic increment is enabled.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn auto_increment_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(Ctrl2::read(self)?.if_add_inc())
    }

    /// Perform a software reset.
    ///
    /// This function performs a software reset by updating the `soft_reset` field in the `CTRL2` register.
    /// A software reset restores the default values in all user registers.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn reset_set(&mut self) -> Result<(), Error<B::Error>> {
        let mut ctrl2 = Ctrl2::read(self)?;
        ctrl2.set_soft_reset(PROPERTY_ENABLE);
        ctrl2.write(self)
    }

    /// Get the software reset status.
    ///
    /// This function retrieves the current value of the `soft_reset` field from the `CTRL2` register.
    /// The value indicates whether a software reset has been performed.
    ///
    /// ### Returns
    /// - `Ok(u8)`: The current value of the `soft_reset` field:
    ///   - `0`: No reset in progress.
    ///   - `1`: Reset in progress.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn reset_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(Ctrl2::read(self)?.soft_reset())
    }

    /// Reboot memory content and reload calibration parameters.
    ///
    /// This function triggers a reboot of the device's memory content by updating the `boot` field in the `CTRL2` register.
    /// The reboot operation reloads the calibration parameters from non-volatile memory.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn boot_set(&mut self) -> Result<(), Error<B::Error>> {
        let mut ctrl2 = Ctrl2::read(self)?;
        ctrl2.set_boot(PROPERTY_ENABLE);
        ctrl2.write(self)
    }

    /// Get the reboot memory content status.
    ///
    /// This function retrieves the current value of the `boot` field from the `CTRL2` register.
    /// The value indicates whether a reboot operation is in progress.
    ///
    /// ### Returns
    /// - `Ok(u8)`: The current value of the `boot` field:
    ///   - `0`: No reboot in progress.
    ///   - `1`: Reboot in progress.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn boot_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(Ctrl2::read(self)?.boot())
    }

    /// Enable or disable the sensor self-test.
    ///
    /// This function configures the self-test mode of the sensor by updating the `st` field in the `CTRL3` register.
    /// The self-test mode allows verifying the functionality of the sensor without external stimuli.
    ///
    /// ### Arguments
    /// - `val`: A [`St`] value representing the desired self-test mode:
    ///   - `XlStDisable`: Self-test disabled (default).
    ///   - `XlStPositive`: Positive sign self-test.
    ///   - `XlStNegative`: Negative sign self-test.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn self_test_set(&mut self, val: St) -> Result<(), Error<B::Error>> {
        let mut ctrl3 = Ctrl3::read(self)?;
        ctrl3.set_st(val as u8);
        ctrl3.write(self)
    }

    /// Get the sensor self-test mode.
    ///
    /// This function retrieves the current self-test mode of the sensor from the `st` field in the `CTRL3` register.
    ///
    /// ### Returns
    /// - `Ok(St)`: The current self-test mode as a [`St`] value:
    ///   - `XlStDisable`: Self-test disabled (default).
    ///   - `XlStPositive`: Positive sign self-test.
    ///   - `XlStNegative`: Negative sign self-test.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn self_test_get(&mut self) -> Result<St, Error<B::Error>> {
        Ok(St::try_from(Ctrl3::read(self)?.st()).unwrap_or_default())
    }

    /// Set the data-ready interrupt mode.
    ///
    /// This function configures the data-ready interrupt mode by updating the `drdy_pulsed` field in the `CTRL7` register.
    /// The data-ready interrupt can be configured as either latched or pulsed mode.
    ///
    /// ### Arguments
    /// - `val`: A [`DrdyPulsed`] value representing the desired data-ready interrupt mode:
    ///   - `Latched`: Latched mode (default).
    ///   - `Pulsed`: Pulsed mode.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn data_ready_mode_set(&mut self, val: DrdyPulsed) -> Result<(), Error<B::Error>> {
        let mut ctrl7 = Ctrl7::read(self)?;
        ctrl7.set_drdy_pulsed(val as u8);
        ctrl7.write(self)
    }

    /// Get the data-ready interrupt mode.
    ///
    /// This function retrieves the current data-ready interrupt mode from the `drdy_pulsed` field in the `CTRL7` register.
    ///
    /// ### Returns
    /// - `Ok(DrdyPulsed)`: The current data-ready interrupt mode as a [`DrdyPulsed`] value:
    ///   - `Latched`: Latched mode (default).
    ///   - `Pulsed`: Pulsed mode.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn data_ready_mode_get(&mut self) -> Result<DrdyPulsed, Error<B::Error>> {
        Ok(DrdyPulsed::try_from(Ctrl7::read(self)?.drdy_pulsed()).unwrap_or_default())
    }

    /// Set the accelerometer filtering path for outputs.
    ///
    /// This function configures the filtering path for accelerometer outputs by updating the `fds` field in the `CTRL6` register
    /// and the `usr_off_on_out` field in the `CTRL7` register.
    ///
    /// ### Arguments
    /// - `val`: A [`Fds`] value representing the desired filtering path:
    ///   - `LpfOnOut`: Low-pass filter on output (default).
    ///   - `UserOffsetOnOut`: User offset on output.
    ///   - `HighPassOnOut`: High-pass filter on output.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn filter_path_set(&mut self, val: Fds) -> Result<(), Error<B::Error>> {
        let mut ctrl6 = Ctrl6::read(self)?;
        ctrl6.set_fds(val.fds());
        ctrl6.write(self)?;

        let mut ctrl7 = Ctrl7::read(self)?;
        ctrl7.set_usr_off_on_out(val.usr_off_on_out());
        ctrl7.write(self)
    }

    /// Get the accelerometer filtering path for outputs.
    ///
    /// This function retrieves the current filtering path for accelerometer outputs by reading the `fds` field from the `CTRL6` register
    /// and the `usr_off_on_out` field from the `CTRL7` register.
    ///
    /// ### Returns
    /// - `Ok(Fds)`: The current filtering path as a [`Fds`] value:
    ///   - `LpfOnOut`: Low-pass filter on output (default).
    ///   - `UserOffsetOnOut`: User offset on output.
    ///   - `HighPassOnOut`: High-pass filter on output.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn filter_path_get(&mut self) -> Result<Fds, Error<B::Error>> {
        let ctrl6 = Ctrl6::read(self)?;
        let ctrl7 = Ctrl7::read(self)?;

        Ok(Fds::new(ctrl6.fds(), ctrl7.usr_off_on_out()))
    }

    /// Set the accelerometer cutoff filter frequency.
    ///
    /// This function configures the cutoff frequency for the accelerometer's low-pass or high-pass filter by updating the `bw_filt` field in the `CTRL6` register.
    ///
    /// ### Arguments
    /// - `val`: A [`BwFilt`] value representing the desired cutoff frequency:
    ///   - `OdrDiv2`: ODR/2 (default).
    ///   - `OdrDiv4`: ODR/4.
    ///   - `OdrDiv10`: ODR/10.
    ///   - `OdrDiv20`: ODR/20.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn filter_bandwidth_set(&mut self, val: BwFilt) -> Result<(), Error<B::Error>> {
        let mut ctrl6 = Ctrl6::read(self)?;
        ctrl6.set_bw_filt(val as u8);
        ctrl6.write(self)
    }

    /// Get the accelerometer cutoff filter frequency.
    ///
    /// This function retrieves the current cutoff frequency for the accelerometer's low-pass or high-pass filter by reading the `bw_filt` field from the `CTRL6` register.
    ///
    /// ### Returns
    /// - `Ok(BwFilt)`: The current cutoff frequency as a [`BwFilt`] value:
    ///   - `OdrDiv2`: ODR/2 (default).
    ///   - `OdrDiv4`: ODR/4.
    ///   - `OdrDiv10`: ODR/10.
    ///   - `OdrDiv20`: ODR/20.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn filter_bandwidth_get(&mut self) -> Result<BwFilt, Error<B::Error>> {
        Ok(BwFilt::try_from(Ctrl6::read(self)?.bw_filt()).unwrap_or_default())
    }

    /// Enable or disable the high-pass filter reference mode.
    ///
    /// This function configures the high-pass filter reference mode by updating the `hp_ref_mode` field in the `CTRL7` register.
    ///
    /// ### Arguments
    /// - `val`: The desired value for the `hp_ref_mode` field:
    ///   - `0`: Disable high-pass filter reference mode.
    ///   - `1`: Enable high-pass filter reference mode.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn reference_mode_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut ctrl7 = Ctrl7::read(self)?;
        ctrl7.set_hp_ref_mode(val);
        ctrl7.write(self)
    }

    /// Get the high-pass filter reference mode status.
    ///
    /// This function retrieves the current status of the high-pass filter reference mode from the `hp_ref_mode` field in the `CTRL7` register.
    ///
    /// ### Returns
    /// - `Ok(u8)`: The current value of the `hp_ref_mode` field:
    ///   - `0`: High-pass filter reference mode is disabled.
    ///   - `1`: High-pass filter reference mode is enabled.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn reference_mode_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(Ctrl7::read(self)?.hp_ref_mode())
    }

    /// Set the SPI serial interface mode.
    ///
    /// This function configures the SPI serial interface mode by updating the `sim` field in the `CTRL2` register.
    /// The SPI interface can operate in either 4-wire or 3-wire mode.
    ///
    /// ### Arguments
    /// - `val`: A [`Sim`] value representing the desired SPI mode:
    ///   - `Spi4Wire`: 4-wire SPI mode (default).
    ///   - `Spi3Wire`: 3-wire SPI mode.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn spi_mode_set(&mut self, val: Sim) -> Result<(), Error<B::Error>> {
        let mut ctrl2 = Ctrl2::read(self)?;
        ctrl2.set_sim(val as u8);
        ctrl2.write(self)
    }

    /// Get the SPI serial interface mode.
    ///
    /// This function retrieves the current SPI serial interface mode from the `sim` field in the `CTRL2` register.
    ///
    /// ### Returns
    /// - `Ok(Sim)`: The current SPI mode as a [`Sim`] value:
    ///   - `Spi4Wire`: 4-wire SPI mode (default).
    ///   - `Spi3Wire`: 3-wire SPI mode.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn spi_mode_get(&mut self) -> Result<Sim, Error<B::Error>> {
        Ok(Sim::try_from(Ctrl2::read(self)?.sim()).unwrap_or_default())
    }

    /// Enable or disable the I²C interface.
    ///
    /// This function configures the I²C interface by updating the `i2c_disable` field in the `CTRL2` register.
    /// The I²C interface can be enabled or disabled based on the provided value.
    ///
    /// ### Arguments
    /// - `val`: A [`I2cDisable`] value representing the desired I²C interface state:
    ///   - `I2cEnable`: Enable the I²C interface (default).
    ///   - `I2cDisable`: Disable the I²C interface.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn i2c_interface_set(&mut self, val: I2cDisable) -> Result<(), Error<B::Error>> {
        let mut ctrl2 = Ctrl2::read(self)?;
        ctrl2.set_i2c_disable(val as u8);
        ctrl2.write(self)
    }

    /// Get the I²C interface state.
    ///
    /// This function retrieves the current state of the I²C interface from the `i2c_disable` field in the `CTRL2` register.
    ///
    /// ### Returns
    /// - `Ok(I2cDisable)`: The current I²C interface state as a [`I2cDisable`] value:
    ///   - `I2cEnable`: I²C interface is enabled (default).
    ///   - `I2cDisable`: I²C interface is disabled.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn i2c_interface_get(&mut self) -> Result<I2cDisable, Error<B::Error>> {
        Ok(I2cDisable::try_from(Ctrl2::read(self)?.i2c_disable()).unwrap_or_default())
    }

    /// Configure the CS pull-up resistor.
    ///
    /// This function configures the CS pull-up resistor by updating the `cs_pu_disc` field in the `CTRL2` register.
    /// The pull-up resistor can be connected or disconnected based on the provided value.
    ///
    /// ### Arguments
    /// - `val`: A [`CsPuDisc`] value representing the desired CS pull-up configuration:
    ///   - `PullUpConnect`: Connect the pull-up resistor (default).
    ///   - `PullUpDisconnect`: Disconnect the pull-up resistor.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn cs_mode_set(&mut self, val: CsPuDisc) -> Result<(), Error<B::Error>> {
        let mut ctrl2 = Ctrl2::read(self)?;
        ctrl2.set_cs_pu_disc(val as u8);
        ctrl2.write(self)
    }

    /// Get the CS pull-up resistor configuration.
    ///
    /// This function retrieves the current CS pull-up resistor configuration from the `cs_pu_disc` field in the `CTRL2` register.
    ///
    /// ### Returns
    /// - `Ok(CsPuDisc)`: The current CS pull-up configuration as a [`CsPuDisc`] value:
    ///   - `PullUpConnect`: Pull-up resistor is connected (default).
    ///   - `PullUpDisconnect`: Pull-up resistor is disconnected.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn cs_mode_get(&mut self) -> Result<CsPuDisc, Error<B::Error>> {
        Ok(CsPuDisc::try_from(Ctrl2::read(self)?.cs_pu_disc()).unwrap_or_default())
    }

    /// Interrupt active-high/low.
    ///
    /// # Arguments
    ///
    /// * `val`: change the values of h_lactive in reg CTRL3.
    ///
    /// # Returns
    ///
    /// * `Result`
    ///     * `()`
    ///     * `Err`: Returns an error if the operation fails.
    pub fn pin_polarity_set(&mut self, val: HLactive) -> Result<(), Error<B::Error>> {
        let mut ctrl3 = Ctrl3::read(self)?;
        ctrl3.set_h_lactive(val as u8);
        ctrl3.write(self)
    }

    /// Interrupt active-high/low.
    ///
    /// # Returns
    ///
    /// * `Result`
    ///     * `HLactive`: Get the values of h_lactive in reg CTRL3.
    ///     * `Err`: Returns an error if the operation fails.
    pub fn pin_polarity_get(&mut self) -> Result<HLactive, Error<B::Error>> {
        Ok(HLactive::try_from(Ctrl3::read(self)?.h_lactive()).unwrap_or_default())
    }

    /// Latched/pulsed interrupt.
    ///
    /// # Arguments
    ///
    /// * `val`: change the values of lir in reg CTRL3.
    ///
    /// # Returns
    ///
    /// * `Result`
    ///     * `()`
    ///     * `Err`: Returns an error if the operation fails.
    pub fn int_notification_set(&mut self, val: Lir) -> Result<(), Error<B::Error>> {
        let mut ctrl3 = Ctrl3::read(self)?;
        ctrl3.set_lir(val as u8);
        ctrl3.write(self)
    }

    /// Latched/pulsed interrupt.
    ///
    /// # Arguments
    ///
    /// * `val`: Get the values of lir in reg CTRL3.
    ///
    /// # Returns
    ///
    /// * `Result`
    ///     * `()`
    pub fn int_notification_get(&mut self) -> Result<Lir, Error<B::Error>> {
        Ok(Lir::try_from(Ctrl3::read(self)?.lir()).unwrap_or_default())
    }

    /// Push-pull/open drain selection on interrupt pads.
    ///
    /// # Arguments
    ///
    /// * `val`: change the values of pp_od in reg CTRL3.
    ///
    /// # Returns
    ///
    /// * `Result`
    ///     * `()`
    ///     * `Err`: Returns an error if the operation fails.
    pub fn pin_mode_set(&mut self, val: PpOd) -> Result<(), Error<B::Error>> {
        let mut ctrl3 = Ctrl3::read(self)?;
        ctrl3.set_pp_od(val as u8);
        ctrl3.write(self)
    }

    /// Push-pull/open drain selection on interrupt pads.
    ///
    /// # Returns
    ///
    /// * `Result`
    ///     * `PpOd`: Get the values of pp_od in reg CTRL3.
    ///     * `Err`: Returns an error if the operation fails.
    pub fn pin_mode_get(&mut self) -> Result<PpOd, Error<B::Error>> {
        Ok(PpOd::try_from(Ctrl3::read(self)?.pp_od()).unwrap_or_default())
    }

    /// Select the signal that need to route on int1 pad.
    pub fn pin_int1_route_set(&mut self, val: &Ctrl4Int1PadCtrl) -> Result<(), Error<B::Error>> {
        let ctrl5 = Ctrl5Int2PadCtrl::read(self)?;
        let mut ctrl7: Ctrl7 = Ctrl7::read(self)?;

        if (ctrl5.int2_sleep_state()
            | ctrl5.int2_sleep_chg()
            | val.int1_tap()
            | val.int1_ff()
            | val.int1_wu()
            | val.int1_single_tap()
            | val.int1_6d())
            != 0
        {
            ctrl7.set_interrupts_enable(PROPERTY_ENABLE);
        } else {
            ctrl7.set_interrupts_enable(PROPERTY_DISABLE);
        }

        val.write(self)?;
        ctrl7.write(self)
    }

    /// Select the signal that need to route on int1 pad.
    pub fn pin_int1_route_get(&mut self) -> Result<Ctrl4Int1PadCtrl, Error<B::Error>> {
        Ctrl4Int1PadCtrl::read(self)
    }

    /// Select the signal that need to route on int2 pad.
    pub fn pin_int2_route_set(&mut self, val: &Ctrl5Int2PadCtrl) -> Result<(), Error<B::Error>> {
        let ctrl4 = Ctrl4Int1PadCtrl::read(self)?;
        let mut ctrl7 = Ctrl7::read(self)?;

        if (val.int2_sleep_state()
            | val.int2_sleep_chg()
            | ctrl4.int1_tap()
            | ctrl4.int1_ff()
            | ctrl4.int1_wu()
            | ctrl4.int1_single_tap()
            | ctrl4.int1_6d())
            != 0
        {
            ctrl7.set_interrupts_enable(PROPERTY_ENABLE);
        } else {
            ctrl7.set_interrupts_enable(PROPERTY_DISABLE);
        }

        val.write(self)?;
        ctrl7.write(self)
    }

    /// Select the signal that need to route on int2 pad.
    ///
    /// # Returns
    ///
    /// * `Result`
    ///     * `Ctrl5Int2PadCtrl`: register CTRL5_INT2_PAD_CTRL.
    ///     * `Err`: Returns an error if the operation fails.
    pub fn pin_int2_route_get(&mut self) -> Result<Ctrl5Int2PadCtrl, Error<B::Error>> {
        Ctrl5Int2PadCtrl::read(self)
    }

    /// All interrupt signals become available on INT1 pin.
    ///
    /// # Arguments
    ///
    /// * `val`: Change the values of int2_on_int1 in reg CTRL_REG7.
    ///
    /// # Returns
    ///
    /// * `Result`
    ///     * `()`
    ///     * `Err`: Returns an error if the operation fails.
    pub fn all_on_int1_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut reg = Ctrl7::read(self)?;
        reg.set_int2_on_int1(val);
        reg.write(self)
    }

    /// All interrupt signals become available on INT1 pin.
    ///
    /// # Returns
    ///
    /// * `Result`
    ///     * `u8`: change the values of int2_on_int1 in reg CTRL_REG7.
    ///     * `Err`: Returns an error if the operation fails.
    pub fn all_on_int1_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(Ctrl7::read(self)?.int2_on_int1())
    }

    /// Set the wake-up threshold.
    ///
    /// This function configures the wake-up threshold by updating the `wk_ths` field in the `WAKE_UP_THS` register.
    /// The threshold is expressed in LSB, where 1 LSB = FS_XL / 64.
    ///
    /// ### Arguments
    /// - `val`: The desired wake-up threshold value.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn wkup_threshold_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut reg = WakeUpThs::read(self)?;
        reg.set_wk_ths(val);
        reg.write(self)
    }

    /// Get the wake-up threshold.
    ///
    /// This function retrieves the current wake-up threshold from the `wk_ths` field in the `WAKE_UP_THS` register.
    /// The threshold is expressed in LSB, where 1 LSB = FS_XL / 64.
    ///
    /// ### Returns
    /// - `Ok(u8)`: The current wake-up threshold value.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn wkup_threshold_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(WakeUpThs::read(self)?.wk_ths())
    }

    /// Set the wake-up duration event.
    ///
    /// This function configures the wake-up duration by updating the `wake_dur` field in the `WAKE_UP_DUR` register.
    /// The duration is expressed in LSB, where 1 LSB = 1 / ODR.
    ///
    /// ### Arguments
    /// - `val`: The desired wake-up duration value.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn wkup_dur_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut reg = WakeUpDur::read(self)?;
        reg.set_wake_dur(val);
        reg.write(self)
    }

    /// Get the wake-up duration event.
    ///
    /// This function retrieves the current wake-up duration from the `wake_dur` field in the `WAKE_UP_DUR` register.
    /// The duration is expressed in LSB, where 1 LSB = 1 / ODR.
    ///
    /// ### Returns
    /// - `Ok(u8)`: The current wake-up duration value.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn wkup_dur_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(WakeUpDur::read(self)?.wake_dur())
    }

    /// Set the data sent to the wake-up interrupt function.
    ///
    /// This function configures the data source for the wake-up interrupt function by updating the `usr_off_on_wu` field in the `CTRL7` register.
    /// The data source can be either high-pass filtered data or user offset data.
    ///
    /// ### Arguments
    /// - `val`: A [`UsrOffOnWu`] value representing the desired data source:
    ///   - `HpFeed`: High-pass filtered data (default).
    ///   - `UserOffsetFeed`: User offset data.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn wkup_feed_data_set(&mut self, val: UsrOffOnWu) -> Result<(), Error<B::Error>> {
        let mut reg = Ctrl7::read(self)?;
        reg.set_usr_off_on_wu(val as u8);
        reg.write(self)
    }

    /// Get the data sent to the wake-up interrupt function.
    ///
    /// This function retrieves the current data source for the wake-up interrupt function from the `usr_off_on_wu` field in the `CTRL7` register.
    ///
    /// ### Returns
    /// - `Ok(UsrOffOnWu)`: The current data source as a [`UsrOffOnWu`] value:
    ///   - `HpFeed`: High-pass filtered data (default).
    ///   - `UserOffsetFeed`: User offset data.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn wkup_feed_data_get(&mut self) -> Result<UsrOffOnWu, Error<B::Error>> {
        Ok(UsrOffOnWu::try_from(Ctrl7::read(self)?.usr_off_on_wu()).unwrap_or_default())
    }

    /// Configure activity/inactivity or stationary/motion detection.
    ///
    /// This function configures the activity/inactivity or stationary/motion detection by updating the `sleep_on` field in the `WAKE_UP_THS` register
    /// and the `stationary` field in the `WAKE_UP_DUR` register.
    ///
    /// ### Arguments
    /// - `val`: A [`SleepOn`] value representing the desired detection mode:
    ///   - `NoDetection`: No detection (default).
    ///   - `DetectActInact`: Detect activity/inactivity.
    ///   - `DetectStatMotion`: Detect stationary/motion.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn act_mode_set(&mut self, val: SleepOn) -> Result<(), Error<B::Error>> {
        let mut wake_up_ths = WakeUpThs::read(self)?;
        let mut wake_up_dur: WakeUpDur = WakeUpDur::read(self)?;

        wake_up_ths.set_sleep_on(val.sleep_on());
        wake_up_dur.set_stationary(val.stationary());

        wake_up_ths.write(self)?;
        wake_up_dur.write(self)
    }

    /// Get the activity/inactivity or stationary/motion detection configuration.
    ///
    /// This function retrieves the current detection mode by reading the `sleep_on` field from the `WAKE_UP_THS` register
    /// and the `stationary` field from the `WAKE_UP_DUR` register.
    ///
    /// ### Returns
    /// - `Ok(SleepOn)`: The current detection mode as a [`SleepOn`] value:
    ///   - `NoDetection`: No detection (default).
    ///   - `DetectActInact`: Detect activity/inactivity.
    ///   - `DetectStatMotion`: Detect stationary/motion.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn act_mode_get(&mut self) -> Result<SleepOn, Error<B::Error>> {
        let wake_up_ths = WakeUpThs::read(self)?;
        let wake_up_dur: WakeUpDur = WakeUpDur::read(self)?;

        Ok(SleepOn::new(
            wake_up_ths.sleep_on(),
            wake_up_dur.stationary(),
        ))
    }

    /// Set the duration to enter sleep mode.
    ///
    /// This function configures the duration required to enter sleep mode by updating the `sleep_dur` field in the `WAKE_UP_DUR` register.
    /// The duration is expressed in LSB, where 1 LSB = 512 / ODR.
    ///
    /// ### Arguments
    /// - `val`: The desired sleep duration value.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn act_sleep_dur_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut reg = WakeUpDur::read(self)?;
        reg.set_sleep_dur(val);
        reg.write(self)
    }

    /// Get the duration to enter sleep mode.
    ///
    /// This function retrieves the current sleep duration from the `sleep_dur` field in the `WAKE_UP_DUR` register.
    /// The duration is expressed in LSB, where 1 LSB = 512 / ODR.
    ///
    /// ### Returns
    /// - `Ok(u8)`: The current sleep duration value.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn act_sleep_dur_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(WakeUpDur::read(self)?.sleep_dur())
    }

    /// Set the threshold for tap recognition on the X-axis.
    ///
    /// This function configures the tap threshold for the X-axis by updating the `tap_thsx` field in the `TAP_THS_X` register.
    ///
    /// ### Arguments
    /// - `val`: The desired tap threshold value for the X-axis.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn tap_threshold_x_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut reg = TapThsX::read(self)?;
        reg.set_tap_thsx(val);
        reg.write(self)
    }

    /// Get the threshold for tap recognition on the X-axis.
    ///
    /// This function retrieves the current tap threshold for the X-axis from the `tap_thsx` field in the `TAP_THS_X` register.
    ///
    /// ### Returns
    /// - `Ok(u8)`: The current tap threshold value for the X-axis.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn tap_threshold_x_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(TapThsX::read(self)?.tap_thsx())
    }

    /// Set the threshold for tap recognition on the Y-axis.
    ///
    /// This function configures the tap threshold for the Y-axis by updating the `tap_thsy` field in the `TAP_THS_Y` register.
    ///
    /// ### Arguments
    /// - `val`: The desired tap threshold value for the Y-axis.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn tap_threshold_y_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut reg = TapThsY::read(self)?;
        reg.set_tap_thsy(val);
        reg.write(self)
    }

    /// Get the threshold for tap recognition on the Y-axis.
    ///
    /// This function retrieves the current tap threshold for the Y-axis from the `tap_thsy` field in the `TAP_THS_Y` register.
    ///
    /// ### Returns
    /// - `Ok(u8)`: The current tap threshold value for the Y-axis.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn tap_threshold_y_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(TapThsY::read(self)?.tap_thsy())
    }

    /// Set the axis priority for tap detection.
    ///
    /// This function configures the axis priority for tap detection by updating the `tap_prior` field in the `TAP_THS_Y` register.
    ///
    /// ### Arguments
    /// - `val`: A [`TapPrior`] value representing the desired axis priority:
    ///   - `Xyz`: X > Y > Z (default).
    ///   - `Yxz`: Y > X > Z.
    ///   - `Xzy`: X > Z > Y.
    ///   - `Zyx`: Z > Y > X.
    ///   - `Yzx`: Y > Z > X.
    ///   - `Zxy`: Z > X > Y.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn tap_axis_priority_set(&mut self, val: TapPrior) -> Result<(), Error<B::Error>> {
        let mut reg = TapThsY::read(self)?;
        reg.set_tap_prior(val as u8);
        reg.write(self)
    }

    /// Get the axis priority for tap detection.
    ///
    /// This function retrieves the current axis priority for tap detection from the `tap_prior` field in the `TAP_THS_Y` register.
    ///
    /// ### Returns
    /// - `Ok(TapPrior)`: The current axis priority as a [`TapPrior`] value.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn tap_axis_priority_get(&mut self) -> Result<TapPrior, Error<B::Error>> {
        Ok(TapPrior::try_from(TapThsY::read(self)?.tap_prior()).unwrap_or_default())
    }

    /// Set the threshold for tap recognition on the Z-axis.
    ///
    /// This function configures the tap threshold for the Z-axis by updating the `tap_thsz` field in the `TAP_THS_Z` register.
    ///
    /// ### Arguments
    /// - `val`: The desired tap threshold value for the Z-axis.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn tap_threshold_z_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut reg = TapThsZ::read(self)?;
        reg.set_tap_thsz(val);
        reg.write(self)
    }

    /// Get the threshold for tap recognition on the Z-axis.
    ///
    /// This function retrieves the current tap threshold for the Z-axis from the `tap_thsz` field in the `TAP_THS_Z` register.
    ///
    /// ### Returns
    /// - `Ok(u8)`: The current tap threshold value for the Z-axis.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn tap_threshold_z_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(TapThsZ::read(self)?.tap_thsz())
    }

    /// Enable Z direction in tap recognition.
    ///
    /// This function enables or disables tap recognition on the Z-axis by updating the `tap_z_en` field in the `TAP_THS_Z` register.
    ///
    /// ### Arguments
    /// - `val`: The desired value for the `tap_z_en` field:
    ///   - `0`: Disable Z-axis tap recognition.
    ///   - `1`: Enable Z-axis tap recognition.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn tap_detection_on_z_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut reg = TapThsZ::read(self)?;
        reg.set_tap_z_en(val);
        reg.write(self)
    }

    /// Get the Z direction tap recognition status.
    ///
    /// This function retrieves the current status of tap recognition on the Z-axis from the `tap_z_en` field in the `TAP_THS_Z` register.
    ///
    /// ### Returns
    /// - `Ok(u8)`: The current value of the `tap_z_en` field:
    ///   - `0`: Z-axis tap recognition is disabled.
    ///   - `1`: Z-axis tap recognition is enabled.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn tap_detection_on_z_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(TapThsZ::read(self)?.tap_z_en())
    }

    /// Enable Y direction in tap recognition.
    ///
    /// This function enables or disables tap recognition on the Y-axis by updating the `tap_y_en` field in the `TAP_THS_Z` register.
    ///
    /// ### Arguments
    /// - `val`: The desired value for the `tap_y_en` field:
    ///   - `0`: Disable Y-axis tap recognition.
    ///   - `1`: Enable Y-axis tap recognition.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn tap_detection_on_y_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut reg = TapThsZ::read(self)?;
        reg.set_tap_y_en(val);
        reg.write(self)
    }

    /// Get the Y direction tap recognition status.
    ///
    /// This function retrieves the current status of tap recognition on the Y-axis from the `tap_y_en` field in the `TAP_THS_Z` register.
    ///
    /// ### Returns
    /// - `Ok(u8)`: The current value of the `tap_y_en` field:
    ///   - `0`: Y-axis tap recognition is disabled.
    ///   - `1`: Y-axis tap recognition is enabled.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn tap_detection_on_y_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(TapThsZ::read(self)?.tap_y_en())
    }

    /// Enable X direction in tap recognition.
    ///
    /// This function enables or disables tap recognition on the X-axis by updating the `tap_x_en` field in the `TAP_THS_Z` register.
    ///
    /// ### Arguments
    /// - `val`: The desired value for the `tap_x_en` field:
    ///   - `0`: Disable X-axis tap recognition.
    ///   - `1`: Enable X-axis tap recognition.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn tap_detection_on_x_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut reg = TapThsZ::read(self)?;
        reg.set_tap_x_en(val);
        reg.write(self)
    }

    /// Get the X direction tap recognition status.
    ///
    /// This function retrieves the current status of tap recognition on the X-axis from the `tap_x_en` field in the `TAP_THS_Z` register.
    ///
    /// ### Returns
    /// - `Ok(u8)`: The current value of the `tap_x_en` field:
    ///   - `0`: X-axis tap recognition is disabled.
    ///   - `1`: X-axis tap recognition is enabled.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn tap_detection_on_x_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(TapThsZ::read(self)?.tap_x_en())
    }

    /// Set the maximum duration for tap recognition.
    ///
    /// This function configures the maximum time an over-threshold signal is detected to be recognized as a tap event.
    /// The duration is set in the `shock` field of the `INT_DUR` register.
    /// - The default value (`00b`) corresponds to `4 * ODR_XL` time.
    /// - If the `shock` bits are set to a different value, 1 LSB corresponds to `8 * ODR_XL` time.
    ///
    /// ### Arguments
    /// - `val`: The desired maximum duration value for tap recognition.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn tap_shock_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut reg = IntDur::read(self)?;
        reg.set_shock(val);
        reg.write(self)
    }

    /// Get the maximum duration for tap recognition.
    ///
    /// This function retrieves the current maximum time an over-threshold signal is detected to be recognized as a tap event.
    /// The duration is stored in the `shock` field of the `INT_DUR` register.
    ///
    /// ### Returns
    /// - `Ok(u8)`: The current maximum duration value for tap recognition.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn tap_shock_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(IntDur::read(self)?.shock())
    }

    /// Set the quiet time for tap recognition.
    ///
    /// This function configures the quiet time after the first detected tap during which no over-threshold event should occur.
    /// The quiet time is set in the `quiet` field of the `INT_DUR` register.
    /// - The default value (`00b`) corresponds to `2 * ODR_XL` time.
    /// - If the `quiet` bits are set to a different value, 1 LSB corresponds to `4 * ODR_XL` time.
    ///
    /// ### Arguments
    /// - `val`: The desired quiet time value.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn tap_quiet_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut reg = IntDur::read(self)?;
        reg.set_quiet(val);
        reg.write(self)
    }

    /// Get the quiet time for tap recognition.
    ///
    /// This function retrieves the current quiet time after the first detected tap during which no over-threshold event should occur.
    /// The quiet time is stored in the `quiet` field of the `INT_DUR` register.
    ///
    /// ### Returns
    /// - `Ok(u8)`: The current quiet time value.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn tap_quiet_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(IntDur::read(self)?.quiet())
    }

    /// Set the maximum duration for double-tap recognition.
    ///
    /// This function configures the maximum time between two consecutive detected taps to determine a double-tap event.
    /// The duration is set in the `latency` field of the `INT_DUR` register.
    /// - The default value (`0000b`) corresponds to `16 * ODR_XL` time.
    /// - If the `latency` bits are set to a different value, 1 LSB corresponds to `32 * ODR_XL` time.
    ///
    /// ### Arguments
    /// - `val`: The desired maximum duration value for double-tap recognition.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn tap_dur_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut reg = IntDur::read(self)?;
        reg.set_latency(val);
        reg.write(self)
    }

    /// Get the maximum duration for double-tap recognition.
    ///
    /// This function retrieves the current maximum time between two consecutive detected taps to determine a double-tap event.
    /// The duration is stored in the `latency` field of the `INT_DUR` register.
    ///
    /// ### Returns
    /// - `Ok(u8)`: The current maximum duration value for double-tap recognition.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn tap_dur_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(IntDur::read(self)?.latency())
    }

    /// Enable or disable single/double-tap event detection.
    ///
    /// This function configures the single/double-tap event detection by updating the `single_double_tap` field in the `WAKE_UP_THS` register.
    /// The mode determines whether only single-tap events or both single- and double-tap events are detected.
    ///
    /// ### Arguments
    /// - `val`: A [`SingleDoubleTap`] value representing the desired tap mode:
    ///   - `OnlySingle`: Detect only single-tap events (default).
    ///   - `BothSingleDouble`: Detect both single- and double-tap events.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn tap_mode_set(&mut self, val: SingleDoubleTap) -> Result<(), Error<B::Error>> {
        let mut reg = WakeUpThs::read(self)?;
        reg.set_single_double_tap(val as u8);
        reg.write(self)
    }

    /// Get the single/double-tap event detection mode.
    ///
    /// This function retrieves the current single/double-tap event detection mode from the `single_double_tap` field in the `WAKE_UP_THS` register.
    ///
    /// ### Returns
    /// - `Ok(SingleDoubleTap)`: The current tap mode as a [`SingleDoubleTap`] value:
    ///   - `OnlySingle`: Detect only single-tap events (default).
    ///   - `BothSingleDouble`: Detect both single- and double-tap events.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn tap_mode_get(&mut self) -> Result<SingleDoubleTap, Error<B::Error>> {
        Ok(
            SingleDoubleTap::try_from(WakeUpThs::read(self)?.single_double_tap())
                .unwrap_or_default(),
        )
    }

    /// Read the tap/double-tap source register.
    ///
    /// This function retrieves the tap/double-tap source information from the `TAP_SRC` register.
    /// The `TAP_SRC` register provides details about the tap events, such as the axis of detection and the type of tap event.
    ///
    /// ### Returns
    /// - `Ok(TapSrc)`: The tap source information as a [`TapSrc`] struct.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn tap_src_get(&mut self) -> Result<TapSrc, Error<B::Error>> {
        TapSrc::read(self)
    }

    /// Set the threshold for 4D/6D orientation detection.
    ///
    /// This function configures the threshold for 4D/6D orientation detection by updating the `6d_ths` field in the `TAP_THS_X` register.
    ///
    /// ### Arguments
    /// - `val`: The desired threshold value for 4D/6D orientation detection.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn sixd_threshold_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut reg = TapThsX::read(self)?;
        reg.set_six_d_ths(val);
        reg.write(self)
    }

    /// Get the threshold for 4D/6D orientation detection.
    ///
    /// This function retrieves the current threshold for 4D/6D orientation detection from the `6d_ths` field in the `TAP_THS_X` register.
    ///
    /// ### Returns
    /// - `Ok(u8)`: The current threshold value for 4D/6D orientation detection.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn sixd_threshold_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(TapThsX::read(self)?.six_d_ths())
    }

    /// Enable or disable 4D orientation detection.
    ///
    /// This function configures the 4D orientation detection by updating the `4d_en` field in the `TAP_THS_X` register.
    ///
    /// ### Arguments
    /// - `val`: The desired value for the `4d_en` field:
    ///   - `0`: Disable 4D orientation detection.
    ///   - `1`: Enable 4D orientation detection.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn fourd_mode_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut reg = TapThsX::read(self)?;
        reg.set_four_d_en(val);
        reg.write(self)
    }

    /// Get the 4D orientation detection status.
    ///
    /// This function retrieves the current status of 4D orientation detection from the `4d_en` field in the `TAP_THS_X` register.
    ///
    /// ### Returns
    /// - `Ok(u8)`: The current value of the `4d_en` field:
    ///   - `0`: 4D orientation detection is disabled.
    ///   - `1`: 4D orientation detection is enabled.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn fourd_mode_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(TapThsX::read(self)?.four_d_en())
    }

    /// Read the 6D tap source register.
    ///
    /// This function retrieves the 6D tap source information from the `SIXD_SRC` register.
    /// The `SIXD_SRC` register provides details about the 6D orientation events, such as axis-specific thresholds and event detection.
    ///
    /// ### Returns
    /// - `Ok(SixdSrc)`: The 6D source information as a [`SixdSrc`] struct.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn sixd_src_get(&mut self) -> Result<SixdSrc, Error<B::Error>> {
        SixdSrc::read(self)
    }

    /// Set the data source for the 6D interrupt function.
    ///
    /// This function configures the data source for the 6D interrupt function by updating the `lpass_on6d` field in the `CTRL7` register.
    /// The data source can be either ODR/2 low-pass filtered data or LPF2 output data.
    ///
    /// ### Arguments
    /// - `val`: A [`LpassOn6d`] value representing the desired data source:
    ///   - `OdrDiv2Feed`: ODR/2 low-pass filtered data (default).
    ///   - `Lpf2Feed`: LPF2 output data.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn sixd_feed_data_set(&mut self, val: LpassOn6d) -> Result<(), Error<B::Error>> {
        let mut reg = Ctrl7::read(self)?;
        reg.set_lpass_on6d(val as u8);
        reg.write(self)
    }

    /// Get the data source for the 6D interrupt function.
    ///
    /// This function retrieves the current data source for the 6D interrupt function from the `lpass_on6d` field in the `CTRL7` register.
    ///
    /// ### Returns
    /// - `Ok(LpassOn6d)`: The current data source as a [`LpassOn6d`] value.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn sixd_feed_data_get(&mut self) -> Result<LpassOn6d, Error<B::Error>> {
        Ok(LpassOn6d::try_from(Ctrl7::read(self)?.lpass_on6d()).unwrap_or_default())
    }

    /// Set the wake-up duration event.
    ///
    /// This function configures the wake-up duration event by updating the `ff_dur` field in the `WAKE_UP_DUR` and `FREE_FALL` registers.
    /// The duration is expressed in LSB, where 1 LSB = 1 / ODR.
    ///
    /// ### Arguments
    /// - `val`: The desired wake-up duration value.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn ff_dur_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut wake_up_dur = WakeUpDur::read(self)?;
        let mut free_fall = FreeFall::read(self)?;

        wake_up_dur.set_ff_dur((val & 0x20) >> 5);
        free_fall.set_ff_dur(val & 0x1F);

        wake_up_dur.write(self)?;
        free_fall.write(self)
    }

    /// Get the wake-up duration event.
    ///
    /// This function retrieves the current wake-up duration event from the `ff_dur` field in the `WAKE_UP_DUR` and `FREE_FALL` registers.
    /// The duration is expressed in LSB, where 1 LSB = 1 / ODR.
    ///
    /// ### Returns
    /// - `Ok(u8)`: The current wake-up duration value.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn ff_dur_get(&mut self) -> Result<u8, Error<B::Error>> {
        let wake_up_dur = WakeUpDur::read(self)?;
        let free_fall = FreeFall::read(self)?;

        Ok((wake_up_dur.ff_dur() << 5) + free_fall.ff_dur())
    }

    /// Set the free-fall threshold.
    ///
    /// This function configures the free-fall threshold by updating the `ff_ths` field in the `FREE_FALL` register.
    /// The threshold determines the sensitivity of the free-fall detection.
    ///
    /// ### Arguments
    /// - `val`: A [`FfThs`] value representing the desired free-fall threshold:
    ///   - `FfTsh5lsbFs2g`: 5 LSB @ ±2g (default).
    ///   - `FfTsh7lsbFs2g`: 7 LSB @ ±2g.
    ///   - `FfTsh8lsbFs2g`: 8 LSB @ ±2g.
    ///   - `FfTsh10lsbFs2g`: 10 LSB @ ±2g.
    ///   - `FfTsh11lsbFs2g`: 11 LSB @ ±2g.
    ///   - `FfTsh13lsbFs2g`: 13 LSB @ ±2g.
    ///   - `FfTsh15lsbFs2g`: 15 LSB @ ±2g.
    ///   - `FfTsh16lsbFs2g`: 16 LSB @ ±2g.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn ff_threshold_set(&mut self, val: FfThs) -> Result<(), Error<B::Error>> {
        let mut reg = FreeFall::read(self)?;
        reg.set_ff_ths(val as u8);
        reg.write(self)
    }

    /// Get the free-fall threshold.
    ///
    /// This function retrieves the current free-fall threshold from the `ff_ths` field in the `FREE_FALL` register.
    ///
    /// ### Returns
    /// - `Ok(FfThs)`: The current free-fall threshold as a [`FfThs`] value.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn ff_threshold_get(&mut self) -> Result<FfThs, Error<B::Error>> {
        Ok(FfThs::try_from(FreeFall::read(self)?.ff_ths()).unwrap_or_default())
    }

    /// Set the FIFO watermark level.
    ///
    /// This function configures the FIFO watermark level by updating the `fth` field in the `FIFO_CTRL` register.
    /// The watermark level determines the threshold at which the FIFO generates an interrupt when the number of unread samples reaches the specified level.
    ///
    /// ### Arguments
    /// - `val`: The desired FIFO watermark level.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn fifo_watermark_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut reg = FifoCtrl::read(self)?;
        reg.set_fth(val);
        reg.write(self)
    }

    /// Get the FIFO watermark level.
    ///
    /// This function retrieves the current FIFO watermark level from the `fth` field in the `FIFO_CTRL` register.
    ///
    /// ### Returns
    /// - `Ok(u8)`: The current FIFO watermark level.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn fifo_watermark_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(FifoCtrl::read(self)?.fth())
    }

    /// Set the FIFO mode.
    ///
    /// This function configures the FIFO operating mode by updating the `fmode` field in the `FIFO_CTRL` register.
    /// The FIFO mode determines how data is managed in the FIFO buffer.
    ///
    /// ### Arguments
    /// - `val`: A [`Fmode`] value representing the desired FIFO mode:
    ///   - `BypassMode`: FIFO is disabled (default).
    ///   - `FifoMode`: FIFO stops collecting data when full.
    ///   - `StreamToFifoMode`: Stream mode until a trigger event, then FIFO mode.
    ///   - `BypassToStreamMode`: Bypass mode until a trigger event, then stream mode.
    ///   - `StreamMode`: Continuously updates FIFO, overwriting old data when full.
    ///
    /// ### Returns
    /// - `Ok(())`: If the operation is successful.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read or write operation.
    pub fn fifo_mode_set(&mut self, val: Fmode) -> Result<(), Error<B::Error>> {
        let mut reg = FifoCtrl::read(self)?;
        reg.set_fmode(val as u8);
        reg.write(self)
    }

    /// Get the FIFO mode.
    ///
    /// This function retrieves the current FIFO operating mode from the `fmode` field in the `FIFO_CTRL` register.
    ///
    /// ### Returns
    /// - `Ok(Fmode)`: The current FIFO mode as a [`Fmode`] value.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn fifo_mode_get(&mut self) -> Result<Fmode, Error<B::Error>> {
        Ok(Fmode::try_from(FifoCtrl::read(self)?.fmode()).unwrap_or_default())
    }

    /// Get the number of unread samples stored in the FIFO.
    ///
    /// This function retrieves the number of unread samples currently stored in the FIFO buffer from the `diff` field in the `FIFO_SAMPLES` register.
    ///
    /// ### Returns
    /// - `Ok(u8)`: The number of unread samples in the FIFO.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn fifo_data_level_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(FifoSamples::read(self)?.diff())
    }

    /// Get the FIFO overrun status.
    ///
    /// This function retrieves the FIFO overrun status from the `fifo_ovr` field in the `FIFO_SAMPLES` register.
    /// The overrun status indicates whether the FIFO buffer has overwritten old data due to being full.
    ///
    /// ### Returns
    /// - `Ok(u8)`: The current FIFO overrun status:
    ///   - `0`: No overrun has occurred.
    ///   - `1`: FIFO has overwritten old data.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn fifo_ovr_flag_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(FifoSamples::read(self)?.fifo_ovr())
    }

    /// Get the FIFO threshold status flag.
    ///
    /// This function retrieves the FIFO threshold status flag from the `fifo_fth` field in the `FIFO_SAMPLES` register.
    /// The threshold status indicates whether the number of unread samples in the FIFO has reached the configured watermark level.
    ///
    /// ### Returns
    /// - `Ok(u8)`: The current FIFO threshold status flag:
    ///   - `0`: FIFO filling is below the threshold level.
    ///   - `1`: FIFO filling has reached or exceeded the threshold level.
    /// - `Err(Error::Bus)`: If there is an error at the bus level during the read operation.
    pub fn fifo_wtm_flag_get(&mut self) -> Result<u8, Error<B::Error>> {
        Ok(FifoSamples::read(self)?.fifo_fth())
    }
}

/// Convert from full-scale ±2g to mg.
///
/// This function converts a raw sensor value in least significant bits (LSB) to mg for a full-scale range of ±2g.
///
/// ### Arguments
/// - `lsb`: The raw value in LSB.
///
/// ### Returns
/// - `f32`: The converted value in mg.
pub fn from_fs2_to_mg(lsb: i16) -> f32 {
    (lsb as f32) * 0.244
}

/// Convert from full-scale ±4g to mg.
///
/// This function converts a raw sensor value in least significant bits (LSB) to mg for a full-scale range of ±4g.
///
/// ### Arguments
/// - `lsb`: The raw value in LSB.
///
/// ### Returns
/// - `f32`: The converted value in mg.
pub fn from_fs4_to_mg(lsb: i16) -> f32 {
    // (lsb as f32) * 0.122
    (lsb as f32) * 0.488
}

/// Convert from full-scale ±8g to mg.
///
/// This function converts a raw sensor value in least significant bits (LSB) to mg for a full-scale range of ±8g.
///
/// ### Arguments
/// - `lsb`: The raw value in LSB.
///
/// ### Returns
/// - `f32`: The converted value in mg.
pub fn from_fs8_to_mg(lsb: i16) -> f32 {
    (lsb as f32) * 0.976
}

/// Convert from full-scale ±16g to mg.
///
/// This function converts a raw sensor value in least significant bits (LSB) to mg for a full-scale range of ±16g.
///
/// ### Arguments
/// - `lsb`: The raw value in LSB.
///
/// ### Returns
/// - `f32`: The converted value in mg.
pub fn from_fs16_to_mg(lsb: i16) -> f32 {
    (lsb as f32) * 1.952
}

/// Convert from full-scale ±2g (low-power mode 1) to mg.
///
/// This function converts a raw sensor value in least significant bits (LSB) to mg for a full-scale range of ±2g in low-power mode 1.
///
/// ### Arguments
/// - `lsb`: The raw value in LSB.
///
/// ### Returns
/// - `f32`: The converted value in mg.
pub fn from_fs2_lp1_to_mg(lsb: i16) -> f32 {
    (lsb as f32) * 0.976
}

/// Convert from full-scale ±4g (low-power mode 1) to mg.
///
/// This function converts a raw sensor value in least significant bits (LSB) to mg for a full-scale range of ±4g in low-power mode 1.
///
/// ### Arguments
/// - `lsb`: The raw value in LSB.
///
/// ### Returns
/// - `f32`: The converted value in mg.
pub fn from_fs4_lp1_to_mg(lsb: i16) -> f32 {
    (lsb as f32) * 1.952
}

/// Convert from full-scale ±8g (low-power mode 1) to mg.
///
/// This function converts a raw sensor value in least significant bits (LSB) to mg for a full-scale range of ±8g in low-power mode 1.
///
/// ### Arguments
/// - `lsb`: The raw value in LSB.
///
/// ### Returns
/// - `f32`: The converted value in mg.
pub fn from_fs8_lp1_to_mg(lsb: i16) -> f32 {
    (lsb as f32) * 3.904
}

/// Convert from full-scale ±16g (low-power mode 1) to mg.
///
/// This function converts a raw sensor value in least significant bits (LSB) to mg for a full-scale range of ±16g in low-power mode 1.
///
/// ### Arguments
/// - `lsb`: The raw value in LSB.
///
/// ### Returns
/// - `f32`: The converted value in mg.
pub fn from_fs16_lp1_to_mg(lsb: i16) -> f32 {
    (lsb as f32) * 7.808
}

/// Convert from LSB to Celsius.
///
/// This function converts a raw temperature value in least significant bits (LSB) to degrees Celsius (°C).
///
/// ### Arguments
/// - `lsb`: The raw temperature value in LSB.
///
/// ### Returns
/// - `f32`: The temperature in degrees Celsius.
pub fn from_lsb_to_celsius(lsb: i16) -> f32 {
    (lsb as f32 / 16.0) + 25.0
}

/// I²C Address Map.
///
/// This enum represents the possible I²C addresses for the IIS2DLPC sensor, depending on the configuration of the SA0 pin.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum I2CAddress {
    /// I²C address when SA0 is connected to GND.
    I2cAddL = 0x18,

    /// I²C address when SA0 is connected to VDD.
    I2cAddH = 0x19,
}

/// Device ID for the IIS2DLPC sensor.
///
/// The `WhoAmI` register contains this value to identify the device.
pub const ID: u8 = 0x44;

pub const PROPERTY_ENABLE: u8 = 1;
pub const PROPERTY_DISABLE: u8 = 0;
