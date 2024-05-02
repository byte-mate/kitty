use std::fs;
use poise::{serenity_prelude as serenity};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use axum::{ Router};
use axum::routing::{get};
use poise::serenity_prelude::{CacheHttp };
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;

mod commands {
    pub mod fun {
        pub mod cat;
        pub mod cookie;

    }

    pub mod utils {
        pub mod help;
    }
}

use commands::fun::cat::cat;
use commands::fun::cookie::cookie;
use commands::utils::help::help;
#[derive(Deserialize)]
struct Config {
    token: String,
}



struct Data {
    has_started: AtomicBool,
    poise: Arc<sqlx::PgPool>,
}
type BotError = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, BotError>;




#[tokio::main]
async fn main() {
    let config_contents= fs::read_to_string("config.yaml").expect("Failed to read config file");
    let config: Config = serde_yaml::from_str(&config_contents).expect("Failed to parse config file");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            event_handler: |framework, event| Box::pin(event_handler(framework, event)),
            commands: vec![
                cookie(),
                cat(),
                help(),
            ],
            ..Default::default()
        })
        .build();



    let client = serenity::ClientBuilder::new(&*config.token, serenity::GatewayIntents::all())
        .framework(framework)
        .activity(poise::serenity_prelude::ActivityData::listening("your commands"))
        .data(Arc::new(Data {
            has_started: AtomicBool::new(false),
            poise: Arc::new(
                PgPoolOptions::new()
                    .max_connections(5)
                    .connect("postgres://postgres:postgres@localhost:5432/postgres")
                    .await.expect("Failed to connect to database")
            )
        }) as _)
        .await;

    let mut client_unwrapped = client.unwrap();

    let ctx = client_unwrapped.cache.clone();
    let _db = Arc::clone(&client_unwrapped.data::<Data>().poise);

    tokio::task::spawn(async move {

        let app = Router::new()
            .route("/", get(|| async { "test" }))
            .route("/api/__botguilds", get(move || async move {
                let guilds = ctx.guilds();
                let mut guilds_ids = Vec::new();
                for guild in guilds {
                    guilds_ids.push(guild);
                }
                serde_json::to_string(&guilds_ids).unwrap()
            }));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:2137").await.unwrap();

        println!("Listening on: {}", listener.local_addr().unwrap());

        axum::serve(listener, app).await.unwrap();
    });


    client_unwrapped.start().await.unwrap();

}

async fn event_handler(
    framework: poise::FrameworkContext<'_, Data, BotError>,
    event: &serenity::FullEvent,
) -> Result<(), BotError> {
    let data = framework.user_data();
    let ctx = framework.serenity_context;


    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            let has_started = &data.has_started;

            if !has_started.load(std::sync::atomic::Ordering::Relaxed) {
                let commands = &framework.options().commands;
                poise::builtins::register_globally(ctx.http(), commands).await?;
                println!("Successfully registered slash commands!");
                has_started.store(true, std::sync::atomic::Ordering::Relaxed);
                println!("Logged in as {}", data_about_bot.user.name);
            }

        }
        _ => {}
    }
    Ok(())
}