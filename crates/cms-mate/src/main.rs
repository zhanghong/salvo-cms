use clap::Parser;
use dotenvy::dotenv;

mod console;
mod domain;
mod enums;
mod route;
mod service;

pub use enums::CommandEnum;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let args = console::CommandArgs::parse();
    println!("{:?}", args);
    if let Some(command) = args.name {
        match command {
            CommandEnum::AppRefresh  => {
                println!("App refresh");
            },
            _ => {
                println!("Unknown command")
            }
        }
    } else {
        console::server_start().await;
    }
    
}