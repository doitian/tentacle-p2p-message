fn main() {
    {
        use log::LevelFilter::Info;
        env_logger::builder().filter_level(Info).init();
    }

    let mut rt = tokio::runtime::Runtime::new().expect("create tokio runtime");

    rt.block_on(async {
        log::info!("Hello, Tokio");
    });
}
