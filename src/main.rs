use std::{process, time::Duration};

use clap::Parser;
use tonic::{
    transport::{Certificate, Channel, ClientTlsConfig},
    Request,
};
use tonic_health::proto::{health_client::HealthClient, HealthCheckRequest};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Flags {
    /// (required) tcp host:port to connect
    #[clap(long)]
    addr: String,

    /// service name to check
    #[clap(long, default_value = "\"\"")]
    service: String,

    /// user-agent header value of health check requests
    #[clap(long, default_value = "grpc_health_probe")]
    user_agent: String,

    /// timeout for establishing connection
    #[clap(long, default_value_t = 1)]
    connect_timeout: u64,

    /// timeout for health check rpc
    #[clap(long, default_value_t = 1)]
    rpc_timeout: u64,

    /// use TLS (default: false, INSECURE plaintext transport)
    #[clap(long)]
    tls: bool,

    /// (with --tls) don't verify the certificate (INSECURE) presented by the server (default: false)
    #[clap(long)]
    tls_no_verify: bool,

    /// (with -tls) override the hostname used to verify the server certificate
    #[clap(long, default_value = "")]
    tls_server_name: String,
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

    let vault_ca_cert = "";

    let mut channel_builder = Channel::from_shared(flags.addr)?
        .user_agent(flags.user_agent)?
        .connect_timeout(Duration::from_secs(flags.connect_timeout))
        .timeout(Duration::from_secs(flags.rpc_timeout));

    if flags.tls {
        let ca = Certificate::from_pem(&vault_ca_cert);
        let tls_config = ClientTlsConfig::new()
            .ca_certificate(ca)
            .domain_name(flags.tls_server_name);

        channel_builder = channel_builder.tls_config(tls_config)?;
    }

    let channel = channel_builder.connect().await?;

    let mut client = HealthClient::new(channel);

    let request = Request::new(HealthCheckRequest {
        service: flags.service,
    });

    let response = client.check(request).await.expect("Failed");

    println!("{:?}", response);

    Ok(())
}
