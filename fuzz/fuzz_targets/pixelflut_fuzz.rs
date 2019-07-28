#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate pixelflut;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    let _  = pixelflut::client::Client.handle_message_response(data);
});
