use std::marker::PhantomData;

#[derive(Default, Clone)]
pub struct RequestBuilder<Url, Method, S> {
    url: Url,
    method: Method,
    body: Option<String>,
    headers: Vec<(String, String)>,
    marker_seal: PhantomData<S>,
}

#[derive(Default, Clone)]
pub struct MissingUrl;

#[derive(Default, Clone)]
pub struct HasUrl(String);

#[derive(Default, Clone)]
pub struct MissingMethod;

#[derive(Default, Clone)]
pub struct HasMethod(String);

#[derive(Default, Clone)]
pub struct MissingSeal;

pub struct HasSeal;

impl RequestBuilder<MissingUrl, MissingMethod, MissingSeal> {
    pub fn new() -> Self {
        RequestBuilder::default()
    }
}

impl RequestBuilder<HasUrl, HasMethod, HasSeal> {
    pub fn build(self) -> Request {
        Request {
            url: self.url.0,
            method: self.method.0,
            body: self.body,
            header: self.headers,
        }
    }
}

impl RequestBuilder<HasUrl, HasMethod, MissingSeal> {
    // [url, method, body, header] methods are not implemented for any type of Url and Method with HasSeal
    pub fn seal(self) -> RequestBuilder<HasUrl, HasMethod, HasSeal> {
        RequestBuilder {
            url: self.url,
            method: self.method,
            body: self.body,
            headers: self.headers,
            marker_seal: PhantomData,
        }
    }
}

// Any type of Url and Method with specific seal type MissingSeal
impl<Url, Method> RequestBuilder<Url, Method, MissingSeal> {
    pub fn url(self, url: impl Into<String>) -> RequestBuilder<HasUrl, Method, MissingSeal> {
        RequestBuilder {
            url: HasUrl(url.into()),
            method: self.method,
            body: self.body,
            headers: self.headers,
            marker_seal: PhantomData,
        }
    }

    pub fn method(self, method: impl Into<String>) -> RequestBuilder<Url, HasMethod, MissingSeal> {
        RequestBuilder {
            url: self.url,
            method: HasMethod(method.into()),
            body: self.body,
            headers: self.headers,
            marker_seal: PhantomData,
        }
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body.replace(body.into());
        self
    }

    pub fn header(mut self, header: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((header.into(), value.into()));
        self
    }
}

pub struct Request {
    pub url: String,
    pub method: String,
    pub body: Option<String>,
    pub header: Vec<(String, String)>,
}

#[cfg(test)]
mod tests {
    use crate::type_state_builder_pattern::RequestBuilder;

    #[test]
    fn build_request_success() {
        let req_builder = RequestBuilder::new()
            .method("GET")
            .url("url")
            .body("body")
            .header("xxx-user-agent", "something")
            .seal()
            .build();

        assert_eq!(req_builder.url, "url");
    }
}
