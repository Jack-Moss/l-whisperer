use circular_buffer::CircularBuffer;
use std::f32;

// is it possible to simulate an i2c connection for this project? would be a lot easier to develop if so.
#[derive(PartialEq, Debug, Clone)]
struct Direction(f32,f32,f32);
struct DataPoint{
  accel: Direction,
  // gyro: Direction,
  // magnetic: Direction,
}


// I think I need a custom implementation of the circular buffer that allows me to store arrays rather than singular numbers as I want
// {acc,gyro,mag} readings at each point
fn main() {
  //create buffer with len 100
  //start to fill buffer
  let mut data_buffer =  CircularBuffer::<3000, DataPoint>::new();
  // i should try to split this into two threads, and have one stop the other on
  // a detection?
  let mut detected = false;
  while data_buffer.len() < 3000 {
    data_buffer.push_back(simulate_read());

    let mut last_five = CircularBuffer::<5, DataPoint>::new();
    last_five.push_back(simulate_read());
    detected = check_acceleration(last_five)

  }
  while !detected {
    data_buffer.push_back(simulate_read());
    let mut last_five = CircularBuffer::<5, DataPoint>::new();
    last_five.push_back(simulate_read());
    detected = check_acceleration(last_five)

  }
  //commence dump of data here
}

fn check_acceleration(last_five:CircularBuffer<5,DataPoint>) -> bool{
  // let acceleration: f32;
  // let boundary: f32;
  let g: f32 = 9.80665;
  //need to implement some kind of timer

  let mut accel_x:f32 = 0.0;
  let mut accel_y:f32 = 0.0;
  let mut accel_z:f32 = 0.0;
  for datapoint in last_five {
    //acceleration
    accel_x =  accel_x +   datapoint.accel.0;
    accel_y =  accel_y +   datapoint.accel.1;
    accel_z =  accel_z +   datapoint.accel.2;

    // let gyro_x:f32 =     datapoint.gyro.0;
    // let gyro_y:f32 =     datapoint.gyro.1;
    // let gyro_z:f32 =     datapoint.gyro.2;

    }
    //Get the average of the five and stick that in to get an average for that chunk
    accel_x =  accel_x / 5.0;
    accel_y =  accel_y / 5.0;
    accel_z =  accel_z / 5.0;

    let accel = (accel_x.powf(2.0)+ accel_y.powf(2.0) + accel_z.powf(2.0)).sqrt();
    if accel  <= g {
      println!("registered accelleration is {}",accel);
      return true


    //total_acceleration = math.sqrt(self.Accel[0] ** 2 + self.Accel[1] ** 2 + self.Accel[2] ** 2)

    // let mag_x:f32 =     datapoint.magnetic.0;
    // let mag_y:f32 =     datapoint.magnetic.1;
    // let mag_z:f32 =     datapoint.magnetic.2;

    
  }
  false

}
fn create_randomised_direction()->Direction { 
  Direction(rand::random::<f32>(),
            rand::random::<f32>(),
            rand::random::<f32>(),
)
}

fn simulate_read() -> DataPoint {
  let accel = create_randomised_direction();
  // let gyro = create_randomised_direction();
  // let mag = create_randomised_direction();
  DataPoint{ accel: accel }
}

// fn real_read(){
//   todo!();
// }

// fn calculate_kn(climber_weight: f32, max_speed: f32) ->f32 {
//   climber_weight * max_speed
// }

// #[test]
// fn test_calculate_kilonewtons(){
//     assert_eq!(calculate_kn(2.0,3.0), 6.0);
// }
#[test]
fn test_populate_circular_buffer(){
  let mut circle_buffer = CircularBuffer::<3000, DataPoint>::new();
  while circle_buffer.len() < 3000 {
    circle_buffer.push_back(simulate_read())
  }
  assert_eq!(circle_buffer.len(), 3000);
}
#[test]
fn test_simulate_detection(){
  let accel = Direction(0.0, 0.0,0.0,);
  let simulate_hit = DataPoint{accel:accel.clone()}; 

  let mut last_five = CircularBuffer::<5,DataPoint>::new();
  while last_five.len() <= 4 {
    last_five.push_back(simulate_read())
  }
  last_five.push_back(simulate_hit);
  check_acceleration(last_five);
}
