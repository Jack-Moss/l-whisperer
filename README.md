# loose-whisperer

This is going to be an iot embedded rust sensor using an mpu6050 that will record motion data from loosing an arrow.
https://github.com/juliangaal/mpu6050/blob/master/examples/README.md
https://docs.rust-embedded.org/book/unsorted/speed-vs-size.html
Gonna need a read speed test at some point, somehow. and maybe use that to check the rest. would be interesting for sure.
As of 2018-09-18 rustc supports three "optimize for speed" levels: opt-level = 1, 2 and 3. When you run cargo build --release you are using the release profile which defaults to opt-level = 3.
