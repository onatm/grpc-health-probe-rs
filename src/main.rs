use std::{process, time::Duration};

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

    /// timeout for establishing connection
    #[clap(long, default_value_t = 1, required = false)]
    connect_timeout: u64,

    /// timeout for health check rpc
    #[clap(long, default_value_t = 1, required = false)]
    rpc_timeout: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let flags = Flags::parse();

    if flags.connect_timeout == 0 {
        println!("--connect-timeout must be greater than zero");
        process::exit(1);
    }

    if flags.rpc_timeout == 0 {
        println!("--rpc-timeout must be greater than zero");
        process::exit(1);
    }

    let channel_builder = Channel::from_shared(flags.addr)?
        .user_agent(flags.user_agent)?
        .connect_timeout(Duration::from_secs(flags.connect_timeout))
        .timeout(Duration::from_secs(flags.rpc_timeout));

    let channel = channel_builder.connect().await?;

    let mut client = HealthClient::new(channel);

    let request = Request::new(HealthCheckRequest {
        service: flags.service,
    });

    let _response = client.check(request).await.expect("Failed");

    Ok(())
}
