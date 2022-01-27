#!/bin/bash

(cargo afl build) &&
	(cargo afl fuzz -t 9999 -D -i in/coord -o out/coord -- target/debug/pixelflut_fuzzer -- coordinate)
