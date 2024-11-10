use crate::ina219::calibration::Calibration;
use crate::ina219::config::Config;
use core::borrow::BorrowMut;
use core::marker::PhantomData;
use embassy_rp::i2c;
use embassy_rp::i2c::{Async, I2c};

struct Ina219<'d, I: i2c::Instance + 'd, B: BorrowMut<I2c<'d, I, Async>>> {
    i2c_bus: B,
    address: u16,
    config: Config,
    calibration: Calibration,
    phantom: PhantomData<&'d I>,
}

impl<'d, I: i2c::Instance + 'd, B: BorrowMut<I2c<'d, I, Async>>> Ina219<'d, I, B> {
    pub async fn new(
        i2c_bus: B,
        address: u16,
        config: Config,
        calibration: Calibration,
    ) -> Result<Self, i2c::Error> {
        let mut ina219 = Self {
            i2c_bus,
            address,
            config,
            calibration,
            phantom: Default::default(),
        };
        ina219.reset().await?;
        Ok(ina219)
    }

    pub async fn reset(&mut self) -> Result<(), i2c::Error> {
        self.write_register(0, 0x8000).await?;
        if self.config != Default::default() {
            self.write_config().await?;
        }
        if self.calibration.as_u16() != 0 {
            self.write_calibration().await?;
        }
        Ok(())
    }

    pub async fn set_config(&mut self, config: Config) -> Result<(), i2c::Error> {
        if self.config != config {
            self.config = config;
            self.write_config().await?;
        }
        Ok(())
    }

    pub async fn set_calibration(&mut self, calibration: Calibration) -> Result<(), i2c::Error> {
        let old_value = self.calibration.as_u16();
        self.calibration = calibration;
        if old_value != self.calibration.as_u16() {
            self.write_calibration().await?;
        }
        Ok(())
    }

    async fn write_register(&mut self, register_address: u8, data: u16) -> Result<(), i2c::Error> {
        let bus = self.i2c_bus.borrow_mut();
        let mut bytes = [register_address, 0, 0];
        bytes[1..].copy_from_slice(&data.to_be_bytes());
        bus.write_async(self.address, bytes).await
    }

    async fn write_config(&mut self) -> Result<(), i2c::Error> {
        self.write_register(0, self.config.as_u16()).await
    }

    async fn write_calibration(&mut self) -> Result<(), i2c::Error> {
        self.write_register(5, self.calibration.as_u16()).await
    }
}
