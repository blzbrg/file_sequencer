extern crate serde;

pub mod sequence;

use std::result::Result;
use std::option::Option;

#[derive(Debug)]
pub enum Error {
    SerdeError(serde_json::error::Error),
    FileError(::std::io::Error),
    FilenameUnicodeError,
    DirectoryReadError(std::io::Error),
}

impl From<::std::io::Error> for Error {
    fn from(e : ::std::io::Error) -> Self {
        Self::FileError(e)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(e : serde_json::error::Error) -> Self {
        Self::SerdeError(e)
    }
}

pub fn load(sequences_path : &::std::path::Path) -> ::std::result::Result<sequence::Sequences, Error> {
    let file : std::fs::File = ::std::fs::File::open(sequences_path)?;
    serde_json::from_reader(file)
        .map_err(Error::from)
}

pub fn save(sequences_path : &::std::path::Path, sequences : sequence::Sequences)
            -> ::std::result::Result<(), Error> {
    let file = ::std::fs::OpenOptions::new().write(true).create(true)
            .open(sequences_path)?;
        serde_json::to_writer(file, &sequences)
            .map_err(Error::from)
}

/// Create a map from filename to the sequence attached at that name.
///
/// Each key is a sequence attachment point and each value is a sequence. This will arbitrarily
/// select one sequence to discard if two sequences have the same attachment point.
pub fn create_attachment_point_map(sequences : &sequence::Sequences)
                                   -> std::collections::hash_map::HashMap<&str, &sequence::Sequence> {
    let mut ret = std::collections::hash_map::HashMap::new();
    for seq in sequences {
        ret.insert(seq.attachment_point(), seq);
    }
    ret
}

/// Convert an `OsString` into a native (owned) `String`.
///
/// Danger: unknown performance implications.
pub fn import_os_string(s : std::ffi::OsString) -> Result<String, Error> {
    s.into_string().map_err(|_| Error::FilenameUnicodeError)
}

pub enum NameOrSeq<'a> {
    Name(String),
    Seq(&'a sequence::Sequence)
}

/// Process an actual filename from the directory listing, either passing it through unchanged, or
/// fetching the appropriate `Sequence` to replace it.
///
/// The sequences are reference into the map, whereas the strings are returned by value, since
/// `DirEntry` only gives back filenames by-value.
pub fn entry_to_name_or_seq<'a, 'b>(
    maybe_entry : std::io::Result<std::fs::DirEntry>,
    att_map : &std::collections::hash_map::HashMap<&str, &'a sequence::Sequence>)
    -> Result<NameOrSeq<'a>, Error> {
    let entry : std::fs::DirEntry = maybe_entry?;
    let ffi_name : std::ffi::OsString = entry.file_name();
    let name : String = import_os_string(ffi_name)?;
    match att_map.get(name.as_str()) {
        Option::Some(seq) => {Result::Ok(NameOrSeq::Seq(seq))}
        Option::None      => {Result::Ok(NameOrSeq::Name(name))}
    }
}