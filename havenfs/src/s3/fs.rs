use super::object::{BucketInfo, ObjectAPI};
use super::storage::StorageAPI;
use crate::util::error::S3Result;
use std::path::Path;
use std::path::PathBuf;

unsafe impl<S: StorageAPI> Send for FsObject<S> {}
unsafe impl<S: StorageAPI> Sync for FsObject<S> {}

pub(crate) struct FsObject<S: StorageAPI> {
    storage: S,       // 存储引擎
    fs_path: PathBuf, // 文件系统路径
}

impl<S: StorageAPI> FsObject<S> {
    pub fn new<P: AsRef<Path>>(s: S, fs_path: P) -> FsObject<S> {
        Self {
            storage: s,
            fs_path: fs_path.as_ref().to_path_buf(),
        }
    }
}

impl<S: StorageAPI> ObjectAPI for FsObject<S> {
    fn make_bucket(&self, bucket: &str) -> S3Result<()> {
        // todo 校验名字是否正确
        self.storage.make_volume(bucket)
    }

    fn list_bucket(&self) -> S3Result<Vec<BucketInfo>> {
        let vols = self.storage.list_volume()?;
        let buckets = vols
            .into_iter()
            .filter_map(|e| {
                Some(BucketInfo {
                    name: e.name,
                    created_time: e.created_time,
                })
            })
            .collect();
        Ok(buckets)
    }

    fn delete_bucket(&self, bucket: &str) -> S3Result<()> {
        todo!()
    }

    fn delete_object(&self, bucket: &str, object: &str) -> S3Result<()> {
        todo!()
    }

    fn get_object(&self, bucket: &str, object: &str) -> S3Result<()> {
        todo!()
    }

    fn list_object(&self, bucket: &str) -> S3Result<Vec<String>> {
        todo!()
    }
    fn put_object(&self, bucket: &str, object: &str) -> S3Result<()> {
        todo!()
    }
    fn stat_bucket(&self, bucket: &str) -> S3Result<BucketInfo> {
        todo!()
    }
    fn stat_object(&self, bucket: &str, object: &str) -> S3Result<()> {
        todo!()
    }
}
