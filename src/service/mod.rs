pub mod upload;
pub mod preview;
mod request;
mod response;

pub use request::{Request, Base64Image};
pub use response::Response;


#[cfg(test)]
mod tests {
    use std::sync::Arc;
   
    use actix_web::{test, App};

    use crate::storage::{Storage, MemoryStorage};
    use crate::models::{Image, ImageId, ImageFormat};
    use super::{upload, preview, Request, Base64Image, Response};

    const TEST_IMAGE: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/test-data/servo.png"));

    #[test]
    fn upload_base64() {
        let storage = Arc::new(MemoryStorage::default());
        let mut app = test::init_service(
            App::new()
                .data(storage.clone())
                .service(upload::bind::<MemoryStorage>("/upload"))
        );
        
        let image = Base64Image {
            format: ImageFormat::PNG,
            data: base64::encode(TEST_IMAGE)
        };

        let request = test::TestRequest::post()
            .uri("/upload")
            .set_json(&Request::Base64 {
               images: vec![image] 
            })
            .to_request();

        let response: Response = test::read_response_json(&mut app, request);
        assert_eq!(response, Response::Ids(vec![ImageId(0)]));
        assert!(storage.load(ImageId(0)).is_ok());
    }

    #[test]
    fn upload_from_link() {
        let storage = Arc::new(MemoryStorage::default());
        let mut app = test::init_service(
            App::new()
                .data(storage.clone())
                .service(upload::bind::<MemoryStorage>("/upload"))
        );

        let url = mockito::server_url();
        let request = test::TestRequest::post()
            .uri("/upload")
            .set_json(&Request::Remote {
               locations: vec![url] 
            })
            .to_request();

        let _m = mockito::mock("GET", "/")
            .with_status(200)
            .with_header("Content-Type", "image/png")
            .with_body(TEST_IMAGE)
            .create();
        
        let response: Response = test::read_response_json(&mut app, request);
        assert_eq!(response, Response::Ids(vec![ImageId(0)]));
        assert!(storage.load(ImageId(0)).is_ok());
    }

    #[test]
    fn generate_preview() {
        let image = Image::decode(TEST_IMAGE, ImageFormat::PNG).unwrap();
        let storage = Arc::new(MemoryStorage::with_image(image.clone()));

        let mut app = test::init_service(
            App::new()
                .data(storage)
                // NOTE: upload service is not required for this test,
                //       but it is included to make sure that routing
                //       works as expected
                .service(upload::bind::<MemoryStorage>("/images/upload"))
                .service(preview::bind::<MemoryStorage>("/images"))
        );

        let request = test::TestRequest::get()
            .uri("/images/0/preview")
            .to_request();

        let response = test::call_service(&mut app, request);
        assert!(response.status().is_success());
        
        let content_type = response.headers()
            .get("Content-Type").unwrap()
            .to_str().unwrap();
        assert_eq!(content_type, "image/png");
        
        let body = test::read_body(response);
        let preview = Image::decode(&body, ImageFormat::PNG).unwrap();
        assert_eq!(preview.format(), ImageFormat::PNG);
        assert_eq!(preview.data().unwrap(), image.preview((100, 100)).data().unwrap());
    }
}

