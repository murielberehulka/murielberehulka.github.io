fn main() {
    let server = rust_net::Server::new(Default::default(), 0);
    println!("Server running ...");
    server.run();
}
