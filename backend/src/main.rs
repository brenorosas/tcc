use std::net::SocketAddr;

use backend::controller::routes::build_routes;
use structopt::StructOpt;
use tracing::{event, Level};

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "kebab-case")]
#[structopt(about = "TCC - Api Cli")]
struct CLI {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    Server(Server),
}

#[derive(StructOpt, Debug)]
struct Server {
    #[structopt(long, default_value = "8000", help = "Port to serve the http server")]
    http_port: u16,
}

async fn server(opt: Server) -> Result<(), anyhow::Error> {
    event!(Level::INFO, "Starting server");

    let http_addr: SocketAddr = format!("0.0.0.0:{}", opt.http_port)
        .parse()
        .expect("valid address");

    event!(
        Level::INFO,
        "Setting up http routes at http://{}",
        http_addr
    );

    axum::Server::bind(&http_addr)
        .serve(build_routes().into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt().init();

    let opt = CLI::from_args();
    match opt.cmd {
        Command::Server(cmd) => server(cmd).await?,
    };

    Ok(())
}
