#![feature(bind_by_move_pattern_guards)]

extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;

mod logger;
mod util;

use chrono::Local;
use crypto::aead::AeadEncryptor;
use crypto::aes::KeySize;
use crypto::aes_gcm::AesGcm;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use logger::Logger;
use psimple::Simple;
use pulse::sample;
use pulse::stream::Direction;
use std::fs;
use std::io::ErrorKind;
use std::thread;
use util::generate_key;

// The duration of the individual audio clips in seconds
// Increase this if you're willing to pay the price in RAM
const INTERVAL: f64 = 60.0;

fn main() {
    let mut logger = Logger::new();
    match fs::create_dir("/home/linde_5/recordings") {
        Ok(_) => (),
        Err(e) if e.kind() == ErrorKind::AlreadyExists => (),
        Err(e) if e.kind() == ErrorKind::PermissionDenied => {
            logger.error("Permission denied when creating directory 'recordings'");
            return;
        }
        Err(e) => {
            logger.error(format!("Unrecognized error: {}", e));
            return;
        }
    }

    let spec = sample::Spec {
        format: sample::SAMPLE_S24NE,
        channels: 2,
        rate: 44100,
    };

    assert!(spec.is_valid());

    let simple = Simple::new(
        None,
        "Wyrtap",
        Direction::Record,
        None,
        "Recording",
        &spec,
        None,
        None,
    )
    .expect("Failed to connect to audio server");

    loop {
        logger.started();
        let start = Local::now();
        let mut audio: Vec<u8> = vec![0; (((2.12 * INTERVAL) / 8.0) * 1000000.0) as usize];
        let mut buffer = audio.as_mut_slice();

        simple
            .read(&mut buffer)
            .expect("Failed to read from audio server");

        logger.stopped();

        thread::spawn(move || {
            let date = start.format("%Y-%m-%d").to_string();
            let time = start.format("%H:%M:%S").to_string();

            match fs::create_dir(format!("/home/linde_5/recordings/{}", &date)) {
                Ok(_) => (),                                          // Cool
                Err(e) if e.kind() == ErrorKind::AlreadyExists => (), // Cool
                Err(e) if e.kind() == ErrorKind::PermissionDenied => {
                    panic!("Permission denied while writing audio to disk!")
                }
                Err(e) => panic!("Error writing audio to disk: {}", e),
            };

            // Compress with LZMA
            let compressed: Vec<u8> = lzma::compress(audio.as_slice(), 6).expect("LZMA error");

            // Encrypt tarball
            let (public_key, shared_secret) = generate_key();
            let mut hasher = Sha3::sha3_256();
            hasher.input(shared_secret.as_slice());
            let mut key = [0u8; 32];
            hasher.result(&mut key);
            let iv: [u8; 12] = rand::random();
            let mut aes_gcm = AesGcm::new(KeySize::KeySize256, &key, &iv, Vec::new().as_slice());
            let mut ciphertext = vec![0u8; compressed.len()];
            let mut tag = [0u8; 16];
            aes_gcm.encrypt(compressed.as_slice(), ciphertext.as_mut_slice(), &mut tag);

            // Save encrypted audio and public key to disk
            let audio_path = format!("/home/linde_5/recordings/{}/{}.xz.wyr", &date, &time);
            let wyr = [ciphertext.as_slice(), public_key.as_slice(), &iv, &tag].concat();
            fs::write(audio_path, &wyr).expect("Failed to write audio to disk");
        });
    }
}
