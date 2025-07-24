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
    validate,
    validate_in_property,
    precision::to_f64,
    // Model Introspection API
    get_model,
    validate_ucum,
    get_properties,
    validate_canonical_units,
    get_common_display,
    // Advanced Conversion API
    convert_with_context,
    AdvancedConversionContext,
    DecimalPrecision,
    RoundingMode,
    TemperatureScale,
    // Performance Optimizations
    find_unit_optimized,
    get_cache_stats,
    clear_global_cache,
    get_cache_sizes,
    // Enhanced Error Handling
    UcumError,
    ErrorKind,
    SuggestionEngine,
    // Extended Functionality
    optimize_expression,
    canonicalize_expression,
    simplify_expression,
    MeasurementContext,
    Domain,
};
use std::io;

/// Display an enhanced error message with suggestions and context information
fn display_enhanced_error(error: &UcumError) {
    // Main error message
    println!("❌ Error: {}", error.message);
    
    // Show error kind for context
    match &error.kind {
        ErrorKind::UnitNotFound { unit, similar } => {
            println!("   📍 Unknown unit: '{}'", unit);
            if !similar.is_empty() {
                println!("   📍 Similar units found: {}", similar.join(", "));
            }
        },
        ErrorKind::ConversionError { from, to, reason } => {
            println!("   📍 Conversion from '{}' to '{}': {}", from, to, reason);
        },
        ErrorKind::DimensionMismatch { expected, found, operation } => {
            println!("   📍 Dimension mismatch in {}: expected {:?}, found {:?}", operation, expected, found);
        },
        ErrorKind::ParseError { expected, found } => {
            println!("   📍 Parse error: expected '{}', found '{}'", expected, found);
        },
        ErrorKind::InvalidExpression { reason } => {
            println!("   📍 Invalid expression: {}", reason);
        },
        ErrorKind::InvalidPercentPlacement { position } => {
            println!("   📍 Invalid percent placement at position {}", position);
        },
        ErrorKind::PrecisionOverflow { operation, value } => {
            println!("   📍 Precision overflow in {}: value '{}'", operation, value);
        },
        ErrorKind::InvalidProperty { property, available } => {
            println!("   📍 Invalid property: '{}', available: {}", property, available.join(", "));
        },
        ErrorKind::MultipleSlash => {
            println!("   📍 Multiple slash operators detected");
        },
        ErrorKind::SpecialUnitError { unit, reason } => {
            println!("   📍 Special unit error for '{}': {}", unit, reason);
        },
    }
    
    // Show source location if available
    if let Some(span) = &error.span {
        println!("   📍 Location: characters {}-{} in '{}'", span.start, span.end, span.source);
    }
    
    // Show suggestions
    if !error.suggestions.is_empty() {
        println!("\n💡 Suggestions:");
        for suggestion in &error.suggestions {
            println!("   • {}", suggestion);
        }
    }
    
    // Show additional context
    if !error.context.is_empty() {
        println!("\n📝 Additional context:");
        for context in &error.context {
            println!("   • {}", context);
        }
    }
}

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
    /// Show UCUM model information and metadata.
    Model {
        /// Show detailed model information
        #[arg(short, long)]
        detailed: bool,
    },
    /// Validate the UCUM implementation for self-consistency.
    SelfValidate,
    /// List all available properties in the UCUM model.
    Properties,
    /// Validate canonical unit forms.
    ValidateCanonical {
        /// Unit expression to validate
        unit: String,
        /// Expected canonical form
        canonical: String,
    },
    /// Get display name for a unit code.
    Display {
        /// Unit code to get display name for
        code: String,
    },
    /// Advanced unit conversion with precision control.
    ConvertAdvanced {
        /// Input value
        value: f64,
        /// Source unit expression
        from: String,
        /// Destination unit expression
        to: String,
        /// Number of decimal places for fixed precision
        #[arg(short, long)]
        precision: Option<u32>,
        /// Number of significant figures
        #[arg(short, long)]
        significant: Option<u32>,
        /// Rounding mode: nearest, up, down, truncate
        #[arg(short, long, default_value = "nearest")]
        rounding: String,
        /// Temperature scale: kelvin, celsius, fahrenheit
        #[arg(short, long, default_value = "kelvin")]
        temperature_scale: String,
    },
    /// Show performance cache statistics and metrics.
    CacheStats {
        /// Clear cache before showing stats
        #[arg(short, long)]
        clear: bool,
    },
    /// Performance benchmark comparing optimized vs standard lookup methods.
    Benchmark {
        /// Number of iterations to run
        #[arg(short, long, default_value_t = 10000)]
        iterations: usize,
        /// Unit codes to test (comma-separated)
        #[arg(short, long, default_value = "kg,mg,g,m,cm,mm,s,min,h")]
        units: String,
    },
    /// Optimize a unit expression for better readability.
    Optimize {
        /// Unit expression to optimize
        expression: String,
    },
    /// Convert a unit expression to its canonical (base units) form.
    Simplify {
        /// Unit expression to simplify
        expression: String,
    },
    /// Create and explore measurement contexts for different domains.
    Context {
        /// Domain type: medical, engineering, physics, chemistry, general
        #[arg(short, long, default_value = "general")]  
        domain: String,
        /// Show context details
        #[arg(short, long)]
        details: bool,
        /// Get unit suggestions for the given context
        #[arg(short, long)]
        suggest: Option<String>,
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
        Commands::Model { detailed } => handle_model(detailed)?,
        Commands::SelfValidate => handle_self_validate()?,
        Commands::Properties => handle_properties()?,
        Commands::ValidateCanonical { unit, canonical } => {
            handle_validate_canonical(&unit, &canonical)?
        }
        Commands::Display { code } => handle_display(&code)?,
        Commands::ConvertAdvanced {
            value,
            from,
            to,
            precision,
            significant,
            rounding,
            temperature_scale,
        } => handle_convert_advanced(
            value,
            from,
            to,
            precision,
            significant,
            &rounding,
            &temperature_scale,
        )?,
        Commands::CacheStats { clear } => handle_cache_stats(clear)?,
        Commands::Benchmark { iterations, units } => handle_benchmark(iterations, &units)?,
        Commands::Optimize { expression } => handle_optimize(&expression)?,
        Commands::Simplify { expression } => handle_simplify(&expression)?,
        Commands::Context { domain, details, suggest } => handle_context(&domain, details, suggest.as_deref())?,
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
            display_enhanced_error(&e);
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
    value * to_f64(res.factor) + to_f64(res.offset)
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
    let result = (canonical - to_f64(to_res.offset)) / to_f64(to_res.factor);
    println!("{} {} = {} {}", value, from_str, result, to_str);
    Ok(())
}

fn handle_explain(code: &str) -> anyhow::Result<()> {
    use octofhir_ucum_core::find_unit;

    // First try direct unit lookup
    if let Some(unit) = find_unit(code) {
        println!("Unit code:      {}", unit.code);
        println!("Display name:   {}", unit.display_name);
        println!("Common display: {}", get_common_display(code));
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
            println!("Common display:  {}", get_common_display(code));
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
            display_enhanced_error(&e);
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
            display_enhanced_error(&e);
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
            display_enhanced_error(&e);
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
            display_enhanced_error(&e);
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
            display_enhanced_error(&e);
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
            display_enhanced_error(&e);
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
                            let result = (canonical - to_f64(to_res.offset)) / to_f64(to_res.factor);
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
                        display_enhanced_error(&e);

                        // Show additional fuzzy suggestions if error doesn't already contain suggestions
                        if e.suggestions.is_empty() {
                            let suggestions = search_units_fuzzy(&unit_input, 30);
                            if !suggestions.is_empty() {
                                println!("\n🔍 Alternative units with similar names:");
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

// Model Introspection Commands

fn handle_model(detailed: bool) -> anyhow::Result<()> {
    let model = get_model();
    
    println!("📚 UCUM Model Information");
    println!("========================");
    println!("Version: {}", model.version);
    println!("Revision Date: {}", model.revision_date);
    println!("Total Units: {}", model.units.len());
    println!("Total Prefixes: {}", model.prefixes.len());
    
    if detailed {
        println!("\n🔧 Prefixes:");
        for (i, prefix) in model.prefixes.iter().take(10).enumerate() {
            println!("  {}. {} ({}) - factor: {}", i + 1, prefix.symbol, prefix.display_name, prefix.factor);
        }
        if model.prefixes.len() > 10 {
            println!("  ... and {} more prefixes", model.prefixes.len() - 10);
        }
        
        println!("\n📊 Unit Statistics by Property:");
        let mut property_counts = std::collections::HashMap::new();
        for unit in model.units {
            *property_counts.entry(&unit.property).or_insert(0) += 1;
        }
        let mut sorted_props: Vec<_> = property_counts.iter().collect();
        sorted_props.sort_by(|a, b| b.1.cmp(a.1));
        
        for (property, count) in sorted_props.iter().take(15) {
            println!("  {}: {}", property, count);
        }
        if sorted_props.len() > 15 {
            println!("  ... and {} more properties", sorted_props.len() - 15);
        }
    }
    
    Ok(())
}

fn handle_self_validate() -> anyhow::Result<()> {
    println!("🔍 Validating UCUM Implementation...");
    
    let issues = validate_ucum();
    
    if issues.is_empty() {
        println!("✅ UCUM implementation is valid - no issues found!");
    } else {
        println!("⚠️  Found {} validation issues:", issues.len());
        for (i, issue) in issues.iter().enumerate() {
            println!("  {}. {}", i + 1, issue);
        }
        
        if issues.len() > 20 {
            println!("  ... and {} more issues", issues.len() - 20);
        }
    }
    
    Ok(())
}

fn handle_properties() -> anyhow::Result<()> {
    let properties = get_properties();
    
    println!("📋 Available UCUM Properties");
    println!("============================");
    println!("Total properties: {}\n", properties.len());
    
    let mut sorted_props: Vec<_> = properties.iter().collect();
    sorted_props.sort();
    
    for (i, property) in sorted_props.iter().enumerate() {
        println!("{}. {}", i + 1, property);
    }
    
    Ok(())
}

fn handle_validate_canonical(unit: &str, canonical: &str) -> anyhow::Result<()> {
    match validate_canonical_units(unit, canonical) {
        Ok(is_valid) => {
            if is_valid {
                println!("✅ '{}' has canonical form '{}'", unit, canonical);
            } else {
                // Show what the actual canonical form is
                match get_canonical_units(unit) {
                    Ok(actual_canonical) => {
                        println!("❌ '{}' does NOT have canonical form '{}'", unit, canonical);
                        println!("   Actual canonical form: '{}'", actual_canonical.unit);
                        println!("   Factor: {:.6}", actual_canonical.factor);
                        if actual_canonical.offset != 0.0 {
                            println!("   Offset: {:.6}", actual_canonical.offset);
                        }
                    }
                    Err(e) => {
                        println!("❌ Cannot determine canonical form: {}", e);
                    }
                }
                std::process::exit(1);
            }
        }
        Err(e) => {
            display_enhanced_error(&e);
            std::process::exit(1);
        }
    }
    Ok(())
}

fn handle_display(code: &str) -> anyhow::Result<()> {
    let display_name = get_common_display(code);
    println!("📝 Display name for '{}': {}", code, display_name);
    Ok(())
}

fn handle_convert_advanced(
    value: f64,
    from: String,
    to: String,
    precision: Option<u32>,
    significant: Option<u32>,
    rounding: &str,
    temperature_scale: &str,
) -> anyhow::Result<()> {
    // Parse precision configuration
    let decimal_precision = match (precision, significant) {
        (Some(places), None) => DecimalPrecision::Fixed(places),
        (None, Some(sig_figs)) => DecimalPrecision::Significant(sig_figs),
        (None, None) => DecimalPrecision::Default,
        (Some(_), Some(_)) => {
            anyhow::bail!("Cannot specify both precision and significant figures");
        }
    };
    
    // Parse rounding mode
    let rounding_mode = match rounding.to_lowercase().as_str() {
        "nearest" => RoundingMode::Nearest,
        "up" => RoundingMode::Up,
        "down" => RoundingMode::Down,
        "truncate" => RoundingMode::Truncate,
        _ => anyhow::bail!("Invalid rounding mode: {}. Use: nearest, up, down, truncate", rounding),
    };
    
    // Parse temperature scale
    let temp_scale = match temperature_scale.to_lowercase().as_str() {
        "kelvin" => TemperatureScale::Kelvin,
        "celsius" => TemperatureScale::Celsius,
        "fahrenheit" => TemperatureScale::Fahrenheit,
        _ => anyhow::bail!("Invalid temperature scale: {}. Use: kelvin, celsius, fahrenheit", temperature_scale),
    };
    
    // Create conversion context
    let context = AdvancedConversionContext {
        precision: decimal_precision,
        rounding: rounding_mode,
        temperature_scale: temp_scale,
        use_special_units: true,
    };
    
    // Perform conversion
    match convert_with_context(value, &from, &to, &context) {
        Ok(result) => {
            println!("🔄 Advanced Conversion:");
            println!("   {} {} = {} {}", value, from, result.value, result.unit);
            println!("   Factor: {:.6}", result.factor);
            if result.offset != 0.0 {
                println!("   Offset: {:.6}", result.offset);
            }
            println!("   Precision: {}", result.precision_info);
            if result.used_special_units {
                println!("   Special unit processing: Yes");
            }
        }
        Err(e) => {
            display_enhanced_error(&e);
            std::process::exit(1);
        }
    }
    
    Ok(())
}

// Performance Commands

fn handle_cache_stats(clear: bool) -> anyhow::Result<()> {
    if clear {
        clear_global_cache().map_err(anyhow::Error::msg)?;
        println!("🧹 Cache cleared");
    }
    
    match get_cache_stats() {
        Ok(stats) => {
            println!("📊 Performance Cache Statistics");
            println!("==============================");
            println!("Expression hits: {}", stats.expression_hits);
            println!("Expression misses: {}", stats.expression_misses);
            println!("Expression hit ratio: {:.2}%", stats.expression_hit_ratio() * 100.0);
            println!();
            println!("Conversion hits: {}", stats.conversion_hits);
            println!("Conversion misses: {}", stats.conversion_misses);
            println!("Conversion hit ratio: {:.2}%", stats.conversion_hit_ratio() * 100.0);
            println!();
            println!("Dimension hits: {}", stats.dimension_hits);
            println!("Dimension misses: {}", stats.dimension_misses);
            println!();
            println!("Overall hit ratio: {:.2}%", stats.overall_hit_ratio() * 100.0);
            
            // Show cache sizes
            if let Ok((expr_cache, conv_cache, dim_cache)) = get_cache_sizes() {
                println!();
                println!("Cache Sizes:");
                println!("  Expressions: {}", expr_cache);
                println!("  Conversions: {}", conv_cache);
                println!("  Dimensions: {}", dim_cache);
            }
        }
        Err(e) => {
            println!("❌ Failed to get cache statistics: {}", e);
        }
    }
    
    Ok(())
}

fn handle_benchmark(iterations: usize, units_str: &str) -> anyhow::Result<()> {
    println!("🏃 Performance Benchmark");
    println!("========================");
    println!("Iterations: {}", iterations);
    
    let units: Vec<&str> = units_str.split(',').map(|s| s.trim()).collect();
    println!("Testing units: {:?}", units);
    println!();
    
    // Clear cache for clean benchmark
    clear_global_cache().map_err(anyhow::Error::msg)?;
    
    // Benchmark standard unit lookup
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        for unit in &units {
            let _ = octofhir_ucum_core::find_unit(unit);
        }
    }
    let standard_time = start.elapsed();
    
    // Benchmark optimized unit lookup
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        for unit in &units {
            let _ = find_unit_optimized(unit);
        }
    }
    let optimized_time = start.elapsed();
    
    // Benchmark expression evaluation with caching
    let expressions: Result<Vec<_>, _> = units.iter()
        .map(|u| parse_expression(u))
        .collect();
    
    match expressions {
        Ok(exprs) => {
            // First run - populate cache
            let start = std::time::Instant::now();
            for _ in 0..iterations {
                for expr in &exprs {
                    let _ = evaluate(expr);
                }
            }
            let first_eval_time = start.elapsed();
            
            // Second run - use cache
            let start = std::time::Instant::now();
            for _ in 0..iterations {
                for expr in &exprs {
                    let _ = evaluate(expr);
                }
            }
            let cached_eval_time = start.elapsed();
            
            // Display results
            println!("📈 Benchmark Results:");
            println!("--------------------");
            
            let total_ops = iterations * units.len();
            println!("Standard unit lookup ({} ops): {:?}", total_ops, standard_time);
            println!("Optimized unit lookup ({} ops): {:?}", total_ops, optimized_time);
            
            if optimized_time < standard_time {
                let speedup = standard_time.as_nanos() as f64 / optimized_time.as_nanos() as f64;
                println!("✅ Optimized lookup is {:.2}x faster", speedup);
            } else {
                println!("⚠️  Optimized lookup is not faster (small dataset effect)");
            }
            
            println!();
            println!("Expression evaluation (first run): {:?}", first_eval_time);
            println!("Expression evaluation (cached): {:?}", cached_eval_time);
            
            if cached_eval_time < first_eval_time {
                let speedup = first_eval_time.as_nanos() as f64 / cached_eval_time.as_nanos() as f64;
                println!("✅ Cached evaluation is {:.2}x faster", speedup);
            }
            
            // Show final cache stats
            if let Ok(stats) = get_cache_stats() {
                println!();
                println!("📊 Final Cache Statistics:");
                println!("Expression hit ratio: {:.2}%", stats.expression_hit_ratio() * 100.0);
                println!("Total cache hits: {}", stats.expression_hits + stats.conversion_hits + stats.dimension_hits);
            }
        }
        Err(e) => {
            println!("❌ Failed to parse test expressions: {}", e);
        }
    }
    
    Ok(())
}

// Extended Functionality Commands

fn handle_optimize(expr: &str) -> anyhow::Result<()> {
    match optimize_expression(expr) {
        Ok(optimized) => {
            println!("⚡ Expression Optimization:");
            println!("   Original: {}", expr);
            println!("   Optimized: {}", optimized);
            
            // Show canonical form for comparison
            if let Ok(canonical) = canonicalize_expression(expr) {
                println!("   Canonical: {}", canonical);
            }
        }
        Err(e) => {
            display_enhanced_error(&e);
            std::process::exit(1);
        }
    }
    Ok(())
}

fn handle_simplify(expr: &str) -> anyhow::Result<()> {
    match simplify_expression(expr) {
        Ok(simplified) => {
            println!("🔧 Expression Simplification:");
            println!("   Original: {}", expr);
            println!("   Simplified: {}", simplified);
            
            // Show analysis to verify equivalence
            if let (Ok(original_analysis), Ok(simplified_analysis)) = (analyse(expr), analyse(&simplified)) {
                if original_analysis.dimension == simplified_analysis.dimension {
                    println!("   ✅ Dimensions preserved: {:?}", original_analysis.dimension);
                    if (original_analysis.factor - simplified_analysis.factor).abs() < 1e-10 {
                        println!("   ✅ Conversion factor preserved: {:.6}", original_analysis.factor);
                    }
                } else {
                    println!("   ⚠️  Dimensions changed - this may indicate an issue");
                }
            }
        }
        Err(e) => {
            display_enhanced_error(&e);
            std::process::exit(1);
        }
    }
    Ok(())
}

fn handle_context(domain_str: &str, show_details: bool, suggest_unit: Option<&str>) -> anyhow::Result<()> {
    // Parse domain string
    let context = match domain_str.to_lowercase().as_str() {
        "medical" => MeasurementContext::medical(),
        "engineering" => MeasurementContext::engineering(),
        "physics" => MeasurementContext::physics(),
        "chemistry" => MeasurementContext::chemistry(),
        "general" => MeasurementContext::default(),
        _ => {
            println!("❌ Invalid domain: {}. Use: medical, engineering, physics, chemistry, general", domain_str);
            std::process::exit(1);
        }
    };
    
    println!("🏗️  Measurement Context: {:?}", context.domain);
    println!("=======================");
    
    if show_details {
        println!("Precision Requirements:");
        println!("  Min Significant Figures: {}", context.precision_requirements.min_significant_figures);
        println!("  Max Relative Error: {:.0e}", context.precision_requirements.max_relative_error);
        println!("  Require Exact: {}", context.precision_requirements.require_exact);
        println!();
        
        println!("Preferred Units:");
        if context.preferred_units.is_empty() {
            println!("  (none specified)");
        } else {
            for (i, unit) in context.preferred_units.iter().enumerate() {
                println!("  {}. {}", i + 1, unit);
                if i >= 9 { // Limit to first 10
                    println!("  ... and {} more", context.preferred_units.len() - 10);
                    break;
                }
            }
        }
        println!();
        
        println!("Avoided Units:");
        if context.avoided_units.is_empty() {
            println!("  (none specified)");
        } else {
            for unit in &context.avoided_units {
                println!("  • {}", unit);
            }
        }
        println!();
    }
    
    // Handle unit suggestions
    if let Some(unit) = suggest_unit {
        println!("Unit Analysis for '{}':", unit);
        
        // Check if preferred
        if context.is_preferred_unit(unit) {
            println!("  ✅ This unit is preferred in the {} domain", domain_str);
        } else if context.is_avoided_unit(unit) {
            println!("  ❌ This unit should be avoided in the {} domain", domain_str);
        } else {
            println!("  ℹ️  This unit is neither preferred nor avoided in the {} domain", domain_str);
        }
        
        // Get alternatives
        match context.suggest_alternatives(unit) {
            Ok(alternatives) => {
                if !alternatives.is_empty() {
                    println!("  💡 Suggested alternatives:");
                    for alt in alternatives.iter().take(5) {
                        println!("     • {}", alt);
                    }
                } else {
                    println!("  📝 No specific alternatives found for this unit");
                }
            }
            Err(e) => {
                println!("  ❌ Error getting alternatives: {}", e);
            }
        }
    }
    
    Ok(())
}
