/* Downloads the sound files at compile time */

extern crate reqwest;
extern crate rayon;

use rayon::iter::ParallelBridge;
use rayon::prelude::*;

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::io;
use std::fs;

fn download_file(linkr: io::Result<String>) {
    let link = linkr.expect("failed to read from piano_sound_links");
    let mut resp = reqwest::get(&link).expect("request failed");

    let filename = format!("sounds/{}", link.clone().split_off(link.len() - 7));
    let mut file = File::create(filename).expect("file creation failed");

    io::copy(&mut resp, &mut file).expect("failed to copy to file");
}

fn get_sound_files() {
    let handle = File::open("piano_sound_links")
        .expect("unable to find piano_sound_links");

    // Concurrently iterate over all of the links in piano_sound_links
    // and download them
    BufReader::new(handle).lines()
        .par_bridge()
        .map(download_file)
        .for_each(drop);
}

fn main() {
    // If the "sounds" directory does not exist, then create one
    // and download all of the necessary sounds into it
    if !fs::metadata("sounds").is_ok() {
        fs::create_dir("sounds").expect("could not create sounds directory");
        get_sound_files();
    }
}