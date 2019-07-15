use std::sync::Arc;

use failure::{Error, format_err, bail};
use actix_multipart::{Multipart, Field};
use actix_web::{web, guard, HttpResponse, FromRequest};
use actix_web::dev::HttpServiceFactory;
use actix_web::http::Uri;
use actix_web::client::Client;
use futures::stream::{self, Stream};
use futures::future::{self, Future, Either};

use crate::models::{ImageFormat, Image, ImageId};
use crate::storage::Storage;
use super::{Request, Response, Base64Image};


/// simple interface for testing purposes
fn upload_form() -> HttpResponse {
    let html = 
        r#"<html>
            <head><title>Upload Test</title></head>
            <body>
                <form target="/images/upload" method="post" enctype="multipart/form-data">
                    <input type="file" name="file"/>
                    <input type="submit" value="Submit"></button>
                </form>
            </body>
        </html>"#;

    HttpResponse::Ok().body(html)
}

/// extract image type
fn extract_type(data: &Field) -> Result<ImageFormat, Error> {
    let content_type = data.content_type();
    let type_ = content_type.type_().as_str();
    
    if type_ != "image" {
        bail!("Unsupported content type: {}", type_);
    }
    
    let subtype = content_type.subtype().as_str();
    let format = subtype.parse::<ImageFormat>()?;
    Ok(format)
}

/// parse image from multipart item
fn extract_image(data: Field) -> impl Future<Item=Image, Error=Error> {
    match extract_type(&data) {
        Err(e) => Either::B(future::err(e)),
        Ok(format) => Either::A(
            data.concat2()
                .map_err(|e| format_err!("Multipart error: {}", e))
                .and_then(move |bytes| Image::decode(&bytes, format))
        )
    }
}

/// save images to storage asynchronously and return their ids
fn store_images<S>(storage: Arc<S>, images: Vec<Image>) -> impl Future<Item=Vec<ImageId>, Error=Error>
    where S: Storage
{
    web::block(move || storage.store(images))
        .map_err(|e| format_err!("{}", e))
}

/// multipart/form-data handler
fn upload_multipart<S>(state: web::Data<Arc<S>>, stream: Multipart) -> impl Future<Item=HttpResponse, Error=Error> 
    where S: Storage 
{
    let storage = state.get_ref().clone();
    stream
        .map_err(|e| format_err!("Multipart error: {}", e))
        .and_then(extract_image)
        .collect()
        .and_then(move |images| store_images(storage, images))
        .map(|ids| HttpResponse::Ok().json(Response::Ids(ids)))
}

/// base64 images handler
fn upload_base64<S>(storage: Arc<S>, images: Vec<Base64Image>) -> impl Future<Item=HttpResponse, Error=Error>
    where S: Storage
{
    let images: Result<Vec<Image>, Error> = images.into_iter()
        .map(|image| image.decode())
        .collect();
    
    match images {
        Err(e) => Either::B(
            future::ok(
                HttpResponse::BadRequest()
                    .json(Response::Error(format!("{}", e)))
            )
        ),
        Ok(images) => Either::A(
            store_images(storage, images)
                .map(|ids| HttpResponse::Ok().json(Response::Ids(ids)))
        )
    }
}

fn download_image(uri: Uri) -> impl Future<Item=Image, Error=Error> {
    let client = Client::default();

    client.get(&uri)
        .send()
        .map_err(|e| format_err!("{}", e))
        // get image format from content type
        .and_then(|response| {
            response.headers()
                .get("Content-Type")
                .ok_or_else(|| format_err!("No Content-Type header"))
                .and_then(|header| 
                    header.to_str()
                        .map_err(|e| format_err!("Invalid Content-Type header: {}", e))
                )
                .and_then(ImageFormat::from_content_type)
                .map(|format| (response, format))
        })
        // receive body
        .and_then(|(response, format)| {
            response.concat2()
                .map_err(|e| format_err!("{}", e))
                .map(move |body| (body, format))
        })
        .and_then(|(body, format)| 
            Image::decode(&body, format)
                .map_err(|e| format_err!("Failed to decode image: {}", e))
        )
}

/// upload from remote location
fn upload_from_links<S>(storage: Arc<S>, locations: Vec<String>) -> impl Future<Item=HttpResponse, Error=Error>
    where S: Storage
{
    let links = locations.iter()
        .map(|link| link.parse::<Uri>())
        .collect::<Result<Vec<_>, _>>();

    match links {
        Err(e) => Either::B(
            future::ok(
                HttpResponse::BadRequest()
                    .json(Response::Error(format!("Invalid uri: {}", e)))
            )
        ),
        Ok(links) => Either::A(
            stream::iter_ok(links)
                .and_then(|link| 
                    download_image(link.clone())
                        .map_err(move |e| format_err!("Failed to download image from {}: {}", link, e))
                )
                .collect()
                .and_then(move |images| store_images(storage, images))
                .map(|ids| HttpResponse::Ok().json(Response::Ids(ids)))
        )
    }
}

/// json request handler
fn upload_json<S: Storage>(
    state: web::Data<Arc<S>>, 
    request: web::Json<Request>
) -> impl Future<Item=HttpResponse, Error=Error> {

    let storage = state.get_ref().clone();
    match request.into_inner() {
        Request::Base64 { images } => Either::A(upload_base64(storage, images)),
        Request::Remote { locations } => Either::B(upload_from_links(storage, locations))
    }
}

pub fn bind<S>(path: &'static str) -> impl HttpServiceFactory
    where S: Storage
{
    web::resource(path)
        .data(web::Json::<Request>::configure(|cfg| {
            cfg.limit(1024 * 1024) // 1MB
        }))
        .route(web::get().to(upload_form))
        .route(
            web::post()
                .guard(guard::Header("Content-Type", "application/json"))
                .to_async(upload_json::<S>)
        )
        .route(
            web::post()
                .to_async(upload_multipart::<S>)
        )
}
