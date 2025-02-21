use super::object::{BucketInfo, ObjectAPI};
use super::storage::StorageAPI;
use crate::util::error::S3Result;

unsafe impl<S: StorageAPI> Send for FsObject<S> {}
unsafe impl<S: StorageAPI> Sync for FsObject<S> {}

pub(crate) struct FsObject<S: StorageAPI> {
    storage: S,
}

impl<S: StorageAPI> FsObject<S> {
    pub fn new(s: S) -> FsObject<S> {
        Self { storage: s }
    }
}

impl<S: StorageAPI> ObjectAPI for FsObject<S> {
    fn make_bucket(&self, bucket: &str) -> S3Result<()> {
        //校验名字是否正确
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
}
