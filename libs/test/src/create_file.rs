use vfs::VfsPath;

pub fn create_test_file(root: &VfsPath, file_name: &str, file_contents: &[u8]) {
    let mut segments = file_name.split('/').collect::<Vec<&str>>();
    segments.pop();
    root.join(segments.join("/"))
        .unwrap()
        .create_dir_all()
        .unwrap();
    let _ = root
        .join(file_name)
        .unwrap()
        .create_file()
        .unwrap()
        .write(file_contents)
        .unwrap();
}
