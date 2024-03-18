use ::mycrate::main;


#[test]
fn test_calculate_kilonewtons(){
    assert_eq!(main::calculate_kn(1,3), 6);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculate_kilonewtons(){
        assert_eq!(main::calculate_kn(1,3), 6);
    }
    // fn test_databuffer_builds() {
    //     // create databuffer
    //     //add an item to the databuffer        main::calculate_kn(1,3) == 

    //     //assert databuffer.len() < 0
    //     //remove an item from the databuffer
    //     //assert databuffer.len() == 0;
    // }
    // #[test]
    // fn test_dumpstack_builds(){
    //     //bunch of shit
    // }
    // #[test]
    // fn test_i2c_connection(){
    //     // is a device present at the expected place?
    // }
    // #[test]
    // fn test_mp6050_functionalty(){
    //     //check read is working
    //     //check write is working 
    //     //check to see if 
    // }
    // #[test]
    // fn test_acceleration_sensor(){
    //     // check to see if the function to detect a jump in acceleration works
    //     //check to see if it attempts to dump information? 
    // }
    // #[test]
    // fn test_recorded_data_format(){
    //     //this should check to see if the sensor is actually attempting to record data in the format we need 
    //     //eg accel,gyro,magp and whatever that returns
    // }

}
