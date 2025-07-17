---
"cyl": minor
---

**Workflow and scripts cleanup:**

- Removed legacy and redundant scripts from `package.json` and the `scripts/` directory.
- Consolidated build, test, and docs scripts to use only the current, working workflow.
- Updated `package.json` scripts for clarity and correctness.
- Ensured all local workflows (build, test, docs, versioning) work as expected before pushing.
- No breaking changes to the public API or CLI, but developer experience and CI reliability are improved.

**Details:**
- Only one version sync script is now used (`update-version.sh`).
- Python setup scripts and diagnostic scripts are retained only if still relevant to current workflows.
- `.changeset/` updated to reflect these changes.
