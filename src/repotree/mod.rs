pub mod contents;

use std::sync::{Arc, Mutex};

use serde::de::DeserializeOwned;

pub struct Fetchable<T: DeserializeOwned> {
    pub request: ehttp::Request,
    pub response: Arc<Mutex<Option<Result<T, String>>>>,
}

impl<T: DeserializeOwned+Send+'static> Fetchable<T> {
    pub fn new(request: ehttp::Request) -> Self {
        Self {
            request,
            response: Default::default(),
        }
    }
    pub fn with_headers(self) -> Self {
        let Self {mut request,response} = self;
        request.headers
            .insert("User-Agent", "LexiBigCheese/GithubOn3DS");
        request.headers.insert("Accept", "application/vnd.github.object+json");
        request.headers.insert("X-GitHub-Api-Version", "2022-11-28");
        Self{request,response}
    }
    pub fn perform(&self) {
        let response = self.response.clone();
        ehttp::fetch(self.request.clone(), move |out| {
            let mut response_access = response.lock().expect("Couldn't lock response!");
            *response_access = Some(out.map(|x| x.json().map_err(|e| e.to_string())).flatten());
        });
    }
}
