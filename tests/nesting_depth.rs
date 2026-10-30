//! The parser and the evaluator are recursive, so an expression nested deep enough
//! (`((((m))))`, or `m/m/m/m`, where each solidus nests the term before it, UCUM §7.4) used to
//! overflow the stack and abort the process, which no `catch_unwind` can stop. Expressions
//! nested deeper than `MAX_NESTING_DEPTH` levels (parentheses, solidi and products) are an
//! `NestingTooDeep` instead. Every test runs on a 1 MiB thread, the stack of a
//! WebAssembly module.

use octofhir_ucum::{
    ErrorKind, MAX_NESTING_DEPTH, UcumError, UnitExpr, analyse, canonicalize_expression, convert,
    display_name, evaluate, get_canonical_units, is_comparable, optimize_expression,
    simplify_expression, unit_divide, unit_multiply, validate,
};

fn on_small_stack(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .stack_size(1 << 20)
        .spawn(f)
        .unwrap()
        .join()
        .unwrap();
}

fn nested_parentheses(depth: usize) -> String {
    format!("{}m{}", "(".repeat(depth), ")".repeat(depth))
}

fn quotient_chain(solidi: usize) -> String {
    vec!["m"; solidi + 1].join("/")
}

/// `m.(m.(...))`: a product and a pair of parentheses per level
fn nested_products(levels: usize) -> String {
    format!("{}m{}", "m.(".repeat(levels), ")".repeat(levels))
}

/// `m/(m/(...))`: a solidus and a pair of parentheses per level
fn nested_quotients(levels: usize) -> String {
    format!("{}m{}", "m/(".repeat(levels), ")".repeat(levels))
}

/// A `NestingTooDeep` over `limit`: `MAX_NESTING_DEPTH` levels for a parsed expression,
/// 3 more nodes for a tree built by hand
fn assert_too_deep_for<T: std::fmt::Debug>(result: Result<T, UcumError>, limit: usize) {
    let err = result.expect_err("too deep");
    assert!(
        matches!(err.kind, ErrorKind::NestingTooDeep { limit: found, .. } if found == limit),
        "{:?}",
        err.kind
    );
    assert_eq!(err.kind.code(), "nesting_too_deep");
}

fn assert_invalid<T: std::fmt::Debug>(result: Result<T, UcumError>) {
    assert_too_deep_for(result, MAX_NESTING_DEPTH);
}

fn assert_too_deep(expression: &str) {
    // `analyse` parses like `validate`
    assert_invalid(analyse(expression));
}

#[test]
fn deep_expressions_are_an_error_not_a_crash() {
    on_small_stack(|| {
        // These used to abort with "thread has overflowed its stack" (from 5000 levels on a
        // 2 MiB thread in a release build)
        for depth in [5_000, 100_000] {
            assert_too_deep(&nested_parentheses(depth));
            assert_too_deep(&quotient_chain(depth));
            assert_too_deep(&nested_quotients(depth));
        }
        // `validate` returns the same error, without suggestions
        let err = validate(&quotient_chain(100_000)).expect_err("too deep");
        assert!(matches!(err.kind, ErrorKind::NestingTooDeep { .. }));
        assert!(err.suggestions.is_empty());
    });
}

#[test]
fn the_limit_counts_parentheses_solidi_and_products() {
    assert_eq!(MAX_NESTING_DEPTH, 128);
    on_small_stack(|| {
        for (accepted, rejected) in [
            (nested_parentheses(128), nested_parentheses(129)),
            (quotient_chain(128), quotient_chain(129)),
            // Exponents and annotations do not count
            (
                vec!["10*2{a}"; 129].join("/"),
                vec!["10*2{a}"; 130].join("/"),
            ),
            // A solidus and parentheses per level
            (nested_quotients(64), nested_quotients(65)),
            // A product and parentheses per level
            (nested_products(64), nested_products(65)),
            // A solidus and a product per level after the first: 2 x 63 + 1, 2 x 64 + 1
            (vec!["m.m"; 64].join("/"), vec!["m.m"; 65].join("/")),
            // Parentheses and the solidi around them add up: 64 + 1 + 63, then 64 + 1 + 64
            (
                format!("({}){}", quotient_chain(64), "/m".repeat(63)),
                format!("({}){}", quotient_chain(64), "/m".repeat(64)),
            ),
            // A leading solidus is a level too
            (
                format!("/{}", quotient_chain(127)),
                format!("/{}", quotient_chain(128)),
            ),
        ] {
            validate(&accepted).unwrap_or_else(|err| panic!("{accepted}: {err}"));
            assert_too_deep(&rejected);
        }
    });
}

#[test]
fn expressions_at_the_limit_still_work() {
    on_small_stack(|| {
        for expression in [
            nested_parentheses(128),
            quotient_chain(128),
            nested_quotients(64),
            format!("({}){}", quotient_chain(64), "/m".repeat(63)),
            nested_products(64),
            vec!["m.m"; 64].join("/"),
        ] {
            validate(&expression).unwrap_or_else(|err| panic!("{expression}: {err}"));
            analyse(&expression).unwrap();
            get_canonical_units(&expression).unwrap();
            display_name(&expression).unwrap();
            optimize_expression(&expression).unwrap();
            canonicalize_expression(&expression).unwrap();
            simplify_expression(&expression).unwrap();
            is_comparable(&expression, &expression).unwrap();
            convert(1.5, &expression, &expression).unwrap();
        }
    });
}

#[test]
fn unit_arithmetic_does_not_return_an_expression_too_deep_to_parse() {
    on_small_stack(|| {
        // Each operand is at the limit, so the joined expression is not
        assert_invalid(unit_multiply(&quotient_chain(128), &quotient_chain(128)));
        assert_invalid(unit_divide(&quotient_chain(128), &quotient_chain(128)));
        // Below it, the result parses back
        for result in [
            unit_multiply(&quotient_chain(60), &quotient_chain(60)),
            unit_divide(&quotient_chain(60), &quotient_chain(60)),
        ] {
            validate(&result.unwrap().expression).unwrap();
        }
    });
}

#[test]
fn a_syntax_tree_built_by_hand_is_limited_too() {
    on_small_stack(|| {
        let nested = |depth: usize| {
            let mut expr = UnitExpr::Symbol("m".into());
            for _ in 1..depth {
                expr = UnitExpr::Product(vec![expr]);
            }
            expr
        };
        // As deep as a parsed tree can be: a node per level, and up to 3 for a component
        // (`m2{a}`: annotation, power, symbol)
        assert!(evaluate(&nested(MAX_NESTING_DEPTH + 3)).is_ok());
        assert_too_deep_for(
            evaluate(&nested(MAX_NESTING_DEPTH + 4)),
            MAX_NESTING_DEPTH + 3,
        );
        // Dropping a tree this deep would overflow the stack as well
        let deep = nested(100_000);
        assert_too_deep_for(evaluate(&deep), MAX_NESTING_DEPTH + 3);
        std::mem::forget(deep);
    });
}

#[test]
fn long_flat_expressions_are_not_limited() {
    on_small_stack(|| {
        // A product is one level however long it is; its exponent overflows first
        validate(&vec!["m"; 100].join(".")).unwrap();
        let err = validate(&vec!["m.s-1"; 10_000].join(".")).expect_err("dimension out of range");
        assert!(
            matches!(err.kind, ErrorKind::PrecisionOverflow { .. }),
            "{:?}",
            err.kind
        );
        // A quotient chain past the limit is rejected for its depth, not its exponent
        assert_too_deep(&vec!["m.m"; 200].join("/"));
    });
}
