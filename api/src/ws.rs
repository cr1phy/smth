use std::error::Error;

use actix::{Actor, StreamHandler};
use actix_web_actors::ws::{Message, ProtocolError, WebsocketContext};
use log::info;

use crate::event::Event;

pub struct Ws;

impl Actor for Ws {
    type Context = WebsocketContext<Self>;
}

impl StreamHandler<Result<Message, ProtocolError>> for Ws {
    fn handle(&mut self, msg: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(Message::Text(text)) => {
                match Self::convert_to_event(text.trim()) {
                    Ok(event) => {
                        info!("{:#?}", event);
                        ctx.text("Ok!")
                    }
                    Err(e) => ctx.text(e),
                    _ => ctx.close(None),
                }
            }
            Ok(Message::Close(reason)) => ctx.close(reason),
            Ok(Message::Ping(bytes)) => ctx.pong(bytes.iter().as_ref()),
            Ok(Message::Pong(bytes)) => ctx.ping(bytes.iter().as_ref()),
            _ => {}
        }
    }
}

impl Ws {
    fn convert_to_event(s: &str) -> Result<Event, String> {
        serde_json::from_str::<Event>(s).map_err(|_| String::from("Oops! Something went wrong."))
    }
    
    fn process_event() -> Result<String, String> {
        todo!()
    }
}
