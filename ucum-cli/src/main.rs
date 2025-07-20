use clap::CommandFactory;
use clap::{Parser, Subcommand, ValueEnum};
use std::io;
use ucum_core::{EvalResult, evaluate, find_unit, parse_expression};

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
    },

    /// Perform basic arithmetic (mul/div) on two unit expressions.
    Arithmetic {
        /// Left operand expression
        left: String,
        /// Operation: mul or div
        #[arg(value_enum)]
        op: ArithOp,
        /// Right operand expression
        right: String,
        /// Numeric coefficient (optional – defaults to 1)
        #[arg(short, long, default_value_t = 1.0)]
        value: f64,
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

#[derive(Clone, ValueEnum, Debug)]
enum ArithOp {
    Mul,
    Div,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate { expression } => handle_validate(&expression)?,
        Commands::Ast { expression, json } => handle_ast(&expression, json)?,
        Commands::Convert { value, from, to } => handle_convert(value, from, to)?,
        Commands::ListUnits { filter } => handle_list_units(filter.as_deref())?,
        Commands::Arithmetic {
            left,
            op,
            right,
            value,
        } => handle_arithmetic(left, op, right, value)?,
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

fn handle_validate(expr: &str) -> anyhow::Result<()> {
    parse_and_eval(expr)?;
    println!("valid");
    Ok(())
}

fn handle_ast(expr: &str, json: bool) -> anyhow::Result<()> {
    let ast = parse_expression(expr).map_err(anyhow::Error::msg)?;
    if json {
        #[cfg(feature = "serde")]
        {
            println!("{}", serde_json::to_string_pretty(&ast)?);
        }
        #[cfg(not(feature = "serde"))]
        {
            println!("Feature 'serde' not enabled in ucum-core – falling back to debug output");
            println!("{:#?}", ast);
        }
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

fn handle_arithmetic(left: String, op: ArithOp, right: String, value: f64) -> anyhow::Result<()> {
    let l = parse_and_eval(left)?;
    let r = parse_and_eval(right)?;

    if l.offset != 0.0 || r.offset != 0.0 {
        anyhow::bail!("Arithmetic on offset units not supported");
    }

    let (factor, dim_arr) = match op {
        ArithOp::Mul => {
            let mut arr = [0i8; 7];
            for i in 0..7 {
                arr[i] = l.dim.0[i] + r.dim.0[i];
            }
            (l.factor * r.factor, arr)
        }
        ArithOp::Div => {
            let mut arr = [0i8; 7];
            for i in 0..7 {
                arr[i] = l.dim.0[i] - r.dim.0[i];
            }
            (l.factor / r.factor, arr)
        }
    };

    let result_value = value * factor;
    println!(
        "value: {} (canonical), dimension vector: {:?}",
        result_value, dim_arr
    );
    Ok(())
}

fn handle_explain(code: &str) -> anyhow::Result<()> {
    use ucum_core::find_unit;
    if let Some(unit) = find_unit(code) {
        println!("Unit code:      {}", unit.code);
        println!("Dimension:      {}", unit.dim);
        println!("Factor:         {}", unit.factor);
        println!("Offset:         {}", unit.offset);
        println!("Special kind:   {:?}", unit.special);
    } else {
        anyhow::bail!("Unit '{}' not found", code);
    }
    Ok(())
}

fn handle_list_units(filter: Option<&str>) -> anyhow::Result<()> {
    println!("Supported UCUM Units:");
    println!("====================\n");

    // Get all known unit codes from the test cases
    let test_units = [
        // Base units
        "m", "kg", "s", "A", "K", "mol", "cd", // Common derived units
        "g", "N", "J", "W", "Pa", "Hz", "V", "Ω", "C", "F", "S", "Wb", "T", "H",
        // Common non-SI units
        "L", "h", "min", "d", "a_t", "a_j", "deg", "rad", "sr", "gon",
        // Temperature units
        "Cel", "degF", "degR", "degRe", // Other common units
        "%", "ppm", "ppb", "ppt", "ppq", "bit", "byte", "B", "bit_s", "Bd", "Bq", "Ci", "Gy",
        "kat", "lm", "lx", "Sv", "W", "Wb", "st", "min", "h", "d", "a_j", "a_t", "a_g", "a", "wk",
        "mo_j", "mo_s", "mo_g", "mo_t", "t", "ar", "l", "L", "ar", "bar", "u", "Da", "eV", "pc",
        "AU", "ua", "bit_s", "Bd", "Bq", "Ci", "R", "RAD", "REM", "G", "m[H2O]", "m[Hg]", "[in_i]",
        "[ft_i]", "[yd_i]", "[mi_i]", "[nmi_i]", "[acr_us]", "[acr_br]", "[acr_br]",
    ];

    println!("Units (partial list, filtering available):");
    println!("----------------------------------------");

    for code in test_units.iter() {
        if let Some(filter_str) = filter {
            if !code.to_lowercase().contains(&filter_str.to_lowercase()) {
                continue;
            }
        }

        if let Some(unit) = find_unit(code) {
            let special = if unit.offset != 0.0 {
                format!(" (offset: {})", unit.offset)
            } else {
                String::new()
            };

            println!(
                "{:<10} = {}{}",
                code,
                if unit.factor != 1.0 {
                    format!("{} ", unit.factor)
                } else {
                    "".to_string()
                },
                special
            );
        } else {
            println!("{:<10} = <not found in registry>", code);
        }
    }

    println!("\nNote: This is a partial list. Use the filter option to search for specific units.");
    println!("Example: octofhir-ucum list-units --filter temp");

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
