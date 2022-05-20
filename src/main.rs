use clap::Parser;
use std::{process, time::Duration};
use tonic::{
    transport::{Certificate, Channel, ClientTlsConfig},
    Request,
};
use tonic_health::proto::{
    health_check_response::ServingStatus, health_client::HealthClient, HealthCheckRequest,
};

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
    #[clap(long, default_value = "grpc_health_probe_rs")]
    user_agent: String,

    /// timeout in milliseconds for establishing connection
    #[clap(long, default_value_t = 1000)]
    connect_timeout: u64,

    /// timeout in milliseconds for health check rpc
    #[clap(long, default_value_t = 1000)]
    rpc_timeout: u64,

    /// use TLS (default: false, INSECURE plaintext transport)
    #[clap(long)]
    tls: bool,

    /// (with -tls, optional) file containing trusted certificates for verifying server
    #[clap(long)]
    tls_ca_cert: Option<String>,

    /// (with -tls) override the hostname used to verify the server certificate
    #[clap(long)]
    tls_server_name: Option<String>,
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

    if !flags.tls && flags.tls_ca_cert.is_some() {
        println!("specified --tls-ca-cert without specifying --tls");
        process::exit(1);
    }

    if !flags.tls && flags.tls_server_name.is_some() {
        println!("specified --tls-server-name without specifying --tls");
        process::exit(1);
    }

    let addr = flags.addr;

    let mut channel_builder = Channel::from_shared(addr.clone())?
        .user_agent(flags.user_agent)?
        .connect_timeout(Duration::from_millis(flags.connect_timeout))
        .timeout(Duration::from_millis(flags.rpc_timeout));

    if flags.tls {
        let mut tls_config = ClientTlsConfig::new();

        if let Some(tls_ca_cert) = flags.tls_ca_cert {
            let pem = tokio::fs::read(tls_ca_cert).await?;

            let ca = Certificate::from_pem(pem);
            tls_config = tls_config.ca_certificate(ca);
        }

        if let Some(tls_server_name) = flags.tls_server_name {
            tls_config = tls_config.domain_name(tls_server_name);
        }

        channel_builder = channel_builder.tls_config(tls_config)?;
    }

    let channel = channel_builder.connect().await.unwrap_or_else(|err| {
        println!(
            "error: failed to connect service at {}: {:?}",
            addr.clone(),
            err
        );
        process::exit(2);
    });

    let mut client = HealthClient::new(channel);

    let request = Request::new(HealthCheckRequest {
        service: flags.service,
    });

    match client.check(request).await {
        Ok(response) => {
            let status = response.into_inner().status();
            match status {
                ServingStatus::Serving => {
                    println!("status: {:?}", ServingStatus::Serving);
                    Ok(())
                }
                _ => {
                    println!("service unhealthy (responded with {:?})", status);
                    process::exit(4);
                }
            }
        }
        Err(status) => {
            match status.code() {
                tonic::Code::Unimplemented =>
                    println!("error: this server does not implement the grpc health protocol (grpc.health.v1.Health): {}", status.message()),
                tonic::Code::DeadlineExceeded => println!("timeout: health rpc did not complete within {}", flags.rpc_timeout),
                _ => println!("error: health rpc failed: {}", status.message()),
            };
            process::exit(3);
        }
    }
}
