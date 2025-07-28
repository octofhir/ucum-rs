use clap::{Parser, Subcommand};
use octofhir_ucum_core::{
    analyse, evaluate, get_canonical_units, is_comparable, parse_expression, precision::to_f64,
    search_units, validate,
};

#[derive(Parser, Debug)]
#[command(
    name = "octofhir-ucum",
    version,
    about = "UCUM command-line tools",
    disable_help_subcommand = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Validate a UCUM expression for correctness.
    Validate {
        /// UCUM expression to validate
        expression: String,
    },
    /// Analyze a UCUM expression and show its properties.
    Analyze {
        /// UCUM expression to analyze
        expression: String,
    },
    /// Get the canonical (base SI units) form of an expression.
    Canonical {
        /// UCUM expression to convert to canonical form
        expression: String,
    },
    /// Convert a value between two units.
    Convert {
        /// Input value
        value: f64,
        /// Source unit expression
        from: String,
        /// Destination unit expression
        to: String,
    },
    /// Check if two units are comparable (same dimension).
    Comparable {
        /// First unit expression
        unit1: String,
        /// Second unit expression
        unit2: String,
    },
    /// Search for units by name or code.
    Search {
        /// Search query
        query: String,
        /// Maximum number of results to show
        #[arg(short, long, default_value_t = 10)]
        limit: usize,
    },
}

fn main() -> anyhow::Result<()> {
    human_panic::setup_panic!();
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate { expression } => handle_validate(&expression)?,
        Commands::Analyze { expression } => handle_analyze(&expression)?,
        Commands::Canonical { expression } => handle_canonical(&expression)?,
        Commands::Convert { value, from, to } => handle_convert(value, from, to)?,
        Commands::Comparable { unit1, unit2 } => handle_comparable(&unit1, &unit2)?,
        Commands::Search { query, limit } => handle_search(&query, limit)?,
    }
    Ok(())
}

fn handle_validate(expr: &str) -> anyhow::Result<()> {
    match validate(expr) {
        Ok(()) => println!("✅ Valid UCUM expression: {}", expr),
        Err(e) => {
            println!("❌ Error: {}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}

fn handle_analyze(expr: &str) -> anyhow::Result<()> {
    match analyse(expr) {
        Ok(analysis) => {
            println!("📊 Unit Analysis: {}", analysis.expression);
            println!("   Factor: {:.6}", analysis.factor);
            println!("   Dimension: {:?}", analysis.dimension);
            println!("   Offset: {:.6}", analysis.offset);
            println!("   Dimensionless: {}", analysis.is_dimensionless);
            println!("   Has offset: {}", analysis.has_offset);
        }
        Err(e) => {
            println!("❌ Error: {}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}

fn handle_canonical(expr: &str) -> anyhow::Result<()> {
    match get_canonical_units(expr) {
        Ok(canonical) => {
            println!("📐 Canonical form of '{}':", expr);
            println!("   Unit: {}", canonical.unit);
            println!("   Factor: {:.6}", canonical.factor);
            if canonical.offset != 0.0 {
                println!("   Offset: {:.6}", canonical.offset);
            }
            println!("   Dimension: {:?}", canonical.dimension);
        }
        Err(e) => {
            println!("❌ Error: {}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}

fn handle_convert(value: f64, from: String, to: String) -> anyhow::Result<()> {
    let from_expr = parse_expression(&from)?;
    let to_expr = parse_expression(&to)?;

    let from_res = evaluate(&from_expr)?;
    let to_res = evaluate(&to_expr)?;

    if from_res.dim != to_res.dim {
        anyhow::bail!("Incompatible dimensions between '{}' and '{}'", from, to);
    }

    let canonical = value * to_f64(from_res.factor) + to_f64(from_res.offset);
    let result = (canonical - to_f64(to_res.offset)) / to_f64(to_res.factor);
    println!("{} {} = {} {}", value, from, result, to);
    Ok(())
}

fn handle_comparable(unit1: &str, unit2: &str) -> anyhow::Result<()> {
    match is_comparable(unit1, unit2) {
        Ok(comparable) => {
            if comparable {
                println!(
                    "✅ '{}' and '{}' are comparable (can be converted)",
                    unit1, unit2
                );
            } else {
                println!(
                    "❌ '{}' and '{}' are NOT comparable (different dimensions)",
                    unit1, unit2
                );
            }
        }
        Err(e) => {
            println!("❌ Error: {}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}

fn handle_search(query: &str, limit: usize) -> anyhow::Result<()> {
    let results = search_units(query);

    if results.is_empty() {
        println!("No units found matching '{}'", query);
        return Ok(());
    }

    println!("🔍 Search results for '{}':", query);
    println!("================================\n");

    for (i, unit) in results.iter().take(limit).enumerate() {
        println!(
            "{}. {} - {} [{}]",
            i + 1,
            unit.code,
            unit.display_name,
            unit.property
        );
    }

    println!(
        "\nShowing {} of {} results",
        results.len().min(limit),
        results.len()
    );
    if results.len() > limit {
        println!("Use --limit to see more results");
    }

    Ok(())
}
