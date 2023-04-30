#[cfg(test)]
mod tests_main {
    use crate::create_app;
    use actix_http::{body::to_bytes, StatusCode};
    use actix_web::{dev::ServiceResponse, http::header::ContentType, test, web::Bytes};

    trait BodyTest {
        fn as_str(&self) -> &str;
    }

    impl BodyTest for Bytes {
        fn as_str(&self) -> &str {
            std::str::from_utf8(self).unwrap()
        }
    }

    #[actix_web::test]
    async fn test_index_get() {
        let app = test::init_service(create_app()).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_index_get_integration_test() {
        let app = test::init_service(create_app()).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp: ServiceResponse = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
        let body = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(body.as_str(), "Hello world!");
    }
}
