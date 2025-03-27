use clap::Parser;
use dotenvy::dotenv;

#[derive(Parser, Debug)]
pub enum CommandEnum {
    #[clap(about = "Refresh App Cache")]
    AppRefresh {
        // 定义 days 参数，默认值为 3
        #[clap(long, default_value_t = 3)]
        days: u32,
    },
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CommandArgs {
    #[clap(subcommand)]
    pub name: Option<CommandEnum>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let args = CommandArgs::parse();
    println!("{:?}", args);
    if let Some(command) = args.name {
        match command {
            CommandEnum::AppRefresh { days } => {
                println!("App refresh with {} days", days);
            }
        }
    }
}
