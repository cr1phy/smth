use actix::{Actor, StreamHandler};
use actix_web_actors::ws::{Message, ProtocolError, WebsocketContext};
use log::info;
use migration::ArrayType::String;
use serde_json::json;

use crate::event::Event;

pub struct Ws;

impl Actor for Ws {
    type Context = WebsocketContext<Self>;
}

impl StreamHandler<Result<Message, ProtocolError>> for Ws {
    fn handle(&mut self, msg: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(Message::Text(text)) = msg {
            todo!()
        }
    }
}

impl Ws {
    fn convert_to_event(s: impl ToString) -> Event {
        todo!()
    }
    
    fn process_event() -> Result<String, String> {
        todo!()
    }
}
