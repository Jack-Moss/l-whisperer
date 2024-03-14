# loose-whisperer

This is going to be an iot embedded rust sensor using an mpu6050 that will record motion data from loosing an arrow.
https://github.com/juliangaal/mpu6050/blob/master/examples/README.md

$ cargo build --examples --target=armv7-unknown-linux-gnueabihf

https://docs.rust-embedded.org/book/unsorted/speed-vs-size.html
Gonna need a read speed test at some point, somehow. and maybe use that to check the rest. would be interesting for sure.
As of 2018-09-18 rustc supports three "optimize for speed" levels: opt-level = 1, 2 and 3. When you run cargo build --release you are using the release profile which defaults to opt-level = 3.

all hail stack overflow for simulating an i2c device (on ubuntu): https://stackoverflow.com/questions/63430327/how-can-i-emulate-an-i2c-device-on-linux

# Set up fake device
$ sudo modprobe i2c-dev
$ sudo modprobe i2c-stub chip_addr=0x03
$ i2cdetect -l
...
i2c-7  unknown    SMBus stub driver    N/A    <<--- Here it is!

# Read data
$ sudo i2cdump -y -r 0-7 7 0x03 b
     0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f    0123456789abcdef
00: 00 00 00 00 00 00 00 00                            .?......        

# Write data
$ sudo i2cset -y 7 0x03 0x01 0x27

# Read data again
$ sudo i2cdump -y -r 0-7 7 0x03 b
     0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f    0123456789abcdef
00: 00 27 00 00 00 00 00 00                            .?......        