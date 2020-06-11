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
    let seq_file : file_sequencer::sequence::SequencesFile =
        file_sequencer::load(args.sequences_path.as_path())
        .expect("Could not load sequences file");

    // Set up for validation
    let dir_entries : std::collections::HashSet<String> = file_sequencer::fs::list(dir)
        .expect("Could not list items in director");

    // Validate them
    let filter_and_warn = |s : &&file_sequencer::sequence::Sequence| {
        // TODO why the fuck does this need to be double-referenced, and why does it not type-infer
        // that?
        match file_sequencer::validate::attachment_point_in_dir(&dir_entries, s) {
            None => {true}
            Some(invalid) => {eprintln!("Ignoring {} because {}",
                                        file_sequencer::validate::show_sequence(s),
                                        invalid.explain());
                              false}
        }
    };
    let valid_seqs : Vec<&file_sequencer::sequence::Sequence> =
        seq_file.sequences.iter().filter(filter_and_warn).collect();

    // Construct att map
    let att_map : std::collections::HashMap<&str, Vec<&file_sequencer::sequence::Sequence>>
        = file_sequencer::validate::attachment_point_map(valid_seqs.as_ref());

    // List the directory
    for entry in dir_entries {
        match att_map.get(entry.as_str()) {
            // Arbitrarily choose the first when there are duplicates
            Some(seqs) => {print_seq(seqs[0])}
            None       => {println!("{}", entry)}
        }
    }
}