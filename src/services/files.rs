use std::path::{PathBuf, Path};

use rocket::fs::NamedFile;

#[get("/<file..>")]
pub async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}
