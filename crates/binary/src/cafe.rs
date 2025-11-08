use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Result;
use system::platform::Platform;
use system::resources::Resource;

use crate::{compile::Compile, metadata::Metadata};

pub struct Cafe;

impl Cafe {
    fn create_rpx(&self, path: &Path, title: &String) -> Result<PathBuf> {
        let rpl_path = path.join(format!("{}.rpx", title));
        let program = system::programs::get_binary("elf2rpl");
        let elf_path = system::resources::fetch(&Platform::Cafe, Resource::ElfBinary);

        Command::new(program)
            .arg(elf_path)
            .arg(&rpl_path)
            .output()?;

        Ok(rpl_path)
    }
}

impl Compile for Cafe {
    fn compile(&self, path: &Path, metadata: &Metadata, icon: &Path) -> Result<(PathBuf, Vec<u8>)> {
        let rpx_path = self.create_rpx(path, &metadata.title)?;
        let content_path = system::resources::fetch(&Platform::Cafe, Resource::RomFS);
        let program = system::programs::get_binary("wuhbtool");
        let output_path = path.join(format!("{}.wuhb", &metadata.title));

        let bytes = Command::new(program)
            .arg(rpx_path)
            .arg(&output_path)
            .arg(format!("--content={}", content_path.display()))
            .arg(format!("--name={}", metadata.title))
            .arg(format!("--short-name={}", metadata.title))
            .arg(format!("--author={}", metadata.author))
            .arg(format!("--icon={icon:?}"))
            .output()
            .and_then(|_| std::fs::read(&output_path))?;

        let output_path = output_path.strip_prefix(path)?;
        Ok((output_path.to_owned(), bytes))
    }
}
