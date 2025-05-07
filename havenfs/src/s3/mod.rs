mod api_function;
mod erasure;
mod fs;
mod meta;
mod object;
mod route;
mod storage;
use fs::FsObject;
use object::ObjectManager;
use std::path::PathBuf;
use storage::Posix;

pub struct ServerCmdConfig {
    server_addr: String,
    disk: String,
    data_path: PathBuf,
}

impl ServerCmdConfig {
    pub fn new(server_addr: String, disk: String, data_path: PathBuf) -> ServerCmdConfig {
        ServerCmdConfig {
            server_addr,
            disk,
            data_path,
        }
    }

    pub fn builder() -> ServerCmdConfigBuilder {
        ServerCmdConfigBuilder::new()
    }

    pub fn server_addr(&self) -> &String {
        &self.server_addr
    }

    pub fn disk(&self) -> &String {
        &self.disk
    }

    pub fn data_path(&self) -> &PathBuf {
        &self.data_path
    }
}

//用建造者模式构建ServerCmdConfig
pub struct ServerCmdConfigBuilder {
    server_addr: String,
    disk: String,
    data_path: PathBuf,
}

impl ServerCmdConfigBuilder {
    pub fn new() -> ServerCmdConfigBuilder {
        ServerCmdConfigBuilder {
            server_addr: String::new(),
            disk: String::new(),
            data_path: PathBuf::new(),
        }
    }

    pub fn server_addr(mut self, server_addr: String) -> ServerCmdConfigBuilder {
        self.server_addr = server_addr;
        self
    }

    pub fn disk(mut self, disk: String) -> ServerCmdConfigBuilder {
        self.disk = disk;
        self
    }

    pub fn data_path(mut self, data_path: PathBuf) -> ServerCmdConfigBuilder {
        self.data_path = data_path;
        self
    }

    pub fn build(self) -> ServerCmdConfig {
        ServerCmdConfig {
            server_addr: self.server_addr,
            disk: self.disk,
            data_path: self.data_path,
        }
    }
}

pub async fn register_s3_server(config: ServerCmdConfig) {
    let storage = Posix::new(&PathBuf::from(&config.disk));
    let fs = FsObject::new(storage, config.data_path());
    let storage_impl = ObjectManager::new(fs);
    let root = route::register_s3_route(storage_impl);
    println!("listen...{}", "0.0.0.0:9300");
    warp::serve(root).run(([0, 0, 0, 0], 9300)).await;
}
