//! Command-line interface for UCUM operations.
//!
//! This module provides a CLI for validating, analyzing, converting, and searching UCUM units.
//! It's only available when the "cli" feature is enabled.

use clap::{Parser, Subcommand};
use octofhir_ucum::{
    AdvancedConversionContext, Case, analyse, convert_with_context, get_canonical_units,
    is_comparable, search_units, validate_with_case,
};

#[derive(Parser, Debug)]
#[command(
    name = "octofhir-ucum",
    version,
    about = "UCUM command-line tools",
    disable_help_subcommand = true
)]
struct Cli {
    /// Read symbols case insensitively (UCUM §3.4): `PAL` for `Pa`
    #[arg(short = 'i', long, global = true)]
    case_insensitive: bool,
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
    /// Get the canonical form of an expression, in the UCUM base units.
    Canonical {
        /// UCUM expression to convert to canonical form
        expression: String,
    },
    /// Convert a value between two units.
    Convert {
        /// Input value
        #[arg(allow_negative_numbers = true)]
        value: f64,
        /// Source unit expression
        from: String,
        /// Destination unit expression
        to: String,
    },
    /// Check if two units are commensurable.
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

fn main() -> Result<(), anyhow::Error> {
    human_panic::setup_panic!();
    let cli = Cli::parse();

    let case = if cli.case_insensitive {
        Case::Insensitive
    } else {
        Case::Sensitive
    };
    match cli.command {
        Commands::Validate { expression } => handle_validate(&expression, case)?,
        Commands::Analyze { expression } => handle_analyze(&expression)?,
        Commands::Canonical { expression } => handle_canonical(&expression)?,
        Commands::Convert { value, from, to } => handle_convert(value, from, to, case)?,
        Commands::Comparable { unit1, unit2 } => handle_comparable(&unit1, &unit2)?,
        Commands::Search { query, limit } => handle_search(&query, limit)?,
    }
    Ok(())
}

fn handle_validate(expr: &str, case: Case) -> anyhow::Result<()> {
    match validate_with_case(expr, case) {
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
            println!("   Factor: {}", analysis.factor);
            println!("   Dimension: {}", analysis.dimension);
            for (atom, exp) in &analysis.arbitrary {
                println!("   Arbitrary unit: {atom} ^ {exp}");
            }
            if let Some(kind) = analysis.special {
                println!("   Special unit: {kind:?}, scale {}", analysis.scale);
                if analysis.offset != 0.0 {
                    println!("   Offset: {}", analysis.offset);
                }
            }
            println!("   Dimensionless: {}", analysis.is_dimensionless);
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
            println!("   Factor: {}", canonical.factor);
            if canonical.offset != 0.0 {
                println!("   Offset: {}", canonical.offset);
            }
            println!("   Dimension: {}", canonical.dimension);
        }
        Err(e) => {
            println!("❌ Error: {}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}

fn handle_convert(value: f64, from: String, to: String, case: Case) -> anyhow::Result<()> {
    let context = AdvancedConversionContext {
        case,
        ..Default::default()
    };
    let result = convert_with_context(value, &from, &to, &context)?;
    println!("{} {} = {} {}", value, from, result.value, to);
    Ok(())
}

fn handle_comparable(unit1: &str, unit2: &str) -> anyhow::Result<()> {
    match is_comparable(unit1, unit2) {
        Ok(comparable) => {
            if comparable {
                println!(
                    "✅ '{}' and '{}' are commensurable (can be converted)",
                    unit1, unit2
                );
            } else {
                println!("❌ '{}' and '{}' are NOT commensurable", unit1, unit2);
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
