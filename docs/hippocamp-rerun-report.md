# HippoCamp Benchmark Re-run Report (Fullset)

This page preserves the human-readable summary of the HippoCamp Fullset re-run
using the current public product language: **raw Hermes vs the same Hermes with
Jikji find attached**.

## Corpus

Profiles: Adam, Bei, Victoria. Fullset total: 551 cases.

```text
Profile   Files  Corpus bytes  Eval cases
--------  -----  ------------  ----------
Adam        342      355.7 MB         122
Bei         826    4,102.9 MB         206
Victoria    705    2,346.7 MB         223
```

## Deterministic Suite

No LLM calls. This is a search-layer regression signal, not the public agent
headline.

```text
Mode   Hit@1   Hit@3   Hit@5   MRR
-----  ------  ------  ------  ------
raw    0.2850  0.3847  0.4465  0.3606
Jikji  0.3666  0.5045  0.5789  0.4587
```

## Hermes Fullset Agent Result

Historical run provenance: gpt-5.4-mini via the old custom account. New Hermes
re-runs should omit provider/model flags and use the current account default.

```text
Mode         Cases  Hit@1   Hit@10  LLM calls  Input tokens  Output tokens  Total tokens  Seconds
-----------  -----  ------  ------  ---------  ------------  -------------  ------------  ---------
raw Hermes     551  0.6697  0.7786      6,420    19,799,362      1,496,916    21,296,278   31,231.9
Jikji find     551  0.7949  0.7949        551       228,684         17,632       246,316    1,164.2
```

Gate result:

```text
Hit@1 not lower:          PASS (+0.1252)
Hit@10 not lower:         PASS (+0.0163)
LLM calls ratio:          0.0858  (-91.42%)
Total token ratio:        0.0116  (-98.84%)
Seconds ratio:            0.0373  (-96.27%)
```

Interpretation: Jikji find builds a multi-query, multi-route top-k candidate
slate from file maps, metadata, parser caches, and graph routes. The agent then
judges that slate instead of spending repeated raw filesystem-search turns.

## Reproduce

```bash
jikji hippocamp-suite .benchmarks/hippocamp-full \
  --profiles Adam,Bei,Victoria --split Fullset \
  --max-files 5000 --max-file-bytes 62914560 --max-total-bytes 9663676416 \
  --cases 2000 --top-k 10

jikji hermes-bench .benchmarks/hippocamp-full/Adam \
  --eval-set .benchmarks/hippocamp-full/Adam_hippocamp_eval_set.jsonl \
  --modes raw,jikji --cases 10 --max-turns 8 \
  --candidate-top-k 5 --skills jikji --json
```

Public detailed report: [`jikji-benchmarks.html`](./jikji-benchmarks.html).
