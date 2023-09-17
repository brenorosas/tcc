use structopt::StructOpt;

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
    println!("Server: {:?}", opt.http_port);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let opt = CLI::from_args();
    match opt.cmd {
        Command::Server(cmd) => server(cmd).await?,
    };

    Ok(())
}
