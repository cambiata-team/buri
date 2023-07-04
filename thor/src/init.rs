use files::{
    cli_config::{CliConfig, CLI_CONFIG_FILE_NAME},
    workspace_file::{WorkspaceFile, WORKSPACE_FILE_NAME},
};
use vfs::{VfsError, VfsPath};
use virtual_io::VirtualIo;

pub fn do_init(
    root: &VfsPath,
    vio: &mut impl VirtualIo,
    name: &Option<String>,
) -> Result<(), VfsError> {
    create_workspace_file(root, vio, name)?;
    create_version_file(root, vio)?;
    Ok(())
}

fn create_workspace_file(
    root: &VfsPath,
    vio: &mut impl VirtualIo,
    name: &Option<String>,
) -> Result<(), VfsError> {
    let workspace_file = root.join(WORKSPACE_FILE_NAME)?;
    if workspace_file.exists()? {
        vio.println("Workspace already exists, no need to create a new one.");
        return Ok(());
    }
    let mut workspace = WorkspaceFile::new();
    if let Some(name) = name {
        workspace.name = Some(name.to_string());
    }
    let formatted_workspace_file = toml::to_string_pretty(&workspace).unwrap();
    root.join(WORKSPACE_FILE_NAME)?
        .create_file()?
        .write_all(formatted_workspace_file.as_bytes())?;
    vio.println(format!("Created .{}", workspace_file.as_str()));
    Ok(())
}

fn create_version_file(root: &VfsPath, vio: &mut impl VirtualIo) -> Result<(), VfsError> {
    let version_file = root.join(CLI_CONFIG_FILE_NAME)?;
    if version_file.exists()? {
        vio.println("Version file already exists, no need to create a new one.");
        return Ok(());
    }
    let version = CliConfig::from("buri_version=\"0.3.0\"").unwrap();
    let formatted_version_file = version.to_string();

    root.join(CLI_CONFIG_FILE_NAME)?
        .create_file()?
        .write_all(formatted_version_file.as_bytes())?;
    vio.println(format!("Created .{}", version_file.as_str()));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use files::workspace_file::WORKSPACE_FILE_NAME;
    use vfs::MemoryFS;

    #[test]
    fn creates_workspace_file() {
        let root: VfsPath = MemoryFS::new().into();
        let mut vio = virtual_io::VioFakeBuilder::new().build();
        do_init(&root, &mut vio, &None).unwrap();
        let workspace_file = root.join(WORKSPACE_FILE_NAME).unwrap();
        assert!(workspace_file.exists().unwrap());
    }

    #[test]
    fn does_not_overwrite_existing_workspace_file() {
        let root: VfsPath = MemoryFS::new().into();
        root.join(WORKSPACE_FILE_NAME)
            .unwrap()
            .create_file()
            .unwrap()
            .write_all(b"foo")
            .unwrap();
        let mut vio = virtual_io::VioFakeBuilder::new().build();
        create_workspace_file(&root, &mut vio, &None).unwrap();
        let workspace_file = root.join(WORKSPACE_FILE_NAME).unwrap();
        assert_eq!(workspace_file.read_to_string().unwrap(), "foo");
    }

    #[test]
    fn creates_workspace_file_with_name() {
        let root: VfsPath = MemoryFS::new().into();
        let mut vio = virtual_io::VioFakeBuilder::new().build();
        create_workspace_file(&root, &mut vio, &Some("foo".to_string())).unwrap();
        let workspace_file = root.join(WORKSPACE_FILE_NAME).unwrap();
        let workspace =
            toml::from_str::<WorkspaceFile>(&workspace_file.read_to_string().unwrap()).unwrap();
        assert_eq!(workspace.name, Some(String::from("foo")));
    }

    #[test]
    fn prints_that_workspace_file_was_created() {
        let root: VfsPath = MemoryFS::new().into();
        let mut vio = virtual_io::VioFakeBuilder::new()
            .expect_stdout("Created ./WORKSPACE.toml\n")
            .build();
        create_workspace_file(&root, &mut vio, &None).unwrap();
        assert_eq!(vio.get_actual(), vio.get_expected());
    }

    #[test]
    fn shows_error_if_workspace_file_exists() {
        let root: VfsPath = MemoryFS::new().into();
        root.join(WORKSPACE_FILE_NAME)
            .unwrap()
            .create_file()
            .unwrap()
            .write_all(b"foo")
            .unwrap();
        let mut vio = virtual_io::VioFakeBuilder::new()
            .expect_stdout("Workspace already exists, no need to create a new one.\n")
            .build();
        create_workspace_file(&root, &mut vio, &None).unwrap();
        assert_eq!(vio.get_actual(), vio.get_expected());
    }

    #[test]
    fn creates_version_file() {
        let root: VfsPath = MemoryFS::new().into();
        let mut vio = virtual_io::VioFakeBuilder::new().build();
        do_init(&root, &mut vio, &None).unwrap();
        let workspace_file = root.join(CLI_CONFIG_FILE_NAME).unwrap();
        assert!(workspace_file.exists().unwrap());
    }

    #[test]
    fn does_not_overwrite_existing_version_file() {
        let root: VfsPath = MemoryFS::new().into();
        root.join(CLI_CONFIG_FILE_NAME)
            .unwrap()
            .create_file()
            .unwrap()
            .write_all(b"foo")
            .unwrap();
        let mut vio = virtual_io::VioFakeBuilder::new().build();
        create_version_file(&root, &mut vio).unwrap();
        let version_file = root.join(CLI_CONFIG_FILE_NAME).unwrap();
        assert_eq!(version_file.read_to_string().unwrap(), "foo");
    }

    #[test]
    fn prints_that_version_file_was_created() {
        let root: VfsPath = MemoryFS::new().into();
        let mut vio = virtual_io::VioFakeBuilder::new()
            .expect_stdout("Created ./.burirc.toml\n")
            .build();
        create_version_file(&root, &mut vio).unwrap();
        assert_eq!(vio.get_actual(), vio.get_expected());
    }

    #[test]
    fn shows_error_if_version_file_exists() {
        let root: VfsPath = MemoryFS::new().into();
        root.join(CLI_CONFIG_FILE_NAME)
            .unwrap()
            .create_file()
            .unwrap()
            .write_all(b"foo")
            .unwrap();
        let mut vio = virtual_io::VioFakeBuilder::new()
            .expect_stdout("Version file already exists, no need to create a new one.\n")
            .build();
        create_version_file(&root, &mut vio).unwrap();
        assert_eq!(vio.get_actual(), vio.get_expected());
    }
}
