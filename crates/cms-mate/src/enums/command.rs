use clap::Parser;

#[derive(Parser, Debug)]
pub enum CommandEnum {
    #[clap(about = "Refresh App Cache")]
    AppRefresh,
    #[clap(about = "Start Server")]
    ServerUp
}