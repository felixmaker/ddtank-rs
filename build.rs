use std::{
    path::PathBuf,
    process::Command
};

use which::which;

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
    if let Some(packfolder) = find_packfolder() {
        Command::new(packfolder)
            .args(["./src/ui", "./src/ui.rc", "-binary"])
            .status()
            .unwrap();
    }
    println!("cargo:rerun-if-changed=ui");
}
