use crate::util::error::{IoError, IoResult};
use chrono::prelude::*;
use std::ffi::OsString;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

pub(crate) struct VolumeInfo {
    pub name: OsString,
    pub created_time: DateTime<Local>,
}

pub(crate) trait StorageAPI {
    fn make_volume(&self, volume: &str) -> IoResult<()>;
    fn list_volume(&self) -> IoResult<Vec<VolumeInfo>>;
    fn stat_volume(&self, volume: &str) -> IoResult<VolumeInfo>;
    fn delete_volume(&self, volume: &str) -> IoResult<()>;
}

pub struct Posix {
    disk_path: PathBuf,
}

impl Posix {
    pub fn new(disk_path: &Path) -> Posix {
        Self {
            disk_path: disk_path.to_path_buf(),
        }
    }

    fn get_vol_dir(&self, volume: &str) -> IoResult<PathBuf> {
        Ok(self.disk_path.join(volume))
    }
}

impl StorageAPI for Posix {
    fn make_volume(&self, volume: &str) -> IoResult<()> {
        let vol_dir = self.get_vol_dir(volume)?;
        return match fs::create_dir(vol_dir) {
            Err(e) => match e.kind() {
                ErrorKind::AlreadyExists => return Err(IoError::VolumeExists),
                ErrorKind::PermissionDenied => return Err(IoError::VolumeExists),
                _ => return Err(IoError::UnexpectIO(e)),
            },
            Ok(_) => Ok(()),
        };
    }

    fn list_volume(&self) -> IoResult<Vec<VolumeInfo>> {
        let dir = fs::read_dir(&self.disk_path)?;
        let vols_info = dir
            .filter_map(|entry| {
                let e = entry.ok()?;
                if e.path().is_dir() {
                    let data = e.metadata().ok()?;
                    return Some(VolumeInfo {
                        name: e.file_name(),
                        created_time: data.modified().ok()?.clone().into(),
                    });
                }
                None
            })
            .collect();
        Ok(vols_info)
    }

    fn stat_volume(&self, volume: &str) -> IoResult<VolumeInfo> {
        todo!()
    }

    fn delete_volume(&self, volume: &str) -> IoResult<()> {
        Ok(())
    }
}
struct RemotePosix {}

impl StorageAPI for RemotePosix {
    fn make_volume(&self, volume: &str) -> IoResult<()> {
        Ok(())
    }
    fn list_volume(&self) -> IoResult<Vec<VolumeInfo>> {
        todo!()
    }

    fn stat_volume(&self, volume: &str) -> IoResult<VolumeInfo> {
        todo!()
    }

    fn delete_volume(&self, volume: &str) -> IoResult<()> {
        Ok(())
    }
}
