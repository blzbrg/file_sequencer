extern crate file_sequencer;

use std::result::Result;

fn print_seq(seq : &file_sequencer::sequence::Sequence) -> () {
    for filename in &seq.files {
        println!("{}", filename);
    }
}

struct Args {
    pub sequences_path : std::path::PathBuf
}

impl Args {
    pub fn parse_args(mut args_it : std::env::Args) -> Args {
        let _ = args_it.next(); // skip first arg, it's our exec name instead of a real arg
        let first_arg : std::string::String = args_it.next()
            .expect("Needs one positional argument: the path of the sequences file.");
        // copy from args into PathBuf
        let path : Result<std::path::PathBuf, std::convert::Infallible> =
            <std::path::PathBuf as std::str::FromStr>::from_str(first_arg.as_str());
        Args{sequences_path : path.unwrap()}
    }
}

fn main() {
    let args : Args = Args::parse_args(std::env::args());
    let dir = args.sequences_path.parent()
        .expect("Sequences file was somehow not inside a directory?");

    // Load the seq and prepare to apply it
    let seq : file_sequencer::sequence::Sequences = file_sequencer::load(args.sequences_path.as_path())
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