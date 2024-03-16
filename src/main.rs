use circular_buffer::CircularBuffer;
use std::collections::VecDeque;
use mpu6050::*;
use linux_embedded_hal::{I2cdev, Delay};
use i2cdev::linux::LinuxI2CError;

// is it possible to simulate an i2c connection for this project? would be a lot easier to develop if so.
struct DataPoint{
  accel: f32,
  gyro: f32,
  magnetic: f32,
}



// I think I need a custom implementation of the circular buffer that allows me to store arrays rather than singular numbers as I want
// {acc,gyro,mag} readings at each point
fn main() {
  //create buffer with len 100
  let mut buffer: CircularBuffer<300, DataPoint> = CircularBuffer::<300,DataPoint>::new();
  let deque: VecDeque<DataPoint> = VecDeque::with_capacity(1300);


  let data_loop = loop {
    //Start loop and keep the led turning on and off while its workinginto (maybe in a seperate thread? or just flash a bunch at the start)

  };
  //while not detecting sudden acceleration on any axis
  //Read from i2c connection 
  //add to circular buffer
  //if sudden acceleration detected then switch to dumping into the dequeue until its full
  //dump the ciruclar buffer into the front of the deque in reverse order so its all alighned. 
  //end process and turn off led.


}

fn i2c_init(){
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