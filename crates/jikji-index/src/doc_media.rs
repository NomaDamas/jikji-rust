use std::path::Path;
use std::time::Duration;

use jikji_core::PrepareOptions;
use jikji_media_bridge::{
    BridgeRuntime, MediaBridgeConfig, MediaBridgeOutcome, MediaBridgeRequest, MediaKind,
};
use jikji_parser::{ParsedDocument, ParserRegistry};

pub(crate) const MEDIA_EXTENSIONS: &[&str] = &[
    "png", "jpg", "jpeg", "tif", "tiff", "webp", "bmp", "gif", "mp3", "wav", "m4a", "flac", "ogg",
    "aac", "opus", "wma", "mp4", "mov", "mkv", "avi", "webm", "m4v", "wmv", "flv", "mpg", "mpeg",
];

pub(crate) struct DocumentCacheRuntime {
    registry: ParserRegistry,
    bridge: BridgeRuntime,
}

pub(crate) struct CacheEntry {
    pub(crate) parsed: ParsedDocument,
    pub(crate) bridge: Option<MediaBridgeOutcome>,
}

pub(crate) struct SourceDocument<'a> {
    pub(crate) path: &'a Path,
    pub(crate) ext: &'a str,
    pub(crate) byte_len: u64,
}

impl DocumentCacheRuntime {
    pub(crate) fn new() -> Self {
        Self {
            registry: ParserRegistry::with_defaults(),
            bridge: BridgeRuntime::new(MediaBridgeConfig::enabled_from_env(Duration::from_secs(
                30,
            ))),
        }
    }

    pub(crate) fn cache_entry(
        &self,
        source: SourceDocument<'_>,
        options: &PrepareOptions,
    ) -> CacheEntry {
        let parsed = self.registry.parse_path(source.path, 100_000);
        let bridge = self.media_bridge_outcome(&source, options);
        CacheEntry { parsed, bridge }
    }

    fn media_bridge_outcome(
        &self,
        source: &SourceDocument<'_>,
        options: &PrepareOptions,
    ) -> Option<MediaBridgeOutcome> {
        if !options.enable_media_index
            || !MEDIA_EXTENSIONS.contains(&source.ext)
            || !is_within_media_limit(source.byte_len, options.media_index_max_mb)
        {
            return None;
        }
        Some(self.bridge.extract(&MediaBridgeRequest::new(
            source.path.to_path_buf(),
            media_kind(source.ext),
        )))
    }
}

fn media_kind(ext: &str) -> MediaKind {
    match ext {
        "png" | "jpg" | "jpeg" | "tif" | "tiff" | "webp" | "bmp" | "gif" => MediaKind::Image,
        "mp3" | "wav" | "m4a" | "flac" | "ogg" | "aac" | "opus" | "wma" => MediaKind::Audio,
        "mp4" | "mov" | "mkv" | "avi" | "webm" | "m4v" | "wmv" | "flv" | "mpg" | "mpeg" => {
            MediaKind::Video
        }
        _ => MediaKind::Video,
    }
}

fn is_within_media_limit(byte_len: u64, max_mb: f64) -> bool {
    max_mb.is_finite() && max_mb >= 0.0 && (byte_len as f64) <= max_mb * 1024.0 * 1024.0
}
