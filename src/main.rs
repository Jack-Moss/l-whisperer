use circular_buffer::CircularBuffer;
use std::collections::VecDeque;
use mpu6050::*;
use linux_embedded_hal::{I2cdev, Delay};
use i2cdev::linux::LinuxI2CError;

// is it possible to simulate an i2c connection for this project? would be a lot easier to develop if so.

struct Direction(i32,i32,i32);
struct DataPoint{
  accel: Direction,
  gyro: Direction,
  magnetic: Direction,
}
struct datastructure{
  font_buffer: CircularBuffer<3000, DataPoint>
}
// I think I need a custom implementation of the circular buffer that allows me to store arrays rather than singular numbers as I want
// {acc,gyro,mag} readings at each point
fn main() {
  //create buffer with len 100
  let mut buffer: CircularBuffer<300, DataPoint> = CircularBuffer::<300,DataPoint>::new();
  let deque: VecDeque<DataPoint> = VecDeque::with_capacity(1300);


}



fn calculate_kn(climber_weight: i32, max_speed: i32) ->i32 {
  climber_weight * max_speed
}

#[test]
fn test_calculate_kilonewtons(){
    assert_eq!(calculate_kn(2,3), 6);
}
