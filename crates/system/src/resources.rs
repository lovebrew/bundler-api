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
    let (mut icon_ext, mut folder) = ("png", "files.romfs");
    match platform {
        Platform::Hac => icon_ext = "jpg",
        Platform::Cafe => folder = "content",
        _ => {}
    }

    let base_dir = PathBuf::from(RESOURCES_DIRECTORY).join(platform.to_string());
    let icon_name = base_dir.join("icon").with_extension(icon_ext);

    ResourceMap::from([
        (Resource::ElfBinary, base_dir.join("lovepotion.elf")),
        (Resource::DefaultIcon, icon_name),
        (Resource::RomFS, base_dir.join(folder)),
    ])
}

static RESOURCES: LazyLock<PlatformMap> = LazyLock::new(|| {
    let mut result = PlatformMap::new();
    for platform in Platform::ALL {
        result.insert(platform.clone(), make_resources(&platform));
    }
    result
});

pub fn fetch(platform: &Platform, resource: Resource) -> PathBuf {
    if let Some(path) = RESOURCES.get(platform).and_then(|map| map.get(&resource)) {
        return path.to_owned();
    }
    PathBuf::new()
}
