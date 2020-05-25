extern crate file_sequencer;

use std::result::Result;

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
        match file_sequencer::entry_to_name_or_seq(maybe_entry, &att_map) {
            Result::Ok(file_sequencer::NameOrSeq::Name(name)) => {println!("{}", name)}
            Result::Ok(file_sequencer::NameOrSeq::Seq(seq)) => {print_seq(seq)}
            Result::Err(e) => {eprintln!("{:?}", e)}
        }
    }
}