use std::fs;
#[cfg(unix)]
use std::os::unix::fs as unix_fs;

use super::{json_cmd, json_file, root_str, temp_root};

#[cfg(unix)]
#[test]
fn prepare_replaces_symlinked_doc_cache_leaf_files_without_touching_targets() {
    let root = temp_root("doc-cache-leaf-symlink");
    fs::write(root.join("probe.pdf"), "probe document\n").expect("write probe");
    fs::create_dir_all(root.join(".jikji/doc_text")).expect("create doc text");
    fs::create_dir_all(root.join(".jikji/doc_meta")).expect("create doc meta");
    let outside = temp_root("doc-cache-leaf-outside");
    let outside_text = outside.join("outside.txt");
    let outside_meta = outside.join("outside.json");
    fs::write(&outside_text, "outside text sentinel").expect("write outside text");
    fs::write(&outside_meta, "outside meta sentinel").expect("write outside meta");
    let digest = "a0633ba1c684a9fdde66555a13bfe75e7054791ce95e4b46161c70c4995e738d";
    let text_leaf = root.join(format!(".jikji/doc_text/sha256_{digest}.txt"));
    let meta_leaf = root.join(format!(".jikji/doc_meta/sha256_{digest}.json"));
    unix_fs::symlink(&outside_text, &text_leaf).expect("symlink text leaf");
    unix_fs::symlink(&outside_meta, &meta_leaf).expect("symlink meta leaf");

    let prepared = json_cmd([
        "prepare",
        root_str(&root).as_str(),
        "--no-agent-rules",
        "--json",
    ]);

    assert_eq!(prepared["files"], 1);
    assert_eq!(
        fs::read_to_string(outside_text).expect("read outside text"),
        "outside text sentinel"
    );
    assert_eq!(
        fs::read_to_string(outside_meta).expect("read outside meta"),
        "outside meta sentinel"
    );
    assert!(
        !fs::symlink_metadata(&text_leaf)
            .expect("text metadata")
            .file_type()
            .is_symlink()
    );
    assert!(
        !fs::symlink_metadata(&meta_leaf)
            .expect("meta metadata")
            .file_type()
            .is_symlink()
    );
    assert!(
        fs::read_to_string(text_leaf)
            .expect("read generated text")
            .contains("# Source: probe.pdf")
    );
    assert_eq!(json_file(meta_leaf)["path"], "probe.pdf");
}
