use seed::prelude::*;
use seed::fetch::Result;
use serde::{Deserialize, Serialize};

use seed_rocket_diesel_template::student::*;

const API_URL: &str = "http://localhost:8000";

pub async fn get_students() -> Result<Vec<Student>> {
    fetch(format!("{}/student", API_URL))
        .await?
        .json()
        .await
}

pub async fn post_student(item: NewStudent) -> Result<Response> {
    fetch::Request::new(format!("{}/student", API_URL))
        .method(Method::Post)
        .json(&item)?
        .fetch()
        .await
}

pub async fn put_student(item: Student) -> Result<Response> {
    fetch::Request::new(format!("{}/student", API_URL))
        .method(Method::Put)
        .json(&item)?
        .fetch()
        .await
}

pub async fn delete_student(item: NewStudent) -> Result<Response> {
    fetch::Request::new(format!("{}/student", API_URL))
        .method(Method::Delete)
        .json(&item)?
        .fetch()
        .await
}
