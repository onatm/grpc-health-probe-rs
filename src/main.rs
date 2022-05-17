use clap::Parser;
use tonic::{transport::Channel, Request};
use tonic_health::proto::{health_client::HealthClient, HealthCheckRequest};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Flags {
    /// (required) tcp host:port to connect
    #[clap(long)]
    addr: String,

    /// service name to check
    #[clap(long, default_value = "\"\"", required = false)]
    service: String,

    /// user-agent header value of health check requests
    #[clap(long, default_value = "grpc_health_probe", required = false)]
    user_agent: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let flags = Flags::parse();

    let channel = Channel::from_shared(flags.addr)?
        .user_agent(flags.user_agent)?
        .connect()
        .await?;

    let mut client = HealthClient::new(channel);

    let request = Request::new(HealthCheckRequest {
        service: flags.service,
    });

    let _response = client.check(request).await.expect("Failed");

    Ok(())
}
