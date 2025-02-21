use super::object::ObjectAPI;
use crate::util::error::S3Result;

unsafe impl Send for Erasure {}
unsafe impl Sync for Erasure {}

struct Erasure {}
