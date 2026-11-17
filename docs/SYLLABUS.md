# M2 source gate — blocked

Requested official baseline: https://jeeadv.ac.in/documents/jee-advanced-2026-syllabus.pdf

On 17 November 2026 the actual HTTPS request failed with proxy `CONNECT tunnel failed, response 403`. The same request remained blocked after saving domain additions to the configuration draft. Saving a draft did not apply runtime access. The official homepage could not be checked, so existence of a 2027 edition remains unverified. No topics have been fabricated or populated from memory.

## Continuation once access is applied

1. Use `python3 scripts/fetch_syllabus.py` to download and extract every source page. It preserves the PDF, SHA-256 manifest and page text; it does not pretend extraction alone is an audited topic dataset.
2. Inspect every page. Create source-preserving section/heading/subtopic records with stable semantic IDs, exact wording, page numbers, parent section, chapter mapping, version, prerequisites and review flags for uncertain splits.
3. Audit all source page/section coverage, duplicates, exact wording, heading counts and ID stability. Preserve non-topic source spans explicitly. Require zero unexplained omissions. Do not enable "complete official syllabus" until this passes.
4. Add audited dataset fixtures to core integration tests, implement the source-linked topic workspace and transactional progress mutations; test each status after reload, linked notes/tasks, search, filters and all five progress measures independently.

Current State.topics models the eight requested statuses, but no topic UI, completed ingestion, completeness audit or progress metric is claimed. M2 is incomplete. Chapter labels in tasks/notes are user labels, not official topic IDs.
