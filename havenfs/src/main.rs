mod s3;
mod util;

use s3::ServerCmdConfig;

#[tokio::main]
async fn main() {
    let config = ServerCmdConfig {
        server_addr: "xxxx".to_string(),
        disk: "/opt/s3iodata".to_string(),
    };
    s3::register_s3_server(config).await
}
