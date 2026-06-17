# Jikji LLM Wiki / knowledge graph layer

Jikji now compiles a deterministic local LLM Wiki and knowledge graph during `jikji prepare`. The goal is to reduce local-agent file-discovery turns: agents should receive a small ranked route sheet first instead of reading the whole map, crawling folders, or searching every generated JSONL artifact.

## Design

The design follows the recent LLM Wiki pattern used by projects such as SwarmVault and Karpathy-style LLM wiki notes:

1. **Raw sources** remain in their original local paths. Jikji never moves, renames, or deletes them.
2. **Extracted text/cache** lives under `.jikji/doc_text/` for parser-supported documents and media.
3. **Markdown wiki** lives under `.jikji/wiki/`:
   - `.jikji/wiki/index.md`
   - `.jikji/wiki/sources/*.md`
4. **Knowledge graph** lives at `.jikji/knowledge_graph.json` with corpus, folder, source, term, intent, and duplicate-group nodes.
5. **Low-token routes** live at `.jikji/graph_routes.jsonl`, one compact route row per source.

This implementation is fully local and deterministic. It does not require LLM calls, embeddings, cloud APIs, vector databases, or network access.

## Agent protocol

Use the compact route first:

```bash
jikji brief /path/to/root "natural language clue" --top-k 10 --compact --json
```

The compact payload returns:

- `candidates[].p`: original relative path.
- `candidates[].wiki`: compact source wiki page.
- `candidates[].cache`: parser text cache when available.
- `candidates[].terms` / `intents`: graph evidence.
- `candidates[].ev`: bounded evidence preview.

Only when the compact route is empty or ambiguous should the agent read `.jikji/knowledge_graph.json`, `.jikji/graph_routes.jsonl`, full `brief`, or older map JSONL artifacts.

## Deterministic-first retrieval loop

For a single file lookup, the ideal path is zero LLM calls:

```bash
jikji find /path/to/root "user request text" --first
```

Jikji now indexes each file as a fielded document:

- path / folder path
- filename / extension
- extracted body text (`doc_text` or native text)
- metadata tags / summaries / format hints
- deterministic semantic text (`content_terms`, `rare_terms`, `phrase_signatures`, `intent_tags`, evidence previews)

The instant SQLite index stores field term frequencies, field lengths, field IDF, and average field lengths. Search uses field-weighted BM25 first, then Jikji's map/card scoring and duplicate/path heuristics. LLMs should only enter the loop after deterministic `find`/`brief --compact` fails: generate query variants, run Jikji again for each variant, merge top-n, then judge whether a candidate is sufficient.

## Graph exploration commands

Jikji also exposes explicit graph inspection commands for agents or humans who want the LLM Wiki-style traversal layer directly:

```bash
jikji graph status /path/to/root --json
jikji graph query /path/to/root "contract payment clause" --top-k 10 --json
jikji graph explain /path/to/root "contracts/ACME_2026_contract.txt" --json
```

- `graph status` reports wiki/graph artifact paths and graph stats.
- `graph query` searches compact graph routes without reading full JSONL maps.
- `graph explain` returns the selected source route and its graph neighbors.

## Token/call reduction benchmark

A deterministic 160-file synthetic local corpus was generated in a temporary directory with 10 topic queries and `top-k=10`. The benchmark compared existing full JSON `brief` output against the new minified compact graph brief. Both modes used the same search ranking; the metric is prompt-size proxy before any LLM call.

```text
cases: 10
avg full brief chars:    12,972.7
avg compact brief chars:  4,959.1
reduction:               61.8%
same top candidate:     100.0%
```

A smaller smoke corpus showed 3,710 chars → 1,330 chars before minification, a 64.2% reduction, with the same top path and valid wiki/graph artifacts.

## Why this cuts agent calls

Old fallback behavior encouraged agents to read `.jikji_agent_map.md`, `.jikji/agent_routes.md`, `file_cards.jsonl`, `chunk_map.jsonl`, `folder_profile.jsonl`, and sometimes original folders before committing to a path. The compact graph route gives a direct candidate list plus enough evidence to avoid that exploratory loop.

Expected local-agent flow:

```text
one compact brief call → return candidate path or verify top file
```

instead of:

```text
read map → read routes → grep JSONL → grep doc_text → list/open folders → retry query
```

## Safety

All wiki and graph artifacts are generated under `.jikji/` and are listed in `manifest.json` ownership. They may contain source-derived snippets, so they should be treated like `.jikji/doc_text/` and not committed unless the user explicitly wants generated artifacts tracked.
