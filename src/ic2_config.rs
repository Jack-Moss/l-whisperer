//Gonna setup the i2c connection and refactor into this file
use mpu6050::*;
use linux_embedded_hal::{I2cdev, Delay};
use i2cdev::linux::LinuxI2CError;

struct DataPoint{
    accel: f32,
    gyro: f32,
    magnetic: f32,
  }

fn main(){
    let i2c = I2cdev::new("/dev/i2c-1")
    .map_err(Mpu6050Error::I2c)?;
    
    let mut delay = Delay;
    let mut mpu = Mpu6050::new(i2c);
    mpu.init(&mut delay)?;
    let datapoint = DataPoint{
        magnetic: mpu.get_acc_angles(),
        gyro:mpu.get_gyro(),
        accel:mpu.get_acc(),
      };
      buffer.push_back(datapoint);


}