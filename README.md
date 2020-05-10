# ist8310

A rust embedded-hal driver for the 
Isentek IST8310 
3-axis magnetometer.

This sensor is claimed to be pin-compatible with the obsolete
Honeywell HMC5883 magnetometer. 
Like the HMC5883, the IST8310 only supports an I2C interface.

The register map of the IST8310 differs significantly from
the HMC5883, and the reset and configuration sequence is different.

## Status

- [x] Basic i2c setup support
- [x] read of main xyz magentometer vector
- [ ] Tests with mock embedded hal
- [ ] Periodic configuration check (for poor i2c connections)
- [ ] Usage example with `cortex-m` hal
- [ ] Doc comments
- [ ] CI
- [ ] support for cross-axis flow calibration





