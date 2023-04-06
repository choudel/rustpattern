use anyhow::{anyhow, Ok, Result};

#[derive(Debug)]
struct Request {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}
#[derive(Default, Clone)]
struct RequestBuilder {
    url: Option<String>,
    method: Option<String>,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

impl RequestBuilder {
    pub fn new() -> Self {
        RequestBuilder::default()
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }
    pub fn build(self) -> Result<Request> {
        let Some(url)=self.url else{
            return Err(anyhow!("No URL"));
        };
        let method = self.method.unwrap_or_else(|| "GET".to_string());

        Ok(Request {
            url,
            method,
            headers: self.headers,
            body: self.body,
        })
    }
}
fn main() -> Result<()> {
    let req_builder = RequestBuilder::new()
        .url("Https://some-url.com/task/123")
        .method("Post")
        .body("roppy");
    let req_builder = req_builder.header("token", "user_uuid.exp.sign");
    let req = req_builder.clone().build()?;
    println!("{req:#?}");
    let req_builder = req_builder.header("Rohy Botter", "Porry lomo");
    let req = req_builder.clone().build()?;
    println!("{req:#?}");
    let req = req_builder.header("poly Bot", "1.6V").build()?;
    println!("{req:#?}");
    Ok(())
}
