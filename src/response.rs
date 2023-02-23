use reqwest::{header::HeaderMap, Method, Response, StatusCode, Url};

#[derive(Debug, Clone)]
pub struct AngryResponse {
    url: Url,
    status: StatusCode,
    method: Method,
    text: String,
    content_length: u64,
    line_count: usize,
    word_count: usize,
    headers: HeaderMap,
}

impl Default for AngryResponse {
    fn default() -> Self {
        Self {
            url: Url::parse("http://localhost").unwrap(),
            status: Default::default(),
            method: Method::default(),
            text: "".to_string(),
            content_length: 0,
            line_count: 0,
            word_count: 0,
            headers: Default::default(),
        }
    }
}

impl AngryResponse {
    pub fn status(&self) -> &StatusCode {
        &self.status
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn content_length(&self) -> u64 {
        self.content_length
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = String::from(text);
        self.content_length = self.text.len() as u64;
        self.line_count = self.text.lines().count();
        self.word_count = self
            .text
            .lines()
            .map(|s| s.split_whitespace().count())
            .sum();
    }

    pub fn drop_text(&mut self) {
        self.text = String::new();
    }

    pub fn line_count(&self) -> usize {
        self.line_count
    }

    pub fn word_count(&self) -> usize {
        self.word_count
    }

    pub async fn from(response: Response, method: &str) -> Self {
        let url = response.url().clone();
        let status = response.status();
        let headers = response.headers().clone();
        let content_length = response.content_length().unwrap_or(0);

        let text = response.text().await.unwrap_or_default();

        let content_length = content_length.max(text.len() as u64);

        let line_count = text.lines().count();
        let word_count: usize = text.lines().map(|s| s.split_whitespace().count()).sum();

        AngryResponse {
            url,
            status,
            headers,
            content_length,
            line_count,
            word_count,
            text,
            method: Method::from_bytes(method.as_bytes()).unwrap_or(Method::GET),
        }
    }
}
