use std::error::Error;

use reqwest::Client;

use crate::{AppWindow, slint_generatedAppWindow};

pub enum Events {
    Request(slint_generatedAppWindow::Request),
}

impl Events {
    pub async fn exec(&self, app_weak: slint::Weak<AppWindow>) -> Result<(), Box<dyn Error>> {
        match self {
            Events::Request(request) => {
                let client = Client::new();

                let method = match request.method {
                    slint_generatedAppWindow::MethodHttp::GET => reqwest::Method::GET,
                    slint_generatedAppWindow::MethodHttp::POST => reqwest::Method::POST,
                };

                let response = client
                    .request(method, request.url.to_string())
                    .send()
                    .await?;
                let body = response.text().await?;

                slint::invoke_from_event_loop(move || {
                    if let Some(app) = app_weak.upgrade() {
                        app.set_response(slint_generatedAppWindow::Response { body: body.into() });
                    }
                })?;

                Ok(())
            }
        }
    }
}
