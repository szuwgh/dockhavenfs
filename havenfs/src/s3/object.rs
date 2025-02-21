use super::storage::StorageAPI;
use crate::util::error::S3Result;
use chrono::prelude::*;
use std::ffi::OsString;
use std::sync::Arc;

pub struct BucketInfo {
    pub name: OsString,
    pub created_time: DateTime<Local>,
}

pub trait ObjectAPI {
    fn make_bucket(&self, bucket: &str) -> S3Result<()>;
    fn list_bucket(&self) -> S3Result<Vec<BucketInfo>>;
    fn stat_bucket(&self, bucket: &str) -> S3Result<BucketInfo>;
    fn delete_bucket(&self, bucket: &str) -> S3Result<()>;
    fn list_object(&self, bucket: &str) -> S3Result<Vec<String>>;

    //object api
    fn put_object(&self, bucket: &str, object: &str) -> S3Result<()>;
    fn get_object(&self, bucket: &str, object: &str) -> S3Result<()>;
    fn delete_object(&self, bucket: &str, object: &str) -> S3Result<()>;
    fn stat_object(&self, bucket: &str, object: &str) -> S3Result<()>;
}

unsafe impl Sync for ObjectManager {}
unsafe impl Send for ObjectManager {}

pub struct ObjectManager(Arc<dyn ObjectAPI>);

impl ObjectManager {
    pub fn new<T: ObjectAPI + 'static>(m: T) -> ObjectManager {
        ObjectManager(Arc::new(m))
    }

    pub fn make_bucket(&self, bucket: &str) -> S3Result<()> {
        self.0.make_bucket(bucket)
    }

    pub fn list_bucket(&self) -> S3Result<Vec<BucketInfo>> {
        self.0.list_bucket()
    }
}

impl Clone for ObjectManager {
    fn clone(&self) -> Self {
        ObjectManager(self.0.clone())
    }
}

pub struct MultiObject {
    storages: Vec<Arc<dyn StorageAPI>>,
}
