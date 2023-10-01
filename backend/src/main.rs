use backend::{
    controller::routes::build_routes,
    jwt::service::JwtService,
    storage::postgres::{config::PostgresConfig, PostgresStorage},
    user::service::UsersService,
};
use dotenv::dotenv;
use std::{net::SocketAddr, sync::Arc};
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

    let postgres_config = PostgresConfig::new();
    let postgres_storage = Arc::new(PostgresStorage::new(postgres_config).await.unwrap());
    let jwt_service = Arc::new(JwtService::new());
    let user_service = Arc::new(UsersService::new(postgres_storage, jwt_service));

    axum::Server::bind(&http_addr)
        .serve(build_routes(user_service).into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt().init();
    dotenv().ok();

    let opt = CLI::from_args();
    match opt.cmd {
        Command::Server(cmd) => server(cmd).await?,
        Command::Migrations => migrations().await?,
    };

    Ok(())
}
