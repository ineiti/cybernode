// This uses actix_web to serve the GET and POST requests.
// TODO: if there is a simpler thing than actix_web which doesn't do parallel
// requests, replace it. Currently all parallel requests are serialized,
// as the 'Broker' can only do one request at a time.

use std::{
    sync::mpsc::{channel, Sender},
    thread,
};

use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Result};
use backend::{
    api::stats::StatsReply,
    simul::{broker::Broker, trusted::NodeInfo},
};
use primitive_types::U256;
use tracing::error;

#[get("/v1/stats")]
async fn greet() -> HttpResponse {
    HttpResponse::Ok().json(StatsReply { ids: vec![] })
}

struct Main {
    tx: Sender<FromWeb>,
}

impl Main {
    fn new() -> Self {
        Self { tx: Self::listen() }
    }

    fn listen() -> Sender<FromWeb> {
        let (tx, rx) = channel::<FromWeb>();

        thread::spawn(move || loop {
            let mut broker = Broker::default().expect("Couldn't start broker");
            match rx.recv() {
                Ok(msg) => match msg {
                    FromWeb::Nop => todo!(),
                    FromWeb::Register(tx, secret) => {
                        let id = broker.register(secret);
                        let ni = broker.get_node_info(id).unwrap();
                        tx.send(ni)
                            .unwrap_or_else(|e| error!("While answering request: {e:?}"));
                    }
                },
                Err(_) => todo!(),
            }
        });

        tx
    }

    fn config(config: &mut web::ServiceConfig) {
        config.service(
            web::scope("")
                .app_data(web::Data::new(Main::new()))
                .service(web::resource("/v1/register").route(web::get().to(Self::register))),
        );
    }

    async fn register(state: web::Data<Main>) -> Result<HttpResponse> {
        let (tx, rx) = channel();
        match state.tx.send(FromWeb::Register(tx, U256::zero())) {
            Ok(_) => Ok(HttpResponse::Ok()
                .content_type("text/plain")
                .body(format!("Stats"))),
            Err(_) => todo!(),
        }
    }
}

enum FromWeb {
    Register(Sender<NodeInfo>, U256),
    Nop,
}

enum ToWeb {}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(Main::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
