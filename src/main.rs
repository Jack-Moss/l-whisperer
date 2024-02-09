use mpu6050::*;
use linux_embedded_hal::{I2cdev, Delay};
use i2cdev::linux::LinuxI2CError;
use circular_buffer::CircularBuffer;


fn main() -> Result<(), Mpu6050Error<LinuxI2CError>> {
  let i2c = I2cdev::new("/dev/i2c-1")
          .map_err(Mpu6050Error::I2c)?;

  let mut delay = Delay;
  let mut mpu = Mpu6050::new(i2c);
  //create buffer with len 100
  let mut buffer = CircularBuffer::<100,u32>::new();
  mpu.init(&mut delay)?;

  loop {



    // get roll and pitch estimate? is there a yaw here too?

    let acc = mpu.get_acc_angles()?;
    println!("r/p: {:?}", ang);
    // get gyro data, scaled with sensitivity 
    let gyro = mpu.get_gyro()?;
    println!("gyro: {:?}", gyro);
    // get accelerometer data, scaled with sensitivity
    let acc = mpu.get_acc()?;
    println!("acc: {:?}", acc);
    let map: ???
    buffer.push_back({acc ,gyro, ang});
    //Do some unholy maths to get a general 
    //apply a kalman filter to moderate noise
    //Do a current accerlation check
    // add the current value to the buffer
    buffer.push_back(current_acceleration);

  }
}