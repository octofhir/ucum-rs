use clap::CommandFactory;
use clap::{Parser, Subcommand, ValueEnum};
use dialoguer::{Confirm, FuzzySelect, Input, Select, theme::ColorfulTheme};
use octofhir_ucum_core::{
    EvalResult,
    analyse,
    evaluate,
    get_all_units,
    get_canonical_units,
    get_defined_forms,
    is_comparable,
    parse_expression,
    search_units,
    search_units_by_property,
    search_units_fuzzy,
    search_units_regex,
    unit_divide,
    unit_multiply,
    // New ADR-001 API functions
    validate,
    validate_in_property,
};
use std::io;

/// Command‐line interface for UCUM utilities.
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
    /// Validate that an expression is syntactically and semantically correct.
    Validate {
        /// UCUM expression to validate
        expression: String,
        /// Show detailed analysis information
        #[arg(short, long)]
        detailed: bool,
    },
    /// Analyze a UCUM expression and show comprehensive information.
    Analyze {
        /// UCUM expression to analyze
        expression: String,
        /// Output as JSON instead of human-readable format
        #[arg(short, long)]
        json: bool,
    },
    /// Validate that a unit is appropriate for a given property.
    ValidateProperty {
        /// UCUM expression to validate
        expression: String,
        /// Physical property (e.g., length, mass, time, force, energy)
        property: String,
    },
    /// Check if two units are comparable (can be converted between each other).
    Comparable {
        /// First unit expression
        unit1: String,
        /// Second unit expression
        unit2: String,
    },
    /// Get the canonical (base) form of a unit expression.
    Canonical {
        /// UCUM expression to convert to canonical form
        expression: String,
    },
    /// Search for units by text, property, or pattern.
    Search {
        /// Search query
        query: String,
        /// Search by property instead of text
        #[arg(short, long)]
        property: bool,
        /// Use fuzzy matching
        #[arg(short, long)]
        fuzzy: bool,
        /// Fuzzy matching threshold (0-100, higher is more strict)
        #[arg(short, long, default_value_t = 50)]
        threshold: i64,
        /// Use regular expression matching
        #[arg(short, long)]
        regex: bool,
        /// Case-sensitive matching (for regex)
        #[arg(short, long)]
        case_sensitive: bool,
        /// Maximum number of results to show
        #[arg(short, long, default_value_t = 20)]
        limit: usize,
    },
    /// Interactive unit search with fuzzy selection.
    Interactive {
        /// Start with a specific property filter
        #[arg(short, long)]
        property: Option<String>,
    },
    /// Show all defined forms of a unit (base unit and prefixed variants).
    Forms {
        /// Base unit code (e.g., 'g' for gram)
        base_code: String,
    },
    /// Print the parsed Abstract Syntax Tree of an expression.
    Ast {
        /// Expression to parse
        expression: String,
        /// Output as JSON instead of debug format
        #[arg(short, long)]
        json: bool,
    },
    /// Generate shell completion scripts for supported shells.
    Completions {
        /// Target shell
        #[arg(value_enum)]
        shell: Shell,
    },
    /// Convert a numeric value between two units.
    Convert {
        /// Input value
        value: f64,
        /// Source unit expression
        from: String,
        /// Destination unit expression
        to: String,
    },
    /// List all supported UCUM units with their properties.
    ListUnits {
        /// Filter units by name (case-insensitive substring match)
        #[arg(short, long)]
        filter: Option<String>,
        /// Filter by property
        #[arg(short, long)]
        property: Option<String>,
        /// Maximum number of units to show
        #[arg(short, long, default_value_t = 50)]
        limit: usize,
    },
    /// Perform arithmetic operations on unit expressions.
    Multiply {
        /// First unit expression
        unit1: String,
        /// Second unit expression
        unit2: String,
    },
    /// Divide unit expressions.
    Divide {
        /// Numerator unit expression
        numerator: String,
        /// Denominator unit expression
        denominator: String,
    },
    /// Show information about a unit code.
    Explain {
        /// UCUM unit code to explain
        code: String,
    },
}

#[derive(Clone, ValueEnum, Debug)]
enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Elvish,
}

fn main() -> anyhow::Result<()> {
    human_panic::setup_panic!();

    let cli = Cli::parse();

    match cli.command {
        Commands::Validate {
            expression,
            detailed,
        } => handle_validate(&expression, detailed)?,
        Commands::Analyze { expression, json } => handle_analyze(&expression, json)?,
        Commands::ValidateProperty {
            expression,
            property,
        } => handle_validate_property(&expression, &property)?,
        Commands::Comparable { unit1, unit2 } => handle_comparable(&unit1, &unit2)?,
        Commands::Canonical { expression } => handle_canonical(&expression)?,
        Commands::Search {
            query,
            property,
            fuzzy,
            threshold,
            regex,
            case_sensitive,
            limit,
        } => handle_search(
            &query,
            property,
            fuzzy,
            threshold,
            regex,
            case_sensitive,
            limit,
        )?,
        Commands::Interactive { property } => handle_interactive_search(property.as_deref())?,
        Commands::Forms { base_code } => handle_forms(&base_code)?,
        Commands::Ast { expression, json } => handle_ast(&expression, json)?,
        Commands::Convert { value, from, to } => handle_convert(value, from, to)?,
        Commands::ListUnits {
            filter,
            property,
            limit,
        } => handle_list_units(filter.as_deref(), property.as_deref(), limit)?,
        Commands::Multiply { unit1, unit2 } => handle_multiply(&unit1, &unit2)?,
        Commands::Divide {
            numerator,
            denominator,
        } => handle_divide(&numerator, &denominator)?,
        Commands::Completions { shell } => emit_completions(shell),
        Commands::Explain { code } => handle_explain(&code)?,
    }
    Ok(())
}

fn parse_and_eval(expr: impl AsRef<str>) -> anyhow::Result<EvalResult> {
    let expr = expr.as_ref();
    let ast = parse_expression(expr).map_err(anyhow::Error::msg)?;
    let res = evaluate(&ast).map_err(anyhow::Error::msg)?;
    Ok(res)
}

fn handle_validate(expr: &str, detailed: bool) -> anyhow::Result<()> {
    match validate(expr) {
        Ok(()) => {
            if detailed {
                match analyse(expr) {
                    Ok(analysis) => {
                        println!("✅ Valid UCUM expression: {}", expr);
                        println!("   Factor: {:.6}", analysis.factor);
                        println!("   Dimension: {:?}", analysis.dimension);
                        if analysis.has_offset {
                            println!("   Offset: {:.6}", analysis.offset);
                        }
                        if analysis.is_dimensionless {
                            println!("   Type: Dimensionless");
                        }
                    }
                    Err(e) => {
                        println!("✅ Valid (but analysis failed: {})", e);
                    }
                }
            } else {
                println!("✅ Valid");
            }
        }
        Err(e) => {
            println!("❌ Invalid: {}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}

fn handle_ast(expr: &str, json: bool) -> anyhow::Result<()> {
    let ast = parse_expression(expr).map_err(anyhow::Error::msg)?;
    if json {
        println!("JSON serialization not available – using debug output");
        println!("{:#?}", ast);
    } else {
        println!("{:#?}", ast);
    }
    Ok(())
}

fn canonical_value(value: f64, res: &EvalResult) -> f64 {
    value * res.factor + res.offset
}

fn handle_convert(value: f64, from: String, to: String) -> anyhow::Result<()> {
    let from_str = from.as_str();
    let to_str = to.as_str();

    let from_res = parse_and_eval(&from)?;
    let to_res = parse_and_eval(&to)?;

    if from_res.dim != to_res.dim {
        anyhow::bail!(
            "Incompatible dimensions between '{}' and '{}'",
            from_str,
            to_str
        );
    }

    let canonical = canonical_value(value, &from_res);
    let result = (canonical - to_res.offset) / to_res.factor;
    println!("{} {} = {} {}", value, from_str, result, to_str);
    Ok(())
}

fn handle_explain(code: &str) -> anyhow::Result<()> {
    use octofhir_ucum_core::find_unit;

    // First try direct unit lookup
    if let Some(unit) = find_unit(code) {
        println!("Unit code:      {}", unit.code);
        println!("Display name:   {}", unit.display_name);
        println!("Dimension:      {}", unit.dim);
        println!("Factor:         {}", unit.factor);
        println!("Offset:         {}", unit.offset);
        println!("Special kind:   {:?}", unit.special);
        println!("Property:       {}", unit.property);
        return Ok(());
    }

    // If direct lookup fails, try parsing as an expression (handles prefixed units like "mg")
    match parse_and_eval(code) {
        Ok(result) => {
            println!("Unit expression: {}", code);
            println!("Dimension:       {}", result.dim);
            println!("Factor:          {}", result.factor);
            println!("Offset:          {}", result.offset);
            println!("Note:            This is a compound/prefixed unit");
        }
        Err(_) => {
            anyhow::bail!("Unit '{}' not found", code);
        }
    }
    Ok(())
}

fn handle_list_units(
    filter: Option<&str>,
    property: Option<&str>,
    limit: usize,
) -> anyhow::Result<()> {
    let units = if let Some(prop) = property {
        search_units_by_property(prop)
    } else {
        get_all_units().iter().collect()
    };

    println!("Supported UCUM Units:");
    println!("====================\n");

    let mut count = 0;
    for unit in units {
        if count >= limit {
            println!(
                "... (showing first {} units, use --limit to see more)",
                limit
            );
            break;
        }

        // Apply text filter if provided
        if let Some(filter_str) = filter {
            if !unit
                .code
                .to_lowercase()
                .contains(&filter_str.to_lowercase())
                && !unit
                    .display_name
                    .to_lowercase()
                    .contains(&filter_str.to_lowercase())
                && !unit
                    .property
                    .to_lowercase()
                    .contains(&filter_str.to_lowercase())
            {
                continue;
            }
        }

        let special = if unit.offset != 0.0 {
            format!(" (offset: {})", unit.offset)
        } else {
            String::new()
        };

        println!(
            "{:<15} = {:<30} [{}] {}{}",
            unit.code,
            unit.display_name,
            unit.property,
            if unit.factor != 1.0 {
                format!("{} ", unit.factor)
            } else {
                "".to_string()
            },
            special
        );
        count += 1;
    }

    println!("\nShowing {} units", count);
    if property.is_some() {
        println!("Filtered by property: {}", property.unwrap());
    }
    if filter.is_some() {
        println!("Filtered by text: {}", filter.unwrap());
    }

    Ok(())
}

fn handle_analyze(expr: &str, json: bool) -> anyhow::Result<()> {
    match analyse(expr) {
        Ok(analysis) => {
            if json {
                // JSON output for UnitAnalysis - using manual serialization for now
                println!("{{");
                println!("  \"expression\": \"{}\",", analysis.expression);
                println!("  \"factor\": {},", analysis.factor);
                println!("  \"offset\": {},", analysis.offset);
                println!("  \"dimension\": {:?},", analysis.dimension.0);
                println!("  \"is_dimensionless\": {},", analysis.is_dimensionless);
                println!("  \"has_offset\": {}", analysis.has_offset);
                println!("}}");
            } else {
                println!("📊 Unit Analysis: {}", analysis.expression);
                println!("   Factor: {:.6}", analysis.factor);
                println!("   Dimension: {:?}", analysis.dimension);
                println!("   Offset: {:.6}", analysis.offset);
                println!("   Dimensionless: {}", analysis.is_dimensionless);
                println!("   Has offset: {}", analysis.has_offset);
            }
        }
        Err(e) => {
            println!("❌ Analysis failed: {}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}

fn handle_validate_property(expr: &str, property: &str) -> anyhow::Result<()> {
    match validate_in_property(expr, property) {
        Ok(is_valid) => {
            if is_valid {
                println!("✅ '{}' is valid for property '{}'", expr, property);
            } else {
                println!("❌ '{}' is NOT valid for property '{}'", expr, property);
                std::process::exit(1);
            }
        }
        Err(e) => {
            println!("❌ Property validation failed: {}", e);
            std::process::exit(1);
        }
    }
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
            println!("❌ Comparison failed: {}", e);
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
            println!("❌ Canonical conversion failed: {}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}

fn handle_search(
    query: &str,
    property: bool,
    fuzzy: bool,
    threshold: i64,
    regex: bool,
    case_sensitive: bool,
    limit: usize,
) -> anyhow::Result<()> {
    let results = if regex {
        match search_units_regex(query, case_sensitive) {
            Ok(units) => units.into_iter().map(|u| (u, 100i64)).collect(),
            Err(e) => {
                println!("❌ Regex search failed: {}", e);
                std::process::exit(1);
            }
        }
    } else if fuzzy {
        search_units_fuzzy(query, threshold)
    } else if property {
        search_units_by_property(query)
            .into_iter()
            .map(|u| (u, 100i64))
            .collect()
    } else {
        search_units(query)
            .into_iter()
            .map(|u| (u, 100i64))
            .collect()
    };

    if results.is_empty() {
        println!("No units found matching '{}'", query);
        return Ok(());
    }

    println!("🔍 Search results for '{}':", query);
    println!("================================\n");

    for (i, (unit, score)) in results.iter().take(limit).enumerate() {
        if fuzzy {
            println!(
                "{}. {} - {} [{}] (score: {})",
                i + 1,
                unit.code,
                unit.display_name,
                unit.property,
                score
            );
        } else {
            println!(
                "{}. {} - {} [{}]",
                i + 1,
                unit.code,
                unit.display_name,
                unit.property
            );
        }
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

fn handle_forms(base_code: &str) -> anyhow::Result<()> {
    let forms = get_defined_forms(base_code);

    if forms.is_empty() {
        println!("No defined forms found for '{}'", base_code);
        return Ok(());
    }

    println!("📋 Defined forms of '{}':", base_code);
    println!("========================\n");

    for (i, unit) in forms.iter().enumerate() {
        println!("{}. {} - {}", i + 1, unit.code, unit.display_name);
        if unit.factor != 1.0 {
            println!("   Factor: {}", unit.factor);
        }
    }

    println!("\nFound {} forms", forms.len());
    Ok(())
}

fn handle_multiply(unit1: &str, unit2: &str) -> anyhow::Result<()> {
    match unit_multiply(unit1, unit2) {
        Ok(result) => {
            println!("🧮 Unit Multiplication:");
            println!("   {} × {} = {}", unit1, unit2, result.expression);
            println!("   Factor: {:.6}", result.factor);
            println!("   Dimension: {:?}", result.dimension);
            if result.is_dimensionless {
                println!("   Result: Dimensionless");
            }
        }
        Err(e) => {
            println!("❌ Multiplication failed: {}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}

fn handle_divide(numerator: &str, denominator: &str) -> anyhow::Result<()> {
    match unit_divide(numerator, denominator) {
        Ok(result) => {
            println!("🧮 Unit Division:");
            println!("   {} ÷ {} = {}", numerator, denominator, result.expression);
            println!("   Factor: {:.6}", result.factor);
            println!("   Dimension: {:?}", result.dimension);
            if result.is_dimensionless {
                println!("   Result: Dimensionless");
            }
        }
        Err(e) => {
            println!("❌ Division failed: {}", e);
            std::process::exit(1);
        }
    }
    Ok(())
}

fn handle_interactive_search(property_filter: Option<&str>) -> anyhow::Result<()> {
    let theme = ColorfulTheme::default();

    println!("🔍 Interactive UCUM Unit Search");
    println!("================================\n");

    // Get all units or filter by property
    let units: Vec<_> = if let Some(prop) = property_filter {
        search_units_by_property(prop)
    } else {
        get_all_units().iter().collect()
    };

    if units.is_empty() {
        println!("No units found for the specified criteria.");
        return Ok(());
    }

    // Create display items for fuzzy selection
    let items: Vec<String> = units
        .iter()
        .map(|unit| format!("{} - {} [{}]", unit.code, unit.display_name, unit.property))
        .collect();

    // Interactive search options
    loop {
        println!("Choose an action:");
        let actions = vec![
            "🔍 Search units by fuzzy matching",
            "📋 Browse units by property",
            "🧮 Interactive unit conversion",
            "❓ Get help with unit validation",
            "🚪 Exit",
        ];

        let selection = Select::with_theme(&theme)
            .with_prompt("What would you like to do?")
            .items(&actions)
            .default(0)
            .interact()?;

        match selection {
            0 => {
                // Fuzzy search
                println!("\n🔍 Fuzzy Unit Search");
                println!("Type to search through {} units:", units.len());

                let selection = FuzzySelect::with_theme(&theme)
                    .with_prompt("Select a unit")
                    .items(&items)
                    .default(0)
                    .interact_opt()?;

                if let Some(index) = selection {
                    let selected_unit = units[index];
                    display_unit_details(selected_unit)?;

                    // Ask if user wants to perform actions on this unit
                    if Confirm::with_theme(&theme)
                        .with_prompt("Would you like to analyze this unit?")
                        .default(true)
                        .interact()?
                    {
                        match analyse(&selected_unit.code) {
                            Ok(analysis) => {
                                println!("\n📊 Unit Analysis:");
                                println!("   Expression: {}", analysis.expression);
                                println!("   Factor: {:.6}", analysis.factor);
                                println!("   Dimension: {:?}", analysis.dimension);
                                if analysis.has_offset {
                                    println!("   Offset: {:.6}", analysis.offset);
                                }
                                if analysis.is_dimensionless {
                                    println!("   Type: Dimensionless");
                                }
                            }
                            Err(e) => println!("❌ Analysis failed: {}", e),
                        }
                    }
                }
            }
            1 => {
                // Browse by property
                let properties: Vec<String> = {
                    let mut props: Vec<String> = get_all_units()
                        .iter()
                        .map(|u| u.property.to_string())
                        .collect::<std::collections::HashSet<_>>()
                        .into_iter()
                        .collect();
                    props.sort();
                    props
                };

                let prop_selection = Select::with_theme(&theme)
                    .with_prompt("Select a property")
                    .items(&properties)
                    .interact()?;

                let selected_property = &properties[prop_selection];
                let property_units = search_units_by_property(selected_property);

                println!("\n📋 Units with property '{}':", selected_property);
                for (i, unit) in property_units.iter().take(10).enumerate() {
                    println!("  {}. {} - {}", i + 1, unit.code, unit.display_name);
                }

                if property_units.len() > 10 {
                    println!("  ... and {} more units", property_units.len() - 10);
                }
            }
            2 => {
                // Interactive conversion
                println!("\n🧮 Interactive Unit Conversion");

                let from_unit: String = Input::with_theme(&theme)
                    .with_prompt("Enter source unit")
                    .interact_text()?;

                let to_unit: String = Input::with_theme(&theme)
                    .with_prompt("Enter target unit")
                    .interact_text()?;

                let value: f64 = Input::with_theme(&theme)
                    .with_prompt("Enter value to convert")
                    .interact_text()?;

                // Perform conversion
                match (parse_and_eval(&from_unit), parse_and_eval(&to_unit)) {
                    (Ok(from_res), Ok(to_res)) => {
                        if from_res.dim != to_res.dim {
                            println!(
                                "❌ Incompatible dimensions between '{}' and '{}'",
                                from_unit, to_unit
                            );
                        } else {
                            let canonical = canonical_value(value, &from_res);
                            let result = (canonical - to_res.offset) / to_res.factor;
                            println!("✅ {} {} = {} {}", value, from_unit, result, to_unit);
                        }
                    }
                    _ => println!("❌ Invalid unit expression(s)"),
                }
            }
            3 => {
                // Help with validation
                println!("\n❓ Unit Validation Help");

                let unit_input: String = Input::with_theme(&theme)
                    .with_prompt("Enter a unit expression to validate")
                    .interact_text()?;

                match validate(&unit_input) {
                    Ok(()) => {
                        println!("✅ '{}' is a valid UCUM expression", unit_input);

                        if let Ok(analysis) = analyse(&unit_input) {
                            println!("   Factor: {:.6}", analysis.factor);
                            println!("   Dimension: {:?}", analysis.dimension);
                        }
                    }
                    Err(e) => {
                        println!("❌ '{}' is invalid: {}", unit_input, e);

                        // Suggest similar units
                        let suggestions = search_units_fuzzy(&unit_input, 30);
                        if !suggestions.is_empty() {
                            println!("\n💡 Did you mean one of these?");
                            for (unit, score) in suggestions.iter().take(5) {
                                println!(
                                    "   {} - {} (similarity: {}%)",
                                    unit.code, unit.display_name, score
                                );
                            }
                        }
                    }
                }
            }
            4 => {
                println!("👋 Goodbye!");
                break;
            }
            _ => unreachable!(),
        }

        println!(); // Add spacing
    }

    Ok(())
}

fn display_unit_details(unit: &octofhir_ucum_core::UnitRecord) -> anyhow::Result<()> {
    println!("\n📋 Unit Details:");
    println!("   Code: {}", unit.code);
    println!("   Name: {}", unit.display_name);
    println!("   Property: {}", unit.property);
    println!("   Factor: {}", unit.factor);
    if unit.offset != 0.0 {
        println!("   Offset: {}", unit.offset);
    }
    println!("   Dimension: {:?}", unit.dim);
    Ok(())
}

fn emit_completions(shell: Shell) {
    use clap_complete::{generate, shells};
    let mut cmd = Cli::command();
    let name = cmd.get_name().to_string();
    match shell {
        Shell::Bash => generate(shells::Bash, &mut cmd, &name, &mut io::stdout()),
        Shell::Zsh => generate(shells::Zsh, &mut cmd, &name, &mut io::stdout()),
        Shell::Fish => generate(shells::Fish, &mut cmd, &name, &mut io::stdout()),
        Shell::PowerShell => generate(shells::PowerShell, &mut cmd, &name, &mut io::stdout()),
        Shell::Elvish => generate(shells::Elvish, &mut cmd, &name, &mut io::stdout()),
    }
}
