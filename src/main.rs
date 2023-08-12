use win_boiler_attr::start_now;

mod bot;
mod service;
mod settings;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let result = tokio::spawn(async move {
        let mut bot = bot::get_client().await.unwrap();
        bot.start().await.unwrap();
    });
    worker()?;
    result.await?;
    Ok(())
}

/// worker function to run while bot is running 
#[start_now("amos")]
fn worker() -> anyhow::Result<()> {
    log::info!("Service started");
    loop{}
    Ok(())
}

