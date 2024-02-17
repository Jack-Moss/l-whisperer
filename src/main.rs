use mpu6050::*;
use linux_embedded_hal::{I2cdev, Delay};
use i2cdev::linux::LinuxI2CError;
use circular_buffer::CircularBuffer;




struct DataPoint{
  accel: u32,
  gyro: u32,
  magnetic: u32,
}

impl DataPoint{
  //holyshit this would let you set custom processes if you wanted so 
//if I wanted to I could just pull raw data and have it edited at the datapoint level which is 
//pretty darned wild. I like that a lot not sure if its more efficent though
//
}


struct StackDump{
  stack: DataPoint,
}

// I think I need a custom implementation of the circular buffer that allows me to store arrays rather than singular numbers as I want
// {acc,gyro,mag} readings at each point
fn main() -> Result<(), Mpu6050Error<LinuxI2CError>> {
  let i2c = I2cdev::new("/dev/i2c-1")
          .map_err(Mpu6050Error::I2c)?;

  let mut delay = Delay;
  let mut mpu = Mpu6050::new(i2c);
  //create buffer with len 100
  let mut buffer: CircularBuffer<100, DataPoint> = CircularBuffer::<100,DataPoint>::new();
  let mut dump_stack;
  mpu.init(&mut delay)?;

  loop {




    let datapoint = DataPoint{
      magnetic: mpu.get_acc_angles(),
      gyro:mpu.get_gyro(),
      accel:mpu.get_acc(),
    };

    buffer.push_back(datapoint);




    // let acc = mpu.get_acc_angles()?;
    // println!("r/p: {:?}", ang);
    // let gyro = mpu.get_gyro()?;
    // println!("gyro: {:?}", gyro);
    // let acc = mpu.get_acc()?;
    // println!("acc: {:?}", acc);

  }
}

