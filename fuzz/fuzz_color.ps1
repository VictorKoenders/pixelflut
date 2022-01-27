#!/bin/bash

(cargo afl build) &&
	(cargo afl fuzz -D -i in/color -o out/color -- target/debug/pixelflut_fuzzer -- color)
