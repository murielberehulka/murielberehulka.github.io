fn main() {
    let server = rust_net::Server::new(
        rust_net::Settings {
            static_files: Some(rust_net::StaticFilesSettings {
                root_path: "docs",
                enable_cache: false,
            }),
            ..Default::default()
        },
        0,
    );
    println!("Server running ...");
    server.run();
}
