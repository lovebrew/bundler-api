use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Result;
use system::platform::Platform;
use system::resources::Resource;

use crate::{compile::Compile, metadata::Metadata};

pub struct Ctr;

impl Ctr {
    fn create_smdh(&self, path: &Path, metadata: &Metadata, icon: &Path) -> Result<PathBuf> {
        let smdh_path = path.join(format!("{}.smdh", &metadata.title));
        let program = system::programs::get_binary("smdhtool");
        Command::new(program)
            .arg("--create")
            .arg(&metadata.title)
            .arg(&metadata.description)
            .arg(&metadata.author)
            .arg(icon)
            .arg(&smdh_path)
            .output()?;
        Ok(smdh_path)
    }
}

impl Compile for Ctr {
    fn compile(&self, path: &Path, metadata: &Metadata, icon: &Path) -> Result<PathBuf> {
        let smdh_path = self.create_smdh(path, metadata, icon)?;
        let elf_path = system::resources::fetch(&Platform::Ctr, Resource::ElfBinary);
        let romfs_path = system::resources::fetch(&Platform::Ctr, Resource::RomFS);
        let program = system::programs::get_binary("3dsxtool");
        let output_path = path.join(format!("{}.3dsx", &metadata.title));

        Command::new(program)
            .arg(elf_path)
            .arg(&output_path)
            .arg(format!("--smdh={}", smdh_path.display()))
            .arg(format!("--romfs={}", romfs_path.display()))
            .output()?;

        std::fs::remove_file(smdh_path)?;
        Ok(output_path)
    }
}
