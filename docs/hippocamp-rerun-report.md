# HippoCamp Benchmark Re-run Report (Full split)

This report captures the official benchmark re-run on the HippoCamp **Full
split (`Fullset`)** — the complete per-profile corpus and QA set — instead of
the earlier bounded `*_Subset` slice.

1. The visible root agent map is a hidden dotfile (`.jikji_agent_map.md`).
2. The agent skill (`SKILL.md`) forces a Jikji search-first protocol.
3. The Hermes agent benchmark tracks LLM call count and **separate input
   (prompt) / output (completion) token usage** per task.
4. The ranking core uses filename component tokenization, rarity-sharpened
   BM25, and Contextual Anchor Weighting.
5. **Token diet:** the Jikji→agent handoff is a compact map-first pass —
   `candidate-top-k` 5, a **single** evidence snippet per candidate
   hard-truncated to **120 chars**, and a bounded 1-turn map handoff.

Profiles: **Adam, Bei, Victoria** (HippoCamp **`Fullset`**). The deterministic
suite scores the **full 551 cases** (Adam 122, Bei 206, Victoria 223); the
real-agent suite uses 10 cases/profile (30 total) as a product sanity check.

## 0. Bounded full-set fetch

The Full corpus is downloaded with a bounded workflow: a 60 MB per-file cap
skips only large video so the **entire document + audio corpus** is captured.

| Profile  | files | corpus bytes | skipped (>60MB video) | eval cases |
|----------|-------|--------------|-----------------------|------------|
| Adam     | 342   | 355.7 MB     | 2   | 122 |
| Bei      | 826   | 4,102.9 MB   | 49  | 206 |
| Victoria | 705   | 2,346.7 MB   | 1   | 223 |

## 1. Deterministic suite (lexical scorer, no LLM)

Aggregate over **551 cases** (`jikji` = Jikji-assisted scorer, `raw` = naive
filesystem lexical search), top_k=10.

| Metric  | raw    | jikji  |
|---------|--------|--------|
| Hit@1   | 0.2850 | **0.3666** |
| Hit@3   | 0.3847 | **0.5045** |
| Hit@5   | 0.4465 | **0.5789** |
| MRR     | 0.3606 | **0.4587** |

Per-profile (top_k=10):

| Profile  | mode  | cases | Hit@1 | Hit@3 | Hit@5 | Hit@10 | SetR@10 | MRR |
|----------|-------|-------|-------|-------|-------|--------|---------|-----|
| Adam     | raw   | 122 | 0.5410 | 0.6721 | 0.7049 | 0.7623 | 0.6365 | 0.6154 |
| Adam     | jikji | 122 | **0.7213** | **0.8361** | **0.8443** | **0.9098** | **0.7796** | **0.7850** |
| Bei      | raw   | 206 | 0.2427 | 0.3010 | 0.3301 | 0.4078 | 0.3284 | 0.2857 |
| Bei      | jikji | 206 | **0.2864** | **0.4126** | **0.4757** | **0.5340** | **0.4094** | **0.3626** |
| Victoria | raw   | 223 | 0.1839 | 0.3049 | 0.4126 | 0.6457 | 0.5553 | 0.2904 |
| Victoria | jikji | 223 | **0.2466** | **0.4081** | **0.5291** | **0.7399** | **0.6325** | **0.3690** |

## 2. Hermes real-agent benchmark (token-diet, 10 cases/profile)

Model: `google/gemini-2.5-flash` via `openrouter` — the current low-cost,
high-capability flash model that replaces the legacy `openai/gpt-4o-mini` agent
benchmark model; **10 cases/profile** (30 total);
`raw` max-turns 8; `jikji` = token-diet map-first 1-turn handoff
(`--candidate-top-k 5`, single 120-char evidence snippet). `raw` = agent must
browse the full corpus; `jikji` = agent receives the compact Jikji candidate
list first.

Input/output tokens are reported separately (prompt = input, completion =
output).

| Profile  | mode  | Hit@10 | llm_calls | input (prompt) | output (completion) | total tokens |
|----------|-------|--------|-----------|----------------|---------------------|--------------|
| Adam     | raw   | 0.100 | 50  | 403,970 | 31,807 | 435,777 |
| Adam     | jikji | **0.800** | **10** | **89,670** | **4,724** | **94,394** |
| Bei      | raw   | 0.300 | 27  | 152,520 | 17,169 | 169,689 |
| Bei      | jikji | **0.500** | **10** | **85,685** | **7,520** | **93,205** |
| Victoria | raw   | 0.600 | 29  | 210,216 | 30,408 | 240,624 |
| Victoria | jikji | **0.800** | **10** | **103,325** | **6,325** | **109,650** |

Aggregate (30 cases/mode):

| mode  | Hit@10 | llm_calls | input (prompt) | output (completion) | total tokens |
|-------|--------|-----------|----------------|---------------------|--------------|
| raw   | 0.333 | 106 | 766,706 | 79,384 | 846,090 |
| jikji | **0.700** | **30** | **278,680** | **18,569** | **297,249** |

Observations:

- **Token diet works on the full corpus.** With Jikji the agent answers in a
  single bounded turn, so total tokens fall **846,090 → 297,249 (−64.9%, 2.85×)**
  and input tokens fall **766,706 → 278,680 (−63.7%)** versus raw browsing of the
  full Full-split corpus.
- **LLM calls drop ~3.5×** (106 → 30, i.e. 1 call/case) — the dominant driver
  of the token savings.
- **Accuracy improves** in aggregate (Hit@10 0.333 → 0.700; accuracy
  0.333 → 0.700). Adam's raw run scores 1/10 because the agent struggles to
  locate evidence by browsing the much larger Full corpus within 8 turns, while
  Jikji candidates recover it (0.800). Victoria's raw is the strongest raw
  profile (0.600) and Jikji lifts it to 0.800 while using ~3× fewer calls and
  ~2.2× fewer tokens.
- **Harder than Subset.** The Full split is a substantially larger and harder
  corpus (551 deterministic cases vs 49), so absolute scores are lower than the
  Subset re-run while Jikji's relative lift holds across every profile.

`llm_calls`, `prompt_tokens`, and `completion_tokens` are read per session from
the Hermes session store (`state.db` + session transcript) keyed on the
`session_id` emitted by each `hermes chat` invocation, then summed per task
and per mode into the JSON report.

## Reproduce

```bash
# Deterministic suite (full 551 cases; bounded full-set fetch)
jikji hippocamp-suite .benchmarks/hippocamp-full \
  --profiles Adam,Bei,Victoria --split Fullset \
  --max-files 5000 --max-file-bytes 62914560 --max-total-bytes 9663676416 \
  --cases 2000 --top-k 10

# Real-agent benchmark with token diet (per profile, 10 cases)
jikji hermes-bench .benchmarks/hippocamp-full/Adam \
  --eval-set .benchmarks/hippocamp-full/Adam_hippocamp_eval_set.jsonl \
  --modes raw,jikji-fast --cases 10 --max-turns 8 --fast-max-turns 1 \
  --candidate-top-k 5 --skills jikji \
  --provider openrouter --model google/gemini-2.5-flash --json
```

Machine-readable aggregate: [`hippocamp-rerun-report.json`](./hippocamp-rerun-report.json).
