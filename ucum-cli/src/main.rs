use clap::{Parser, Subcommand};
use ucum_core::{parser::UcumParser, registry::UcumRegistry};

#[derive(Parser)]
#[command(name = "octofhir-ucum")]
#[command(about = "UCUM (Unified Code for Units of Measure) command-line interface")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate a UCUM expression
    Validate {
        /// The UCUM expression to validate
        expression: String,
    },
    /// Get AST for a UCUM expression
    Ast {
        /// The UCUM expression to get AST for
        expression: String,
        /// Output format (json, debug)
        #[arg(short, long, default_value = "debug")]
        format: String,
    },
    /// List available units
    List {
        /// Filter units by pattern
        #[arg(short, long)]
        filter: Option<String>,
    },
    /// Convert between units
    Convert {
        /// Source value and unit (e.g., "100 mg")
        from: String,
        /// Target unit
        to: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Validate { expression } => {
            let parser = UcumParser::new();
            match parser.parse(expression) {
                Ok(_) => {
                    println!("✓ Valid UCUM expression: {}", expression);
                    std::process::exit(0);
                }
                Err(e) => {
                    eprintln!("✗ Invalid UCUM expression: {}", expression);
                    eprintln!("  Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Ast { expression, format } => {
            let parser = UcumParser::new();
            match parser.parse(expression) {
                Ok(ast) => {
                    match format.as_str() {
                        "json" => {
                            match serde_json::to_string_pretty(&ast) {
                                Ok(json) => println!("{}", json),
                                Err(e) => eprintln!("Error serializing to JSON: {}", e),
                            }
                        }
                        "debug" => {
                            println!("AST for '{}':", expression);
                            println!("{:#?}", ast);
                        }
                        _ => {
                            eprintln!("Unknown format: {}. Use 'json' or 'debug'", format);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("✗ Failed to parse UCUM expression: {}", expression);
                    eprintln!("  Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::List { filter } => {
            let _registry = UcumRegistry::new();
            // TODO: Implement unit listing with filter
            println!("Listing units... (filter: {:?})", filter);
        }
        Commands::Convert { from, to } => {
            // TODO: Implement unit conversion
            println!("Converting {} to {}", from, to);
        }
    }

    Ok(())
} 