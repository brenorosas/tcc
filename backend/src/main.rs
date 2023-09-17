use std::net::SocketAddr;

use backend::{controller::routes::build_routes, storage::postgres::config::PostgresConfig};
use structopt::StructOpt;
use tokio_postgres::NoTls;
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
    Migrations,
}

#[derive(StructOpt, Debug)]
struct Server {
    #[structopt(long, default_value = "8000", help = "Port to serve the http server")]
    http_port: u16,
}

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

async fn migrations() -> Result<(), anyhow::Error> {
    event!(Level::INFO, "Starting migrations");

    let config = PostgresConfig::new();
    let mut pg_config = tokio_postgres::Config::new();
    pg_config.host(&config.host);
    pg_config.port(config.port);
    pg_config.user(&config.user);
    pg_config.password(&config.password);
    pg_config.dbname(&config.dbname);
    pg_config.application_name("tcc-api");

    let (mut client, connection) = pg_config.connect(NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("An error occured while connecting to database: {e}");
        }
    });

    embedded::migrations::runner()
        .run_async(&mut client)
        .await
        .expect("unable to run migrations");
    Ok(())
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
        Command::Migrations => migrations().await?,
    };

    Ok(())
}
