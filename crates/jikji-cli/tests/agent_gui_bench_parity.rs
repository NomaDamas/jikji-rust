#[path = "agent_gui_bench_parity/mod.rs"]
mod helpers;

use std::fs;

use helpers::{GuiChild, assert_rejected, json_cmd, path_str, run_ok};

#[test]
fn task6_public_agent_and_benchmark_commands_match_contract() {
    let temp = tempfile::tempdir().expect("tempdir");
    let root = temp.path().join("root");
    fs::create_dir(&root).expect("root");
    fs::write(root.join("ACME_contract.txt"), "ACME payment contract").expect("fixture");

    let help = run_ok(["--help"]);
    let help_text = String::from_utf8(help.stdout).expect("help utf8");
    for command in [
        "agent-skill-install",
        "codex-skill-install",
        "skill-export",
        "gui",
        "eval-generate",
        "eval",
        "bench-analyze",
        "bench-run",
        "beir-import",
        "edith-suite",
        "hardbench-build",
    ] {
        assert!(
            help_text.contains(command),
            "missing help command {command}"
        );
    }

    let skill_dest = temp.path().join("agent/skills/jikji/SKILL.md");
    let installed = json_cmd([
        "agent-skill-install",
        "--agent",
        "codex",
        "--dest",
        path_str(&skill_dest).as_str(),
        "--no-prepare",
        "--json",
    ]);
    assert_eq!(installed["installed_any"], true);
    assert!(skill_dest.exists());

    let exported = json_cmd(["skill-export", "--json"]);
    assert!(
        exported["skill_markdown"]
            .as_str()
            .expect("skill markdown")
            .contains("Never move, rename, delete, or reorganize")
    );

    json_cmd(["prepare", path_str(&root).as_str(), "--json"]);
    let generated = json_cmd([
        "eval-generate",
        path_str(&root).as_str(),
        "--cases",
        "3",
        "--json",
    ]);
    let eval_set = generated["eval_set"].as_str().expect("eval_set");
    let evaluated = json_cmd([
        "eval",
        path_str(&root).as_str(),
        "--eval-set",
        eval_set,
        "--json",
    ]);
    assert!(
        evaluated["report"]
            .as_str()
            .expect("report")
            .ends_with(".json")
    );

    let analyzed = json_cmd(["bench-analyze", path_str(&root).as_str(), "--json"]);
    assert_eq!(analyzed["cases"], 1);
    let bench = json_cmd([
        "bench-run",
        path_str(&root).as_str(),
        "--eval-set",
        eval_set,
        "--json",
    ]);
    assert!(
        bench["metrics"]
            .as_object()
            .expect("metrics")
            .contains_key("jikji")
    );
    let beir = json_cmd([
        "beir-import",
        path_str(&temp.path().join("beir")).as_str(),
        "--cases",
        "1",
        "--json",
    ]);
    assert_eq!(beir["network"], "not_used");
}

#[test]
fn task6_gui_management_token_protects_root_and_refresh() {
    let temp = tempfile::tempdir().expect("tempdir");
    let root1 = temp.path().join("root1");
    let root2 = temp.path().join("root2");
    fs::create_dir(&root1).expect("root1");
    fs::create_dir(&root2).expect("root2");
    fs::write(root1.join("ACME_contract.txt"), "ACME payment contract").expect("fixture1");
    fs::write(root2.join("BETA_notes.txt"), "BETA migration memo").expect("fixture2");
    json_cmd(["prepare", path_str(&root1).as_str(), "--json"]);
    json_cmd(["prepare", path_str(&root2).as_str(), "--json"]);

    let gui = GuiChild::start(&root1);

    let unauthorized_root = gui.post(&format!("/api/root?path={}", path_str(&root2)));
    let unauthorized_refresh = gui.post("/api/refresh");
    assert_rejected(&unauthorized_root);
    assert_rejected(&unauthorized_refresh);

    let switch = gui.post(&format!(
        "/api/root?path={}&token={}",
        path_str(&root2),
        gui.manage_token()
    ));
    assert!(switch.starts_with("HTTP/1.1 200 OK"), "{switch}");

    let status = gui.get("/api/status");
    let search = gui.get("/api/search?q=BETA");
    assert!(status.starts_with("HTTP/1.1 200 OK"), "{status}");
    assert!(status.contains("root2"), "{status}");
    assert!(search.starts_with("HTTP/1.1 200 OK"), "{search}");
    assert!(search.contains("BETA_notes.txt"), "{search}");
}

#[test]
fn task6_gui_download_rejects_traversal() {
    let temp = tempfile::tempdir().expect("tempdir");
    let root = temp.path().join("root");
    fs::create_dir(&root).expect("root");
    fs::write(root.join("ACME_contract.txt"), "ACME payment contract").expect("fixture");
    json_cmd(["prepare", path_str(&root).as_str(), "--json"]);

    let gui = GuiChild::start(&root);
    let traversal = gui.get("/download?path=../outside.txt");

    assert_rejected(&traversal);
}
