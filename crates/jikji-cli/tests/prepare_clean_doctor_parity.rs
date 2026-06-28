use std::ffi::OsStr;
use std::fs;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value;

#[path = "prepare_clean_doctor_parity/clean_security.rs"]
mod clean_security;
#[path = "prepare_clean_doctor_parity/prepare_security.rs"]
mod prepare_security;

#[test]
fn prepare_clean_doctor_and_map_match_task_three_contract() {
    let root = temp_root("cli-parity");
    fs::create_dir_all(root.join("docs")).expect("create docs");
    fs::write(root.join("docs/meeting.txt"), "Jikji smoke").expect("write text");
    fs::write(root.join("report.rtf"), r"{\rtf1 stale body}").expect("write rtf");
    fs::write(root.join(".env"), "secret").expect("write sensitive file");
    fs::create_dir_all(root.join(".hidden")).expect("create hidden dir");
    fs::write(root.join(".hidden/hidden.txt"), "hidden").expect("write hidden");

    let prepared = json_cmd(["prepare", root_str(&root).as_str(), "--json"]);
    assert_eq!(prepared["files"], 2);
    assert!(root.join("docs/meeting.txt").exists());
    assert!(root.join(".jikji/agent_map.md").exists());
    assert!(root.join(".jikji_agent_map.md").exists());
    let file_rows = jsonl(root.join(".jikji/file_index.jsonl"));
    assert!(
        file_rows
            .iter()
            .any(|row| row["path"] == "docs/meeting.txt")
    );
    assert!(file_rows.iter().all(|row| row["path"] != ".env"));
    assert!(
        file_rows
            .iter()
            .all(|row| row["path"] != ".hidden/hidden.txt")
    );

    let manifest = json_file(root.join(".jikji/manifest.json"));
    assert_eq!(manifest["non_destructive"], true);
    assert_eq!(
        manifest["source_tree_signature"]["algorithm"],
        "sha256(relpath,size,mtime_ns).v1"
    );
    assert!(
        manifest["owned_paths"]
            .as_array()
            .expect("owned paths")
            .contains(&Value::String(".jikji/doc_text/".to_owned()))
    );
    assert_eq!(manifest["media_index"]["enabled"], false);

    let doc_cache = root.join(
        jsonl(root.join(".jikji/document_index.jsonl"))[0]["text_cache_path"]
            .as_str()
            .expect("cache path"),
    );
    assert!(doc_cache.exists());
    fs::remove_file(root.join("report.rtf")).expect("delete rtf");
    let refreshed = json_cmd(["refresh", root_str(&root).as_str(), "--json"]);
    assert_eq!(refreshed["deleted"], 1);
    assert!(!doc_cache.exists());

    let doctor = json_cmd(["doctor", root_str(&root).as_str(), "--json"]);
    assert_eq!(doctor["ok"], true);
    assert_eq!(doctor["errors"].as_array().expect("errors").len(), 0);

    let map = run_ok(["map", root_str(&root).as_str()]);
    let map_text = String::from_utf8(map.stdout).expect("utf8 map");
    assert!(map_text.contains("Jikji Agent Map"));
    assert!(map_text.contains("docs/meeting.txt"));

    fs::write(root.join(".jikji/user-created-note.txt"), "user").expect("write user note");
    let dry = json_cmd(["clean", root_str(&root).as_str(), "--dry-run", "--json"]);
    assert_eq!(dry["dry_run"], true);
    assert!(
        dry["would_remove"]
            .as_array()
            .expect("would remove")
            .iter()
            .any(|path| path
                .as_str()
                .expect("path")
                .ends_with(".jikji/manifest.json"))
    );
    assert!(
        !dry["would_remove"]
            .as_array()
            .expect("would remove")
            .iter()
            .any(|path| path
                .as_str()
                .expect("path")
                .ends_with("user-created-note.txt"))
    );
    let cleaned = json_cmd(["clean", root_str(&root).as_str(), "--json"]);
    assert_eq!(cleaned["ok"], true);
    assert!(root.join(".jikji/user-created-note.txt").exists());
    assert!(!root.join(".jikji/manifest.json").exists());
    assert!(root.join("docs/meeting.txt").exists());
}

#[test]
fn prepare_has_no_default_cap_and_explicit_max_files_is_partial() {
    let no_cap = temp_root("no-cap");
    for idx in 0..5001 {
        fs::write(no_cap.join(format!("bulk_{idx:04}.bin")), []).expect("write bulk");
    }
    let prepared = json_cmd(["prepare", root_str(&no_cap).as_str(), "--json"]);
    assert_eq!(prepared["files"], 5001);

    let capped = temp_root("capped");
    for idx in 0..4 {
        fs::write(capped.join(format!("bulk_{idx}.txt")), "x").expect("write capped");
    }
    let partial = json_cmd([
        "prepare",
        root_str(&capped).as_str(),
        "--max-files",
        "3",
        "--json",
    ]);
    assert_eq!(partial["files"], 3);
    assert_eq!(json_file(capped.join(".jikji/manifest.json"))["files"], 3);
}

#[test]
fn stale_lock_media_policy_and_clean_safety_classes_are_covered() {
    let root = temp_root("safety");
    fs::write(root.join("photo.png"), minimal_png()).expect("write png");
    fs::create_dir_all(root.join(".jikji")).expect("create sidecar");
    fs::write(
        root.join(".jikji/.lock"),
        r#"{"pid":1,"started_at_unix":1}"#,
    )
    .expect("stale lock");
    let prepared = json_cmd([
        "prepare",
        root_str(&root).as_str(),
        "--enable-media-index",
        "--json",
    ]);
    assert_eq!(prepared["files"], 1);
    let manifest = json_file(root.join(".jikji/manifest.json"));
    assert_eq!(manifest["media_index"]["enabled"], true);
    assert_eq!(manifest["media_index"]["status"], "enabled_bounded");
    assert!(!root.join(".jikji/.lock").exists());

    let outside = temp_root("outside");
    fs::write(outside.join("outside.txt"), "outside").expect("write outside");
    replace_manifest_root(&root, &outside);
    let refused = run(["clean", root_str(&root).as_str(), "--json"]);
    assert!(!refused.status.success());
    assert!(outside.join("outside.txt").exists());
}

#[test]
fn clean_preserves_user_only_jikji_dir_without_verified_manifest() {
    let root = temp_root("user-only");
    fs::create_dir(root.join(".jikji")).expect("create sidecar");
    fs::write(root.join(".jikji/user-created-note.txt"), "user").expect("write note");

    let output = run(["clean", root_str(&root).as_str(), "--json"]);

    assert!(!output.status.success());
    assert!(root.join(".jikji/user-created-note.txt").exists());
}

pub(crate) fn run<const N: usize>(args: [&str; N]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_jikji"))
        .args(args)
        .output()
        .expect("run jikji")
}

fn run_ok<const N: usize>(args: [&str; N]) -> Output {
    let output = run(args);
    assert!(
        output.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

pub(crate) fn json_cmd<const N: usize>(args: [&str; N]) -> Value {
    let output = run_ok(args);
    serde_json::from_slice(&output.stdout).expect("json stdout")
}

pub(crate) fn json_file(path: PathBuf) -> Value {
    serde_json::from_str(&fs::read_to_string(path).expect("read json")).expect("parse json")
}

fn jsonl(path: PathBuf) -> Vec<Value> {
    fs::read_to_string(path)
        .expect("read jsonl")
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str(line).expect("jsonl row"))
        .collect()
}

fn replace_manifest_root(root: &Path, outside: &Path) {
    let path = root.join(".jikji/manifest.json");
    let mut manifest = json_file(path.clone());
    manifest["root"] = Value::String(outside.display().to_string());
    fs::write(
        path,
        serde_json::to_string_pretty(&manifest).expect("serialize"),
    )
    .expect("write manifest");
}

pub(crate) fn root_str(root: &Path) -> String {
    root.display().to_string()
}

pub(crate) struct TempRoot {
    path: PathBuf,
}

impl Deref for TempRoot {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl AsRef<Path> for TempRoot {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}

impl AsRef<OsStr> for TempRoot {
    fn as_ref(&self) -> &OsStr {
        self.path.as_os_str()
    }
}

impl Drop for TempRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

pub(crate) fn temp_root(label: &str) -> TempRoot {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    let root = std::env::temp_dir().join(format!("jikji-{label}-{}-{nonce}", std::process::id()));
    fs::create_dir_all(&root).expect("create temp root");
    TempRoot { path: root }
}

fn minimal_png() -> &'static [u8] {
    b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x00\x01\x00\x00\x00\x01\x08\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00IEND\xaeB`\x82"
}
