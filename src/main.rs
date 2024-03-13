use circular_buffer::CircularBuffer;
use ::mycrate::i2c_config;
use std::collections::VecDeque;
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
  let mut buffer: CircularBuffer<100, DataPoint> = CircularBuffer::<100,DataPoint>::new();
  let dump_stack: Vec<DataPoint> = Vec::with_capacity(1000);
  let deque: VecDeque<DataPoint> = VecDeque::with_capacity(300);

  let dataloop = loop {mut
    mut


    // loop of adding items to the circular buffer

  };
}

