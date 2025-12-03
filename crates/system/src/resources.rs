use std::{collections::HashMap, path::PathBuf, sync::LazyLock};

use crate::{downloads::RESOURCES_DIRECTORY, platform::Platform};

type ResourceMap = HashMap<Resource, PathBuf>;
type PlatformMap = HashMap<Platform, ResourceMap>;

#[derive(Hash, PartialEq, Eq)]
pub enum Resource {
    ElfBinary,
    DefaultIcon,
    RomFS,
}

fn make_resources(platform: &Platform) -> ResourceMap {
    let base_dir = PathBuf::from(RESOURCES_DIRECTORY).join(platform.to_string());

    ResourceMap::from([
        (Resource::ElfBinary, base_dir.join("lovepotion.elf")),
        (Resource::RomFS, base_dir.join("files.romfs")),
    ])
}

static RESOURCES: LazyLock<PlatformMap> = LazyLock::new(|| {
    let mut result = PlatformMap::new();
    for platform in Platform::ALL {
        result.insert(platform.clone(), make_resources(&platform));
    }
    result
});

pub fn fetch_icon() -> PathBuf {
    let base_dir = PathBuf::from(RESOURCES_DIRECTORY);
    base_dir.join("default.png")
}

pub fn fetch(platform: &Platform, resource: Resource) -> PathBuf {
    if let Some(path) = RESOURCES.get(platform).and_then(|map| map.get(&resource)) {
        return path.to_owned();
    }
    PathBuf::new()
}
