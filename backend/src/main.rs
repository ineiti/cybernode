// This uses actix_web to serve the GET and POST requests.
// TODO: if there is a simpler thing than actix_web which doesn't do parallel
// requests, replace it. Currently all parallel requests are serialized,
// as the 'Broker' can only do one request at a time.

use std::{
    error::Error,
    sync::mpsc::{channel, Sender},
    thread,
    time::{SystemTime, UNIX_EPOCH},
};

use actix_web::{
    error, get,
    http::{header::ContentType, StatusCode},
    middleware, web, App, HttpResponse, HttpServer, Result,
};
use backend::{
    api::stats::StatsReply,
    simul::{
        broker::Broker,
        node::{Node, NodeInfo},
    },
};
use derive_more::Display;
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
            let broker = Broker::default(Self::_now()).expect("Couldn't start broker");
            if let Ok(msg) = rx.recv() {
                if let Err(e) = Main::handle_msg(broker, msg.clone()) {
                    error!("While treating {msg:?}: {e:?}");
                }
            } else {
                return;
            }
        });

        tx
    }

    fn handle_msg(mut broker: Broker, msg: FromWeb) -> Result<(), Box<dyn Error>> {
        Ok(match msg {
            FromWeb::Register(tx, secret) => {
                let id = broker.register(secret);
                let ni = broker.get_node_info(id).unwrap();
                tx.send(ni)?
            }
            FromWeb::Alive(tx, secret) => {
                let id = Node::secret_to_id(secret);
                let mana = broker.alive(id)?;
                tx.send(mana)?
            }
        })
    }

    fn config(config: &mut web::ServiceConfig) {
        config.service(
            web::scope("")
                .app_data(web::Data::new(Main::new()))
                .service(web::resource("/v1/register").route(web::get().to(Self::register)))
                .service(web::resource("/v1/alive").route(web::get().to(Self::alive))),
        );
    }

    async fn alive(state: web::Data<Main>) -> Result<HttpResponse> {
        let (tx, rx) = channel();
        state
            .tx
            .send(FromWeb::Alive(tx, U256::zero()))
            .map_err(|_| UserError::InternalError)?;
        let ni = rx.recv().map_err(|_| UserError::InternalError)?;
        Ok(HttpResponse::Ok().json(ni))
    }

    async fn register(state: web::Data<Main>) -> Result<HttpResponse> {
        let (tx, rx) = channel();
        state
            .tx
            .send(FromWeb::Register(tx, U256::zero()))
            .map_err(|_| UserError::InternalError)?;
        let ni = rx.recv().map_err(|_| UserError::InternalError)?;
        Ok(HttpResponse::Ok().json(ni))
    }

    fn _now() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }
}

#[derive(Debug, Clone)]
enum FromWeb {
    Register(Sender<NodeInfo>, U256),
    Alive(Sender<U256>, U256),
}

// enum ToWeb {}

#[derive(Debug, Display, derive_more::Error)]
enum UserError {
    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalError,
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

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
