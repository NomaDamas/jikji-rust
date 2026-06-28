use std::path::PathBuf;

use clap::{Parser, Subcommand};

// allow: SIZE_OK - this file is the typed Clap command registry for the Rust/Python parity surface.
#[derive(Debug, Parser)]
#[command(
    name = "jikji",
    version,
    about = "Non-destructive local file knowledge maps for AI agents.",
    arg_required_else_help = true
)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    #[command(about = "Create or update .jikji artifacts for a folder.")]
    Prepare(PrepareArgs),
    #[command(about = "Alias for prepare.")]
    Refresh(PrepareArgs),
    #[command(about = "Remove only Jikji-owned generated artifacts.")]
    Clean(CleanArgs),
    #[command(about = "Print the generated agent map for a folder.")]
    Map { root: PathBuf },
    #[command(about = "Verify expected generated artifacts for a folder.")]
    Doctor {
        root: PathBuf,
        #[arg(long)]
        json: bool,
    },
    #[command(about = "Query an existing Jikji index.")]
    Find(FindArgs),
    #[command(about = "Rank files from an existing Jikji index.")]
    Search(SearchArgs),
    #[command(about = "Emit an agent search brief.")]
    Brief(BriefArgs),
    #[command(about = "Emit the discovery answer-pack contract.")]
    Discover(FindArgs),
    #[command(about = "Query graph/wiki route artifacts.")]
    Graph(GraphArgs),
    #[command(about = "Serve a local loopback web UI.")]
    Gui(GuiArgs),
    #[command(about = "Install the Jikji auto-use skill for local agents.")]
    AgentSkillInstall(AgentSkillArgs),
    #[command(about = "Install the Jikji skill into ~/.hermes/skills.")]
    HermesSkillInstall(SkillAliasArgs),
    #[command(about = "Install the Jikji skill for Codex.")]
    CodexSkillInstall(SkillAliasArgs),
    #[command(about = "Install the Jikji skill for OMX.")]
    OmxSkillInstall(SkillAliasArgs),
    #[command(about = "Install the Jikji skill for Claude.")]
    ClaudeSkillInstall(SkillAliasArgs),
    #[command(about = "Install the Jikji skill for OpenCode.")]
    OpencodeSkillInstall(SkillAliasArgs),
    #[command(about = "Install the Jikji skill for OpenClo.")]
    OpencloSkillInstall(SkillAliasArgs),
    #[command(about = "Install the Jikji skill for NanoClo.")]
    NanocloSkillInstall(SkillAliasArgs),
    #[command(about = "Print or write the universal Jikji SKILL.md.")]
    SkillExport(SkillExportArgs),
    #[command(about = "Generate local search evaluation cases.")]
    EvalGenerate(EvalGenerateArgs),
    #[command(about = "Generate curated realistic evaluation cases.")]
    EvalGenerateRealistic(EvalGenerateArgs),
    #[command(about = "Generate locked holdout evaluation cases.")]
    EvalGenerateHoldout(EvalGenerateArgs),
    #[command(about = "Evaluate local search quality.")]
    Eval(EvalRunArgs),
    #[command(about = "Analyze benchmark failures and answerability.")]
    BenchAnalyze(BenchAnalyzeArgs),
    #[command(about = "Import a bounded HippoCamp eval set.")]
    HippocampImport(ImportArgs),
    #[command(about = "Compare raw filesystem search with Jikji-assisted search.")]
    BenchRun(BenchRunArgs),
    #[command(about = "Repeat a deterministic benchmark.")]
    BenchIterate(BenchIterateArgs),
    #[command(about = "Download/materialize one BEIR dataset as local files.")]
    BeirImport(PublicImportArgs),
    #[command(about = "Download/materialize a bounded HippoCamp subset as local files.")]
    HippocampFetch(PublicImportArgs),
    #[command(about = "Run public BEIR local-file retrieval suite.")]
    BeirSuite(PublicSuiteArgs),
    #[command(about = "Inspect public EDiTh benchmark metadata.")]
    EdithSummary(PublicImportArgs),
    #[command(about = "Materialize a bounded EDiTh benchmark.")]
    EdithImport(PublicImportArgs),
    #[command(about = "Run bounded public EDiTh diagnostics.")]
    EdithSuite(PublicSuiteArgs),
    #[command(about = "Build a Korean public-data local-agent corpus.")]
    PublicdataBuild(PublicImportArgs),
    #[command(about = "Build and run public-data diagnostics.")]
    PublicdataSuite(PublicSuiteArgs),
    #[command(about = "Build a bounded Workspace-Bench-Lite corpus.")]
    WorkspacebenchBuild(PublicImportArgs),
    #[command(about = "Build and run Workspace-Bench-Lite diagnostics.")]
    WorkspacebenchSuite(PublicSuiteArgs),
    #[command(about = "Build a hard mixed-document benchmark corpus.")]
    HardbenchBuild(PublicImportArgs),
    #[command(about = "Build and run hard mixed-document diagnostics.")]
    HardbenchSuite(PublicSuiteArgs),
    #[command(about = "Run a bounded multi-profile HippoCamp suite.")]
    HippocampSuite(PublicSuiteArgs),
    #[command(hide = true, about = "Run queued post-install prepares.")]
    PostInstallPrepare(PostInstallPrepareArgs),
    #[command(about = "Python-only Hermes benchmark compatibility command.")]
    HermesBench(HermesBenchArgs),
    #[command(about = "Python-only Hermes report comparison compatibility command.")]
    HermesCompare(HermesCompareArgs),
    #[command(about = "Python-only benchmark value report compatibility command.")]
    BenchmarkValueReport(BenchmarkValueReportArgs),
}

#[derive(Debug, Parser)]
pub(crate) struct PrepareArgs {
    pub(crate) root: PathBuf,
    #[arg(long)]
    pub(crate) json: bool,
    #[arg(long)]
    pub(crate) include_hidden: bool,
    #[arg(long)]
    pub(crate) include_sensitive: bool,
    #[arg(long)]
    pub(crate) max_files: Option<usize>,
    #[arg(long)]
    pub(crate) enable_media_index: bool,
    #[arg(long, default_value_t = 25.0)]
    pub(crate) media_index_max_mb: f64,
    #[arg(long)]
    pub(crate) no_agent_rules: bool,
}

#[derive(Debug, Parser)]
pub(crate) struct CleanArgs {
    pub(crate) root: PathBuf,
    #[arg(long)]
    pub(crate) json: bool,
    #[arg(long)]
    pub(crate) dry_run: bool,
    #[arg(long)]
    pub(crate) force: bool,
}

#[derive(Debug, Parser)]
pub(crate) struct FindArgs {
    pub(crate) root: PathBuf,
    pub(crate) query: String,
    #[arg(long, default_value_t = 20)]
    pub(crate) top_k: usize,
    #[arg(long)]
    pub(crate) first: bool,
    #[arg(long)]
    pub(crate) json: bool,
    #[arg(long)]
    pub(crate) fresh: bool,
    #[arg(long, default_value_t = false)]
    pub(crate) auto_prepare: bool,
    #[arg(long = "no-auto-prepare", default_value_t = false)]
    pub(crate) no_auto_prepare: bool,
    #[arg(long, default_value_t = 24 * 60 * 60, allow_hyphen_values = true)]
    pub(crate) stale_after_seconds: i64,
    #[arg(long)]
    pub(crate) after_jikji_retry: bool,
    #[arg(long, default_value = "")]
    pub(crate) retry_proof: String,
}

#[derive(Debug, Parser)]
pub(crate) struct SearchArgs {
    pub(crate) root: PathBuf,
    pub(crate) query: String,
    #[arg(long, default_value_t = 20)]
    pub(crate) top_k: usize,
    #[arg(long)]
    pub(crate) json: bool,
    #[arg(long)]
    pub(crate) fresh: bool,
    #[arg(long, default_value_t = false)]
    pub(crate) auto_prepare: bool,
    #[arg(long = "no-auto-prepare", default_value_t = false)]
    pub(crate) no_auto_prepare: bool,
    #[arg(long, default_value_t = 24 * 60 * 60, allow_hyphen_values = true)]
    pub(crate) stale_after_seconds: i64,
}

#[derive(Debug, Parser)]
pub(crate) struct BriefArgs {
    pub(crate) root: PathBuf,
    pub(crate) query: String,
    #[arg(long, default_value_t = 10)]
    pub(crate) top_k: usize,
    #[arg(long)]
    pub(crate) json: bool,
    #[arg(long)]
    pub(crate) compact: bool,
    #[arg(long)]
    pub(crate) fresh: bool,
    #[arg(long, default_value_t = false)]
    pub(crate) auto_prepare: bool,
    #[arg(long = "no-auto-prepare", default_value_t = false)]
    pub(crate) no_auto_prepare: bool,
    #[arg(long, default_value_t = 24 * 60 * 60, allow_hyphen_values = true)]
    pub(crate) stale_after_seconds: i64,
}

#[derive(Debug, Parser)]
pub(crate) struct GraphArgs {
    pub(crate) root: PathBuf,
    #[command(subcommand)]
    pub(crate) command: GraphCommand,
}

#[derive(Debug, Subcommand)]
pub(crate) enum GraphCommand {
    Status {
        #[arg(long)]
        json: bool,
    },
    Query {
        query: String,
        #[arg(long, default_value_t = 10)]
        top_k: usize,
        #[arg(long)]
        json: bool,
    },
    Explain {
        source_path: String,
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Parser)]
pub(crate) struct GuiArgs {
    #[arg(default_value = ".")]
    pub(crate) root: PathBuf,
    #[arg(long, default_value = "127.0.0.1")]
    pub(crate) host: String,
    #[arg(long, default_value_t = 8765)]
    pub(crate) port: u16,
    #[arg(long)]
    pub(crate) no_open: bool,
    #[arg(long)]
    pub(crate) prepare: bool,
    #[arg(long)]
    pub(crate) background: bool,
    #[arg(long)]
    pub(crate) json: bool,
    #[arg(long, hide = true)]
    pub(crate) serve_child: bool,
    #[arg(long, hide = true)]
    pub(crate) manage_token: Option<String>,
}

#[derive(Debug, Parser)]
pub(crate) struct AgentSkillArgs {
    #[arg(long = "agent")]
    pub(crate) agents: Vec<String>,
    #[arg(long)]
    pub(crate) dest: Option<PathBuf>,
    #[arg(long = "prepare-root")]
    pub(crate) prepare_roots: Vec<PathBuf>,
    #[arg(long)]
    pub(crate) no_prepare: bool,
    #[arg(long)]
    pub(crate) foreground_prepare: bool,
    #[arg(long, default_value_t = 5.0)]
    pub(crate) parse_timeout: f64,
    #[arg(long)]
    pub(crate) max_files: Option<usize>,
    #[arg(long)]
    pub(crate) force: bool,
    #[arg(long)]
    pub(crate) json: bool,
}

#[derive(Debug, Parser)]
pub(crate) struct SkillAliasArgs {
    #[arg(long)]
    pub(crate) dest: Option<PathBuf>,
    #[arg(long = "prepare-root")]
    pub(crate) prepare_roots: Vec<PathBuf>,
    #[arg(long)]
    pub(crate) no_prepare: bool,
    #[arg(long)]
    pub(crate) foreground_prepare: bool,
    #[arg(long, default_value_t = 5.0)]
    pub(crate) parse_timeout: f64,
    #[arg(long)]
    pub(crate) max_files: Option<usize>,
    #[arg(long)]
    pub(crate) force: bool,
    #[arg(long)]
    pub(crate) json: bool,
}

#[derive(Debug, Parser)]
pub(crate) struct SkillExportArgs {
    #[arg(long)]
    pub(crate) dest: Option<PathBuf>,
    #[arg(long = "prepare-root")]
    pub(crate) prepare_roots: Vec<PathBuf>,
    #[arg(long)]
    pub(crate) no_prepare: bool,
    #[arg(long)]
    pub(crate) foreground_prepare: bool,
    #[arg(long, default_value_t = 5.0)]
    pub(crate) parse_timeout: f64,
    #[arg(long)]
    pub(crate) max_files: Option<usize>,
    #[arg(long)]
    pub(crate) force: bool,
    #[arg(long)]
    pub(crate) json: bool,
}

#[derive(Debug, Parser)]
pub(crate) struct EvalGenerateArgs {
    #[arg(default_value = ".")]
    pub(crate) root: PathBuf,
    #[arg(long, default_value_t = 80)]
    pub(crate) cases: usize,
    #[arg(long)]
    pub(crate) out: Option<PathBuf>,
    #[arg(long)]
    pub(crate) json: bool,
}

#[derive(Debug, Parser)]
pub(crate) struct EvalRunArgs {
    #[arg(default_value = ".")]
    pub(crate) root: PathBuf,
    #[arg(long)]
    pub(crate) eval_set: Option<PathBuf>,
    #[arg(long, default_value_t = 10)]
    pub(crate) top_k: usize,
    #[arg(long)]
    pub(crate) json: bool,
}

#[derive(Debug, Parser)]
pub(crate) struct BenchAnalyzeArgs {
    #[arg(default_value = ".")]
    pub(crate) root: PathBuf,
    #[arg(long)]
    pub(crate) report: Option<PathBuf>,
    #[arg(long)]
    pub(crate) json: bool,
}

#[derive(Debug, Parser)]
pub(crate) struct ImportArgs {
    pub(crate) path: PathBuf,
    #[arg(long)]
    pub(crate) annotation: Option<PathBuf>,
    #[arg(long, default_value_t = 200)]
    pub(crate) cases: usize,
    #[arg(long)]
    pub(crate) out: Option<PathBuf>,
    #[arg(long)]
    pub(crate) json: bool,
}

#[derive(Debug, Parser)]
pub(crate) struct BenchRunArgs {
    pub(crate) root: PathBuf,
    #[arg(long)]
    pub(crate) eval_set: Option<PathBuf>,
    #[arg(long, default_value = "raw,jikji")]
    pub(crate) modes: String,
    #[arg(long, default_value_t = 10)]
    pub(crate) top_k: usize,
    #[arg(long)]
    pub(crate) prepare: bool,
    #[arg(long)]
    pub(crate) allow_leak: bool,
    #[arg(long)]
    pub(crate) json: bool,
}

#[derive(Debug, Parser)]
pub(crate) struct BenchIterateArgs {
    pub(crate) root: PathBuf,
    #[arg(long)]
    pub(crate) eval_set: PathBuf,
    #[arg(long, default_value_t = 20)]
    pub(crate) iterations: usize,
    #[arg(long, default_value = "raw,jikji")]
    pub(crate) modes: String,
    #[arg(long, default_value_t = 10)]
    pub(crate) top_k: usize,
    #[arg(long)]
    pub(crate) json: bool,
}

#[derive(Debug, Parser)]
pub(crate) struct PublicImportArgs {
    pub(crate) dest: PathBuf,
    #[arg(long, default_value = "fixture")]
    pub(crate) dataset: String,
    #[arg(long, default_value_t = 3)]
    pub(crate) cases: usize,
    #[arg(long)]
    pub(crate) no_fetch: bool,
    #[arg(long)]
    pub(crate) no_docs: bool,
    #[arg(long)]
    pub(crate) json: bool,
}

#[derive(Debug, Parser)]
pub(crate) struct PublicSuiteArgs {
    pub(crate) dest: PathBuf,
    #[arg(long, default_value_t = 3)]
    pub(crate) cases: usize,
    #[arg(long, default_value_t = 10)]
    pub(crate) top_k: usize,
    #[arg(long)]
    pub(crate) no_fetch: bool,
    #[arg(long)]
    pub(crate) no_prepare: bool,
    #[arg(long)]
    pub(crate) json: bool,
}

#[derive(Debug, Parser)]
pub(crate) struct PostInstallPrepareArgs {
    pub(crate) roots: Vec<PathBuf>,
    #[arg(long)]
    pub(crate) max_files: Option<usize>,
    #[arg(long, default_value_t = 5.0)]
    pub(crate) parse_timeout: f64,
    #[arg(long)]
    pub(crate) json: bool,
}

#[derive(Debug, Parser)]
pub(crate) struct HermesBenchArgs {
    pub(crate) root: PathBuf,
    #[arg(long)]
    pub(crate) eval_set: Option<PathBuf>,
    #[arg(long, default_value = "raw,jikji")]
    pub(crate) modes: String,
    #[arg(long, default_value_t = 0)]
    pub(crate) cases: usize,
    #[arg(long)]
    pub(crate) out: Option<PathBuf>,
    #[arg(long, default_value = "hermes")]
    pub(crate) hermes_bin: String,
    #[arg(long, default_value = "")]
    pub(crate) model: String,
    #[arg(long, default_value = "")]
    pub(crate) provider: String,
    #[arg(long, default_value_t = 240)]
    pub(crate) timeout: u64,
    #[arg(long, default_value_t = 20)]
    pub(crate) max_turns: usize,
    #[arg(long, default_value_t = 1)]
    pub(crate) fast_max_turns: usize,
    #[arg(long, default_value = "")]
    pub(crate) skills: String,
    #[arg(long, default_value_t = 20)]
    pub(crate) candidate_top_k: usize,
    #[arg(long, default_value_t = 1)]
    pub(crate) retries: usize,
    #[arg(long)]
    pub(crate) yolo: bool,
    #[arg(long)]
    pub(crate) allow_leak: bool,
    #[arg(long)]
    pub(crate) json: bool,
}

#[derive(Debug, Parser)]
pub(crate) struct HermesCompareArgs {
    pub(crate) raw_report: PathBuf,
    pub(crate) jikji_report: PathBuf,
    #[arg(long, default_value = "raw")]
    pub(crate) raw_mode: String,
    #[arg(long, default_value = "jikji")]
    pub(crate) jikji_mode: String,
    #[arg(long, default_value_t = 0.75)]
    pub(crate) max_token_ratio: f64,
    #[arg(long, default_value_t = 0.75)]
    pub(crate) max_call_ratio: f64,
    #[arg(long, default_value_t = 1.0)]
    pub(crate) max_seconds_ratio: f64,
    #[arg(long)]
    pub(crate) max_avg_llm_calls: Option<f64>,
    #[arg(long)]
    pub(crate) max_p95_llm_calls: Option<usize>,
    #[arg(long)]
    pub(crate) json: bool,
}

#[derive(Debug, Parser)]
pub(crate) struct BenchmarkValueReportArgs {
    #[arg(
        long,
        default_value = ".benchmarks/hippocamp-full/_hermes_full_gpt54mini_chunks5"
    )]
    pub(crate) raw_report_dir: PathBuf,
    #[arg(long)]
    pub(crate) raw_discover_dir: Option<PathBuf>,
    #[arg(
        long,
        default_value = ".benchmarks/hippocamp-full/_hermes_answer_pack_full_20260623_anchorfix/full_answer_pack_aggregate_report.json"
    )]
    pub(crate) find_candidate_report: PathBuf,
    #[arg(long)]
    pub(crate) answer_pack_report: Option<PathBuf>,
    #[arg(
        long,
        default_value = ".benchmarks/hippocamp-full/_hermes_answer_pack_full_20260623_anchorfix"
    )]
    pub(crate) find_candidate_dir: PathBuf,
    #[arg(long)]
    pub(crate) answer_pack_dir: Option<PathBuf>,
    #[arg(long, default_value_t = 20)]
    pub(crate) judge_top_k: usize,
    #[arg(long, default_value_t = 1.5)]
    pub(crate) llm_latency_seconds: f64,
    #[arg(long, default_value = "docs/jikji-value-report.json")]
    pub(crate) out: PathBuf,
    #[arg(long, default_value_t = 0.30)]
    pub(crate) input_per_1m_usd: f64,
    #[arg(long, default_value_t = 2.50)]
    pub(crate) output_per_1m_usd: f64,
    #[arg(long, default_value_t = 1380.0)]
    pub(crate) usd_to_krw: f64,
    #[arg(long)]
    pub(crate) json: bool,
}
