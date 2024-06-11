use actix::{Actor, StreamHandler};
use actix_web_actors::ws::{Message, ProtocolError, WebsocketContext};

struct Ws;
impl Actor for Ws {
    type Context = WebsocketContext<Self>;
}
impl StreamHandler<Result<Message, ProtocolError>> for Ws {
    fn handle(&mut self, msg: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
       match msg {
           Ok(Message::Text(text)) => { ctx.text(Self::convert_to_event()) }
           _ => {}
       }
    }
}

impl Ws {
    fn convert_to_event() -> String {
        String::new()
    }
}