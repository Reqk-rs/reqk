#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use tokio::sync::mpsc;

use crate::events::Events;

mod events;

slint::include_modules!();

#[tokio::main(worker_threads = 1)]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = AppWindow::new()?;
    let (tx, mut rx) = mpsc::unbounded_channel::<Events>();

    let app_weak = app.as_weak();
    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            if let Err(error) = event.exec(app_weak.clone()).await {
                eprintln!("Event execution failed: {}", error)
            }
        }
    });

    let tx_req = tx.clone();
    app.on_req(move || {
        let _ = tx_req.send(Events::Req);
    });

    app.run()?;

    Ok(())
}
