use salvo::{prelude::*, http::form::FilePart};
use std::path::Path;
use uuid::Uuid;
use super::query::{SimpleResponse, ErrorMessage};


fn handle_file_copy(file: &FilePart, dest: String, filename: String) -> Result<SimpleResponse, ErrorMessage> {
    match std::fs::copy(&file.path(), Path::new(&dest)) {
        Ok(_) => Ok(SimpleResponse { message: filename}),
        Err(e) => Err(ErrorMessage { error: e.to_string()}),
    }
}

#[handler]
pub async fn upload(req: &mut Request, res: &mut Response) {
    let file = req.file("file").await;
    if let Some(file) = file {
        let id = Uuid::new_v4();
        let filename = format!("{id}.{}", file.name().unwrap());
        let dest = format!("data/images/{}", filename);
        match handle_file_copy(file, dest, filename) {
            Ok(msg) => {
                res.status_code(StatusCode::CREATED);
                res.render(Json(msg))
            },
            Err(e) => {
                res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                res.render(Json(e))
            }
        }
    } else {
        res.status_code(StatusCode::BAD_REQUEST);
        res.render(Json(ErrorMessage{error: String::from("File not found")}));
    };
}
