use crate::errors::ErrCode;

use std::ffi::CString;
use std::path::PathBuf;

#[derive(Clone)]
pub struct ContainerOps {
    pub path: CString,
    pub argv: Vec<CString>,
    pub uid: u32,
    pub mount_dir: PathBuf,
}

impl ContainerOps {
    pub fn new(command: String, uid: u32, mount_dir: PathBuf) -> Result<ContainerOps, ErrCode> {
        let argv: Vec<CString> = command
            .split_ascii_whitespace()
            .map(|s| CString::new(s).expect("Cannot read arg"))
            .collect();
        let path = argv[0].clone();

        Ok(ContainerOps {
            path,
            argv,
            uid,
            mount_dir,
        })
    }
}
