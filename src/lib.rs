/*
Copyright (c) 2020 Todd Stellanova
LICENSE: BSD3 (see LICENSE file)
*/

#![no_std]


/// Errors in this crate
#[derive(Debug)]
pub enum Error<CommE> {
    /// Sensor communication error
    Comm(CommE),
}

/// This device supports multiple addresses depending on
/// the configuration of CAD0 and CAD1
/// The format of these address is ADDR_CAD0_CAD1_nBIT,
/// Where 0 indicates tie to ground, 1 to Vdd
/// If CAD0 and CAD1 are floating, I2C address will be 0x0E / 0x1C.
pub const ADDR_0_0_7BIT:u8 = 0x0C;
pub const ADDR_0_1_7BIT:u8 = 0x0D;
pub const ADDR_1_0_7BIT:u8 = 0x0E;
pub const ADDR_1_1_7BIT:u8 = 0x0F;
pub const ADDR_7BIT_DEFAULT:u8 = 0x0E;

pub const ADDR_0_0_8BIT:u8 = 0x18;
pub const ADDR_0_1_8BIT:u8 = 0x1A;
pub const ADDR_1_0_8BIT:u8 = 0x1C;
pub const ADDR_1_1_8BIT:u8 = 0x1E;
pub const ADDR_8BIT_DEFAULT:u8 = 0x1C;

pub const DEFAULT_ADDRESS:u8 = ADDR_7BIT_DEFAULT;


const SEND_BUF_LEN: usize = 32;

struct IST8310<I2C> {
    address: u8,
    i2c_port: I2C,
    packet_buf: [u8;SEND_BUF_LEN ]
}

impl<I2C, CommE> IST8310<I2C>
    where
        I2C: embedded_hal::blocking::i2c::Write<Error = CommE>
        + embedded_hal::blocking::i2c::Read<Error = CommE>
        + embedded_hal::blocking::i2c::WriteRead<Error = CommE>,
{

    fn default(i2c: I2C) -> Self {
        Self::new(i2c, DEFAULT_ADDRESS)
    }

    fn new(i2c: I2C, addr: u8) ->  Self {
        Self {
            i2c_port: i2c,
            address: addr,
            packet_buf: [0;SEND_BUF_LEN]
        }
    }

    fn transfer(&mut self, address: u8, send_buf: &[u8], recv_buf: &mut [u8]) {

    }


    /// Read a block from a specific register
    /// reg: The register address to read from
    /// recv_buf: The buffer to receive into
    fn read(&mut self, reg: u8, recv_buf: &mut [u8]) -> Result<(), Error<CommE>> {
        let cmd_buf = [reg];
        self.i2c_port
            .write_read(self.address, &cmd_buf, recv_buf)
            .map_err(Error::Comm)?;
        Ok(())
    }

    ///	 Write a block to a specific register
    /// reg: The register address to write to
    /// send_buf: The buffer to send
    fn write(&mut self, reg: u8, send_buf: &[u8]) -> Result<(), Error<CommE>>{
        self.packet_buf[0] = reg;
        //this panics if send_buf is bigger than expected:
        self.packet_buf[1..send_buf.len()+1].copy_from_slice(send_buf);
        self.i2c_port
            .write(self.address, &self.packet_buf)
            .map_err(Error::Comm)?;
        Ok(())
    }


    fn read_reg(&mut self, reg: u8, val: &mut u8 ) {
        let mut read_buf = [val];
        self.read(reg,&mut read_buf);
        val = read_buf[0];
    }

    fn write_reg(&mut self, reg: u8, val: u8) {

    }

}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
