use mpu6050::*;
use linux_embedded_hal::{I2cdev, Delay};
use i2cdev::linux::LinuxI2CError;
use circular_buffer::CircularBuffer;

struct Data_point{
  accel: u32,
  gyro: u32,
  magnetic: u32,
}
// I think I need a custom implementation of the circular buffer that allows me to store arrays rather than singular numbers as I want
// {acc,gyro,mag} readings at each point
fn main() -> Result<(), Mpu6050Error<LinuxI2CError>> {
  let i2c = I2cdev::new("/dev/i2c-1")
          .map_err(Mpu6050Error::I2c)?;

  let mut delay = Delay;
  let mut mpu = Mpu6050::new(i2c);
  //create buffer with len 100
  let mut buffer: CircularBuffer<100, Data_point> = CircularBuffer::<100,Data_point>::new();
  mpu.init(&mut delay)?;

  loop {
    let datapoint = Data_point{
      magnetic: mpu.get_acc_angles(),
      gyro:mpu.get_gyro(),
      accel:mpu.get_acc(),
    };
    buffer.push_back(datapoint);

    // get roll and pitch estimate? is there a yaw here too?

    // let acc = mpu.get_acc_angles()?;
    // println!("r/p: {:?}", ang);
    // // get gyro data, scaled with sensitivity 
    // let gyro = mpu.get_gyro()?;
    // println!("gyro: {:?}", gyro);
    // // get accelerometer data, scaled with sensitivity
    // let acc = mpu.get_acc()?;
    // println!("acc: {:?}", acc);

  }
}