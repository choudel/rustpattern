use anyhow::{anyhow, Ok, Result};

#[derive(Debug)]
pub struct Request {
    pub url: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}
#[derive(Default)]
pub struct RequestBuilder {
    pub url: Option<String>,
    pub method: Option<String>,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

impl RequestBuilder {
    pub fn new() -> Self {
        RequestBuilder::default()
    }
    pub fn url(&mut self, url: impl Into<String>) -> &mut Self {
        self.url = Some(url.into());
        self
    }
    pub fn method(&mut self, method: impl Into<String>) -> &mut Self {
        self.method = Some(method.into());
        self
    }
    pub fn body(&mut self, body: impl Into<String>) -> &mut Self {
        self.body = Some(body.into());
        self
    }
    pub fn header(&mut self, name: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.headers.push((name.into(), value.into()));
        self
    }
    pub fn build(&self) -> Result<Request> {
        let Some(url)=self.url.as_ref() else{
            return Err(anyhow!("No URL"));
        };
        let method = self
            .method
            .as_ref()
            .cloned()
            .unwrap_or_else(|| "GET".to_string());

        Ok(Request {
            url: url.to_string(),
            method,
            headers: self.headers.clone(),
            body: self.body.clone(),
        })
    }
}
fn main() -> Result<()> {
    let mut req_builder = RequestBuilder::new();
    req_builder
        .url("Https://some-url.com/task/123")
        .method("Post");
    let req = req_builder.header("token", "user_uuid.exp.sign").build()?;
    println!("{req:#?}");
    req_builder.header("Client-Version", "1.2");
    let req = req_builder.build()?;
    println!("{req:#?}");
    Ok(())
}
