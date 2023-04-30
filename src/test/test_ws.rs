#[cfg(test)]
mod tests_main {

    use crate::create_app;
    use actix_http::ws;
    use actix_web::web::Bytes;

    use futures_util::{SinkExt as _, StreamExt as _}; // this is needed for send method in Framed

    #[actix_web::test]
    async fn test_websocket1() {
        let mut srv = actix_test::start(create_app);

        let mut framed = srv.ws_at("/ws").await.unwrap();

        framed.send(ws::Message::Text("text".into())).await.unwrap();
        let item = framed.next().await.unwrap().unwrap();
        assert_eq!(item, ws::Frame::Text(Bytes::from_static(b"send: text")));
    }
}
