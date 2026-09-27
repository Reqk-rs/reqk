use std::error::Error;

pub enum Events {
    Req,
}

impl Events {
    pub async fn exec(
        &self,
        app_weak: slint::Weak<crate::AppWindow>,
    ) -> Result<(), Box<dyn Error>> {
        match self {
            Events::Req => {
                let res = reqwest::get("https://jsonplaceholder.typicode.com/todos/1")
                    .await?
                    .text()
                    .await?;

                slint::invoke_from_event_loop(move || {
                    if let Some(app) = app_weak.upgrade() {
                        app.set_name(res.into())
                    }
                })?;

                Ok(())
            }
        }
    }
}
