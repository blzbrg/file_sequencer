use super::sequence::Sequence;

type AttMap<'a> = ::std::collections::hash_map::HashMap<&'a str, Vec<&'a Sequence>>;

pub enum InvalidReason {
    AttachmentPointMissing,
    HasNoFiles,
    /// This attachment point is given by more than one sequence.
    DuplicateAttachmentPoint,
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

fn show_sequence(seq : &Sequence) -> String {
    serde_json::to_string_pretty(seq)
        .unwrap_or_else(|err| {format!("<Cannot show seq due to {:?}>", err)})
}

pub fn explain_invalid(invalid : &Invalid, att_map : &AttMap) -> String {
    match invalid.reason {
        InvalidReason::AttachmentPointMissing => {
            format!("Attachment point {} not found in directoryfor sequence {}",
                    invalid.seq.attachment_point().unwrap(), show_sequence(invalid.seq))} // TODO: avoid unwrap?
        InvalidReason::HasNoFiles => {
            match invalid.seq.attachment {
                Some(at) => {format!("A sequence with attachment {} has no files in it!", at)}
                None     => {"A sequence with no attachment listed has no
                             files in it (is it just an emtpy {}???)".to_string()}
            }
        }
        InvalidReason::DuplicateAttachmentPoint => { // TODO: make this useful
            format!("Multiple sequences share the attachment point {}", invalid.seq.attachment_point().unwrap())
        }
    }
}

pub fn attachment_point_map<'a>(seqs : &'a [Sequence]) -> AttMap<'a> {
    let mut ret : AttMap<'a> = std::collections::HashMap::new();

    for seq in seqs {
        match seq.attachment_point() {
            None => {()}
            Some(attach_point) => {
                match ret.get_mut(attach_point) {
                    Some(v) => {v.push(seq);}
                    None    => {let mut v = Vec::new();
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
fn attachment_point_in_dir<'a>(
    filenames : &std::collections::HashSet<&str>,
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