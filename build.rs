use std::{
    path::PathBuf,
    process::Command
};

use which::which;
use embed_manifest::{embed_manifest, new_manifest};

fn find_packfolder() -> Option<PathBuf> {
    if let Ok(path) = which("packfolder") {
        return Some(path)
    }
    if let Ok(path) = which("usciter") {
        if let Some(parent) = path.parent() {
            let mut parent = parent.to_path_buf();
            parent.set_file_name("packfolder");
            if let Ok(path) = which(parent) {
                return Some(path)
            }
        }
    }
    None
}

fn main() {
    if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
        embed_manifest(new_manifest("Contoso.Sample"))
            .expect("unable to embed manifest file");
    }

    if let Some(packfolder) = find_packfolder() {
        Command::new(packfolder)
            .args(["./src/ui", "./src/ui.rc", "-binary"])
            .status()
            .unwrap();
    }   

    println!("cargo:rerun-if-changed=ui"); 
}
