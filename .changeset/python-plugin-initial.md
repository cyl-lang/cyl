---
"cyl": minor
---

Initial implementation of Python plugin support:

- Added plugin loading infrastructure for Python plugins.
- Created example plugin files in `plugins/` (e.g., `example_plugin.py`).
- Integrated plugin loader with the design/compiler workflow (early stage).
- No breaking changes to the CLI or public API yet, but this lays the foundation for extensibility via Python plugins.

Further work will expand plugin APIs and documentation.
