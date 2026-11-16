use sql_rs::server::Server;

#[tokio::main]
async fn main() {
    let server = match Server::build() {
        Ok(s) => s,
        Err(e) => panic!("{e}"),
    };

    match server.serve().await {
        Ok(_) => {}
        Err(e) => panic!("{e}"),
    }
}
