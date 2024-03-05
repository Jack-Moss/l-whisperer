//Gonna setup the i2c connection and refactor into this file

fn main(){
    let i2c = I2cdev::new("/dev/i2c-1")
    .map_err(Mpu6050Error::I2c)?;

    let mut delay = Delay;
    let mut mpu = Mpu6050::new(i2c);

}