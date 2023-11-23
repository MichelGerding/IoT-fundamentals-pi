use std::path::PathBuf;
use actix_files as fs;
use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::{get, HttpRequest, Result, Error};


#[get("/")]
pub(crate) async fn index() -> Result<fs::NamedFile> {
    let path: PathBuf = PathBuf::from("./src/pages/index.html");
    Ok(fs::NamedFile::open(path)?)
}


#[get("/history")]
pub(crate) async fn history() -> Result<fs::NamedFile> {
    let path: PathBuf = PathBuf::from("./src/pages/history.html");
    Ok(fs::NamedFile::open(path)?)
}

#[get("/config")]
pub(crate) async fn config() -> Result<fs::NamedFile> {
    let path: PathBuf = PathBuf::from("./src/pages/config.html");
    Ok(fs::NamedFile::open(path)?)
}




/// Static files handler
///
/// This handler uses `actix_files` crate to serve static files.
/// this will only store file in the pages/static folder
#[get("/static/{filename:.*}")]
async fn static_files(req: HttpRequest) -> Result<fs::NamedFile, Error> {
    // get the filepath
    let filename = req.match_info().query("filename");
    let path: PathBuf = PathBuf::from(format!("./src/pages/static/{}", filename));

    // serve the file, based on the path
    let file = fs::NamedFile::open(path)?;
    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
}