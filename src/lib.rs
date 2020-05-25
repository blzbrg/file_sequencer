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

pub fn create_attachment_point_map(sequences : &sequence::Sequences)
                                   -> std::collections::hash_map::HashMap<&str, &sequence::Sequence> {
    let mut ret = std::collections::hash_map::HashMap::new();
    for seq in sequences {
        let attach_point : &str = match seq.effective_attachment() {
            sequence::Attachment::FirstFile => {seq.files[0].as_str()}
            sequence::Attachment::LastFile  => {seq.files[seq.files.len() - 1].as_str()}
        };
        ret.insert(attach_point, seq);
    }
    ret
}

pub fn lookup_by_os_str<'a, 'b>(
    os_str : &std::ffi::OsStr,
    map : &'b std::collections::hash_map::HashMap<&str, &'a sequence::Sequence>)
    -> Result<Option<&'b &'a sequence::Sequence>, Error> {
    match os_str.to_str() {
        Some(s) => {Result::Ok(map.get(s))}
        None    => {Result::Err(Error::FilenameUnicodeError)}
    }
}

