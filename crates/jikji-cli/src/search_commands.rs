use std::path::Path;
use std::process::ExitCode;

use jikji_core::PrepareOptions;
use jikji_index::prepare;
use jikji_search::{
    BriefOptions, DiscoverOptions, IndexStatus, SearchOptions, brief_payload,
    compact_brief_payload, discover, explain_source, graph_query, graph_status, search,
    search_index_status,
};

use crate::args::{BriefArgs, FindArgs, GraphArgs, GraphCommand, SearchArgs};
use crate::output::{print_json, print_json_compact};

pub(crate) fn run_search(args: SearchArgs) -> jikji_core::Result<ExitCode> {
    let prepared = maybe_prepare_for_search(
        &args.root,
        args.fresh,
        args.auto_prepare && !args.no_auto_prepare,
        args.stale_after_seconds,
    )?;
    if prepared.status == IndexStatus::Missing {
        print_missing_index(&args.root);
        return Ok(ExitCode::from(1));
    }
    let candidates = search(&args.root, &args.query, SearchOptions { top_k: args.top_k })?;
    let payload = serde_json::json!({
        "root": args.root.display().to_string(),
        "query": args.query,
        "top_k": args.top_k,
        "index_status": prepared.status.as_str(),
        "foreground_prepared": prepared.foreground_prepared,
        "background_refresh_started": false,
        "candidates": candidates,
    });
    if args.json {
        print_json(&payload)?;
    } else {
        print_search_candidates(&payload);
    }
    Ok(ExitCode::SUCCESS)
}

pub(crate) fn run_brief(args: BriefArgs) -> jikji_core::Result<ExitCode> {
    let prepared = maybe_prepare_for_search(
        &args.root,
        args.fresh,
        args.auto_prepare && !args.no_auto_prepare,
        args.stale_after_seconds,
    )?;
    if prepared.status == IndexStatus::Missing {
        print_missing_index(&args.root);
        return Ok(ExitCode::from(1));
    }
    let candidates = search(&args.root, &args.query, SearchOptions { top_k: args.top_k })?;
    let options = BriefOptions {
        top_k: args.top_k,
        foreground_prepared: prepared.foreground_prepared,
        background_refresh_started: false,
    };
    let payload = if args.compact {
        compact_brief_payload(
            &args.root,
            &args.query,
            prepared.status.as_str(),
            options,
            &candidates,
        )?
    } else {
        brief_payload(
            &args.root,
            &args.query,
            prepared.status.as_str(),
            options,
            &candidates,
        )
    };
    if args.json && args.compact {
        print_json_compact(&payload)?;
    } else if args.json {
        print_json(&payload)?;
    } else {
        println!("{}", payload);
    }
    Ok(ExitCode::SUCCESS)
}

pub(crate) fn run_find(args: FindArgs) -> jikji_core::Result<ExitCode> {
    let prepared = maybe_prepare_for_search(
        &args.root,
        args.fresh,
        args.auto_prepare && !args.no_auto_prepare,
        args.stale_after_seconds,
    )?;
    if prepared.status == IndexStatus::Missing {
        print_missing_index(&args.root);
        return Ok(ExitCode::from(1));
    }
    let mut payload = discover_payload(&args)?;
    payload["mode"] = serde_json::json!("find");
    payload["command"] = serde_json::json!("jikji find");
    payload["index_status"] = serde_json::json!(prepared.status.as_str());
    if args.first {
        for key in ["answer_paths", "paths", "candidates", "evidence_pack"] {
            truncate_array_field(&mut payload, key, 1);
        }
    }
    if args.json {
        print_json_compact(&payload)?;
    } else if let Some(paths) = payload["paths"].as_array() {
        for path in paths {
            println!("{}", path.as_str().unwrap_or(""));
        }
    }
    Ok(ExitCode::SUCCESS)
}

pub(crate) fn run_discover(args: FindArgs) -> jikji_core::Result<ExitCode> {
    let prepared = maybe_prepare_for_search(
        &args.root,
        args.fresh,
        args.auto_prepare && !args.no_auto_prepare,
        args.stale_after_seconds,
    )?;
    if prepared.status == IndexStatus::Missing {
        print_missing_index(&args.root);
        return Ok(ExitCode::from(1));
    }
    let mut payload = discover_payload(&args)?;
    payload["index_status"] = serde_json::json!(prepared.status.as_str());
    if args.json {
        print_json_compact(&payload)?;
    } else {
        println!("{}", payload);
    }
    Ok(ExitCode::SUCCESS)
}

pub(crate) fn run_graph(args: GraphArgs) -> jikji_core::Result<ExitCode> {
    match args.command {
        GraphCommand::Status { json } => {
            let payload = graph_status(&args.root);
            if json {
                print_json(&payload)?;
            } else {
                println!("{payload}");
            }
        }
        GraphCommand::Query { query, top_k, json } => {
            let payload = serde_json::json!({
                "root": args.root.display().to_string(),
                "query": query,
                "candidates": graph_query(&args.root, &query, top_k)?,
            });
            if json {
                print_json(&payload)?;
            } else {
                println!("{payload}");
            }
        }
        GraphCommand::Explain { source_path, json } => {
            let payload = explain_source(&args.root, &source_path);
            if json {
                print_json(&payload)?;
            } else {
                println!("{payload}");
            }
        }
    }
    Ok(ExitCode::SUCCESS)
}

fn discover_payload(args: &FindArgs) -> jikji_core::Result<serde_json::Value> {
    discover(
        &args.root,
        &args.query,
        DiscoverOptions {
            top_k: args.top_k,
            retry_exhausted: args.after_jikji_retry,
            retry_proof: args.retry_proof.clone(),
        },
    )
}

struct PreparedSearchStatus {
    status: IndexStatus,
    foreground_prepared: bool,
}

fn maybe_prepare_for_search(
    root: &Path,
    fresh: bool,
    auto_prepare: bool,
    stale_after_seconds: i64,
) -> jikji_core::Result<PreparedSearchStatus> {
    let status = search_index_status(root, stale_after_seconds);
    if fresh || (status.should_prepare && auto_prepare) {
        prepare(root, &PrepareOptions::default())?;
        let next = search_index_status(root, stale_after_seconds);
        return Ok(PreparedSearchStatus {
            status: if status.should_prepare {
                IndexStatus::Ready
            } else {
                next.status
            },
            foreground_prepared: true,
        });
    }
    Ok(PreparedSearchStatus {
        status: status.status,
        foreground_prepared: false,
    })
}

fn print_missing_index(root: &Path) {
    eprintln!(
        "No Jikji search index found under {}. Run: jikji prepare {}",
        root.display(),
        root.display()
    );
}

fn print_search_candidates(payload: &serde_json::Value) {
    for (idx, item) in payload["candidates"]
        .as_array()
        .into_iter()
        .flatten()
        .enumerate()
    {
        println!(
            "{:02} {:>8} {}",
            idx + 1,
            item["score"],
            item["path"].as_str().unwrap_or("")
        );
    }
}

fn truncate_array_field(payload: &mut serde_json::Value, key: &str, limit: usize) {
    if let Some(array) = payload
        .get_mut(key)
        .and_then(serde_json::Value::as_array_mut)
    {
        array.truncate(limit);
    }
}
