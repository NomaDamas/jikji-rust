# HippoCamp Benchmark Re-run Report

This report captures the full benchmark re-run after the four improvements landed
in this change set:

1. The visible root agent map is now a hidden dotfile (`.jikji_agent_map.md`).
2. The agent skill (`SKILL.md`) and generated skill context force a Jikji
   search-first protocol (no blind `grep`/`ls`/`find`).
3. The Hermes agent benchmark now tracks and aggregates LLM call count and
   input/output token usage per task.
4. The ranking core was upgraded (extension stopwords, filename component-word
   tokenization, rarity-sharpened BM25, and a distinctive-token headline boost).

Profiles: **Adam, Bei, Victoria** (HippoCamp `*_Subset`, 49 deterministic cases).

## 1. Deterministic suite (lexical scorer, no LLM)

Aggregate over 49 cases (`jikji` = Jikji-assisted scorer, `raw` = naive
filesystem lexical search).

| Metric  | raw (after) | jikji BEFORE | jikji AFTER |
|---------|------|--------------|-------------|
| Hit@1   | 0.5102 | 0.5714 | **0.6327** |
| Hit@3   | 0.6530 | 0.6326 | **0.7551** |
| Hit@5   | 0.6939 | 0.7551 | **0.8163** |
| MRR     | 0.6004 | 0.6434 | **0.7058** |

The "jikji BEFORE" column is the previous committed `hippocamp_suite_report.json`
aggregate; "AFTER" is the regenerated suite with the new ranking core. Every
metric improved, and Jikji now beats raw on Hit@1/Hit@3/Hit@5/MRR.

Per-profile (after, `jikji` mode, top_k=10):

| Profile  | cases | Hit@1 | Hit@3 | Hit@5 | Hit@10 |
|----------|-------|-------|-------|-------|--------|
| Adam     | 18    | 0.7778 | 0.8889 | 0.9444 | 0.9444 |
| Bei      | 21    | 0.5238 | 0.6190 | 0.6190 | 0.7619 |
| Victoria | 10    | 0.6000 | 0.8000 | 1.0000 | 1.0000 |

Root cause that the ranking upgrade fixes: filenames like
`Penguin_Model_Sheet.png` were indexed only as a joined token, so a query for
"penguin" could never match them. Component-word tokenization plus a rarity
headline boost now pull the correct document to Hit@1.

## 2. Hermes real-agent benchmark (with LLM call / token tracking)

Model: `openai/gpt-4o-mini` via `openrouter`; 3 cases/profile; `max-turns 8`;
`--candidate-top-k 10`; `--skills jikji`. `raw` = agent must browse the corpus;
`jikji` = agent receives Jikji ranked candidates first.

| Profile  | mode  | Hit@3 | Hit@10 | llm_calls | prompt_tokens | completion_tokens | total_tokens |
|----------|-------|-------|--------|-----------|---------------|-------------------|--------------|
| Adam     | raw   | 0.000 | 0.000  | 22 | 29404  | 1031 | 30435  |
| Adam     | jikji | 0.667 | 0.667  | 13 | 131623 | 1847 | 133470 |
| Bei      | raw   | 0.667 | 0.667  | 16 | 26368  | 734  | 27102  |
| Bei      | jikji | 0.667 | 1.000  | 9  | 87897  | 1807 | 89704  |
| Victoria | raw   | 1.000 | 1.000  | 14 | 16795  | 996  | 17791  |
| Victoria | jikji | 1.000 | 1.000  | 9  | 44689  | 2031 | 46720  |

Observations:

- **LLM calls drop in every profile** with Jikji (Adam 22→13, Bei 16→9,
  Victoria 14→9): the agent stops crawling the filesystem turn after turn.
- **Accuracy is equal or better** with Jikji (Adam 0.0→0.667, Bei Hit@10
  0.667→1.0, Victoria already perfect).
- `prompt_tokens` are higher in `jikji` mode because ranked candidates and
  evidence are front-loaded into the prompt — that is the intended trade: pay
  once for a prebuilt map instead of paying per exploratory turn.

`llm_calls`, `prompt_tokens`, and `completion_tokens` are read per session from
the Hermes session store (`state.db` + session transcript) keyed on the
`session_id` emitted by each `hermes chat -Q` invocation, then summed per task
and per mode into the JSON report.

## Reproduce

```bash
# Deterministic suite (downloads HippoCamp subsets on first run)
jikji hippocamp-suite .benchmarks/hippocamp-large --profiles Adam,Bei,Victoria

# Real-agent benchmark with token tracking (per profile)
jikji hermes-bench Adam_Subset \
  --eval-set Adam_Subset_hippocamp_eval_set.jsonl \
  --modes raw,jikji --cases 3 --max-turns 8 --candidate-top-k 10 \
  --skills jikji --provider openrouter --model openai/gpt-4o-mini --json
```

Machine-readable aggregate: [`hippocamp-rerun-report.json`](./hippocamp-rerun-report.json).
