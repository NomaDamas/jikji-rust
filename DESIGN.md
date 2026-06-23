# Jikji Design Notes

Jikji is an operator-grade local-agent search tool, not a consumer organizer or marketing toy. Public pages should feel like a benchmark-backed command surface: dense enough for engineers, clear enough for a first install decision.

## Visual Direction

- Base palette: near-black background, restrained cyan for file-search affordances, green for verified savings, amber only for warnings or raw-agent waste.
- Avoid purple-led gradients, beige/brown palettes, soft decorative blobs, and card-heavy marketing layouts.
- Prefer full-width sections with constrained content. Cards are allowed for repeated metric blocks and report summaries only.
- Use tabular numerals, compact labels, and plain benchmark evidence.

## Messaging

- Headline comparison must be raw local agent vs the same agent with Jikji find attached.
- Accuracy comes before token reduction. Do not headline a mode that is below raw Hermes on Hit@1 or Hit@10.
- Public file-discovery naming is `Jikji find`; older internal benchmark keys are provenance only and should not appear on public pages.
- Historical NomaDamas/gpt-5.4-mini results can stay as provenance, but new benchmark instructions must omit provider/model flags and use the current Hermes account default GPT/model.

## Landing Page

- First viewport should immediately show `파일 하나 찾을 때마다 ... 실화냐?` with rotating token/time/call/cost waste expressed per single file-search case, without wrapping the rotating hook in literal parentheses or using the word "평균".
- Fullset totals such as 6,420 calls, 21,296,278 tokens, 31,231.9 seconds, and 13,361원 belong in evidence tables only, not in the per-file hook.
- Keep the user-facing proof short: Hit@1, Hit@10, calls, input/output tokens, elapsed time, and estimated cost.
- The hook may use provocative copy, but nearby evidence must state the actual observed full-set numbers: raw max 45 calls per case, 6,420 total calls, 21,296,278 total tokens.
- The first viewport must include a prominent GitHub CTA and a one-line CLI-agent install CTA. Public serving language should point at GitHub Pages static hosting, not a local tunnel.

## Benchmark Pages

- Prefer `Jikji find` for public headline rows.
- Show raw Hermes and the same agent with Jikji find attached. Deterministic raw-vs-index rows are secondary diagnostics.
- Internal experiment names may remain in machine-readable historical artifacts, but not in human-facing benchmark tables.
