fn main() {
    {
        use log::LevelFilter::Info;
        env_logger::builder().filter_level(Info).init();
    }

    log::info!("Hello");
}
