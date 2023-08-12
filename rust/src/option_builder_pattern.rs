#[derive(Default)]
pub struct RequestBuilder {
    url: Option<String>,
    method: Option<String>,
    body: Option<String>,
    headers: Vec<(String, String)>,
}

impl RequestBuilder {
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

    pub fn header(mut self, header: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((header.into(), value.into()));
        self
    }

    pub fn build(self) -> Result<Request, BuildError> {
        if self.url.is_none() {
            return Err(BuildError::MissingUrl);
        }

        if self.method.is_none() {
            return Err(BuildError::MissingMethod);
        }

        Ok(Request {
            url: self.url.unwrap(),
            method: self.method.unwrap(),
            body: self.body,
            headers: self.headers,
        })
    }
}

pub struct Request {
    pub url: String,
    pub method: String,
    pub body: Option<String>,
    pub headers: Vec<(String, String)>,
}

#[derive(PartialEq, Debug)]
pub enum BuildError {
    MissingUrl,
    MissingMethod,
}

#[cfg(test)]
mod tests {
    use crate::option_builder_pattern::{BuildError, RequestBuilder};

    #[test]
    fn missing_url() {
        let req_builder = RequestBuilder::default().body(String::from("body")).build();

        assert_eq!(req_builder.err().unwrap(), BuildError::MissingUrl);
    }

    #[test]
    fn missing_method() {
        let req_builder = RequestBuilder::default().url(String::from("url")).build();

        assert_eq!(req_builder.err().unwrap(), BuildError::MissingMethod);
    }

    #[test]
    fn build_request_success() {
        let req_builder = RequestBuilder::default()
            .url("url")
            .method("GET")
            .body("body")
            .header("xxx-user-agent", "something")
            .build();

        assert!(req_builder.is_ok());
    }
}
