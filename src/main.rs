//! AutoHarness CLI - Command-line interface for code harness synthesis
//!
//! Usage:
//!   autoharness synthesize [OPTIONS] --code <CODE>
//!   autoharness evaluate --code <CODE>
//!   autoharness run --code <CODE> [--input <INPUT>]
//!   autoharness benchmark
//!   autoharness config [show|validate|init]

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};

use autoharness::engine::synthesis::SimpleEvaluator;
use autoharness::engine::{CodeSynthesisEngine, SynthesisConfig, Evaluator};
use autoharness::sandbox::{SandboxConfig, SandboxExecutor};

/// CLI argument parser
#[derive(Parser)]
#[command(name = "autoharness")]
#[command(version = "0.1.0")]
#[command(about = "AutoHarness: Automatically synthesize code harnesses for LLM agents", long_about = None)]
struct Cli {
    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Output format (text, json)
    #[arg(long, global = true, default_value = "text")]
    format: String,

    #[command(subcommand)]
    command: Commands,
}

/// CLI subcommands
#[derive(Subcommand)]
enum Commands {
    /// Synthesize optimized harness code using tree search
    Synthesize {
        /// Initial code to optimize
        #[arg(short, long)]
        code: Option<String>,

        /// Input file containing code
        #[arg(short, long)]
        file: Option<PathBuf>,

        /// Maximum iterations
        #[arg(long, default_value = "50")]
        max_iterations: u32,

        /// Convergence threshold (0.0-1.0)
        #[arg(long, default_value = "0.95")]
        convergence: f64,

        /// Maximum tree depth
        #[arg(long, default_value = "10")]
        max_depth: u32,

        /// Show statistics
        #[arg(short, long)]
        stats: bool,
    },

    /// Evaluate harness code quality
    Evaluate {
        /// Code to evaluate
        #[arg(short, long)]
        code: Option<String>,

        /// Input file containing code
        #[arg(short, long)]
        file: Option<PathBuf>,

        /// Show detailed scores
        #[arg(short, long)]
        detailed: bool,
    },

    /// Run code in sandboxed environment
    Run {
        /// Code to execute
        #[arg(short, long)]
        code: Option<String>,

        /// Input file containing code
        #[arg(short, long)]
        file: Option<PathBuf>,

        /// Input to pass to code
        #[arg(short, long)]
        input: Option<String>,

        /// Memory limit (MB)
        #[arg(long, default_value = "256")]
        memory_limit: u64,

        /// Time limit (ms)
        #[arg(long, default_value = "5000")]
        time_limit: u64,
    },

    /// Run benchmark tests
    Benchmark {
        /// Number of iterations
        #[arg(short, long, default_value = "100")]
        iterations: u32,

        /// Output file for results
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Configuration management
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Show current configuration
    Show,

    /// Validate configuration file
    Validate {
        /// Config file path
        #[arg(short, long)]
        file: Option<PathBuf>,
    },

    /// Initialize default configuration
    Init {
        /// Output path
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

/// Initialize logging based on verbosity
fn init_logging(verbose: bool) {
    let filter = if verbose {
        EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("debug"))
    } else {
        EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"))
    };

    fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_thread_ids(true)
        .init();
}

/// Read code from string or file
fn get_code(code: Option<String>, file: Option<PathBuf>) -> Result<String> {
    if let Some(c) = code {
        Ok(c)
    } else if let Some(f) = file {
        Ok(std::fs::read_to_string(f)?)
    } else {
        anyhow::bail!("Either --code or --file must be provided")
    }
}

/// Handle synthesize command
fn handle_synthesize(
    code: Option<String>,
    file: Option<PathBuf>,
    max_iterations: u32,
    convergence: f64,
    max_depth: u32,
    show_stats: bool,
    format: &str,
) -> Result<()> {
    let initial_code = get_code(code, file)?;
    info!("Starting synthesis with {} chars of code", initial_code.len());

    let config = SynthesisConfig::new()
        .with_max_iterations(max_iterations)
        .with_convergence_threshold(convergence)
        .with_max_depth(max_depth);

    let mut engine = CodeSynthesisEngine::new(config);
    let evaluator = SimpleEvaluator::new();

    let result = engine.synthesize(&initial_code, &evaluator, None)?;

    match format {
        "json" => {
            let response = serde_json::json!({
                "success": true,
                "code": result,
                "stats": {
                    "iterations": engine.stats().iterations,
                    "nodes_explored": engine.stats().nodes_explored,
                    "best_score": engine.stats().best_score,
                    "convergence": engine.stats().convergence_iteration,
                }
            });
            println!("{}", serde_json::to_string_pretty(&response)?);
        },
        _ => {
            println!("{}", result);
            if show_stats {
                println!("\n--- Statistics ---");
                println!("Iterations: {}", engine.stats().iterations);
                println!("Nodes explored: {}", engine.stats().nodes_explored);
                println!("Best score: {:.4}", engine.stats().best_score);
                if let Some(conv) = engine.stats().convergence_iteration {
                    println!("Converged at iteration: {}", conv);
                }
            }
        }
    }

    Ok(())
}

/// Handle evaluate command
fn handle_evaluate(
    code: Option<String>,
    file: Option<PathBuf>,
    detailed: bool,
    format: &str,
) -> Result<()> {
    let code = get_code(code, file)?;
    let evaluator = SimpleEvaluator::new();
    let score = evaluator.evaluate(&code)?;

    match format {
        "json" => {
            let response = serde_json::json!({
                "success": true,
                "score": score,
                "valid": evaluator.is_valid(&code),
            });
            println!("{}", serde_json::to_string_pretty(&response)?);
        },
        _ => {
            println!("Score: {:.4}/1.0", score);
            println!("Valid: {}", evaluator.is_valid(&code));

            if detailed {
                println!("\n--- Detailed Analysis ---");
                let open_braces = code.matches('{').count();
                let close_braces = code.matches('}').count();
                println!("Braces: {} open, {} close - {}", 
                    open_braces, close_braces,
                    if open_braces == close_braces { "balanced" } else { "unbalanced" });

                let open_parens = code.matches('(').count();
                let close_parens = code.matches(')').count();
                println!("Parentheses: {} open, {} close - {}", 
                    open_parens, close_parens,
                    if open_parens == close_parens { "balanced" } else { "unbalanced" });

                println!("Contains 'fn': {}", code.contains("fn "));
                println!("Length: {} chars", code.len());
            }
        }
    }

    Ok(())
}

/// Handle run command (sandbox execution)
async fn handle_run(
    code: Option<String>,
    file: Option<PathBuf>,
    input: Option<String>,
    memory_limit: u64,
    time_limit: u64,
    format: &str,
) -> Result<()> {
    let code = get_code(code, file)?;
    info!("Executing code in sandbox ({}MB, {}ms)", memory_limit, time_limit);

    let config = SandboxConfig::new()
        .with_memory_limit(memory_limit)
        .with_time_limit(time_limit);

    let executor = SandboxExecutor::new(config)?;
    let result = if let Some(inp) = input {
        executor.execute_with_input(&code, &inp).await?
    } else {
        executor.execute(&code).await?
    };

    match format {
        "json" => {
            let response = serde_json::json!({
                "success": result.success(),
                "exit_code": result.exit_code,
                "stdout": result.stdout,
                "stderr": result.stderr,
                "execution_time_ms": result.execution_time_ms,
                "memory_used_mb": result.memory_used_mb,
            });
            println!("{}", serde_json::to_string_pretty(&response)?);
        },
        _ => {
            println!("Exit code: {}", result.exit_code);
            if !result.stdout.is_empty() {
                println!("\n--- STDOUT ---");
                println!("{}", result.stdout);
            }
            if !result.stderr.is_empty() {
                println!("\n--- STDERR ---");
                println!("{}", result.stderr);
            }
            println!("\nExecution time: {}ms", result.execution_time_ms);
            println!("Memory used: {} MB", result.memory_used_mb);
        }
    }

    Ok(())
}

/// Handle benchmark command
fn handle_benchmark(
    iterations: u32,
    output: Option<PathBuf>,
) -> Result<()> {
    info!("Running benchmark with {} iterations", iterations);

    use std::time::Instant;

    let config = SynthesisConfig::new()
        .with_max_iterations(10)
        .with_convergence_threshold(0.8);

    let mut total_time = 0u64;
    let mut results = Vec::new();

    for i in 0..iterations {
        let start = Instant::now();

        let mut engine = CodeSynthesisEngine::new(config.clone());
        let evaluator = SimpleEvaluator::new();
        let code = format!("fn test_{}() {{\n    let x = {};\n    x + 1\n}}", i, i);

        let _ = engine.synthesize(&code, &evaluator, None);

        let elapsed = start.elapsed().as_millis() as u64;
        total_time += elapsed;

        results.push(elapsed);
    }

    let avg = total_time as f64 / iterations as f64;
    println!("Benchmark Results:");
    println!("  Iterations: {}", iterations);
    println!("  Total time: {}ms", total_time);
    println!("  Average: {:.2}ms", avg);

    // Calculate p50, p95
    results.sort();
    let len = results.len();
    if !results.is_empty() {
        let p50_idx = ((len as f64 * 0.5) as usize).min(len - 1);
        let p95_idx = ((len as f64 * 0.95) as usize).min(len - 1);
        let p50 = results[p50_idx];
        let p95 = results[p95_idx];
        println!("  P50: {}ms", p50);
        println!("  P95: {}ms", p95);

        if let Some(out) = output {
            let json = serde_json::json!({
                "iterations": iterations,
                "total_time_ms": total_time,
                "avg_time_ms": avg,
                "p50_ms": p50,
                "p95_ms": p95,
                "results": results,
            });
            std::fs::write(&out, serde_json::to_string_pretty(&json)?)?;
            println!("Results written to: {}", out.display());
        }
    } else {
        println!("  P50: N/A");
        println!("  P95: N/A");
        if let Some(out) = output {
            println!("Results written to: {}", out.display());
        }
    }

    Ok(())
}

/// Handle config command
fn handle_config(action: ConfigAction) -> Result<()> {
    match action {
        ConfigAction::Show => {
            let config = SynthesisConfig::default();
            println!("Current Configuration:");
            println!("  max_iterations: {}", config.max_iterations);
            println!("  convergence_threshold: {}", config.convergence_threshold);
            println!("  max_depth: {}", config.max_depth);
            println!("  mutations_per_node: {}", config.mutations_per_node);
            println!("  exploration_constant: {}", config.exploration_constant);
            println!("  adaptive_sampling: {}", config.adaptive_sampling);
            println!("  target_iterations: {}", config.target_iterations);
            println!("  min_improvement: {}", config.min_improvement);
            println!("  max_nodes: {}", config.max_nodes);
        },
        ConfigAction::Validate { file } => {
            let path = file.unwrap_or_else(|| PathBuf::from("autoharness.toml"));
            if path.exists() {
                let content = std::fs::read_to_string(&path)?;
                let _config: toml::Value = toml::from_str(&content)?;
                println!("Configuration file is valid: {}", path.display());
            } else {
                anyhow::bail!("Config file not found: {}", path.display());
            }
        },
        ConfigAction::Init { output } => {
            let path = output.unwrap_or_else(|| PathBuf::from("autoharness.toml"));
            let default_config = r#"# AutoHarness Configuration
# Version 0.1.0

[engine]
max_iterations = 50
convergence_threshold = 0.95
max_depth = 10
mutations_per_node = 3
exploration_constant = 1.414
adaptive_sampling = true
target_iterations = 20
min_improvement = 0.01
max_nodes = 1000

[sandbox]
memory_limit_mb = 256
time_limit_ms = 5000
max_file_descriptors = 64
max_output_size = 10485760
enable_network = false

[logging]
level = "info"
"#;
            std::fs::write(&path, default_config)?;
            println!("Created default configuration: {}", path.display());
        },
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    init_logging(cli.verbose);

    match cli.command {
        Commands::Synthesize { code, file, max_iterations, convergence, max_depth, stats } => {
            handle_synthesize(code, file, max_iterations, convergence, max_depth, stats, &cli.format)?;
        },
        Commands::Evaluate { code, file, detailed } => {
            handle_evaluate(code, file, detailed, &cli.format)?;
        },
        Commands::Run { code, file, input, memory_limit, time_limit } => {
            handle_run(code, file, input, memory_limit, time_limit, &cli.format).await?;
        },
        Commands::Benchmark { iterations, output } => {
            handle_benchmark(iterations, output)?;
        },
        Commands::Config { action } => {
            handle_config(action)?;
        },
    }

    Ok(())
}