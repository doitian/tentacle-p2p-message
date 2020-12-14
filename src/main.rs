use tentacle::{
    builder::ServiceBuilder,
    context::ServiceContext,
    secio::SecioKeyPair,
    service::{ServiceEvent, TargetProtocol},
    traits::ServiceHandle,
};

struct AppServiceHandle;

impl ServiceHandle for AppServiceHandle {
    fn handle_event(&mut self, _control: &mut ServiceContext, event: ServiceEvent) {
        if let ServiceEvent::ListenStarted { address: _ } = event {
            log::info!("Hello, Tentacle");
        }

        log::info!("handle_event: {:?}", event);
    }
}

struct AppArgs {
    port: u16,
    bootnode: Option<String>,
    target_peer_id: Option<String>,
    message: Option<String>,
}

impl Default for AppArgs {
    fn default() -> Self {
        Self {
            port: 1234,
            bootnode: None,
            target_peer_id: None,
            message: None,
        }
    }
}

/// Parses the command line args.
///
/// ## Usage
///
/// * `p2p-message`: start a node listening on the default port 1234.
/// * `p2p-message port`: start a node listening on the specified port.
/// * `p2p-message port bootnode`: start a node listening on the specified port and connect to
/// another node as the bootnode.
/// * `p2p-message port bootnode target_peer_id message`: start a node, connect to the bootnode, then send a message to `target_peer_id`.
fn parse_args() -> AppArgs {
    let mut parsed_args = AppArgs::default();
    let args: Vec<_> = std::env::args().collect();
    if args.len() > 1 {
        {
            use std::str::FromStr;
            parsed_args.port = u16::from_str(&args[1]).expect("port number");
        }
    }
    if args.len() > 2 && !args[2].is_empty() {
        parsed_args.bootnode = Some(args[2].clone());
    }
    if args.len() > 3 && !args[3].is_empty() {
        parsed_args.target_peer_id = Some(args[3].clone());
    }
    if args.len() > 4 && !args[4].is_empty() {
        parsed_args.message = Some(args[4].clone());
    }

    parsed_args
}

fn main() {
    {
        use log::LevelFilter::Info;
        env_logger::builder().filter_level(Info).init();
    }

    let args = parse_args();

    let mut rt = tokio::runtime::Runtime::new().expect("create tokio runtime");

    rt.block_on(async {
        let key_pair = SecioKeyPair::secp256k1_generated();
        log::info!(
            "listen on /ip4/127.0.0.1/tcp/{}/p2p/{}",
            args.port,
            key_pair.peer_id().to_base58()
        );

        let mut app_service = ServiceBuilder::default()
            .key_pair(key_pair)
            // By default, tentacle auto closes the connection when it is idle for more than 10
            // seconds. Set this timeout to 1 day for this sample application.
            .timeout(std::time::Duration::new(86640, 0))
            .build(AppServiceHandle);

        app_service
            .listen(format!("/ip4/127.0.0.1/tcp/{}", args.port).parse().unwrap())
            .await
            .expect("listen");

        if let Some(bootnode) = args.bootnode {
            log::info!("dial {}", bootnode);
            app_service
                .dial(
                    bootnode.parse().expect("bootnode multiaddr"),
                    TargetProtocol::All,
                )
                .await
                .expect("connect bootnode");
        }

        {
            use futures::stream::StreamExt;
            while app_service.next().await.is_some() {
                // loop
            }
        }
    });
}
