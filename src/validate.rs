use super::sequence::Sequence;

type AttMap<'a> = ::std::collections::hash_map::HashMap<&'a str, Vec<&'a Sequence>>;

pub enum InvalidReason {
    AttachmentPointMissing,
    HasNoFiles,
}

pub struct Invalid<'a> {
    pub seq : &'a Sequence,
    pub reason : InvalidReason,
}

impl<'a> Invalid<'a> {
    pub fn due_to(reason : InvalidReason, seq : &'a Sequence) -> Self {
        Self{seq : seq, reason : reason}
    }
}
impl<'a> Invalid<'a> {
    pub fn explain(self : &Self) -> String {
        match self.reason {
            InvalidReason::AttachmentPointMissing => {
                // TODO: avoid unwrap?
                format!("attachment point {} not found in directory", self.seq.attachment_point().unwrap())
            }
            InvalidReason::HasNoFiles => {String::from("it has no files in it!")}
        }
    }
}

pub fn show_sequence(seq : &Sequence) -> String {
    serde_json::to_string_pretty(seq)
        .unwrap_or_else(|err| {format!("<Cannot show seq due to {:?}>", err)})
}


/// Create map from attachment point to the sequences using that attachment point.
///
/// Ignores sequences with no attachment point.
pub fn attachment_point_map<'a, 'b>(seqs : &'b [&'a Sequence]) -> AttMap<'a> {
    let mut ret : AttMap<'a> = std::collections::HashMap::new();

    for seq in seqs {
        match seq.attachment_point() {
            None => {()}
            Some(attach_point) => {
                match ret.get_mut(attach_point) {
                    Some(v) => {v.push(seq);}
                    None    => {let mut v : Vec<&'a Sequence> = Vec::new();
                                v.push(seq);
                                ret.insert(attach_point, v);}
                };
            }
        };
    };

    ret
}

/// Figure out if the attachment point is in the set of filenames. Also produces `HasNoFiles` for
/// any sequences with no files (and thus no attachment point).
pub fn attachment_point_in_dir<'a>(
    filenames : &std::collections::HashSet<String>,
    seq : &'a Sequence)
    -> Option<Invalid<'a>> {
    match seq.attachment_point() {
        None => {Some(Invalid::due_to(InvalidReason::HasNoFiles, seq))}
        Some(attach_point) => {
            if filenames.contains(attach_point) {
                None
            } else {
                Some(Invalid::due_to(InvalidReason::AttachmentPointMissing, seq))
            }
        }
    }
}