mod api_function;
mod erasure;
mod fs;
mod object;
mod route;
mod storage;

use fs::FsObject;
use object::ObjectManager;
use std::path::PathBuf;
use storage::Posix;

pub struct ServerCmdConfig {
    pub server_addr: String,
    pub disk: String,
}

pub async fn register_s3_server(config: ServerCmdConfig) {
    let storage = Posix::new(&PathBuf::from(&config.disk));
    let fs = FsObject::new(storage);
    let storage_impl = ObjectManager::new(fs);
    let root = route::register_s3_route(storage_impl);
    println!("listen...{}", "0.0.0.0:9300");
    warp::serve(root).run(([0, 0, 0, 0], 9300)).await;
}
