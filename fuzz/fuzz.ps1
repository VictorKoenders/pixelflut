#!/bin/bash

(cargo afl build --release) &&
	(cargo afl fuzz -t 9999 -D -i in -o out -- target/release/pixelflut_fuzzer)
