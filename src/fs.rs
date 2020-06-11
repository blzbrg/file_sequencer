pub fn list(path : &std::path::Path) -> Result<std::collections::HashSet<String>, super::Error> {
    let dir_iter = path.read_dir().map_err(super::Error::FileError)?;
    let mut names : std::collections::HashSet<String> = std::collections::HashSet::new();
    for maybe_entry in dir_iter {
        let entry : std::fs::DirEntry = maybe_entry?; // end early if we cannot see this entry
        // end early is cannot convert to unicode. TODO: do we want to skip it?
        let name : String = super::import_os_string(entry.file_name())?;
        names.insert(name);
    }
    Ok(names)
}