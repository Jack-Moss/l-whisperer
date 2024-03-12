use circular_buffer::CircularBuffer;
use ::mycrate::i2c_config;
use std::collections::VecDeque;
// is it possible to simulate an i2c connection for this project? would be a lot easier to develop if so.




// I think I need a custom implementation of the circular buffer that allows me to store arrays rather than singular numbers as I want
// {acc,gyro,mag} readings at each point
fn main() {
  //create buffer with len 100
  let mut buffer: CircularBuffer<100, T> = CircularBuffer::<100,T>::new();
  let mut dump_stack;
  let deque: VecDeque<T> = VecDeque::with_capacity(300);

  let dataloop = loop {



    // loop of adding items to the circular buffer

  };
}

