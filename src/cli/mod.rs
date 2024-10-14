use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about, version)]
pub struct CLI {
    #[command(subcommand)]
    subcommand: NutterCommand
}

impl CLI {
    pub fn get_command(&self) -> NutterCommand {
        self.subcommand.clone()
    }
}

#[derive(Clone, Subcommand)]
pub enum NutterCommand {
    Init,
    #[command(subcommand)]
    Add(NutterAddSubcommand),
    Start
}

#[derive(Clone, Subcommand)]
pub enum NutterAddSubcommand {
    Service{
        name: String,
        #[arg(long)]
        host: String,
        #[arg(long, short)]
        port: u16
    }
}