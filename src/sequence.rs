/// How the sequenece should be placed into the "natural" order of files.
///
/// For `FirstFile` or `LastFile` the sequence is placed where the first or last file, respectively,
/// would be placed in the "natural" order.
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Attachment {
    FirstFile,
    LastFile
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Sequence {
    files : ::std::vec::Vec<::std::path::PathBuf>, // TODO: efficiency?
    attachment : ::std::option::Option<Attachment>
}