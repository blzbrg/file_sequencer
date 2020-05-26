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
    pub fn attachment_point<'a>(self : &'a Self) -> &'a str {
        let filename : &String = match self.effective_attachment() {
            Attachment::FirstFile => {&self.files[0]}
            Attachment::LastFile  => {&self.files[self.files.len() - 1]}
        };
        filename.as_str()
    }
}

/// Collection of `Sequence`s. This is what serde will return when deserializing a json list.
pub type Sequences = ::std::vec::Vec<Sequence>;