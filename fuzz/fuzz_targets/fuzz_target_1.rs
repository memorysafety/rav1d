#![no_main]

use libfuzzer_sys::fuzz_target;
use rav1d::Rav1dError;

fuzz_target!(|data: &[u8]| {
    let boxed_data = Vec::from(data).into_boxed_slice();
    let _ = decode(boxed_data); // don't care about returned errors
});


fn handle_pending_pictures(dec: &mut rav1d::rust_api::Decoder, drain: bool) -> std::io::Result<()> {
    loop {
        match dec.get_picture() {
            Ok(p) => println!("{:?}", p),
            // Need to send more data to the decoder before it can decode new pictures
            Err(Rav1dError::TryAgain) => return Ok(()),
            Err(e) => {
                return Err(std::io::Error::other(format!("Error getting decoded pictures: {}", e)));
            }
        }

        if !drain {
            break;
        }
    }
    Ok(())
}

fn decode(data: Box<[u8]>) -> std::io::Result<()> {
    let mut dec = rav1d::rust_api::Decoder::new().expect("failed to create decoder instance");

        // Send packet to the decoder
        match dec.send_data(data, None, None, None) {
            Err(Rav1dError::TryAgain) => {
                // If the decoder did not consume all data, output all
                // pending pictures and send pending data to the decoder
                // until it is all used up.
                loop {
                    handle_pending_pictures(&mut dec, false)?;

                    match dec.send_pending_data() {
                        Err(Rav1dError::TryAgain) => continue,
                        Err(e) => {
                            return Err(std::io::Error::other(format!("Error sending pending data to the decoder: {}", e)));
                        }
                        _ => break,
                    }
                }
            }
            Err(e) => {
                return Err(std::io::Error::other(format!("Error sending pending data to the decoder: {}", e)));
            }
            _ => (),
        }

        // Handle all pending pictures before sending the next data.
        handle_pending_pictures(&mut dec, false)?;

    Ok(())
}