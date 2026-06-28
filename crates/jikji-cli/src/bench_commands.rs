use std::process::ExitCode;

use jikji_bench::{
    ImportOptions, RunOptions, analyze_eval, generate_eval_set, import_fixture_dataset,
    public_dataset_contract, run_benchmark,
};
use serde_json::json;

use crate::args::{
    BenchAnalyzeArgs, BenchIterateArgs, BenchRunArgs, EvalGenerateArgs, EvalRunArgs, ImportArgs,
    PublicImportArgs, PublicSuiteArgs,
};
use crate::output::print_json;

pub(crate) fn run_eval_generate(args: EvalGenerateArgs) -> jikji_core::Result<ExitCode> {
    let result = generate_eval_set(&args.root, args.cases, args.out.as_deref())?;
    if args.json {
        print_json(&result)?;
    } else {
        println!("Jikji eval set generated: {}", result.eval_set.display());
    }
    Ok(ExitCode::SUCCESS)
}

pub(crate) fn run_eval(args: EvalRunArgs) -> jikji_core::Result<ExitCode> {
    let result = run_benchmark(
        &args.root,
        &RunOptions {
            eval_set: args.eval_set,
            top_k: args.top_k,
            ..RunOptions::default()
        },
    )?;
    if args.json {
        print_json(&result)?;
    } else {
        println!("Jikji eval complete: {}", result.report.display());
    }
    Ok(ExitCode::SUCCESS)
}

pub(crate) fn run_bench_analyze(args: BenchAnalyzeArgs) -> jikji_core::Result<ExitCode> {
    let result = analyze_eval(&args.root, args.report.as_deref())?;
    if args.json {
        print_json(&result)?;
    } else {
        println!(
            "Jikji benchmark analysis complete: {}",
            result.analysis.display()
        );
    }
    Ok(ExitCode::SUCCESS)
}

pub(crate) fn run_hippocamp_import(args: ImportArgs) -> jikji_core::Result<ExitCode> {
    let result = import_fixture_dataset(
        &args
            .out
            .clone()
            .unwrap_or_else(|| args.path.join(".jikji/eval")),
        &ImportOptions {
            dataset: "hippocamp".to_owned(),
            split: args
                .annotation
                .as_ref()
                .and_then(|path| path.file_stem())
                .and_then(|stem| stem.to_str())
                .unwrap_or("fixture")
                .to_owned(),
            cases: args.cases,
            no_fetch: true,
        },
    )?;
    if args.json {
        print_json(&result)?;
    } else {
        println!("HippoCamp eval imported: {}", result.eval_set.display());
    }
    Ok(ExitCode::SUCCESS)
}

pub(crate) fn run_bench_run(args: BenchRunArgs) -> jikji_core::Result<ExitCode> {
    let result = run_benchmark(
        &args.root,
        &RunOptions {
            eval_set: args.eval_set,
            modes: args.modes.split(',').map(str::to_owned).collect(),
            top_k: args.top_k,
            prepare: args.prepare,
            allow_leak: args.allow_leak,
        },
    )?;
    let payload = json!({"report": result.report, "metrics": result.metrics});
    if args.json {
        print_json(&payload)?;
    } else {
        println!("Benchmark complete: {}", result.report.display());
    }
    Ok(ExitCode::SUCCESS)
}

pub(crate) fn run_bench_iterate(args: BenchIterateArgs) -> jikji_core::Result<ExitCode> {
    let mut best = serde_json::Value::Null;
    let mut report = std::path::PathBuf::new();
    for _ in 0..args.iterations.max(1) {
        let result = run_benchmark(
            &args.root,
            &RunOptions {
                eval_set: Some(args.eval_set.clone()),
                modes: args.modes.split(',').map(str::to_owned).collect(),
                top_k: args.top_k,
                ..RunOptions::default()
            },
        )?;
        best = result.metrics;
        report = result.report;
    }
    let payload =
        json!({"report": report, "iterations": args.iterations.max(1), "best_metrics": best});
    if args.json {
        print_json(&payload)?;
    } else {
        println!("Benchmark repeat loop complete: {}", payload["report"]);
    }
    Ok(ExitCode::SUCCESS)
}

pub(crate) fn run_public_import(
    label: &'static str,
    args: PublicImportArgs,
) -> jikji_core::Result<ExitCode> {
    let cases = args.cases.max(1);
    let dataset = if args.dataset == "fixture" {
        label
    } else {
        args.dataset.as_str()
    };
    let payload = public_dataset_contract(&args.dest, dataset, cases)?;
    if args.json {
        print_json(&payload)?;
    } else {
        println!("{label} benchmark materialized: {}", payload["eval_set"]);
    }
    Ok(ExitCode::SUCCESS)
}

pub(crate) fn run_public_suite(
    label: &'static str,
    args: PublicSuiteArgs,
) -> jikji_core::Result<ExitCode> {
    let payload = public_dataset_contract(&args.dest, label, args.cases.max(1))?;
    let result = run_benchmark(
        payload["corpus_root"].as_str().unwrap_or("").as_ref(),
        &RunOptions {
            eval_set: payload["eval_set"].as_str().map(std::path::PathBuf::from),
            top_k: args.top_k,
            prepare: !args.no_prepare,
            allow_leak: true,
            ..RunOptions::default()
        },
    )?;
    let out = json!({
        "report": result.report,
        "build": payload,
        "deterministic_metrics": result.metrics,
        "network": "not_used",
    });
    if args.json {
        print_json(&out)?;
    } else {
        println!("{label} suite complete: {}", out["report"]);
    }
    Ok(ExitCode::SUCCESS)
}
