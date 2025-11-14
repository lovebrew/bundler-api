use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Result;
use system::platform::Platform;
use system::resources::Resource;

use crate::{compile::Compile, metadata::Metadata};

pub struct Hac;

impl Hac {
    fn create_nacp(&self, path: &Path, metadata: &Metadata) -> Result<PathBuf> {
        let nacp_path = path.join(format!("{}.nacp", &metadata.title));
        let program = system::programs::get_binary("smdhtool");

        Command::new(program)
            .arg("--create")
            .arg(&metadata.title)
            .arg(&metadata.author)
            .arg(&metadata.version)
            .arg(&nacp_path)
            .output()?;

        Ok(nacp_path)
    }
}

impl Compile for Hac {
    fn compile(&self, path: &Path, metadata: &Metadata, icon: &Path) -> Result<PathBuf> {
        let nacp_path = self.create_nacp(path, metadata)?;
        let elf_path = system::resources::fetch(&Platform::Hac, Resource::ElfBinary);
        let romfs_path = system::resources::fetch(&Platform::Hac, Resource::RomFS);
        let program = system::programs::get_binary("elf2nro");
        let output_path = path.join(format!("{}.nro", &metadata.title));

        Command::new(program)
            .arg(elf_path)
            .arg(&output_path)
            .arg(format!("--icon={icon:?}"))
            .arg(format!("--nacp={nacp_path:?}"))
            .arg(format!("--romfs={romfs_path:?}"))
            .output()?;

        let output_path = output_path.strip_prefix(path)?;
        Ok(output_path.to_owned())
    }
}
