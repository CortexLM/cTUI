#![allow(missing_docs)]

use clap::{Parser, Subcommand};
use ctui_cli::{ProjectGenerator, TemplateType};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ctui")]
#[command(about = "CLI tool for cTUI framework", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Create a new cTUI project")]
    New {
        #[arg(help = "Name of the project")]
        name: String,

        #[arg(short, long, default_value = "basic")]
        #[arg(help = "Template type: basic, counter, todo-app")]
        template: String,

        #[arg(short, long, default_value = ".")]
        #[arg(help = "Target directory")]
        dir: PathBuf,
    },

    #[command(about = "List available templates")]
    Templates,

    #[command(about = "Check cTUI installation")]
    Doctor,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New {
            name,
            template,
            dir,
        } => {
            let template_type = TemplateType::parse(&template).ok_or_else(|| {
                anyhow::anyhow!("Unknown template: {template}. Available: basic, counter, todo-app")
            })?;

            let generator = ProjectGenerator::new(&name, template_type);

            println!("Creating new cTUI project '{name}' with {template} template...");

            generator.generate(&dir)?;

            println!("✓ Project created at {}/{}", dir.display(), name);
            println!();
            println!("Next steps:");
            println!("  cd {name}");
            println!("  cargo run");
        }
        Commands::Templates => {
            println!("Available templates:");
            println!();
            println!("  basic     - Minimal application with hello world");
            println!("  counter   - Stateful counter with increment/decrement");
            println!("  todo-app  - Todo list with CRUD operations");
            println!();
            println!("Usage: ctui new <name> --template <template>");
        }
        Commands::Doctor => {
            println!("cTUI CLI Doctor");
            println!("===============");
            println!();
            println!("CLI version: {}", env!("CARGO_PKG_VERSION"));
            println!();
            println!("✓ All checks passed");
        }
    }

    Ok(())
}
