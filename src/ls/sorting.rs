pub fn sort_my_files(by_size: bool, by_name: bool, files: &mut Vec<std::fs::DirEntry>) {
    if by_size {
        files.sort_by_key(|entry| entry.metadata().unwrap().len());
    } else if by_name {
        files.sort_by_key(|entry| entry.file_name());
    }
}
