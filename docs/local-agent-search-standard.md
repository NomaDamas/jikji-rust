# Local Agent Search Standard

This standard defines how local agents should use Jikji for file discovery.

## Product Boundary

Jikji prepares local folders for AI-agent discovery without moving, renaming, or
deleting user files. It creates generated artifacts under `.jikji/` plus
`.jikji_agent_map.md`.

## Public Discovery Command

```bash
jikji find ROOT "query" --json
```

`jikji find` is the only public local-file discovery command agents should use.
It builds a multi-query, multi-route candidate slate from:

- file and folder metadata
- parser text caches
- file cards and route rows
- LLM Wiki source pages
- knowledge-graph routes
- local lexical/content indexes

The agent then uses the returned top-k slate for bounded verification or one LLM
judgment, instead of running repeated raw filesystem searches.

`jikji find` does not build or refresh indexes. It searches prepared Jikji
artifacts only. When the requested root is not prepared yet, agents should tell
the user and ask before running `jikji prepare ROOT --json`.

Jikji uses RAG-style local retrieval context, but it is not a mandatory
embedding/vectorDB/cloud RAG stack. The default search path is local and
deterministic; the agent may spend an LLM call only to judge the returned slate.

## Required Handoff Contract

- Prefer `answer_paths[]`, then `paths[]`.
- Preserve Jikji order when `agent_should_not_rerank` is true.
- Use `evidence_pack[].next_read` or `candidates[].next_read` for verification.
- `direct_use`: no broad raw crawl.
- `jikji_retry`: exactly one sharper `jikji find` retry.
- `raw_fallback_after_retry`: raw search only after the retry failed or remained
  clearly wrong.

## Administrative Commands

```bash
jikji prepare ROOT --json
jikji refresh ROOT --json
jikji doctor ROOT --json
jikji map ROOT
jikji clean ROOT --dry-run --json
```

Agent skill installation may queue background prepare work for common user
material folders and document-heavy folders under the user's home directory.
That initial fast-start indexing is separate from `jikji find`.

## Reporting Standard

Public benchmark rows should compare:

```text
raw local agent
same agent + Jikji find
```

Do not expose internal experiment names as product options in public benchmark
tables or agent instructions.
