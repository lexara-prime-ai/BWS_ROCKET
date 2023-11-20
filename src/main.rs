use rocket::fs::{relative, FileServer};

// Serve files manually
mod manual {
    use rocket::fs::NamedFile;
    use std::path::{Path, PathBuf};

    #[rocket::get("/second/<path..>")]
    pub async fn other(path: PathBuf) -> Option<NamedFile> {
        let mut path = Path::new(super::relative!("static")).join(path);
        if path.is_dir() {
            path.push("index.html");
        }
        NamedFile::open(path).await.ok()
    }
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", rocket::routes![manual::other])
}
