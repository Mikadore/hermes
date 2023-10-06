use poem_openapi::{param::Path, payload::PlainText, OpenApi};

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/add/:a/:b", method = "get")]
    async fn add(&self, a: Path<u64>, b: Path<u64>) -> PlainText<String> {
        PlainText(a.wrapping_add(*b).to_string())
    }
}