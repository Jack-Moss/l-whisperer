use circular_buffer::CircularBuffer;
use std::{f32::consts, intrinsics::sqrtf32};
// is it possible to simulate an i2c connection for this project? would be a lot easier to develop if so.
#[derive(PartialEq, Debug)]
struct Direction(f32,f32,f32);
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
  // i should try to split this into two threads, and have one stop the other on
  // a detection?
  while data_buffer.len() < 3000 {
    data_buffer.push_back(simulate_read());

    let mut last_five = CircularBuffer::<5, DataPoint>::new();
    last_five.push_back(simulate_read());
    check_acceleration(last_five)
  }
}

fn check_acceleration(last_five:CircularBuffer<5,DataPoint>){
  let acceleration: f32;
  let boundary: f32;
  let g: f32 = 9.80665;
  //need to implement some kind of timer
  for datapoint in last_five {
    //acceleration
    let accel_x:f32 =     datapoint.accel.0;
    let accel_y:f32 =     datapoint.accel.1;
    let accel_z:f32 =     datapoint.accel.2;

    let gyro_x:f32 =     datapoint.gyro.0;
    let gyro_y:f32 =     datapoint.gyro.1;
    let gyro_z:f32 =     datapoint.gyro.2;

    //This is only the looking or zero g function, which is probably not what we are after.
    
    if SQRT_2(accel_x ** 2.0 + accel_y ** 2.0 + accel_z ** 2.0) >= 1 {
    }

    //total_acceleration = math.sqrt(self.Accel[0] ** 2 + self.Accel[1] ** 2 + self.Accel[2] ** 2)

    // let mag_x:f32 =     datapoint.magnetic.0;
    // let mag_y:f32 =     datapoint.magnetic.1;
    // let mag_z:f32 =     datapoint.magnetic.2;

    
  }

}

fn create_randomised_direction()->Direction { 
  Direction(    rand::random::<f32>()
,    rand::random::<f32>()
,    rand::random::<f32>()
)
}

fn simulate_read() -> DataPoint {
  let accel = create_randomised_direction();
  let gyro = create_randomised_direction();
  let mag = create_randomised_direction();
  DataPoint{ accel: accel, gyro: gyro, magnetic: mag }
}

fn real_read(){
  todo!();
}

fn calculate_kn(climber_weight: f32, max_speed: f32) ->f32 {
  climber_weight * max_speed
}

#[test]
fn test_calculate_kilonewtons(){
    assert_eq!(calculate_kn(2.0,3.0), 6.0);
}
#[test]
fn test_populate_circular_buffer(){
  let mut circle_buffer = CircularBuffer::<3000, DataPoint>::new();
  while circle_buffer.len() < 3000 {
    circle_buffer.push_back(simulate_read())
  }
  assert_eq!(circle_buffer.len(), 3000);
}
