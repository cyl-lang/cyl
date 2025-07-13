# Cyl Docs Generator Python Environment

## Setup Instructions

1. **Create a virtual environment (recommended):**

   ```bash
   cd docs/generator
   python3 -m venv .venv
   source .venv/bin/activate
   ```

2. **Install dependencies:**

   ```bash
   pip install -r requirements.txt
   ```

3. **Run the generator:**
   ```bash
   python generate-docs.py
   ```

---

- All Python dependencies are kept in `docs/generator/requirements.txt`.
- The virtual environment is local to `docs/generator` and does not affect your project root.
