use anyhow::Result;
use clap::{Parser, Subcommand};
use thorust::{
    api::run_server,
    parser::parse,
    runner::Runner,
    traits::{GraphWorkflow, RunnerWorkflow},
    workflow::Workflow,
};
use tracing::Level;

/// Thorust - command line interface
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct ThorustCmd {
    /// Manifest file to read
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        /// Manifest file to read
        #[clap(short, long)]
        file: String,
    },
    Api {
        /// Manifest file to read
        #[clap(short, long)]
        file: String,
    },
    Ui {
        /// Manifest file to read
        #[clap(short, long)]
        file: String,
    },
    /// Prints the Dot graphviz representation of the workflow
    Dot {
        /// Manifest file to read
        #[clap(short, long)]
        file: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let format = tracing_subscriber::fmt::format()
        .with_target(false)
        .compact();
    tracing_subscriber::fmt()
        .event_format(format)
        .with_max_level(Level::INFO)
        .init();
    // resets db to ensure a clean state on every CLI execution
    let _ = std::fs::remove_file("./db");
    let args = ThorustCmd::parse();

    match &args.command {
        Commands::Run { file } => {
            let manifest = parse(&file).unwrap();
            let workflow = Workflow::new(manifest)?;
            let mut runner = Runner::new(workflow)?;
            runner.run_until_complete().await?;
            println!("{}", runner.workflow.read().await.as_json());
        }
        Commands::Api { file } => {
            run_server(&file, false).await?;
        }
        Commands::Ui { file } => {
            run_server(&file, true).await?;
        }
        Commands::Dot { file } => {
            let manifest = parse(&file).unwrap();
            let workflow = Workflow::new(manifest)?;
            println!("{}", workflow.as_dot());
        }
    }
    Ok(())
}
