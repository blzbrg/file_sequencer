extern crate file_sequencer;

use std::result::Result;
use std::option::Option;

fn print_seq(seq : &file_sequencer::sequence::Sequence) -> () {
    for filename in &seq.files {
        println!("{}", filename);
    }
}

fn main() {
    let sequences_path = std::path::Path::new("./fsequencer.txt"); // TODO
    let dir = sequences_path.parent()
        .expect("Sequences file was somehow not inside a directory?");

    // Load the seq and prepare to apply it
    let seq : file_sequencer::sequence::Sequences = file_sequencer::load(sequences_path)
        .expect("Could not load sequences file");
    let att_map : std::collections::hash_map::HashMap<&str, &file_sequencer::sequence::Sequence>
        = file_sequencer::create_attachment_point_map(&seq);

    // List the directory
    for maybe_entry in dir.read_dir().expect("Could not list items in directory") {
        let entry : std::result::Result<std::fs::DirEntry, file_sequencer::Error>
            = maybe_entry.map_err(file_sequencer::Error::from);
        // TODO: avoid this expect
        let ffi_name : std::ffi::OsString
            = entry.expect("Could not read filename in directory").file_name();
        match file_sequencer::lookup_by_os_str(&ffi_name, &att_map) {
            Result::Ok(Option::Some(seq)) => {print_seq(seq)}
            Result::Ok(Option::None) => {println!("{}", ffi_name.to_str().unwrap())}
            Result::Err(file_sequencer::Error::FilenameUnicodeError) => {
                eprintln!("Warning: could not convert {} to unicode, ignoring",
                          ffi_name.to_string_lossy());
            }
            Result::Err(e) => {panic!(e)}
        }
    }
}