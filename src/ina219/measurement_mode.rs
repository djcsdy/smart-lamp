#[repr(u8)]
pub enum MeasurementMode {
    /// 9 bits of resolution.
    /// 
    /// Conversion time of 84 μs.
    Resolution9Bit = 0b0000,
    /// 10 bits of resolution.
    ///
    /// Conversion time of 148 μs.
    Resolution10Bit = 0b0001,
    /// 11 bits of resolution.
    ///
    /// Conversion time of 276 μs.
    Resolution11Bit = 0b0010,
    /// 12 bits of resolution.
    ///
    /// Conversion time of 532 μs.
    Resolution12Bit = 0b0011,
    /// Average of two samples with 12 bits of resolution.
    /// 
    /// Conversion time of 1.06 ms.
    AverageOf2 = 0b1001,
    /// Average of four samples with 12 bits of resolution.
    ///
    /// Conversion time of 2.13 ms.
    AverageOf4 = 0b1010,
    /// Average of eight samples with 12 bits of resolution.
    ///
    /// Conversion time of 4.26 ms.
    AverageOf8 = 0b1011,
    /// Average of 16 samples with 12 bits of resolution.
    ///
    /// Conversion time of 8.51 ms.
    AverageOf16 = 0b1100,
    /// Average of 32 samples with 12 bits of resolution.
    ///
    /// Conversion time of 17.02 ms.
    AverageOf32 = 0b1101,
    /// Average of 64 samples with 12 bits of resolution.
    ///
    /// Conversion time of 38.05 ms.
    AverageOf64 = 0b1110,
    /// Average of 128 samples with 12 bits of resolution.
    ///
    /// Conversion time of 68.10 ms.
    AverageOf128 = 0b1111,
}