"""
Configuration and constants for the Cyl documentation generator.
"""

import os
from jinja2 import Environment, FileSystemLoader

# Project paths
PROJECT_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), '../../..'))
TEMPLATE_DIR = os.path.join(os.path.dirname(__file__), '../templates')
OUTPUT_DIR = os.path.join(PROJECT_ROOT, 'docs/website')
ASSETS_DIR = os.path.join(OUTPUT_DIR, 'assets')

# Initialize Jinja2 environment
env = Environment(loader=FileSystemLoader(TEMPLATE_DIR))

# Default configuration
DEFAULT_CONFIG = {
    "site": {"title": "Cyl", "description": "A programming language"},
    "branding": {"favicon_path": "icon.png"},
    "footer": {"copyright": "2025 Cyl Programming Language"}
}
