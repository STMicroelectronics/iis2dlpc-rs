use crate::{BusOperation, DelayNs, Error, Iis2dlpc};
use bitfield_struct::bitfield;
use derive_more::TryFrom;
use st_mem_bank_macro::register;

/// IIS2DLPC Register Map.
///
/// This enum represents the memory-mapped registers of the IIS2DLPC sensor. Each variant corresponds to a specific register address.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum Reg {
    /// Temperature output register (low byte).
    OutTL = 0x0D,

    /// Temperature output register (high byte).
    OutTH = 0x0E,

    /// Who Am I register.
    ///
    /// This register is read-only and contains the device ID. Default value: `0x44`.
    WhoAmI = 0x0F,

    /// Control register 1.
    Ctrl1 = 0x20,

    /// Control register 2.
    Ctrl2 = 0x21,

    /// Control register 3.
    Ctrl3 = 0x22,

    /// Control register 4 for INT1 pad control.
    Ctrl4Int1PadCtrl = 0x23,

    /// Control register 5 for INT2 pad control.
    Ctrl5Int2PadCtrl = 0x24,

    /// Control register 6.
    Ctrl6 = 0x25,

    /// Temperature output register (8-bit resolution).
    OutT = 0x26,

    /// Status register.
    Status = 0x27,

    /// X-axis output register (low byte).
    OutXL = 0x28,

    /// X-axis output register (high byte).
    OutXH = 0x29,

    /// Y-axis output register (low byte).
    OutYL = 0x2A,

    /// Y-axis output register (high byte).
    OutYH = 0x2B,

    /// Z-axis output register (low byte).
    OutZL = 0x2C,

    /// Z-axis output register (high byte).
    OutZH = 0x2D,

    /// FIFO control register.
    FifoCtrl = 0x2E,

    /// FIFO samples register.
    FifoSamples = 0x2F,

    /// TAP threshold configuration for the X-axis.
    TapThsX = 0x30,

    /// TAP threshold configuration for the Y-axis.
    TapThsY = 0x31,

    /// TAP threshold configuration for the Z-axis.
    TapThsZ = 0x32,

    /// Interrupt duration configuration.
    IntDur = 0x33,

    /// Wakeup threshold configuration.
    WakeUpThs = 0x34,

    /// Wakeup duration configuration.
    WakeUpDur = 0x35,

    /// Free-fall configuration.
    FreeFall = 0x36,

    /// Status duplicate register.
    StatusDup = 0x37,

    /// Wakeup source register.
    WakeUpSrc = 0x38,

    /// Tap source register.
    TapSrc = 0x39,

    /// 6D source register.
    SixdSrc = 0x3A,

    /// All interrupt source register.
    AllIntSrc = 0x3B,

    /// X-axis user offset register.
    XOfsUsr = 0x3C,

    /// Y-axis user offset register.
    YOfsUsr = 0x3D,

    /// Z-axis user offset register.
    ZOfsUsr = 0x3E,

    /// Control register 7.
    Ctrl7 = 0x3F,
}

/// Temperature output register (12-bit resolution, read-only).
///
/// The `OutT` register contains the raw temperature sensor output as a 12-bit two's complement value.
/// The temperature data is left-justified within the 16-bit register.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::OutTL, access_type = Iis2dlpc, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u16, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u16, order = Lsb))]
pub struct OutT {
    #[bits(4, access = RO, default = 0)]
    not_used: u8,
    /// Temperature sensor output value.
    #[bits(12, access = RO, default = 0)]
    pub temp: i16,
}

/// Control register 1 (R/W).
///
/// The `CTRL1` register is used to configure the operating mode, low-power mode, and output data rate (ODR) of the IIS2DLPC sensor.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::Ctrl1, access_type = Iis2dlpc, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl1 {
    /// Low-power mode selection.
    ///
    /// Configures the low-power mode.
    ///
    /// Default value: `0`.
    #[bits(2, default = 0)]
    pub lp_mode: u8,

    /// Mode selection.
    ///
    /// Configures the operating mode of the sensor.
    ///
    /// Default value: `0`.
    #[bits(2, default = 0)]
    pub mode: u8,

    /// Output data rate (ODR) selection.
    ///
    /// Configures the output data rate and power mode.
    ///
    /// Default value: `0`.
    #[bits(4, default = 0)]
    pub odr: u8,
}

/// Control register 2 (R/W).
///
/// The `CTRL2` register is used to configure the SPI interface mode, I²C disable, address increment, block data update, and other settings.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::Ctrl2, access_type = Iis2dlpc, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl2 {
    /// SPI serial interface mode selection.
    ///
    /// Configures the SPI interface mode:
    /// * `0`: 4-wire interface.
    /// * `1`: 3-wire interface.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub sim: u8,

    /// I²C disable.
    ///
    /// Disables the I²C communication protocol:
    /// * `0`: SPI and I²C interfaces enabled.
    /// * `1`: I²C mode disabled.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub i2c_disable: u8,

    /// Register address auto-increment.
    ///
    /// Enables automatic increment of the register address during multiple byte access:
    /// * `0`: Disabled.
    /// * `1`: Enabled.
    ///
    /// Default value: `1`.
    #[bits(1, default = 1)]
    pub if_add_inc: u8,

    /// Block data update.
    ///
    /// Configures the update behavior of output registers:
    /// * `0`: Continuous update.
    /// * `1`: Output registers not updated until MSB and LSB are read.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub bdu: u8,

    /// Disconnect CS pull-up.
    ///
    /// Configures the pull-up resistor on the CS pin:
    /// * `0`: Pull-up connected to CS pin.
    /// * `1`: Pull-up disconnected from CS pin.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub cs_pu_disc: u8,

    #[bits(1, access = RO, default = 0)]
    not_used_01: u8,

    /// Soft reset.
    ///
    /// Resets all control registers:
    /// * `0`: Disabled.
    /// * `1`: Enabled.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub soft_reset: u8,

    /// Boot.
    ///
    /// Retrieves the correct trimming parameters from nonvolatile memory:
    /// * `0`: Disabled.
    /// * `1`: Enabled.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub boot: u8,
}

/// Control register 3 (R/W).
///
/// The `CTRL3` register is used to configure interrupt polarity, interrupt latching, push-pull/open-drain selection, self-test mode, and single data conversion on demand mode.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::Ctrl3, access_type = Iis2dlpc, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl3 {
    /// Single data conversion on demand mode configuration.
    ///
    /// This field combines two subfields:
    /// - `slp_mode_sel`: Determines the trigger source for single data conversion:
    ///   - `0`: External trigger on INT2.
    ///   - `1`: Triggered by writing `slp_mode_1` to `1` via I²C/SPI.
    /// - `slp_mode_1`: Starts the single data conversion on demand mode when set to `1`.
    ///   This bit is automatically reset to `0` when the data is available, and the device is ready for another triggered session.
    ///
    /// Default value: `0`.
    #[bits(2, default = 0)]
    pub slp_mode: u8,

    #[bits(1, access = RO, default = 0)]
    not_used_01: u8,

    /// Interrupt active level.
    ///
    /// Configures the active level of interrupts:
    /// * `0`: Active high.
    /// * `1`: Active low.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub h_lactive: u8,

    /// Latched interrupt.
    ///
    /// Configures the interrupt mode:
    /// * `0`: Pulsed mode.
    /// * `1`: Latched mode.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub lir: u8,

    /// Push-pull/open-drain selection.
    ///
    /// Configures the interrupt pad type:
    /// * `0`: Push-pull.
    /// * `1`: Open-drain.
    #[bits(1, default = 0)]
    pub pp_od: u8,

    /// Self-test mode selection.
    ///
    /// Configures the self-test mode. Refer to the datasheet for the available self-test modes.
    ///
    /// Default value: `0`.
    #[bits(2, default = 0)]
    pub st: u8,
}

/// Control register 4 (R/W).
///
/// The `CTRL4_INT1_PAD_CTRL` register is used to configure the interrupt signals routed to the INT1 pad.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::Ctrl4Int1PadCtrl, access_type = Iis2dlpc, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl4Int1PadCtrl {
    /// Data-ready interrupt routed to INT1 pad.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub int1_drdy: u8,

    /// FIFO threshold interrupt routed to INT1 pad.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub int1_fth: u8,

    /// FIFO full interrupt routed to INT1 pad.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub int1_diff5: u8,

    /// Double-tap interrupt routed to INT1 pad.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub int1_tap: u8,

    /// Free-fall interrupt routed to INT1 pad.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub int1_ff: u8,

    /// Wakeup interrupt routed to INT1 pad.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub int1_wu: u8,

    /// Single-tap interrupt routed to INT1 pad.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub int1_single_tap: u8,

    /// 6D recognition interrupt routed to INT1 pad.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub int1_6d: u8,
}

/// Control register 5 (R/W).
///
/// The `CTRL5_INT2_PAD_CTRL` register is used to configure the interrupt signals routed to the INT2 pad.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::Ctrl5Int2PadCtrl, access_type = Iis2dlpc, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl5Int2PadCtrl {
    /// Data-ready interrupt routed to INT2 pad.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub int2_drdy: u8,

    /// FIFO threshold interrupt routed to INT2 pad.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub int2_fth: u8,

    /// FIFO full interrupt routed to INT2 pad.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub int2_diff5: u8,

    /// FIFO overrun interrupt routed to INT2 pad.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub int2_ovr: u8,

    /// Temperature data-ready interrupt routed to INT2 pad.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub int2_drdy_t: u8,

    /// Boot status routed to INT2 pad.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub int2_boot: u8,

    /// Sleep change status routed to INT2 pad.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub int2_sleep_chg: u8,

    /// Sleep state status routed to INT2 pad.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub int2_sleep_state: u8,
}

/// Control register 6 (R/W).
///
/// The `CTRL6` register is used to configure the low-noise mode, filter settings, full-scale selection, and bandwidth selection.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::Ctrl6, access_type = Iis2dlpc, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl6 {
    #[bits(2, access = RO, default = 0)]
    not_used_01: u8,

    /// Low-noise mode enable.
    ///
    /// Configures the low-noise mode:
    /// * `0`: Disabled.
    /// * `1`: Enabled.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub low_noise: u8,

    /// Filtered data selection.
    ///
    /// Configures the data path:
    /// * `0`: Low-pass filter path selected.
    /// * `1`: High-pass filter path selected.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub fds: u8,

    /// Full-scale selection.
    ///
    /// Configures the measurement range:
    /// * `00`: ±2 g.
    /// * `01`: ±4 g.
    /// * `10`: ±8 g.
    /// * `11`: ±16 g.
    ///
    /// Default value: `0`.
    #[bits(2, default = 0)]
    pub fs: u8,

    /// Bandwidth selection.
    ///
    /// Configures the bandwidth of the filter:
    /// * `00`: ODR/2.
    /// * `01`: ODR/4.
    /// * `10`: ODR/10.
    /// * `11`: ODR/20.
    ///
    /// Default value: `0`.
    #[bits(2, default = 0)]
    pub bw_filt: u8,
}

/// Status register (R).
///
/// The `STATUS` register provides the status of various events detected by the IIS2DLPC sensor.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::Status, access_type = Iis2dlpc, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Status {
    /// Data-ready status.
    ///
    /// Indicates whether new data is available:
    /// * `0`: Not ready.
    /// * `1`: X-, Y-, and Z-axis new data available.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub drdy: u8,

    /// Free-fall event detection status.
    ///
    /// Indicates whether a free-fall event has been detected:
    /// * `0`: Free-fall event not detected.
    /// * `1`: Free-fall event detected.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub ff_ia: u8,

    /// 6D recognition event status.
    ///
    /// Indicates whether a change in position (portrait/landscape/face-up/face-down) has been detected:
    /// * `0`: No event detected.
    /// * `1`: A change in position detected.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub six_d_ia: u8,

    /// Single-tap event status.
    ///
    /// Indicates whether a single-tap event has been detected:
    /// * `0`: Single-tap event not detected.
    /// * `1`: Single-tap event detected.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub single_tap: u8,

    /// Double-tap event status.
    ///
    /// Indicates whether a double-tap event has been detected:
    /// * `0`: Double-tap event not detected.
    /// * `1`: Double-tap event detected.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub double_tap: u8,

    /// Sleep state status.
    ///
    /// Indicates whether the device is in a sleep state:
    /// * `0`: Sleep event not detected.
    /// * `1`: Sleep event detected.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub sleep_state: u8,

    /// Wakeup event detection status.
    ///
    /// Indicates whether a wakeup event has been detected:
    /// * `0`: Wakeup event not detected.
    /// * `1`: Wakeup event detected.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub wu_ia: u8,

    /// FIFO threshold status.
    ///
    /// Indicates whether the FIFO filling is equal to or higher than the threshold level:
    /// * `0`: FIFO filling is lower than the threshold level.
    /// * `1`: FIFO filling is equal to or higher than the threshold level.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub fifo_ths: u8,
}

/// X-axis accelerometer output register (14-bit resolution, read-only).
///
/// The `OutX` register contains the raw acceleration data for the X-axis as a 14-bit two's complement value.
/// The data is left-justified within the 16-bit register, with 2 unused bits.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::OutXL, access_type = Iis2dlpc, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u16, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u16, order = Lsb))]
pub struct OutX {
    #[bits(2, access = RO, default = 0)]
    not_used: u8,
    /// Raw acceleration data for the X-axis.
    #[bits(14, access = RO, default = 0)]
    pub x: i16,
}

/// Y-axis accelerometer output register (14-bit resolution, read-only).
///
/// The `OutY` register contains the raw acceleration data for the Y-axis as a 14-bit two's complement value.
/// The data is left-justified within the 16-bit register, with 2 unused bits.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::OutYL, access_type = Iis2dlpc, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u16, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u16, order = Lsb))]
pub struct OutY {
    #[bits(2, access = RO, default = 0)]
    not_used: u8,
    /// Raw acceleration data for the Y-axis.
    #[bits(14, access = RO, default = 0)]
    pub y: i16,
}

/// Z-axis accelerometer output register (14-bit resolution, read-only).
///
/// The `OutZ` register contains the raw acceleration data for the Z-axis as a 14-bit two's complement value.
/// The data is left-justified within the 16-bit register, with 2 unused bits.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::OutZL, access_type = Iis2dlpc, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u16, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u16, order = Lsb))]
pub struct OutZ {
    #[bits(2, access = RO, default = 0)]
    not_used: u8,
    /// Raw acceleration data for the Z-axis.
    #[bits(14, access = RO, default = 0)]
    pub z: i16,
}

/// FIFO control register (R/W).
///
/// The `FIFO_CTRL` register is used to configure the FIFO threshold level and mode.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::FifoCtrl, access_type = Iis2dlpc, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FifoCtrl {
    /// FIFO threshold level.
    ///
    /// Configures the FIFO threshold level. The value is a 5-bit unsigned integer.
    ///
    /// Default value: `0`.
    #[bits(5, default = 0)]
    pub fth: u8,

    /// FIFO mode selection.
    ///
    /// Configures the FIFO operating mode:
    /// * `000`: Bypass mode.
    /// * `001`: FIFO mode.
    /// * `011`: Continuous-to-FIFO mode.
    /// * `100`: Bypass-to-Continuous mode.
    /// * `110`: Continuous mode.
    ///
    /// Default value: `0`.
    #[bits(3, default = 0)]
    pub fmode: u8,
}

/// FIFO samples register (R).
///
/// The `FIFO_SAMPLES` register provides the status of the FIFO, including the number of unread samples and overflow/threshold flags.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::FifoSamples, access_type = Iis2dlpc, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FifoSamples {
    /// Number of unread samples in FIFO.
    ///
    /// Represents the number of unread samples stored in the FIFO. The value is a 6-bit unsigned integer.
    ///
    /// Default value: `0`.
    #[bits(6, default = 0, access = RO)]
    pub diff: u8,

    /// FIFO overrun status.
    ///
    /// Indicates whether the FIFO is full and at least one sample has been overwritten:
    /// * `0`: FIFO is not completely filled.
    /// * `1`: FIFO is completely filled and at least one sample has been overwritten.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub fifo_ovr: u8,

    /// FIFO threshold status.
    ///
    /// Indicates whether the FIFO filling is equal to or higher than the threshold level:
    /// * `0`: FIFO filling is lower than the threshold level.
    /// * `1`: FIFO filling is equal to or higher than the threshold level.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub fifo_fth: u8,
}

/// TAP threshold configuration for the X-axis (R/W).
///
/// The `TAP_THS_X` register is used to configure the tap threshold for the X-axis, 6D threshold, and 4D detection enable.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::TapThsX, access_type = Iis2dlpc, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct TapThsX {
    /// Tap threshold for the X-axis.
    ///
    /// Default value: `0`.
    #[bits(5, default = 0)]
    pub tap_thsx: u8,

    /// 6D threshold.
    ///
    /// Configures the threshold for 6D detection.
    ///
    /// Default value: `0`.
    #[bits(2, default = 0)]
    pub six_d_ths: u8,

    /// 4D detection enable.
    ///
    /// Enables 4D detection:
    /// * `0`: Disabled.
    /// * `1`: Enabled.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub four_d_en: u8,
}

/// TAP threshold configuration for the Y-axis (R/W).
///
/// The `TAP_THS_Y` register is used to configure the tap threshold for the Y-axis and the axis priority for tap detection.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::TapThsY, access_type = Iis2dlpc, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct TapThsY {
    /// Tap threshold for the Y-axis.
    ///
    /// Default value: `0`.
    #[bits(5, default = 0)]
    pub tap_thsy: u8,

    /// Axis priority for tap detection.
    ///
    /// Configures the priority of axes for tap detection.
    ///
    /// Default value: `0`.
    #[bits(3, default = 0)]
    pub tap_prior: u8,
}

/// TAP threshold configuration for the Z-axis (R/W).
///
/// The `TAP_THS_Z` register is used to configure the tap threshold for the Z-axis and enable tap detection on specific axes.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::TapThsZ, access_type = Iis2dlpc, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct TapThsZ {
    /// Tap threshold for the Z-axis.
    ///
    /// Default value: `0`.
    #[bits(5, default = 0)]
    pub tap_thsz: u8,

    /// Enable tap detection on the Z-axis.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub tap_z_en: u8,

    /// Enable tap detection on the Y-axis.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub tap_y_en: u8,

    /// Enable tap detection on the X-axis.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub tap_x_en: u8,
}

/// Interrupt duration configuration (R/W).
///
/// The `INT_DUR` register is used to configure the shock, quiet, and latency durations for tap detection.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::IntDur, access_type = Iis2dlpc, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IntDur {
    /// Shock duration.
    ///
    /// Default value: `0`.
    #[bits(2, default = 0)]
    pub shock: u8,

    /// Quiet duration.
    ///
    /// Default value: `0`.
    #[bits(2, default = 0)]
    pub quiet: u8,

    /// Latency duration.
    ///
    /// Default value: `0`.
    #[bits(4, default = 0)]
    pub latency: u8,
}

/// Wakeup threshold configuration (R/W).
///
/// The `WAKE_UP_THS` register is used to configure the wakeup threshold, sleep enable, and single/double-tap enable.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::WakeUpThs, access_type = Iis2dlpc, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct WakeUpThs {
    /// Wakeup threshold.
    ///
    /// Default value: `0`.
    #[bits(6, default = 0)]
    pub wk_ths: u8,

    /// Sleep enable.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub sleep_on: u8,

    /// Single/double-tap enable.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub single_double_tap: u8,
}

/// Wakeup duration configuration (R/W).
///
/// The `WAKE_UP_DUR` register is used to configure the sleep duration, stationary detection, wakeup duration, and free-fall duration.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::WakeUpDur, access_type = Iis2dlpc, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct WakeUpDur {
    /// Sleep duration.
    ///
    /// Default value: `0`.
    #[bits(4, default = 0)]
    pub sleep_dur: u8,

    /// Stationary detection enable.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub stationary: u8,

    /// Wakeup duration.
    ///
    /// Default value: `0`.
    #[bits(2, default = 0)]
    pub wake_dur: u8,

    /// Free-fall duration.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub ff_dur: u8,
}

/// Free-fall configuration (R/W).
///
/// The `FREE_FALL` register is used to configure the free-fall threshold and duration.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::FreeFall, access_type = Iis2dlpc, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FreeFall {
    /// Free-fall threshold.
    ///
    /// Default value: `0`.
    #[bits(3, default = 0)]
    pub ff_ths: u8,

    /// Free-fall duration.
    ///
    /// Default value: `0`.
    #[bits(5, default = 0)]
    pub ff_dur: u8,
}

/// Status duplicate register (R).
///
/// The `STATUS_DUP` register provides the status of various events detected by the IIS2DLPC sensor, including data-ready, free-fall, 6D recognition, and tap events.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::StatusDup, access_type = Iis2dlpc, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct StatusDup {
    /// Data-ready status.
    ///
    /// Indicates whether new data is available:
    /// * `0`: Not ready.
    /// * `1`: X-, Y-, and Z-axis new data available.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub drdy: u8,

    /// Free-fall event detection status.
    ///
    /// Indicates whether a free-fall event has been detected:
    /// * `0`: Free-fall event not detected.
    /// * `1`: Free-fall event detected.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub ff_ia: u8,

    /// 6D recognition event status.
    ///
    /// Indicates whether a change in position (portrait/landscape/face-up/face-down) has been detected:
    /// * `0`: No event detected.
    /// * `1`: A change in position detected.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub six_d_ia: u8,

    /// Single-tap event status.
    ///
    /// Indicates whether a single-tap event has been detected:
    /// * `0`: Single-tap event not detected.
    /// * `1`: Single-tap event detected.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub single_tap: u8,

    /// Double-tap event status.
    ///
    /// Indicates whether a double-tap event has been detected:
    /// * `0`: Double-tap event not detected.
    /// * `1`: Double-tap event detected.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub double_tap: u8,

    /// Sleep state status.
    ///
    /// Indicates whether the device is in a sleep state:
    /// * `0`: Sleep event not detected.
    /// * `1`: Sleep event detected.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub sleep_state_ia: u8,

    /// Temperature data-ready status.
    ///
    /// Indicates whether new temperature data is available:
    /// * `0`: Data not available.
    /// * `1`: A new set of data is available.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub drdy_t: u8,

    /// FIFO overrun status.
    ///
    /// Indicates whether the FIFO is full and at least one sample has been overwritten:
    /// * `0`: FIFO is not completely filled.
    /// * `1`: FIFO is completely filled and at least one sample has been overwritten.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub ovr: u8,
}

/// Wakeup source register (R).
///
/// The `WAKE_UP_SRC` register provides the status of wakeup events, including axis-specific wakeup detection and free-fall events.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::WakeUpSrc, access_type = Iis2dlpc, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct WakeUpSrc {
    /// Wakeup event detection status on the Z-axis.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub z_wu: u8,

    /// Wakeup event detection status on the Y-axis.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub y_wu: u8,

    /// Wakeup event detection status on the X-axis.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub x_wu: u8,

    /// Wakeup event detection status.
    ///
    /// Indicates whether a wakeup event has been detected:
    /// * `0`: Wakeup event not detected.
    /// * `1`: Wakeup event detected.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub wu_ia: u8,

    /// Sleep state status.
    ///
    /// Indicates whether the device is in a sleep state:
    /// * `0`: Sleep event not detected.
    /// * `1`: Sleep event detected.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub sleep_state_ia: u8,

    /// Free-fall event detection status.
    ///
    /// Indicates whether a free-fall event has been detected:
    /// * `0`: Free-fall event not detected.
    /// * `1`: Free-fall event detected.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub ff_ia: u8,

    #[bits(2, access = RO, default = 0)]
    not_used_01: u8,
}

/// Tap source register (R).
///
/// The `TAP_SRC` register provides the status of tap events, including axis-specific tap detection and tap sign.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::TapSrc, access_type = Iis2dlpc, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct TapSrc {
    /// Tap event detection status on the Z-axis.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub z_tap: u8,

    /// Tap event detection status on the Y-axis.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub y_tap: u8,

    /// Tap event detection status on the X-axis.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub x_tap: u8,

    /// Sign of acceleration detected by the tap event.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub tap_sign: u8,

    /// Double-tap event status.
    ///
    /// Indicates whether a double-tap event has been detected:
    /// * `0`: Double-tap event not detected.
    /// * `1`: Double-tap event detected.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub double_tap: u8,

    /// Single-tap event status.
    ///
    /// Indicates whether a single-tap event has been detected:
    /// * `0`: Single-tap event not detected.
    /// * `1`: Single-tap event detected.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub single_tap: u8,

    /// Tap event status.
    ///
    /// Indicates whether a tap event has been detected:
    /// * `0`: Tap event not detected.
    /// * `1`: Tap event detected.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub tap_ia: u8,

    #[bits(1, access = RO, default = 0)]
    not_used_01: u8,
}

/// 6D source register (R).
///
/// The `SIXD_SRC` register provides the status of 6D orientation detection, including axis-specific thresholds and 6D event detection.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::SixdSrc, access_type = Iis2dlpc, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SixdSrc {
    /// X-axis low threshold status.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub xl: u8,

    /// X-axis high threshold status.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub xh: u8,

    /// Y-axis low threshold status.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub yl: u8,

    /// Y-axis high threshold status.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub yh: u8,

    /// Z-axis low threshold status.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub zl: u8,

    /// Z-axis high threshold status.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub zh: u8,

    /// 6D event detection status.
    ///
    /// Indicates whether a 6D orientation event has been detected:
    /// * `0`: No event detected.
    /// * `1`: 6D event detected.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub six_d_ia: u8,

    #[bits(1, access = RO, default = 0)]
    not_used_01: u8,
}

/// All interrupt source register (R).
///
/// The `ALL_INT_SRC` register provides the status of all interrupt events, including free-fall, wakeup, tap, and 6D events.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::AllIntSrc, access_type = Iis2dlpc, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct AllIntSrc {
    /// Free-fall event detection status.
    ///
    /// Indicates whether a free-fall event has been detected:
    /// * `0`: Free-fall event not detected.
    /// * `1`: Free-fall event detected.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub ff_ia: u8,

    /// Wakeup event detection status.
    ///
    /// Indicates whether a wakeup event has been detected:
    /// * `0`: Wakeup event not detected.
    /// * `1`: Wakeup event detected.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub wu_ia: u8,

    /// Single-tap event detection status.
    ///
    /// Indicates whether a single-tap event has been detected:
    /// * `0`: Single-tap event not detected.
    /// * `1`: Single-tap event detected.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub single_tap: u8,

    /// Double-tap event detection status.
    ///
    /// Indicates whether a double-tap event has been detected:
    /// * `0`: Double-tap event not detected.
    /// * `1`: Double-tap event detected.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub double_tap: u8,

    /// 6D event detection status.
    ///
    /// Indicates whether a 6D orientation event has been detected:
    /// * `0`: No event detected.
    /// * `1`: 6D event detected.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub six_d_ia: u8,

    /// Sleep change event detection status.
    ///
    /// Indicates whether a sleep change event has been detected:
    /// * `0`: Sleep change event not detected.
    /// * `1`: Sleep change event detected.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0, access = RO)]
    pub sleep_change_ia: u8,

    #[bits(2, access = RO, default = 0)]
    not_used_01: u8,
}

/// User offset register for the X-axis (read/write).
///
/// The `XOfsUsr` register allows the user to apply a signed offset correction to the X-axis acceleration data.
/// The offset value is an 8-bit two's complement number.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::XOfsUsr, access_type = Iis2dlpc, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct XOfsUsr {
    /// User offset value for the X-axis.
    #[bits(8, default = 0)]
    pub x_ofs_usr: i8,
}

/// User offset register for the Y-axis (read/write).
///
/// The `YOfsUsr` register allows the user to apply a signed offset correction to the Y-axis acceleration data.
/// The offset value is an 8-bit two's complement number.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::YOfsUsr, access_type = Iis2dlpc, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct YOfsUsr {
    /// User offset value for the Y-axis.
    #[bits(8, default = 0)]
    pub y_ofs_usr: i8,
}

/// User offset register for the Z-axis (read/write).
///
/// The `ZOfsUsr` register allows the user to apply a signed offset correction to the Z-axis acceleration data.
/// The offset value is an 8-bit two's complement number.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::ZOfsUsr, access_type = Iis2dlpc, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct ZOfsUsr {
    /// User offset value for the Z-axis.
    #[bits(8, default = 0)]
    pub z_ofs_usr: i8,
}

/// Control register 7 (R/W).
///
/// The `CTRL7` register is used to configure various features, including high-pass filter reference mode, user offset application, and interrupt routing.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::Ctrl7, access_type = Iis2dlpc, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl7 {
    /// Low-pass filter data sent to 6D function.
    ///
    /// Configures the data sent to the 6D interrupt function:
    /// * `0`: ODR/2 low-pass filtered data.
    /// * `1`: LPF2 output data.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub lpass_on6d: u8,

    /// High-pass filter reference mode enable.
    ///
    /// Configures the high-pass filter reference mode:
    /// * `0`: Disabled.
    /// * `1`: Enabled.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub hp_ref_mode: u8,

    /// User offset weight selection.
    ///
    /// Configures the weight of the user offset:
    /// * `0`: 977 μg/LSB.
    /// * `1`: 15.6 mg/LSB.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub usr_off_w: u8,

    /// User offset application for wakeup function.
    ///
    /// Enables the application of user offset for wakeup detection:
    /// * `0`: Disabled.
    /// * `1`: Enabled.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub usr_off_on_wu: u8,

    /// User offset application for output data.
    ///
    /// Enables the application of user offset for output data:
    /// * `0`: Disabled.
    /// * `1`: Enabled.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub usr_off_on_out: u8,

    /// Interrupts enable.
    ///
    /// Enables interrupts:
    /// * `0`: Disabled.
    /// * `1`: Enabled.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub interrupts_enable: u8,

    /// INT2 signal routing to INT1.
    ///
    /// Routes all signals available on INT2 to INT1:
    /// * `0`: Disabled.
    /// * `1`: Enabled.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub int2_on_int1: u8,

    /// Data-ready interrupt mode.
    ///
    /// Configures the data-ready interrupt mode:
    /// * `0`: Latched mode.
    /// * `1`: Pulsed mode.
    ///
    /// Default value: `0`.
    #[bits(1, default = 0)]
    pub drdy_pulsed: u8,
}

/// All interrupt and status sources.
///
/// This struct aggregates the status and interrupt source registers of the IIS2DLPC sensor.
/// It provides a comprehensive view of the device's current status and interrupt events.
pub struct AllSources {
    /// Status duplicate register.
    ///
    /// Contains information about various events such as data-ready, free-fall detection, and tap detection.
    pub status_dup: StatusDup,

    /// Wake-up source register.
    ///
    /// Contains information about wake-up events, including axis-specific wake-up detection.
    pub wake_up_src: WakeUpSrc,

    /// Tap source register.
    ///
    /// Contains information about tap events, including axis-specific tap detection and tap type (single or double).
    pub tap_src: TapSrc,

    /// 6D source register.
    ///
    /// Contains information about 6D orientation events, including axis-specific thresholds and event detection.
    pub sixd_src: SixdSrc,

    /// All interrupt source register.
    ///
    /// Contains a summary of all interrupt events routed to the INT pads.
    pub all_int_src: AllIntSrc,
}

/// Accelerometer operating modes.
///
/// This enum represents the various operating modes of the IIS2DLPC accelerometer. Each mode is associated with specific configurations for:
/// - `mode`: Operating mode.
/// - `lp_mode`: Low-power mode configuration.
/// - `low_noise`: Low-noise mode configuration.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum Mode {
    /// High-performance mode.
    HighPerformance = 0x04,

    /// Continuous low-power mode 4.
    ContLowPwr4 = 0x03,

    /// Continuous low-power mode 3.
    ContLowPwr3 = 0x02,

    /// Continuous low-power mode 2.
    ContLowPwr2 = 0x01,

    /// Continuous low-power mode with 12-bit resolution (default).
    #[default]
    ContLowPwr12bit = 0x00,
    /// Single low-power mode 4.
    SingleLowPwr4 = 0x0B,
    /// Single low-power mode 3.
    SingleLowPwr3 = 0x0A,
    /// Single low-power mode 2.
    SingleLowPwr2 = 0x09,
    /// Single low-power mode with 12-bit resolution.
    SingleLowPwr12bit = 0x08,
    /// High-performance mode with low noise.
    HighPerformanceLowNoise = 0x14,
    /// Continuous low-power mode 4 with low noise.
    ContLowPwrLowNoise4 = 0x13,
    /// Continuous low-power mode 3 with low noise.
    ContLowPwrLowNoise3 = 0x12,
    /// Continuous low-power mode 2 with low noise.
    ContLowPwrLowNoise2 = 0x11,
    /// Continuous low-power mode with 12-bit resolution and low noise.
    ContLowPwrLowNoise12bit = 0x10,
    /// Single low-power mode 4 with low noise.
    SingleLowPwrLowNoise4 = 0x1B,
    /// Single low-power mode 3 with low noise.
    SingleLowPwrLowNoise3 = 0x1A,
    /// Single low-power mode 2 with low noise.
    SingleLowPwrLowNoise2 = 0x19,
    /// Single low-power mode with 12-bit resolution and low noise.
    SingleLowLowNoisePwr12bit = 0x18,
}

impl Mode {
    /// Create a new `Mode` instance.
    ///
    /// This function constructs a `Mode` instance from the given `mode`, `lp_mode`, and `low_noise` values.
    ///
    /// ### Arguments
    /// - `mode`: The operating mode value.
    /// - `lp_mode`: The low-power mode configuration value.
    /// - `low_noise`: The low-noise mode configuration value.
    ///
    /// ### Returns
    /// - A `Mode` instance corresponding to the provided values.
    /// - Defaults to `ContLowPwr12bit` if the provided values do not match a valid mode.
    pub fn new(mode: u8, lp_mode: u8, low_noise: u8) -> Self {
        // low_noise | mode1 | mode2 | lp_mode1 | lp_mode2 |
        Self::try_from((low_noise << 4) + (mode << 2) + lp_mode).unwrap_or_default()
    }

    /// Get the `mode` value.
    ///
    /// Extracts the `mode` field from the `Mode` instance.
    ///
    /// ### Returns
    /// - The `mode` value as a `u8`.
    pub fn mode(&self) -> u8 {
        (*self as u8 & 0x0C) >> 2
    }

    /// Get the `lp_mode` value.
    ///
    /// Extracts the `lp_mode` field from the `Mode` instance.
    ///
    /// ### Returns
    /// - The `lp_mode` value as a `u8`.
    pub fn lp_mode(&self) -> u8 {
        *self as u8 & 0x3
    }

    /// Get the `low_noise` value.
    ///
    /// Extracts the `low_noise` field from the `Mode` instance.
    ///
    /// ### Returns
    /// - The `low_noise` value as a `u8`.
    pub fn low_noise(&self) -> u8 {
        (*self as u8 & 0x10) >> 4
    }
}

/// Accelerometer output data rates (ODR).
///
/// This enum represents the various output data rates supported by the IIS2DLPC accelerometer. Each variant corresponds to a specific ODR configuration.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum Odr {
    /// Accelerometer off (default).
    #[default]
    Off = 0x00,

    /// Accelerometer ODR: 1.6 Hz (low-power only).
    _1_6hzLpOnly = 0x01,

    /// Accelerometer ODR: 12.5 Hz.
    _12_5hz = 0x02,

    /// Accelerometer ODR: 25 Hz.
    _25hz = 0x03,

    /// Accelerometer ODR: 50 Hz.
    _50hz = 0x04,

    /// Accelerometer ODR: 100 Hz.
    _100hz = 0x05,

    /// Accelerometer ODR: 200 Hz.
    _200hz = 0x06,

    /// Accelerometer ODR: 400 Hz.
    _400hz = 0x07,

    /// Accelerometer ODR: 800 Hz.
    _800hz = 0x08,

    /// Accelerometer ODR: 1.6 kHz.
    _1_6khz = 0x09,

    /// Accelerometer ODR: Software trigger.
    SetSwTrig = 0x12,

    /// Accelerometer ODR: Pin trigger.
    SetPinTrig = 0x22,
}

impl Odr {
    /// Create a new `Odr` instance.
    ///
    /// This function constructs an `Odr` instance from the given `odr` and `slp_mode` values.
    ///
    /// ### Arguments
    /// - `odr`: The output data rate value.
    /// - `slp_mode`: The sleep mode configuration value.
    ///
    /// ### Returns
    /// - An `Odr` instance corresponding to the provided values.
    /// - Defaults to `XlOdrOff` if the provided values do not match a valid ODR.
    pub fn new(odr: u8, slp_mode: u8) -> Self {
        Self::try_from((slp_mode << 4) + odr).unwrap_or_default()
    }

    /// Get the `odr` value.
    ///
    /// Extracts the `odr` field from the `Odr` instance.
    ///
    /// ### Returns
    /// - The `odr` value as a `u8`.
    pub fn odr(&self) -> u8 {
        *self as u8 & 0xF
    }

    /// Get the `slp_mode` value.
    ///
    /// Extracts the `slp_mode` field from the `Odr` instance.
    ///
    /// ### Returns
    /// - The `slp_mode` value as a `u8`.
    pub fn slp_mode(&self) -> u8 {
        (*self as u8 & 0x30) >> 4
    }
}

/// Accelerometer full-scale selection.
///
/// This enum represents the full-scale range of the accelerometer, which determines the maximum measurable acceleration.
/// The full-scale range is configured in the `CTRL6` register.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum Fs {
    /// ±2g full-scale range (default).
    #[default]
    _2g = 0,

    /// ±4g full-scale range.
    _4g = 1,

    /// ±8g full-scale range.
    _8g = 2,

    /// ±16g full-scale range.
    _16g = 3,
}

/// User offset weight configuration.
///
/// This enum represents the weight of the user offset bits in the `X_OFS_USR`, `Y_OFS_USR`, and `Z_OFS_USR` registers.
/// The weight determines the scaling factor applied to the user offset values.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum UsrOffW {
    /// 977 μg/LSB (default).
    #[default]
    _977ugLsb = 0,

    /// 15.6 mg/LSB.
    _15_6mgLsb = 1,
}

/// Sensor self-test configuration.
///
/// This enum represents the self-test modes for the IIS2DLPC sensor.
/// The self-test mode is configured in the `CTRL3` register.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum St {
    /// Self-test disabled (default).
    #[default]
    Disable = 0,

    /// Positive sign self-test.
    Positive = 1,

    /// Negative sign self-test.
    Negative = 2,
}

/// Data-ready interrupt mode configuration.
///
/// This enum represents the data-ready interrupt mode for the IIS2DLPC sensor.
/// The mode is configured in the `CTRL7` register.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum DrdyPulsed {
    /// Latched mode (default).
    #[default]
    Latched = 0,

    /// Pulsed mode.
    Pulsed = 1,
}

/// Accelerometer filtering path configuration.
///
/// This enum represents the filtering path options for accelerometer outputs.
/// The filtering path is configured in the `CTRL6` and `CTRL7` registers.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum Fds {
    /// Low-pass filter on output (default).
    #[default]
    LpfOnOut = 0x00,

    /// User offset on output.
    UserOffsetOnOut = 0x01,

    /// High-pass filter on output.
    HighPassOnOut = 0x10,
}

impl Fds {
    /// Create a new `Fds` instance.
    ///
    /// This function constructs an `Fds` instance from the given `fds` and `usr_off_on_out` values.
    ///
    /// ### Arguments
    /// - `fds`: The filtering path configuration value.
    /// - `usr_off_on_out`: The user offset application value.
    ///
    /// ### Returns
    /// - An `Fds` instance corresponding to the provided values.
    /// - Defaults to `LpfOnOut` if the provided values do not match a valid configuration.
    pub fn new(fds: u8, usr_off_on_out: u8) -> Self {
        Self::try_from((fds << 4) + usr_off_on_out).unwrap_or_default()
    }

    /// Get the `fds` value.
    ///
    /// Extracts the `fds` field from the `Fds` instance.
    ///
    /// ### Returns
    /// - The `fds` value as a `u8`.
    pub fn fds(&self) -> u8 {
        ((*self as u8) & 0x10) >> 4
    }

    /// Get the `usr_off_on_out` value.
    ///
    /// Extracts the `usr_off_on_out` field from the `Fds` instance.
    ///
    /// ### Returns
    /// - The `usr_off_on_out` value as a `u8`.
    pub fn usr_off_on_out(&self) -> u8 {
        *self as u8 & 0x01
    }
}

/// Accelerometer cutoff filter frequency configuration.
///
/// This enum represents the cutoff frequency options for the accelerometer's low-pass or high-pass filter.
/// The cutoff frequency is configured in the `CTRL6` register.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum BwFilt {
    /// ODR/2 (default).
    #[default]
    OdrDiv2 = 0,

    /// ODR/4.
    OdrDiv4 = 1,

    /// ODR/10.
    OdrDiv10 = 2,

    /// ODR/20.
    OdrDiv20 = 3,
}

/// SPI serial interface mode configuration.
///
/// This enum represents the SPI serial interface modes for the IIS2DLPC sensor.
/// The mode is configured in the `CTRL2` register.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum Sim {
    /// 4-wire SPI mode (default).
    #[default]
    Spi4Wire = 0,

    /// 3-wire SPI mode.
    Spi3Wire = 1,
}

/// I²C interface configuration.
///
/// This enum represents the enable/disable states of the I²C interface for the IIS2DLPC sensor.
/// The state is configured in the `CTRL2` register.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum I2cDisable {
    /// Enable the I²C interface (default).
    #[default]
    I2cEnable = 0,

    /// Disable the I²C interface.
    I2cDisable = 1,
}

/// CS pull-up resistor configuration.
///
/// This enum represents the configuration of the CS pull-up resistor for the IIS2DLPC sensor.
/// The configuration is set in the `CTRL2` register.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum CsPuDisc {
    /// Connect the pull-up resistor (default).
    #[default]
    PullUpConnect = 0,

    /// Disconnect the pull-up resistor.
    PullUpDisconnect = 1,
}

/// Interrupt active level configuration.
///
/// This enum represents the active level configuration for interrupts.
/// The configuration is set in the `CTRL3` register.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum HLactive {
    /// Active high (default).
    #[default]
    ActiveHigh = 0,

    /// Active low.
    ActiveLow = 1,
}

/// Interrupt latching configuration.
///
/// This enum represents the latching behavior of interrupts.
/// The configuration is set in the `CTRL3` register.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum Lir {
    /// Pulsed interrupt mode (default).
    #[default]
    Pulsed = 0,

    /// Latched interrupt mode.
    Latched = 1,
}

/// Interrupt pad type configuration.
///
/// This enum represents the type of interrupt pad configuration.
/// The configuration is set in the `CTRL3` register.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum PpOd {
    /// Push-pull configuration (default).
    #[default]
    PushPull = 0,

    /// Open-drain configuration.
    OpenDrain = 1,
}

/// Data source for the wake-up interrupt function.
///
/// This enum represents the data source options for the wake-up interrupt function.
/// The data source is configured in the `CTRL7` register.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum UsrOffOnWu {
    /// High-pass filtered data (default).
    #[default]
    HpFeed = 0,

    /// User offset data.
    UserOffsetFeed = 1,
}

/// Activity/inactivity or stationary/motion detection configuration.
///
/// This enum represents the detection modes for activity/inactivity or stationary/motion.
/// The configuration is set in the `WAKE_UP_THS` and `WAKE_UP_DUR` registers.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum SleepOn {
    /// No detection (default).
    #[default]
    NoDetection = 0,

    /// Detect activity/inactivity.
    ActInact = 1,

    /// Detect stationary/motion.
    StatMotion = 3,
}

impl SleepOn {
    /// Create a new `SleepOn` instance.
    ///
    /// This function constructs a `SleepOn` instance from the given `sleep_on` and `stationary` values.
    ///
    /// ### Arguments
    /// - `sleep_on`: The activity/inactivity detection configuration value.
    /// - `stationary`: The stationary/motion detection configuration value.
    ///
    /// ### Returns
    /// - A `SleepOn` instance corresponding to the provided values.
    /// - Defaults to `NoDetection` if the provided values do not match a valid configuration.
    pub fn new(sleep_on: u8, stationary: u8) -> Self {
        Self::try_from((stationary << 1) + sleep_on).unwrap_or_default()
    }

    /// Get the `sleep_on` value.
    ///
    /// Extracts the `sleep_on` field from the `SleepOn` instance.
    ///
    /// ### Returns
    /// - The `sleep_on` value as a `u8`.
    pub fn sleep_on(&self) -> u8 {
        (*self as u8) & 0x01
    }

    /// Get the `stationary` value.
    ///
    /// Extracts the `stationary` field from the `SleepOn` instance.
    ///
    /// ### Returns
    /// - The `stationary` value as a `u8`.
    pub fn stationary(&self) -> u8 {
        ((*self as u8) & 0x02) >> 1
    }
}

/// Axis priority for tap detection.
///
/// This enum represents the axis priority for tap detection.
/// The priority is configured in the `TAP_THS_Y` register.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum TapPrior {
    /// X > Y > Z (default).
    #[default]
    Xyz = 0,

    /// Y > X > Z.
    Yxz = 1,

    /// X > Z > Y.
    Xzy = 2,

    /// Z > Y > X.
    Zyx = 3,

    /// Y > Z > X.
    Yzx = 5,

    /// Z > X > Y.
    Zxy = 6,
}

/// Single/double-tap event detection mode.
///
/// This enum represents the detection mode for single- and double-tap events.
/// The mode is configured in the `WAKE_UP_THS` register.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum SingleDoubleTap {
    /// Detect only single-tap events (default).
    #[default]
    OnlySingle = 0,

    /// Detect both single- and double-tap events.
    BothSingleDouble = 1,
}

/// Data source for the 6D interrupt function.
///
/// This enum represents the data source options for the 6D interrupt function.
/// The data source is configured in the `CTRL7` register.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum LpassOn6d {
    /// ODR/2 low-pass filtered data (default).
    #[default]
    OdrDiv2Feed = 0,

    /// LPF2 output data.
    Lpf2Feed = 1,
}

/// Free-fall threshold configuration.
///
/// This enum represents the free-fall threshold options for the IIS2DLPC sensor.
/// The threshold is configured in the `FREE_FALL` register.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum FfThs {
    /// 5 LSB @ ±2g (default).
    #[default]
    _5Lsb = 0,

    /// 7 LSB @ ±2g.
    _7Lsb = 1,

    /// 8 LSB @ ±2g.
    _8Lsb = 2,

    /// 10 LSB @ ±2g.
    _10Lsb = 3,

    /// 11 LSB @ ±2g.
    _11Lsb = 4,

    /// 13 LSB @ ±2g.
    _13Lsb = 5,

    /// 15 LSB @ ±2g.
    _15Lsb = 6,

    /// 16 LSB @ ±2g.
    _16Lsb = 7,
}

/// FIFO mode configuration.
///
/// This enum represents the FIFO operating modes for the IIS2DLPC sensor.
/// The mode is configured in the `FIFO_CTRL` register.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum Fmode {
    /// Bypass mode (default).
    #[default]
    BypassMode = 0,

    /// FIFO mode: Stops collecting data when FIFO is full.
    FifoMode = 1,

    /// Stream-to-FIFO mode: Stream mode until a trigger event, then FIFO mode.
    StreamToFifoMode = 3,

    /// Bypass-to-Stream mode: Bypass mode until a trigger event, then stream mode.
    BypassToStreamMode = 4,

    /// Stream mode: Continuously updates FIFO, overwriting old data when full.
    StreamMode = 6,
}
