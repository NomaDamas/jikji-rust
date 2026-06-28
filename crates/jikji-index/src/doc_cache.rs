use std::collections::BTreeSet;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

use jikji_core::{PrepareOptions, Result, io_error, json_error};
use jikji_media_bridge::{MediaBridgeOutcome, MediaBridgeStatus};
use jikji_parser::ParseStatus;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use crate::doc_media::{CacheEntry, DocumentCacheRuntime, SourceDocument};
use crate::file_io::{dotted_ext, extension};
use crate::scan::{ScanResult, rel_path};

pub(crate) const DOCUMENT_EXTENSIONS: &[&str] = &[
    "rtf", "pdf", "epub", "eml", "ics", "sqlite", "sqlite3", "db", "doc", "docx", "ppt", "pptx",
    "pps", "ppsx", "xls", "xlsx", "hwp", "hwpx", "odt", "ods", "odp", "png", "jpg", "jpeg", "tif",
    "tiff", "webp", "bmp", "gif", "mp3", "wav", "m4a", "flac", "ogg", "aac", "opus", "wma", "mp4",
    "mov", "mkv", "avi", "webm", "m4v", "wmv", "flv", "mpg", "mpeg", "zip", "jar", "war", "tar",
    "tgz", "tbz", "txz", "7z", "rar",
];

pub(crate) struct DocumentBuild {
    pub(crate) rows: Vec<Value>,
    pub(crate) chunk_rows: Vec<Value>,
    pub(crate) live_digests: BTreeSet<String>,
}

pub(crate) struct CacheDirs<'a> {
    pub(crate) text: &'a Path,
    pub(crate) meta: &'a Path,
}

struct DocumentRecord<'a> {
    ext: &'a str,
    digest: &'a str,
    rel: &'a str,
    entry: &'a CacheEntry,
}

pub(crate) fn document_rows(
    scan: &ScanResult,
    dirs: CacheDirs<'_>,
    options: &PrepareOptions,
) -> Result<DocumentBuild> {
    let runtime = DocumentCacheRuntime::new();
    let mut rows = Vec::new();
    let mut chunks = Vec::new();
    let mut live_digests = BTreeSet::new();
    for path in &scan.files {
        let ext = extension(path);
        if !DOCUMENT_EXTENSIONS.contains(&ext.as_str()) {
            continue;
        }
        let digest = sha256_file(path)?;
        live_digests.insert(digest.clone());
        let rel = rel_path(&scan.root, path);
        let metadata = fs::metadata(path).map_err(|source| io_error(path, source))?;
        let source = SourceDocument {
            path,
            ext: ext.as_str(),
            byte_len: metadata.len(),
        };
        let entry = runtime.cache_entry(source, options);
        let record = DocumentRecord {
            ext: ext.as_str(),
            digest: digest.as_str(),
            rel: rel.as_str(),
            entry: &entry,
        };
        let row = document_row(&record);
        write_generated_cache_file(
            &dirs.text.join(format!("sha256_{digest}.txt")),
            cache_text_for(&record).as_bytes(),
        )?;
        write_doc_meta(dirs.meta, &record)?;
        chunks.push(json!({"path": row["path"], "chunk_id": "chunk_0000", "text_cache_path": row["text_cache_path"]}));
        rows.push(row);
    }
    Ok(DocumentBuild {
        rows,
        chunk_rows: chunks,
        live_digests,
    })
}

pub(crate) fn prune_doc_caches(
    doc_text_dir: &Path,
    doc_meta_dir: &Path,
    live_digests: &BTreeSet<String>,
) -> Result<()> {
    remove_stale_matching(doc_text_dir, live_digests, ".txt")?;
    remove_stale_matching(doc_meta_dir, live_digests, ".json")
}

fn document_row(record: &DocumentRecord<'_>) -> Value {
    let summary = summary_for(record.entry);
    json!({
        "path": record.rel,
        "file_id": format!("sha256:{}", record.digest),
        "name": Path::new(record.rel).file_name().and_then(|name| name.to_str()).unwrap_or(""),
        "ext": dotted_ext(record.ext),
        "sha256": record.digest,
        "parse_status": parse_status(record.entry.parsed.status),
        "parser": record.entry.parsed.parser_name,
        "media_bridge_status": record.entry.bridge.as_ref().map(|outcome| media_bridge_status(outcome.status)),
        "text_cache_path": format!(".jikji/doc_text/sha256_{}.txt", record.digest),
        "doc_meta_path": format!(".jikji/doc_meta/sha256_{}.json", record.digest),
        "summary": summary
    })
}

fn write_doc_meta(doc_meta_dir: &Path, record: &DocumentRecord<'_>) -> Result<()> {
    let path = doc_meta_dir.join(format!("sha256_{}.json", record.digest));
    let value = json!({
            "schema_version": 1,
            "source": "jikji",
            "file_id": format!("sha256:{}", record.digest),
            "path": record.rel,
            "parser": record.entry.parsed.parser_name,
            "parse_status": parse_status(record.entry.parsed.status),
            "parser_metadata": record.entry.parsed.metadata,
            "media_bridge": record.entry.bridge.as_ref().map(media_bridge_meta)
    });
    let text = serde_json::to_string_pretty(&value).map_err(|source| json_error(&path, source))?;
    write_generated_cache_file(&path, text.as_bytes())
}

fn write_generated_cache_file(path: &Path, contents: &[u8]) -> Result<()> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            fs::remove_file(path).map_err(|source| io_error(path, source))?;
        }
        Ok(_) => {}
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => {}
        Err(source) => return Err(io_error(path, source)),
    }
    fs::write(path, contents).map_err(|source| io_error(path, source))
}

fn remove_stale_matching(dir: &Path, live_digests: &BTreeSet<String>, suffix: &str) -> Result<()> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(source) => return Err(io_error(dir, source)),
    };
    for entry in entries {
        let entry = entry.map_err(|source| io_error(dir, source))?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let digest = name
            .strip_prefix("sha256_")
            .and_then(|value| value.strip_suffix(suffix));
        if digest.is_some_and(|digest| !live_digests.contains(digest)) {
            fs::remove_file(entry.path()).map_err(|source| io_error(entry.path(), source))?;
        }
    }
    Ok(())
}

fn sha256_file(path: &Path) -> Result<String> {
    let mut file = File::open(path).map_err(|source| io_error(path, source))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];
    loop {
        let read = file
            .read(&mut buffer)
            .map_err(|source| io_error(path, source))?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn text_for_cache(entry: &CacheEntry) -> &str {
    if let Some(outcome) = &entry.bridge {
        if outcome.status == MediaBridgeStatus::Success && !outcome.text.is_empty() {
            return outcome.text.as_str();
        }
    }
    entry.parsed.text.as_str()
}

fn cache_text_for(record: &DocumentRecord<'_>) -> String {
    format!(
        "# Source: {}\n# File ID: sha256:{}\n# Parsed by: Jikji\n\n{}",
        record.rel,
        record.digest,
        text_for_cache(record.entry)
    )
}

fn summary_for(entry: &CacheEntry) -> String {
    text_for_cache(entry)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .take(240)
        .collect()
}

fn parse_status(status: ParseStatus) -> &'static str {
    match status {
        ParseStatus::Success => "success",
        ParseStatus::ArchiveListing => "archive_listing",
        ParseStatus::MetadataOnly => "metadata_only",
        ParseStatus::Failed => "failed",
        ParseStatus::Unsupported => "unsupported",
    }
}

fn media_bridge_status(status: MediaBridgeStatus) -> &'static str {
    match status {
        MediaBridgeStatus::MetadataOnly => "metadata_only",
        MediaBridgeStatus::Success => "success",
        MediaBridgeStatus::Unavailable => "unavailable",
        MediaBridgeStatus::Failed => "failed",
        MediaBridgeStatus::Timeout => "timeout",
    }
}

fn media_bridge_meta(outcome: &MediaBridgeOutcome) -> Value {
    json!({
        "status": media_bridge_status(outcome.status),
        "metadata": outcome.metadata,
        "error": outcome.error
    })
}
