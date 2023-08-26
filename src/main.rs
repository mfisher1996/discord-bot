use std::env::{set_current_dir, args};

use win_boiler_attr::start_now;

mod bot;
mod service;
mod settings;
mod commands;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    set_current_dir(args().nth(1).unwrap_or(".".to_string())).unwrap();
    let result = tokio::spawn(async move {
        let mut bot = bot::get_client().await.unwrap();
        bot.start().await.unwrap();
    });
    worker();
    result.await?;
    Ok(())
}


#[start_now("amos")]
fn worker() -> anyhow::Result<()> {
    log::info!("Service started");
    Ok(())
}

