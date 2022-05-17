use std::fs::copy;
// use actix_multipart::Multipart;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
// use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer, HttpRequest, Responder};
use actix_easy_multipart::{File, FromMultipart, extractor};
use actix_easy_multipart::extractor::MultipartForm;

#[derive(FromMultipart)]
struct Upload {
   description: String,
   image: File,
}

async fn save_file(form: MultipartForm<Upload>) -> impl Responder {
    println!("TEST");
    println!("{:?}", form.image.file.path());
    let fileName = form.image.filename.as_ref().unwrap();
    let save_path = format!("./{}", fileName);
    copy(form.image.file.path(), save_path).unwrap();
    format!("Received image of size: {} - {} - {}", form.image.size, form.description, fileName)
}

// use futures_util::TryStreamExt as _;
// use uuid::Uuid;

// async fn save_file(mut payload: Multipart, req: HttpRequest) -> Result<HttpResponse, Error> {
//     // iterate over multipart stream
//     println!("{:?}", req);
//     while let Some(mut field) = payload.try_next().await? {
//         // A multipart/form-data stream has to contain `content_disposition`
//         let content_disposition = field.content_disposition();

//         let filename = content_disposition
//             .get_filename()
//             .map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);
//         let filepath = format!("./tmp/{}", filename);

//         // File::create is blocking operation, use threadpool
//         let mut f = web::block(|| std::fs::File::create(filepath)).await??;

//         // Field in turn is stream of *Bytes* object
//         while let Some(chunk) = field.try_next().await? {
//             // filesystem operations are blocking, we have to use threadpool
//             f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
//         }
//     }

//     Ok(HttpResponse::Ok().into())
// }

async fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data">
                <input type="text" name="description" />
                <input type="file" multiple name="image"/>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::fs::create_dir_all("./tmp")?;

    HttpServer::new(|| {
        App::new()
            .app_data(extractor::MultipartFormConfig::default().file_limit(25 * 1024 * 1024))
            .wrap(middleware::Logger::default()).service(
                web::resource("/")
                .route(web::get().to(index))
                .route(web::post().to(save_file))
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
