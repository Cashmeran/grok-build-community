//! `calculator` tool — deterministic arithmetic via kalk + statistics.
//!
//! Pure function, zero side effects. Multi-step expressions separated by `;`
//! or `\n` share a variable environment. The last expression's result is
//! returned along with all variable bindings.
//!
//! Adapted from Caelum's calculator tool.

use crate::types::tool::{ToolKind, ToolNamespace};

const MAX_EXPR_LEN: usize = 8192;
const MAX_LIST_ELEMS: usize = 100_000;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct CalculatorInput {
    #[schemars(description = "The expression to evaluate. Supports: arithmetic, scientific functions (sin/cos/log/exp/sqrt), variables and multi-step (x=5; y=x*2; y+1), statistics (mean/median/stdev/mode/min/max/sum/percentile). Use '=' for equality checks.")]
    pub expression: String,
}

/// Structured calculator output.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CalculatorOutput {
    pub result: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(rename = "type")]
    pub type_name: String,
    pub exact: bool,
}

#[derive(Debug, Default)]
pub struct CalculatorTool;

impl crate::types::tool_metadata::ToolMetadata for CalculatorTool {
    fn kind(&self) -> ToolKind {
        ToolKind::Other
    }

    fn tool_namespace(&self) -> ToolNamespace {
        ToolNamespace::GrokBuild
    }

    fn description_template(&self) -> &str {
        r#"Evaluate a mathematical expression. Supports arithmetic, scientific \
functions (sin/cos/log/exp/sqrt), variables and multi-step (x=5; y=x*2; y+1), \
statistics (mean/median/stdev/mode/min/max/sum/percentile), and equality checks \
(2+2=4 → true). Returns structured JSON with result, value, and type."#
    }
}

impl xai_tool_runtime::Tool for CalculatorTool {
    type Args = CalculatorInput;
    type Output = CalculatorOutput;

    fn id(&self) -> xai_tool_protocol::ToolId {
        xai_tool_protocol::ToolId::new("calculator").expect("valid tool id")
    }

    fn description(
        &self,
        _ctx: &::xai_tool_runtime::ListToolsContext,
    ) -> xai_tool_types::ToolDescription {
        xai_tool_types::ToolDescription::new(
            "calculator",
            crate::types::tool_metadata::ToolMetadata::description_template(self),
        )
    }

    fn capabilities(&self) -> xai_tool_protocol::ToolCapabilities {
        xai_tool_protocol::ToolCapabilities {
            is_read_only: true,
            ..Default::default()
        }
    }

    #[tracing::instrument(name = "tool.calculator", skip_all, fields(expr = %input.expression))]
    async fn run(
        &self,
        _ctx: xai_tool_runtime::ToolCallContext,
        input: CalculatorInput,
    ) -> Result<CalculatorOutput, xai_tool_runtime::ToolError> {
        let expr = input.expression.trim().to_string();
        if expr.is_empty() {
            return Err(xai_tool_runtime::ToolError::execution(
                xai_tool_protocol::ToolId::new("calculator").expect("valid"),
                "'expression' is required".to_string(),
            ));
        }
        if expr.len() > MAX_EXPR_LEN {
            return Err(xai_tool_runtime::ToolError::execution(
                xai_tool_protocol::ToolId::new("calculator").expect("valid"),
                format!("expression too long (max {} bytes)", MAX_EXPR_LEN),
            ));
        }

        // Try statistics functions first
        if let Some(output) = try_stat(&expr) {
            return Ok(output);
        }

        // Delegate to kalk
        let mut ctx = kalk::parser::Context::new()
            .set_timeout(Some(5000))
            .set_max_recursion_depth(64);

        match kalk::parser::eval(&mut ctx, &expr) {
            Ok(Some(result)) => {
                let s = result.to_string_pretty().to_string();
                let (num, type_name) = parse_kalk_result(&s);
                Ok(CalculatorOutput {
                    result: s,
                    value: num,
                    type_name,
                    exact: false,
                })
            }
            Ok(None) => Ok(CalculatorOutput {
                result: "ok".to_string(),
                value: None,
                type_name: "binding".to_string(),
                exact: false,
            }),
            Err(e) => Err(xai_tool_runtime::ToolError::execution(
                xai_tool_protocol::ToolId::new("calculator").expect("valid"),
                format!("{:?}", e),
            )),
        }
    }
}

// ── Kalk result parsing ──

/// Parse kalk's pretty output to extract a numeric value and type.
fn parse_kalk_result(s: &str) -> (Option<f64>, String) {
    let s = s.strip_prefix("= ").unwrap_or(s).trim();
    if s == "true" {
        return (Some(1.0), "boolean".into());
    }
    if s == "false" {
        return (Some(0.0), "boolean".into());
    }
    if s.contains("],[") || (s.starts_with("[[") && s.ends_with("]]")) {
        return (None, "matrix".into());
    }
    if s.starts_with('[') && s.ends_with(']') {
        return (None, "vector".into());
    }
    if let Some(first_word) = s.split_whitespace().next() {
        if let Ok(n) = first_word.parse::<f64>() {
            let has_unit = s.split_whitespace().count() > 1;
            return (Some(n), if has_unit { "quantity".into() } else { "number".into() });
        }
    }
    (None, "expression".into())
}

// ── Statistics ──

fn try_stat(expression: &str) -> Option<CalculatorOutput> {
    let expr = expression.trim();
    if !expr.contains('(') {
        return None;
    }

    let (func, args) = expr.split_once('(')?;
    let func = func.trim().to_lowercase();

    let close_paren = find_stat_paren_end(args)?;
    let stat_args = &args[..close_paren];
    let trailing = args[close_paren + 1..].trim();

    let stat_result = match func.as_str() {
        "mean" | "avg" => stat_unary(stat_args, mean),
        "median" => stat_unary(stat_args, median),
        "stdev" | "std" => stat_unary(stat_args, stdev),
        "variance" | "var" => stat_unary(stat_args, variance),
        "mode" => stat_unary(stat_args, mode),
        "min" => stat_unary(stat_args, list_min),
        "max" => stat_unary(stat_args, list_max),
        "sum" => stat_unary(stat_args, list_sum),
        "percentile" => stat_percentile(stat_args),
        _ => return None,
    };

    let output = match stat_result {
        Ok(o) => o,
        Err(e) => return Some(CalculatorOutput {
            result: e.clone(),
            value: None,
            type_name: "error".into(),
            exact: false,
        }),
    };

    // If trailing content (e.g. "= 3"), feed result + trail to kalk
    if !trailing.is_empty() {
        if let Some(val) = output.value {
            let kalk_expr = format!("{} {}", val, trailing);
            let mut ctx = kalk::parser::Context::new()
                .set_timeout(Some(5000))
                .set_max_recursion_depth(64);
            return match kalk::parser::eval(&mut ctx, &kalk_expr) {
                Ok(Some(result)) => {
                    let s = result.to_string_pretty().to_string();
                    let (num, type_name) = parse_kalk_result(&s);
                    Some(CalculatorOutput { result: s, value: num, type_name, exact: false })
                }
                _ => Some(output),
            };
        }
    }

    Some(output)
}

fn find_stat_paren_end(args: &str) -> Option<usize> {
    let mut depth = 0;
    for (i, c) in args.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                if depth == 0 { return Some(i); }
                depth -= 1;
            }
            _ => {}
        }
    }
    None
}

fn parse_list(input: &str) -> Result<Vec<f64>, String> {
    let s = input.trim().trim_start_matches('[').trim_end_matches(']');
    if s.is_empty() {
        return Err("empty list".into());
    }
    let mut nums = Vec::new();
    for part in s.split([',', ' ', '\t', '\n'].as_ref()) {
        let part = part.trim();
        if part.is_empty() { continue; }
        match part.parse::<f64>() {
            Ok(n) => {
                if nums.len() >= MAX_LIST_ELEMS {
                    return Err(format!("list too long (max {})", MAX_LIST_ELEMS));
                }
                nums.push(n);
            }
            Err(_) => return Err(format!("not a number: '{}'", part)),
        }
    }
    if nums.is_empty() {
        return Err("empty list".into());
    }
    Ok(nums)
}

fn stat_unary(input: &str, f: fn(&[f64]) -> f64) -> Result<CalculatorOutput, String> {
    let nums = parse_list(input)?;
    let result = f(&nums);
    Ok(CalculatorOutput {
        result: fmt_stat(result),
        value: Some(result),
        type_name: "number".into(),
        exact: false,
    })
}

fn fmt_stat(v: f64) -> String {
    if v == v.trunc() && v.abs() < 1e15 {
        format!("{}", v as i64)
    } else {
        let s = format!("{:.6}", v);
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

fn stat_percentile(input: &str) -> Result<CalculatorOutput, String> {
    let list_end = input.rfind(']').unwrap_or(input.len());
    let list_str = &input[..=list_end];
    let p_str = input[list_end + 1..].trim().trim_start_matches(',').trim();
    let nums = parse_list(list_str)?;
    let p: f64 = p_str.parse().map_err(|_| "percentile must be 0-100".to_string())?;
    if !(0.0..=100.0).contains(&p) {
        return Err("percentile must be 0-100".to_string());
    }
    let result = percentile(&nums, p);
    Ok(CalculatorOutput {
        result: fmt_stat(result),
        value: Some(result),
        type_name: "number".into(),
        exact: false,
    })
}

// ── Stat functions ──

fn mean(nums: &[f64]) -> f64 { nums.iter().sum::<f64>() / nums.len() as f64 }

fn median(nums: &[f64]) -> f64 {
    let mut sorted = nums.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = sorted.len();
    if n % 2 == 0 { (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0 } else { sorted[n / 2] }
}

fn mode(nums: &[f64]) -> f64 {
    use std::collections::HashMap;
    let mut counts: HashMap<i64, usize> = HashMap::new();
    for &n in nums {
        if n == n.trunc() && n.abs() < 1e15 {
            *counts.entry(n as i64).or_insert(0) += 1;
        }
    }
    counts.into_iter().max_by_key(|&(_, count)| count).map(|(val, _)| val as f64).unwrap_or(nums[0])
}

fn variance(nums: &[f64]) -> f64 {
    let m = mean(nums);
    nums.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (nums.len() - 1) as f64
}

fn stdev(nums: &[f64]) -> f64 { variance(nums).sqrt() }

fn list_min(nums: &[f64]) -> f64 { nums.iter().cloned().fold(f64::INFINITY, f64::min) }

fn list_max(nums: &[f64]) -> f64 { nums.iter().cloned().fold(f64::NEG_INFINITY, f64::max) }

fn list_sum(nums: &[f64]) -> f64 { nums.iter().sum() }

fn percentile(nums: &[f64], p: f64) -> f64 {
    let mut sorted = nums.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    if sorted.len() == 1 { return sorted[0]; }
    let rank = p / 100.0 * (sorted.len() - 1) as f64;
    let lower = rank.floor() as usize;
    let upper = rank.ceil() as usize;
    if lower == upper { return sorted[lower]; }
    let frac = rank - lower as f64;
    sorted[lower] * (1.0 - frac) + sorted[upper] * frac
}

// ── Tests ──

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::resources::Resources;
    use crate::types::tool_metadata::test_ctx;

    async fn run(expr: &str) -> Result<CalculatorOutput, xai_tool_runtime::ToolError> {
        let resources = Resources::new();
        let tool = CalculatorTool;
        xai_tool_runtime::Tool::run(
            &tool,
            test_ctx(resources.into_shared()),
            CalculatorInput { expression: expr.to_string() },
        ).await
    }

    #[tokio::test]
    async fn basic_arithmetic() {
        let r = run("2 + 3 * 4").await.unwrap();
        assert_eq!(r.value.unwrap(), 14.0);
    }

    #[tokio::test]
    async fn trig() {
        let r = run("sin(pi/2)").await.unwrap();
        assert!(r.value.unwrap() > 0.99);
    }

    #[tokio::test]
    async fn equality_true() {
        let r = run("2 + 2 = 4").await.unwrap();
        assert_eq!(r.value.unwrap(), 1.0);
        assert_eq!(r.type_name, "boolean");
    }

    #[tokio::test]
    async fn multi_step() {
        let r = run("x = 5; y = x * 2; y + 1").await.unwrap();
        assert!((r.value.unwrap() - 11.0).abs() < 0.01);
    }

    #[tokio::test]
    async fn syntax_error() {
        assert!(run("2 +* 3").await.is_err());
    }

    #[tokio::test]
    async fn mean_test() {
        let r = run("mean([1, 2, 3, 4, 5])").await.unwrap();
        assert_eq!(r.value.unwrap(), 3.0);
    }

    #[tokio::test]
    async fn median_test() {
        let r = run("median([1, 3, 2, 5, 4])").await.unwrap();
        assert_eq!(r.value.unwrap(), 3.0);
    }

    #[tokio::test]
    async fn min_max_sum() {
        assert_eq!(run("min([5, 2, 9, 1, 7])").await.unwrap().value.unwrap(), 1.0);
        assert_eq!(run("max([5, 2, 9, 1, 7])").await.unwrap().value.unwrap(), 9.0);
        assert_eq!(run("sum([1, 2, 3, 4, 5])").await.unwrap().value.unwrap(), 15.0);
    }

    #[tokio::test]
    async fn empty_expression() {
        assert!(run("").await.is_err());
    }

    #[test]
    fn tool_name_and_description() {
        let tool = CalculatorTool;
        assert_eq!(xai_tool_runtime::Tool::id(&tool).as_str(), "calculator");
        assert!(crate::types::tool_metadata::ToolMetadata::description_template(&tool).contains("arithmetic"));
    }
}
