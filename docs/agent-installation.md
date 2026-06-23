# Agent Installation

Jikji is a non-destructive local file-discovery layer for AI agents. It prepares
an explicit root into `.jikji/` maps, parser caches, and search artifacts. It
does not move, rename, delete, or reorganize user files.

## Install From GitHub

### One-sentence agent install

CLI 에이전트에게 가장 쉽게 시킬 한 문장:

```text
GitHub 저장소 https://github.com/NomaDamas/jikji 에서 Jikji를 설치하고, 내 CLI 에이전트들이 `jikji find`를 바로 쓰도록 Jikji skill까지 연결해줘.
```

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

## Install The Skill

```bash
jikji agent-skill-install --agent all --json
jikji hermes-skill-install --json
jikji codex-skill-install --json
jikji claude-skill-install --json
jikji opencode-skill-install --json
```

For an arbitrary local agent:

```bash
jikji skill-export --dest /path/to/agent/skills/jikji/SKILL.md --json
```

## Required Agent Behavior

When an explicit root is available, local file/folder/document discovery starts
with:

```bash
jikji find ROOT "natural language file clue" --json
```

The agent follows `handoff_action`:

- `direct_use`: use `answer_paths[]` / `paths[]`; verify only top evidence.
- `jikji_retry`: run exactly one sharper `jikji find` retry.
- `raw_fallback_after_retry`: raw search is allowed only after that retry failed,
  stayed empty, or stayed clearly wrong.

Do not start by crawling with `ls`, `find`, `rg`, `grep`, `tree`, or broad
document opening.

## Optional Root Preparation

Jikji never silently scans Documents, Downloads, Desktop, or cloud-sync folders.
Prepare explicit roots only:

```bash
jikji prepare /mnt/work-drive --json
jikji refresh /mnt/work-drive --json
jikji doctor /mnt/work-drive --json
```

Skill install can queue or run preparation when the user provides a root:

```bash
jikji agent-skill-install --agent all --prepare-root /mnt/work-drive --json
jikji agent-skill-install --agent all --prepare-root /mnt/work-drive --foreground-prepare --json
jikji agent-skill-install --agent all --no-prepare --json
```

## Benchmark Language

Use the public label `Jikji find` in reports and dashboards. Headline comparisons
should be raw local agent vs the same agent with Jikji find attached. Internal
benchmark implementation names should not appear in user-facing setup guidance.
