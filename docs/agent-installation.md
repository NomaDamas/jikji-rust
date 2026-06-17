# Jikji Agent Installation Manual

Jikji is a non-destructive local file-discovery layer for AI agents. It prepares
an explicit local folder so agents can find files, folders, metadata, and parsed
document text without repeatedly crawling the original filesystem.

Use this manual for Claude Code, Codex, Hermes, OpenCode/OpenClone-style agents,
or any local agent that can run CLI commands.

## 1. Install from GitHub

```bash
git clone https://github.com/nomadamas/jikji.git
cd jikji
python3 -m venv .venv
.venv/bin/pip install -e .
.venv/bin/jikji --help
```

Optional developer validation:

```bash
.venv/bin/pip install pytest ruff
.venv/bin/ruff check src tests
.venv/bin/pytest -q
.venv/bin/python -m compileall -q src tests
```

If you want `jikji` globally available, either add the checkout venv to your
agent's PATH or install it into the Python environment that the agent uses.

### Agent self-bootstrap when the CLI is missing

CLI agents such as Hermes/Codex should not stop just because `jikji` is absent. If network access is available, bootstrap from GitHub and then run the venv-local binary:

```bash
mkdir -p ~/.local/share/jikji
if [ ! -d ~/.local/share/jikji/repo/.git ]; then
  git clone https://github.com/nomadamas/jikji.git ~/.local/share/jikji/repo
fi
cd ~/.local/share/jikji/repo
git pull --ff-only
python3 -m venv .venv
.venv/bin/pip install -e .
.venv/bin/jikji --help
```

Use `~/.local/share/jikji/repo/.venv/bin/jikji` when the command is not on PATH.

### Optional image OCR dependency

Jikji indexes image files with lightweight local metadata (format, dimensions,
and selected EXIF datetime when available) without extra tools. OCR text from
images and scanned PDFs requires a local `tesseract` binary on the agent's PATH.
Check the active state with:

```bash
jikji doctor /explicit/root --json
```

In the JSON report, `image_support.ocr_active` is `true` only when Tesseract is
available. Without it, images are still searchable by filename/path/basic file
metadata and the lightweight image metadata, but not by text inside the image.

## 2. Attach Jikji as an agent skill

The reusable skill file lives here:

```text
skills/jikji/SKILL.md
```

Install it with Jikji's installer. Once installed, the skill tells the agent to
select Jikji automatically for local file/folder/document discovery under an
explicit root; the user should not have to say "use Jikji" every time.

Install for every supported local-agent surface:

```bash
jikji agent-skill-install --agent all --json
```

By default this installs the skill and starts one sequential background prepare
job for common user-content roots that exist on the machine: Documents,
Downloads, Desktop, and cloud-sync folders such as Google Drive, OneDrive,
Dropbox, and iCloud Drive. The default number of roots is capped from local
CPU/memory so installation does not overload smaller machines.

Control post-install prepare:

```bash
jikji agent-skill-install --agent all --prepare-root /mnt/work-drive --json
jikji agent-skill-install --agent all --foreground-prepare --json
jikji agent-skill-install --agent all --no-prepare --json
```

Install for one agent:

```bash
jikji codex-skill-install --json      # Codex and OMX share the Codex skill dir
jikji omx-skill-install --json
jikji claude-skill-install --json
jikji hermes-skill-install --json
jikji opencode-skill-install --json
jikji openclo-skill-install --json
jikji nanoclo-skill-install --json
```

Install for any other coding/local agent:

```bash
# If the agent has a skill directory:
jikji skill-export --dest /path/to/that-agent/skills/jikji/SKILL.md --json

# If the agent only has persistent/project instructions:
jikji skill-export
```

The exported `SKILL.md` is not tied to a named vendor. Any agent that loads
Markdown skills or persistent instructions should then auto-select Jikji for
bounded local file/folder/document discovery requests.

Hermes convenience installer:

```bash
jikji hermes-skill-install --json
```

Generic fallback when an agent has a custom skill directory:

```bash
mkdir -p ~/.local/share/agent-skills/jikji
cp skills/jikji/SKILL.md ~/.local/share/agent-skills/jikji/SKILL.md
```

For Claude Code, Codex, OpenCode, or similar tools, the exact skill directory is
agent-specific. The important part is that the agent receives the Jikji protocol
and can run the `jikji` CLI.

## 3. Agent protocol

When a user asks an agent to find a local file or document:

1. Require an explicit root path. Do not silently scan every drive.
2. For a single file path, run `jikji find ROOT "query" --first` first.
   Use `jikji brief ROOT "query" --top-k 10 --compact --json` when evidence/wiki/cache hints are needed.
3. If only ranked paths are needed, run `jikji search ROOT "query" --top-k 10 --json`.
4. Prefer candidate paths returned by Jikji when evidence/reasons match.
5. Open original files only for final verification.
6. If Jikji has no useful result, follow the fallback route in the brief:
   generated map/indexes -> `.jikji/doc_text/` -> original text files.
7. Never move, rename, delete, or reorganize original files.

Minimal commands:
Zero-LLM path-only command:

```bash
jikji find /explicit/root "natural language file clue" --first
```

```bash
jikji brief /explicit/root "natural language file clue" --top-k 10 --compact --json
```

Smaller candidate-only command:

```bash
jikji search /explicit/root "natural language file clue" --top-k 10 --json
```

Administrative preparation, when needed:

```bash
jikji prepare /explicit/root --json
jikji doctor /explicit/root --json
jikji map /explicit/root
```

Human management dashboard, when the user wants to inspect Jikji status or receive a clickable local link:

```bash
jikji gui /explicit/root
jikji gui /explicit/root --background --json
```

The GUI binds to `127.0.0.1` by default. It shows prepare status, file/document counts, LLM Wiki source count, knowledge graph node/edge counts, artifact presence, refresh/root-switch controls, and optional search results with `열기`, `다운로드`, and `폴더 열기` actions. Management POST actions require a page-local token; file actions reject path traversal and files outside the explicit root.

Cleanup generated artifacts from one root:

```bash
jikji clean /explicit/root --dry-run --json
jikji clean /explicit/root --json
```

## 4. What Jikji creates

Jikji does not reorganize user files. It creates generated artifacts only under
one explicit root:

```text
.jikji_agent_map.md          visible route guide for humans/agents
.jikji/agent_routes.md          fallback route order for agents
.jikji/file_index.jsonl         file names, paths, size/time/hash metadata
.jikji/folder_index.jsonl       folder inventory and counts
.jikji/document_index.jsonl     parser-required documents and parse status
.jikji/file_cards.jsonl         map-facing file cards and search hints
.jikji/chunk_map.jsonl          bounded document/body clues
.jikji/doc_text/                extracted PDF/HWP/HWPX/Office/etc. text cache
.jikji/search_index.sqlite      Everything-style instant search accelerator
.jikji/duplicate_map.jsonl      hash/family groups for duplicate-aware search
.jikji/wiki/index.md          local LLM Wiki entry point
.jikji/knowledge_graph.json   source/folder/term/intent graph
.jikji/graph_routes.jsonl     low-token compact route rows
```

These artifacts are disposable and may be regenerated. For Git repositories,
add:

```gitignore
.jikji/
.jikji_agent_map.md
```

## 5. Direct handoff mode

Jikji is most useful when an agent treats it as a tool, not as a folder to read
manually. In direct handoff, the agent asks Jikji for ranked candidates and
returns/uses those candidates without doing a new broad filesystem crawl.

Benchmark command:

```bash
jikji hermes-bench /benchmark/root \
  --eval-set /external/eval.jsonl \
  --modes raw,jikji-fast,jikji-direct \
  --candidate-top-k 10 --json
```

Mode meanings:

```text
raw           agent searches original files/folders without Jikji
jikji-fast    agent receives a compact Jikji candidate handoff
jikji-direct  agent/tool accepts Jikji ranked candidates without an extra exploratory chat turn
```

On the current 600-document Korean public-document hardbench sample,
`jikji-direct` preserved the improved Hit@10 while reducing average discovery
latency from raw Hermes `91.571s` per case to `0.800s` per case on a 4-case
slice.

## 6. Multi-root and local drives

Jikji can prepare any path the agent can read, including mounted drives and cloud
sync folders. Run it per explicit root:

```bash
jikji prepare /mnt/work-drive --json
jikji prepare ~/CloudDrive --json
jikji brief ~/CloudDrive "invoice from vendor in spring" --json
```

Do not default to scanning all drives. Ask for or infer a bounded root from the
user's request.

## 7. Privacy and security

- `.jikji/doc_text/` can contain extracted document text.
- `.jikji/*.jsonl` can contain file names, paths, timestamps, and hashes.
- Defaults skip hidden files and sensitive names such as `.env`, keys,
  certificates, `.git`, virtualenvs, and large dependency/cache folders.
- Jikji does not bypass filesystem permissions.
- Parser failures are recorded instead of hiding missing coverage.

## 8. Recommended agent prompt snippet

```text
Use Jikji for local file discovery when an explicit root is available.
First call: jikji brief ROOT "query" --top-k 10 --json.
Prefer returned candidate paths and evidence over broad filesystem crawling.
Only inspect original files for final verification.
If candidates are insufficient, follow the brief route: Jikji indexes, doc_text,
then original text files excluding .jikji. Never move, rename, delete, or
reorganize user files.
```
