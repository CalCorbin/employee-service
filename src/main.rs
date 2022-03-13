#[actix_rt::main]

async fn main() -> std::io::Result<()> {
    // This manages our environment variables
    dotenv().ok();

    // This function initiates the database connection referened in db.rs
    db::init();

    // This restarts the server when changes are detected in the files
    let mut listenfd ListenFd::from_env()

    // This function, specifically employees::init_routes, abstracts all the routes in this service
    let mut server = HttpServer::new(|| App::new().configure(employees::init_routes));

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Please set host in .env");
            let port = env::var("PORT").expect("Please set port in .env");
            server.bind(format!("{}:{}", host, port))?
        }
    };
    server.run().await
}
