use dirs::{cache_dir, config_dir};
use vfs::{PhysicalFS, VfsPath};

pub struct Context {
    pub root: VfsPath,
    pub args: Vec<String>,
    pub cache_dir: VfsPath,
    pub config_dir: VfsPath,
}

impl Context {
    pub fn new(args: Vec<String>) -> Self {
        let user_cache_dir: VfsPath = PhysicalFS::new(cache_dir().unwrap()).into();
        let buri_cache_dir = user_cache_dir.join("buri").unwrap();
        buri_cache_dir.create_dir_all().unwrap();

        let user_config_dir: VfsPath = PhysicalFS::new(config_dir().unwrap()).into();
        let buri_config_dir = user_config_dir.join("buri").unwrap();
        buri_config_dir.create_dir_all().unwrap();
        Self {
            root: PhysicalFS::new(std::env::current_dir().unwrap()).into(),
            args,
            cache_dir: buri_cache_dir,
            config_dir: buri_config_dir,
        }
    }

    #[cfg(test)]
    pub fn test() -> Self {
        Self {
            root: vfs::MemoryFS::new().into(),
            args: vec![],
            cache_dir: vfs::MemoryFS::new().into(),
            config_dir: vfs::MemoryFS::new().into(),
        }
    }
}
