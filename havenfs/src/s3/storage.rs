use crate::util::error::{IoError, IoResult};
use chrono::prelude::*;
use std::ffi::OsString;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

//转rust 结构体
pub(crate) struct FileInfo {
    pub volume: OsString,
    pub name: OsString,
    pub mod_time: DateTime<Local>,
    pub size: u64,
    pub mode: u32,
}

pub(crate) struct VolumeInfo {
    pub name: OsString,
    pub created_time: DateTime<Local>,
}

pub(crate) trait StorageAPI {
    //创建卷
    fn make_volume(&self, volume: &str) -> IoResult<()>;
    //列出卷
    fn list_volume(&self) -> IoResult<Vec<VolumeInfo>>;
    //查看卷信息
    fn stat_volume(&self, volume: &str) -> IoResult<VolumeInfo>;
    //删除卷
    fn delete_volume(&self, volume: &str) -> IoResult<()>;

    //列出指定卷中某个目录下的所有文件和子目录名称，帮助用户了解目录结构。
    fn list_dir(&self, volume: &str, dir: &str) -> IoResult<Vec<OsString>>;
    //从指定文件的某个偏移位置开始读取数据到提供的缓冲区中，返回实际读取的字节数。适用于需要分段读取大文件的场景。
    fn read_file_at(
        &self,
        volume: &str,
        path: &str,
        offset: u64,
        buf: &mut [u8],
    ) -> IoResult<usize>;
    //为后续文件写入做好准备工作，比如预分配文件空间。这样可以提高写入效率，避免因动态扩展文件而产生性能瓶颈。
    fn prepare_file(&self, volume: &str, path: &str, len: usize) -> IoResult<()>;
    //向现有文件中追加数据。常用于顺序写入操作，比如日志或上传分片数据的合并。
    fn append_file(&self, volume: &str, path: &str, buf: &[u8]) -> IoResult<()>;
    //对文件进行重命名或者移动操作，可以在同一卷内修改名称，也可以跨卷移动文件。
    fn rename_file(
        &self,
        src_vol: &str,
        old_path: &str,
        dst_vol: &str,
        new_path: &str,
    ) -> IoResult<()>;
    //获取文件的元数据信息，包括文件大小、修改时间、权限等。
    fn stat_file(&self, volume: &str, path: &str) -> IoResult<FileInfo>;
    //删除文件
    fn delete_file(&self, volume: &str, path: &str) -> IoResult<()>;
    //一次性读取文件的全部内容，返回一个包含所有数据的字节数组，适用于文件较小且需要全部加载的场景。
    fn read_all(&self, volume: &str, path: &str) -> IoResult<Vec<u8>>;
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
        todo!()
    }

    fn list_dir(&self, volume: &str, dir: &str) -> IoResult<Vec<OsString>> {
        todo!()
    }

    fn append_file(&self, volume: &str, path: &str, buf: &[u8]) -> IoResult<()> {
        todo!()
    }

    fn delete_file(&self, volume: &str, path: &str) -> IoResult<()> {
        todo!()
    }

    fn prepare_file(&self, volume: &str, path: &str, len: usize) -> IoResult<()> {
        todo!()
    }

    fn read_all(&self, volume: &str, path: &str) -> IoResult<Vec<u8>> {
        todo!()
    }

    fn read_file_at(
        &self,
        volume: &str,
        path: &str,
        offset: u64,
        buf: &mut [u8],
    ) -> IoResult<usize> {
        todo!()
    }

    fn rename_file(
        &self,
        src_vol: &str,
        old_path: &str,
        dst_vol: &str,
        new_path: &str,
    ) -> IoResult<()> {
        todo!()
    }

    fn stat_file(&self, volume: &str, path: &str) -> IoResult<FileInfo> {
        todo!()
    }
}
// struct RemotePosix {}

// impl StorageAPI for RemotePosix {
//     fn make_volume(&self, volume: &str) -> IoResult<()> {
//         Ok(())
//     }
//     fn list_volume(&self) -> IoResult<Vec<VolumeInfo>> {
//         todo!()
//     }

//     fn stat_volume(&self, volume: &str) -> IoResult<VolumeInfo> {
//         todo!()
//     }

//     fn delete_volume(&self, volume: &str) -> IoResult<()> {
//         Ok(())
//     }
// }
