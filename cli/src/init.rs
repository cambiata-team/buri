use protobuf::text_format::print_to_string_pretty;
use protos::workspace::WorkspaceFile;
use vfs::{VfsError, VfsPath};
use virtual_io::VirtualIo;

pub fn do_init(
    root: &VfsPath,
    vio: &mut impl VirtualIo,
    name: &Option<String>,
) -> Result<(), VfsError> {
    let workspace_file = root.join("WORKSPACE")?;
    if workspace_file.exists()? {
        vio.println("Workspace already exists, no need to create a new one.");
        return Ok(());
    }
    let mut workspace = WorkspaceFile::new();
    workspace.buriVersion = "nightly".to_string();
    if let Some(name) = name {
        workspace.name = name.to_string();
    }
    let formatted_workspace_file = print_to_string_pretty(&workspace);
    root.join("WORKSPACE")?
        .create_file()?
        .write_all(formatted_workspace_file.as_bytes())?;
    vio.println(format!(
        "Created new workspace at .{}",
        workspace_file.as_str()
    ));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use protobuf::text_format::parse_from_str;
    use vfs::MemoryFS;

    #[test]
    fn creates_workspace_file() {
        let root: VfsPath = MemoryFS::new().into();
        let mut vio = virtual_io::VioFakeBuilder::new().build();
        do_init(&root, &mut vio, &None).unwrap();
        let workspace_file = root.join("WORKSPACE").unwrap();
        assert!(workspace_file.exists().unwrap());
    }

    #[test]
    fn does_not_overwrite_existing_workspace_file() {
        let root: VfsPath = MemoryFS::new().into();
        root.join("WORKSPACE")
            .unwrap()
            .create_file()
            .unwrap()
            .write_all(b"foo")
            .unwrap();
        let mut vio = virtual_io::VioFakeBuilder::new().build();
        do_init(&root, &mut vio, &None).unwrap();
        let workspace_file = root.join("WORKSPACE").unwrap();
        assert_eq!(workspace_file.read_to_string().unwrap(), "foo");
    }

    #[test]
    fn creates_workspace_file_with_name() {
        let root: VfsPath = MemoryFS::new().into();
        let mut vio = virtual_io::VioFakeBuilder::new().build();
        do_init(&root, &mut vio, &Some("foo".to_string())).unwrap();
        let workspace_file = root.join("WORKSPACE").unwrap();
        let workspace: WorkspaceFile =
            parse_from_str::<WorkspaceFile>(&workspace_file.read_to_string().unwrap()).unwrap();
        assert_eq!(workspace.name, "foo");
    }

    #[test]
    fn creates_workspace_file_with_nightly_version() {
        let root: VfsPath = MemoryFS::new().into();
        let mut vio = virtual_io::VioFakeBuilder::new().build();
        do_init(&root, &mut vio, &Some("foo".to_string())).unwrap();
        let workspace_file = root.join("WORKSPACE").unwrap();
        let workspace: WorkspaceFile =
            parse_from_str::<WorkspaceFile>(&workspace_file.read_to_string().unwrap()).unwrap();
        assert_eq!(workspace.buriVersion, "nightly");
    }

    #[test]
    fn prints_that_workspace_file_was_created() {
        let root: VfsPath = MemoryFS::new().into();
        let mut vio = virtual_io::VioFakeBuilder::new()
            .expect_stdout("Created new workspace at ./WORKSPACE\n")
            .build();
        do_init(&root, &mut vio, &None).unwrap();
        assert_eq!(vio.get_actual(), vio.get_expected());
    }

    #[test]
    fn shows_error_if_workspace_file_exists() {
        let root: VfsPath = MemoryFS::new().into();
        root.join("WORKSPACE")
            .unwrap()
            .create_file()
            .unwrap()
            .write_all(b"foo")
            .unwrap();
        let mut vio = virtual_io::VioFakeBuilder::new()
            .expect_stdout("Workspace already exists, no need to create a new one.\n")
            .build();
        do_init(&root, &mut vio, &None).unwrap();
        assert_eq!(vio.get_actual(), vio.get_expected());
    }
}
