use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

use std::fs::File;
use std::path::Path;

use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

#[no_mangle]
pub extern "C" fn get_sample(path: *const c_char) -> *const c_char {
    let cstr = unsafe { CStr::from_ptr(path) };
    let rstr = cstr.to_str().expect("Failed to cast of CStr to &str");

    // Create a media source. Note that the MediaSource trait is automatically implemented for File,
    // among other types.
    let file = Box::new(File::open(Path::new(rstr)).unwrap());

    // Create the media source stream using the boxed media source from above.
    let mss = MediaSourceStream::new(file, Default::default());

    // Create a hint to help the format registry guess what format reader is appropriate. In this
    // example we'll leave it empty.
    let hint = Hint::new();

    // Use the default options when reading and decoding.
    let format_opts: FormatOptions = Default::default();
    let metadata_opts: MetadataOptions = Default::default();
    let decoder_opts: DecoderOptions = Default::default();

    // Probe the media source stream for a format.
    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &format_opts, &metadata_opts)
        .unwrap();

    // Get the format reader yielded by the probe operation.
    let mut format = probed.format;

    // Get the default track.
    let track = format.default_track().unwrap();

    // Create a decoder for the track.
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &decoder_opts)
        .unwrap();

    // Store the track identifier, we'll use it to filter packets.
    let track_id = track.id;

    let mut samples: Vec<f32> = Vec::new();
    let mut sample_buf = None;

    loop {
        match format.next_packet() {
            Ok(_) => (),
            Err(_) => break,
        }

        // Get the next packet from the format reader.
        let packet = format.next_packet().unwrap();

        // If the packet does not belong to the selected track, skip it.
        if packet.track_id() != track_id {
            continue;
        }

        // Decode the packet into audio samples, ignoring any decode errors.
        match decoder.decode(&packet) {
            Ok(audio_buf) => {
                // The decoded audio samples may now be accessed via the audio buffer if per-channel
                // slices of samples in their native decoded format is desired. Use-cases where
                // the samples need to be accessed in an interleaved order or converted into
                // another sample format, or a byte buffer is required, are covered by copying the
                // audio buffer into a sample buffer or raw sample buffer, respectively. In the
                // example below, we will copy the audio buffer into a sample buffer in an
                // interleaved order while also converting to a f32 sample format.

                // If this is the *first* decoded packet, create a sample buffer matching the
                // decoded audio buffer format.
                if sample_buf.is_none() {
                    // Get the audio buffer specification.
                    let spec = *audio_buf.spec();

                    // Get the capacity of the decoded buffer. Note: This is capacity, not length!
                    let duration = audio_buf.capacity() as u64;

                    // Create the f32 sample buffer.
                    sample_buf = Some(SampleBuffer::<f32>::new(duration, spec));
                }

                // Copy the decoded audio buffer into the sample buffer in an interleaved format.
                if let Some(buf) = &mut sample_buf {
                    buf.copy_interleaved_ref(audio_buf);

                    // The samples may now be access via the `samples()` function.
                    for value in buf.samples() {
                        samples.push(*value);
                    }
                }
            }
            Err(Error::DecodeError(_)) => (),
            Err(_) => break,
        }
    }

    let rstring = format!("{:?}", samples);
    let cstring = CString::new(rstring).expect("CString::new failed");
    return cstring.into_raw();
}

#[cfg(target_os = "android")]
pub mod android {
    extern crate jni;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::jstring;
    use self::jni::JNIEnv;
    use std::os::raw::c_char;
    use super::*;

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_totechite_rustpractice_RustSamples_sample(
        env: JNIEnv,
        _: JClass,
        input: JString,
    ) -> jstring {
        let input: String = env
            .get_string(input)
            .expect("invalid pattern string")
            .into();
        let output = env
            .new_string(format!("Android meets {}", input))
            .expect("Couldn't create java string!");
        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_totechite_rustpractice_RustSamples_get_sample(
        _env: JNIEnv,
        _: JClass,
        java_path: jstring
    ) -> jstring {
        let samples = get_sample(java_path as *const c_char);
        samples as jstring
    }
}
