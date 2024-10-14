use std::env;
use std::fs::{create_dir, File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;
use clap::{Parser};
use dialoguer::{Input, Select};
use dialoguer::theme::ColorfulTheme;
use crate::cli::{NutterAddSubcommand, NutterCommand, CLI};
use anyhow::{Result};
use include_dir::{include_dir, Dir};
use nutt_conf_parser::{NuttConfig, ServiceConfig};
use walkdir::WalkDir;

mod cli;

static MAIN_TEMPLATE_DIR: Dir = include_dir!("templates/services");
static SERVICE_TEMPLATE_DIR: Dir = include_dir!("templates/service_template");


#[tokio::main]
async fn main() -> Result<()> {
    let cli = CLI::parse();
    match cli.get_command() {
        NutterCommand::Init => {
            let project_name: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Project name")
                .default("project".into())
                .interact()?;
            let path: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Project path")
                .default(".".into())
                .interact()?;
            let frontend_options = ["None", "React", "Vue", "Angular", "Next"];
            let curr_fronted = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select frontend framework")
                .default(0)
                .items(&frontend_options)
                .interact()?;

            if !PathBuf::from(&path).exists() {
                create_dir(&path)?;
            }

            let project_path = env::current_dir()?.join(&path);

            let buf_env = env::current_dir()?;

            env::set_current_dir(&project_path)?;

            Command::new("dockit")
                .args(["init", "-p", project_path.to_str().unwrap()])
                .spawn()?;

            Command::new("dockit")
                .args([
                    "add", "service",
                    "--name", "main",
                    "-p", "80:8080",
                    "-b", project_path.join("services").join("main").join("Dockerfile").to_str().unwrap(),
                    "-n", "backend-net"
                ])
                .spawn()?;

            Command::new("dockit")
                .args([
                    "add", "network",
                    "-n", "backend-net",
                    "-d", "bridge"
                ])
                .spawn()?;

            env::set_current_dir(buf_env)?;

            let mut nutt_config = NuttConfig::new(&project_name, "0.0.0");
            nutt_config.push_service_config("main", ServiceConfig::new("127.0.0.1", 8080));
            File::create(project_path.join("nutt.conf.toml"))?.write_all(
                toml::to_string(&nutt_config)?.as_bytes()
            )?;

            MAIN_TEMPLATE_DIR.extract(project_path.join("services"))?;
        }
        NutterCommand::Add(subcommand) => {
            match subcommand {
                NutterAddSubcommand::Service { name, host, port } => {
                    let wd = WalkDir::new(env::current_dir()?);

                    let mut conf_path = PathBuf::default();

                    let mut buf = String::new();

                    for dir in wd {
                        if let Ok(dir) = dir {
                            if dir.file_name().to_str().unwrap() == "nutt.conf.toml" {
                                conf_path = dir.path().to_path_buf();
                                break;
                            }
                        }
                    }

                    let mut file = OpenOptions::new()
                        .read(true)
                        .open(&conf_path)?;
                    file.read_to_string(&mut buf)?;

                    let mut conf = toml::from_str::<NuttConfig>(&buf)?;
                    conf.push_service_config(&name, ServiceConfig::new(&host, port));

                    let mut file = OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open(&conf_path)?;

                    file.write_all(
                        toml::to_string(&conf)?.as_bytes()
                    )?;

                    let services_path = conf_path.parent().unwrap().join("services").join(&name);
                    create_dir(&services_path)?;

                    SERVICE_TEMPLATE_DIR.extract(services_path)?;
                }
            }
        }
        NutterCommand::Start => {
            tokio::task::spawn(async {
                Command::new("docker-compose")
                    .args(["up"])
                    .spawn()
            });
            loop {

            }
        }
    }
    Ok(())
}
