# Workspace-Bench-Lite adapter for Jikji

Workspace-Bench evaluates whether an agent can complete realistic workspace
output-generation tasks with cross-file dependencies. Jikji does not solve those
workspace tasks directly. This adapter turns Workspace-Bench-Lite into a
**file-discovery** benchmark: given the task instruction, can the agent identify
the source/input files needed before producing the requested output?

## Why it is relevant

Workspace-Bench is closer to Jikji's target than plain document IR benchmarks:

```text
Workspace-Bench dimension             Jikji relevance
------------------------------------  -----------------------------------------
Workspace Exploration                 folder maps, file cards, route guides
Task-supporting file utilization      ranked candidates and evidence snippets
Heterogeneous file understanding      parser text caches for Office/text files
File dependency patterns              expected_paths from data_manifest/graph
```

## What the adapter builds

```bash
jikji workspacebench-build .benchmarks/workspacebench_lite_jikji/run_20260602 \
  --max-tasks 12 --json

jikji workspacebench-suite .benchmarks/workspacebench_lite_jikji/run_20260602 \
  --max-tasks 12 --top-k 10 --json
```

The adapter downloads bounded Workspace-Bench-Lite tasks from Hugging Face,
materializes only task data files under `corpus/task_<id>/data/...`, and writes
metadata/eval files outside the corpus so agents cannot read the answer key.

Each eval case uses:

```text
query          = Workspace-Bench task instruction + "find source/input files"
expected_paths = files from file_dep_graph.from mapped through data_manifest,
                 or all data_manifest files when no graph subset is available
```

This is intentionally **not** a full Workspace-Bench scoring harness. It measures
Jikji's part of the workflow: helping the local agent discover the right files.

## 2026-06-02 bounded run

```text
Item              Value
----------------  --------
tasks             12
files             74
bytes             46,574,657
eval cases        12
prepare seconds   0.647
```

Deterministic file-discovery diagnostic:

```text
Mode                      Cases  Hit@1   Hit@3   Hit@5   Hit@10  SetR@5  SetR@10  MRR     Seconds
------------------------  -----  ------  ------  ------  ------  ------  -------  ------  -------
raw lexical diagnostic       12  0.4167  0.6667  0.7500  0.8333  0.5222   0.6861  0.5687    0.115
Jikji index diagnostic       12  0.5833  0.7500  0.9167  0.9167  0.6028   0.6944  0.6764    0.452
```

Actual Hermes agent comparison on the first 6 cases:

```text
Agent mode       Cases  Hit@1   Hit@3   Hit@5   Hit@10  Seconds  Avg sec/case
---------------  -----  ------  ------  ------  ------  -------  ------------
raw Hermes           6  1.0000  1.0000  1.0000  1.0000  249.454        41.576
Hermes + Jikji       6  0.8333  1.0000  1.0000  1.0000  203.742        33.957
```

Interpretation: on this small actual-agent subset, raw Hermes already found at
least one required source file in the top results for every case. Jikji preserved
Hit@5/Hit@10 and reduced elapsed time by about 1.22x, but it did not improve
Hit@5 because the selected cases were already easy for raw Hermes. The
12-task deterministic diagnostic still shows that Jikji's map/index layer ranks
workspace-supporting files better than a raw lexical baseline.

## Recommended use

Use Workspace-Bench-Lite as a **secondary product-fit benchmark**:

1. Run deterministic diagnostics over more Lite tasks to identify map/index
   weaknesses cheaply.
2. Run bounded actual-agent comparisons on held-out task slices.
3. Report both file-discovery metrics and full Workspace-Bench task scores only
   when a separate output-generation harness is added.
4. Do not claim Workspace-Bench leaderboard performance from this adapter; it is
   a Jikji-specific file-discovery adaptation.
