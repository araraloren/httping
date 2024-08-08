use std::io::stdout;

mod app;

use app::App;
use cote::prelude::*;
use httping::Itdog;
use httping::Ui;
use tracing_subscriber::filter::LevelFilter;

#[derive(Debug, Cote)]
#[cote(aborthelp, width = 100)]
struct Cli {
    /// Print debug message
    #[arg(alias = "-d")]
    pub debug: bool,

    /// Print verbose debug message
    #[arg(alias = "-v")]
    pub verbose: bool,

    /// Log the debug message to the file
    #[arg(alias = "-l")]
    pub log: Option<String>,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let Cli {
        debug,
        verbose,
        log,
    } = Cli::parse_env()?;

    if let Some(path) = log {
        let file = std::fs::File::create(path)?;
        let subscriber = tracing_subscriber::fmt::fmt()
            .with_writer(file)
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_filter_reloading();
        let reload_handler = subscriber.reload_handle();

        subscriber.init();

        if verbose {
            reload_handler.modify(|filter| {
                *filter = tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive(LevelFilter::TRACE.into());
            })?;
        } else if debug {
            reload_handler.modify(|filter| {
                *filter = tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive(LevelFilter::DEBUG.into());
            })?;
        }
    } else {
        let subscriber = tracing_subscriber::fmt::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_filter_reloading();
        let reload_handler = subscriber.reload_handle();

        subscriber.init();

        if verbose {
            reload_handler.modify(|filter| {
                *filter = tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive(LevelFilter::TRACE.into());
            })?;
        } else if debug {
            reload_handler.modify(|filter| {
                *filter = tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive(LevelFilter::DEBUG.into());
            })?;
        }
    }

    let mut ui = Ui::new(stdout())?;
    let mut app = App::default().with_server(Itdog);

    ui.run_loop(&mut app, App::view, App::update, App::handler)?;

    Ok(())
}
