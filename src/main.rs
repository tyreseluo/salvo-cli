#![recursion_limit = "256"]

use anyhow::Result;
use clap::Parser;
mod tests;
mod utils;
use i18n::set_locale;
mod git;
mod i18n;
mod namer;
mod printer;
mod project;
mod templates;
mod updater;
mod visual_initializer;

rust_i18n::i18n!("locales", fallback = "en");
#[derive(Parser, Debug)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = "Fankai liu <liufankai137@outlook.com>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    New(NewCmd),
    #[clap(name = "visual-initializer")]
    VisualInitializer(VisualInitializerCmd),
}
#[derive(Parser, Debug, Clone)]
pub struct NewCmd {
    pub project_name: String,
    #[clap(short, long)]
    lang: Option<String>,
}

#[derive(Parser, Debug, Clone)]
pub struct VisualInitializerCmd {
    #[clap(long, default_value_t = 7878)]
    port: u16,
}
#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub lang: String,
}
#[tokio::main]
async fn main() -> Result<()> {
    printer::print_logo();
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::New(NewCmd { project_name, lang }) => {
            set_locale(&lang);
            let proj = Project {
                name: project_name,
                lang: lang.unwrap_or("en".to_string()),
            };
            updater::check_for_updates().await;
            match project::create(&proj) {
                Ok(_) => (),
                Err(e) => printer::error(e.to_string()),
            };
        }
        SubCommand::VisualInitializer(VisualInitializerCmd { port }) => {
            visual_initializer::run(port).await?;
        }
    }
    Ok(())
}
