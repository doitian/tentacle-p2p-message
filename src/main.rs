use tentacle::{
    builder::ServiceBuilder, context::ServiceContext, secio::SecioKeyPair, service::ServiceEvent,
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

fn main() {
    {
        use log::LevelFilter::Info;
        env_logger::builder().filter_level(Info).init();
    }

    let mut rt = tokio::runtime::Runtime::new().expect("create tokio runtime");

    rt.block_on(async {
        let mut app_service = ServiceBuilder::default()
            .key_pair(SecioKeyPair::secp256k1_generated())
            .build(AppServiceHandle);

        app_service
            .listen("/ip4/127.0.0.1/tcp/1234".parse().unwrap())
            .await
            .expect("listen on 127.0.0.1:1234");

        {
            use futures::stream::StreamExt;
            while app_service.next().await.is_some() {
                // loop
            }
        }
    });
}
