use circular_buffer::CircularBuffer;
// is it possible to simulate an i2c connection for this project? would be a lot easier to develop if so.
#[derive(PartialEq, Debug)]
struct Direction(i32,i32,i32);
struct DataPoint{
  accel: Direction,
  gyro: Direction,
  magnetic: Direction,
}


// I think I need a custom implementation of the circular buffer that allows me to store arrays rather than singular numbers as I want
// {acc,gyro,mag} readings at each point
fn main() {
  //create buffer with len 100
  //start to fill buffer
  let mut data_buffer =  CircularBuffer::<3000, DataPoint>::new();
  let mut last_five: circular_buffer = CircularBuffer::<5, DataPoint>::new();
  // i should try to split this into two threads, and have one stop the other on
  // a detection?
  while data_buffer.len() < 3000 {
    data_buffer.push_back(simulate_read());
    last_five.push_back(simulate_read());
    check_acceleration(last_five)
  }
}

fn check_acceleration(last_five:CircularBuffer){

}


fn simulate_read() -> DataPoint {
  let accel = Direction(1,2,3);
  let gyro = Direction(1,2,3);
  let mag = Direction(1,2,3);
  DataPoint{ accel: accel, gyro: gyro, magnetic: mag }
}


fn calculate_kn(climber_weight: i32, max_speed: i32) ->i32 {
  climber_weight * max_speed
}

// fn chech_for_accel(data_point:DataPoint, )
// fn chech_for_accel(data_point:DataPoint, )

#[test]
fn test_calculate_kilonewtons(){
    assert_eq!(calculate_kn(2,3), 6);
}
#[test]
fn test_sim_build(){
  let data_entry = simulate_read();
  assert_eq!(data_entry.accel, Direction(1,2,3));
  assert_eq!(data_entry.gyro, Direction(1,2,3));
  assert_eq!(data_entry.magnetic, Direction(1,2,3));
  // assert_eq!(circle_buffer[0])

}
#[test]
fn test_populate_circular_buffer(){
  let mut circle_buffer = CircularBuffer::<3000, DataPoint>::new();
  while circle_buffer.len() < 3000 {
    circle_buffer.push_back(simulate_read())
  }
  assert_eq!(circle_buffer.len(), 3000);
}
