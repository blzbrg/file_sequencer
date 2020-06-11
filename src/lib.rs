extern crate serde;

pub mod fs;
pub mod sequence;
pub mod validate;

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

pub fn load(sequences_path : &::std::path::Path) -> Result<sequence::SequencesFile, Error> {
    let file : std::fs::File = ::std::fs::File::open(sequences_path)?;
    serde_json::from_reader(file)
        .map_err(Error::from)
}

pub fn save(sequences_path : &::std::path::Path, sequences : sequence::SequencesFile)
            -> Result<(), Error> {
    let file = ::std::fs::OpenOptions::new().write(true).create(true)
            .open(sequences_path)?;
        serde_json::to_writer_pretty(file, &sequences)
            .map_err(Error::from)
}

/// Convert an `OsString` into a native (owned) `String`.
///
/// Danger: unknown performance implications.
pub fn import_os_string(s : std::ffi::OsString) -> Result<String, Error> {
    s.into_string().map_err(|_| Error::FilenameUnicodeError)
}