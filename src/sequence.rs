//! The API for sequences. This library contains the data-model for the file format and methods for
//! asking questions directly of the data. Complicated manipulations
//! (eg. `create_attachment_point_map`) are deliberately elsewhere to keep this file focused.

/// How the sequenece should be placed into the "natural" order of files.
///
/// For `FirstFile` or `LastFile` the sequence is placed where the first or last file, respectively,
/// would be placed in the "natural" order.
///
/// The attachment property determines which member of the sequence is the attachment point.
#[derive(Copy, Clone, serde::Serialize, serde::Deserialize)]
pub enum Attachment {
    FirstFile,
    LastFile
}

impl core::fmt::Display for Attachment {
    fn fmt(&self, f : &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(match self {
            Attachment::FirstFile => {"FirstFile"}
            Attachment::LastFile  => {"LastFile"}
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Sequence {
    pub files : ::std::vec::Vec<::std::string::String>, // TODO: efficiency?
    pub attachment : ::std::option::Option<Attachment>
}

impl Sequence {
    /// The effective attachment point of a sequence. This is the value loaded from the description
    /// text, or if absent, it is the default.
    pub fn effective_attachment(self : &Self) -> Attachment {
        match self.attachment {
            None    => {Attachment::FirstFile}
            Some(a) => {a}
        }
    }

    /// Return the filename at which this sequence is attached. This is one of the filenames in the
    /// sequence; which one is determined by the `effective_attachment`.
    ///
    /// This returns None iff there are no files listed in the sequence. It is desirable to parse
    /// this sequence to a) be forgiving of user mistakes and b) handle partially-created or
    /// partially-delteted sequences gracefully/
    pub fn attachment_point<'a>(self : &'a Self) -> Option<&'a str> {
        if self.files.is_empty() {
            Option::None
        } else {
            let filename : &String = match self.effective_attachment() {
                Attachment::FirstFile => {&self.files[0]}
                Attachment::LastFile  => {&self.files[self.files.len() - 1]}
            };
            Option::Some(filename.as_str())
        }
    }
}

/// Collection of `Sequence`s. This is what serde will return when deserializing a json list.
pub type Sequences = ::std::vec::Vec<Sequence>;

/// Type representing the entire file.
///
/// If possible, fields should only be changed in this in a backwards compatible way:
/// - newly added fields should be optional (with defaults)
/// - instead of removing a mandatory field, change it to optional
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SequencesFile {
    pub sequences : Sequences,
}