#![no_main]
use libfuzzer_sys::fuzz_target;
use weezl::{BitOrder, encode, decode};

fuzz_target!(|data: &[u8]| {
    let mut encoder = encode::Encoder::with_tiff_size_switch(BitOrder::Msb, 8);
    let mut buffer = Vec::with_capacity(2*data.len() + 40);
    let _ = encoder.into_stream(&mut buffer).encode_all(data);

    let mut decoder = decode::Decoder::with_tiff_size_switch(BitOrder::Msb, 8);
    let mut compare = vec![];
    let result = decoder.into_stream(&mut compare).decode_all(buffer.as_slice());
    assert!(result.status.is_ok(), "{:?}", result.status);
    assert_eq!(data, &*compare);
});
