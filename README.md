# Jikji

<p align="center">
  <a href="https://nomadamas.github.io/jikji/">
    <img src="docs/jikji-readme-hero.svg" alt="Jikji — local file maps for AI agents" width="100%" />
  </a>
</p>

## English

**Jikji prepares local folders for AI-agent discovery without moving, renaming, or deleting user files.**

Raw local agents often waste turns guessing search terms, listing folders, grepping files, and opening documents one by one. Jikji gives them a prebuilt map: ranked paths, evidence snippets, folder context, document text caches, and safe fallback routes.

```bash
git clone https://github.com/nomadamas/jikji.git
cd jikji
python3 -m venv .venv
.venv/bin/pip install -e .
.venv/bin/jikji brief ~/Documents "contract pdf from last spring" --top-k 10 --json
```

## 한국어

**직지(Jikji)는 로컬 에이전트가 파일을 잘 찾도록 폴더를 지도화합니다. 원본 파일을 옮기거나, 이름을 바꾸거나, 삭제하지 않습니다.**

그냥 에이전트는 검색어를 추측하고, 폴더를 뒤지고, 문서를 하나씩 열어보며 헤맬 수 있습니다. 직지는 미리 만든 지도와 인덱스를 제공합니다: 후보 경로, 근거 스니펫, 폴더 맥락, 문서 텍스트 캐시, 안전한 다음 탐색 경로.

```bash
jikji brief ~/Documents "작년 봄 계약서 PDF" --top-k 10 --json
jikji search ~/Documents "파일명, 본문 단서, 문서 설명" --top-k 10 --json
```

## What Jikji creates / 직지가 만드는 것

```text
000_JIKJI_AGENT_MAP.md      visible route guide / 루트 지도
.jikji/search_index.sqlite  instant search index / 즉시 검색 인덱스
.jikji/doc_text/            parsed document text / 문서 본문 텍스트 캐시
.jikji/file_cards.jsonl     per-file cards and evidence / 파일별 단서 카드
.jikji/folder_profile.jsonl folder context / 폴더 맥락
.jikji/agent_routes.md      safe fallback routes / 안전한 탐색 경로
```

## Agent protocol / 에이전트 사용 규칙

1. Use an explicit root. Do not scan every drive by default.  
   명시된 루트만 사용하고, 모든 드라이브를 기본 스캔하지 않습니다.
2. Call `jikji brief ROOT "query" --json` first.  
   먼저 `jikji brief ROOT "질문" --json`을 호출합니다.
3. Prefer returned candidate paths when the evidence matches.  
   근거가 맞으면 반환된 후보 경로를 우선 사용합니다.
4. Open original files only for final verification.  
   원본 파일은 마지막 확인용으로만 엽니다.
5. Never move, rename, delete, or reorganize source files.  
   원본 파일을 이동/이름변경/삭제/재정리하지 않습니다.

## Install as a skill / 스킬로 장착하기

Use `skills/jikji/SKILL.md` as the reusable skill instruction for Claude Code, Codex, Hermes, OpenCode/OpenClone-style agents, or any CLI-capable local agent.

Claude Code, Codex, Hermes, OpenCode/OpenClone 같은 로컬 에이전트에 `skills/jikji/SKILL.md`를 스킬 지침으로 넣으면 됩니다.

```bash
jikji hermes-skill-install --json   # Hermes convenience installer
```

## Docs

- [Agent installation manual](docs/agent-installation.md)
- [Local-agent search standard](docs/local-agent-search-standard.md)
- [Promo page](https://nomadamas.github.io/jikji/) / [source](docs/jikji-value.html)
- [Hardbench benchmark notes](docs/hardbench-benchmark.md)

## Development

```bash
python3 -m venv .venv
.venv/bin/pip install -e .
.venv/bin/pip install pytest ruff
.venv/bin/ruff check src tests
.venv/bin/pytest -q
.venv/bin/python -m compileall -q src tests
```
