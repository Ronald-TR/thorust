use anyhow::Result;
use clap::{Parser, Subcommand};
use thorust::{
    parser::parse,
    runner::Runner,
    traits::{GraphWorkflow, RunnerWorkflow},
    workflow::Workflow,
};

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
        .with_level(false)
        .with_target(false)
        .compact();
    tracing_subscriber::fmt().event_format(format).init();

    let args = ThorustCmd::parse();

    match &args.command {
        Commands::Run { file } => {
            let manifest = parse(&file).unwrap();
            let workflow = Workflow::new(&manifest).unwrap();
            let mut runner = Runner::new(workflow);
            runner.run_until_complete().await?;
        }
        Commands::Dot { file } => {
            let manifest = parse(&file).unwrap();
            let workflow = Workflow::new(&manifest).unwrap();
            workflow.print_dot();
        }
    }
    Ok(())
}
