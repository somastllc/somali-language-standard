# Lexicon data

The core dictionary lives in [`core/`](core/), partitioned by Somali initial
letter or digraph. Its record policy is
[SLS-0100](../../spec/lexicon/0100-dictionary-standard.md); decisions are recorded
in the [maintainer review log](../../docs/standards/SLS-0100-review-log.md).
Each entry carries a permanent ID, bilingual definitions, and provenance.
Use `metadata.review_status` to distinguish reviewed records from drafts.

## Reserved files

| File | Purpose | Validation state |
|---|---|---|
| [`loanwords.jsonl`](loanwords.jsonl) | Reserved loanword-entry file | Routed to the existing lexicon-entry schema; currently empty |
| [`morphology/noun-paradigms.jsonl`](morphology/noun-paradigms.jsonl) | Reserved noun paradigms | Empty; no morphology schema or validator route yet |
| [`morphology/verb-conjugations.jsonl`](morphology/verb-conjugations.jsonl) | Reserved verb conjugations | Empty; no morphology schema or validator route yet |

These files are zero-byte skeletons and contribute no records to the reviewed
seed count. Do not insert comments, blank lines, or example records into them.
Core entries already include reviewed loanwords; do not copy those entries or
reuse their IDs in `loanwords.jsonl`. IDs are unique across routed datasets.
Before populating morphology files, define their record contracts and add
schema routing; passing today's repository check does not validate morphology
data. Frequency data remains separate future work.

## Review and validation

The [reviewer packet](../../docs/standards/SLS-0100-reviewer-packet.md) holds
unapproved candidates. Create new reviewed records only after their required
fields and exact bilingual definitions have recorded maintainer approval.
Allocate IDs after the highest assigned ID, including draft records, and
preserve IDs when correcting records.

From the repository root, run:

```sh
cargo run --offline -q -p sls-validators -- check --root .
```

The command checks routed record schemas, duplicate IDs, and supported
references. Linguistic approval is documented separately in the review log.
