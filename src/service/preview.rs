use std::sync::Arc;

use actix_web::dev::HttpServiceFactory;
use actix_web::{web, HttpResponse};
use futures::Future;
use failure::{Error, format_err};

use crate::storage::Storage;
use crate::models::ImageId;


fn generate_preview<S>(
    state: web::Data<Arc<S>>, 
    info: web::Path<(u32,)>
) -> impl Future<Item=HttpResponse, Error=Error>
    where S: Storage
{
    let storage = state.get_ref().clone();
    let id = ImageId(info.0 as i32);
    web::block(move || {
        storage.load(id)
            .map_err(|e| format_err!("Failed to load image: {}", e))
    })
    .map_err(|e| format_err!("{}", e))
    .map(|image| image.preview((100, 100)))
    .and_then(|preview| preview.data().map(|data| (preview.format(), data)))
    .map(|(format, data)|
         HttpResponse::Ok()
            .content_type(format!("image/{}", format))
            .content_length(data.len() as u64)
            .body(data)
    )
}

pub fn bind<S>(prefix: &str) -> impl HttpServiceFactory
    where S: Storage
{
    let path = prefix.to_string() + "/{id}/preview";
    web::resource(&path)
        .route(web::get().to_async(generate_preview::<S>))
    
}
