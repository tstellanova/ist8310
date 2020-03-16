
use ist8310::IST8310;


// use embedded_hal::prelude::*;
// use embedded_hal::blocking::i2c::{Read, Write, WriteRead};
use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTransaction};


#[test]
fn test_init() {
    const SRST_POR_FLAG: u8 = 0x01 << 0;
    const SRPD_MODE_LOW_POWER: u8 = 0xC0;
    const AVG_CTRL_16X: u8 = 0x24;

    let addr = ist8310::DEFAULT_ADDRESS;

    // Configure expectations
    let expectations = [
        I2cTransaction::write(addr, vec![ist8310::REG_CTRL2, SRST_POR_FLAG]),
        I2cTransaction::write(addr, vec![ist8310::REG_AVG_CTRL, AVG_CTRL_16X]),
        I2cTransaction::write(addr, vec![ist8310::REG_SENS_MODE_SELECT, SRPD_MODE_LOW_POWER]),
        I2cTransaction::write_read(addr, vec![ist8310::REG_WAI], vec![0x10]),
    ];

    let i2c_port = I2cMock::new(&expectations);
    let sensor_res = IST8310::default(i2c_port);
    assert!(sensor_res.is_ok());

}