"""
Utility functions for the documentation generator.
"""

import os
import json
import shutil
from datetime import datetime

# Get paths relative to this file
_current_dir = os.path.dirname(__file__)
PROJECT_ROOT = os.path.abspath(os.path.join(_current_dir, '../../../..'))
TEMPLATE_DIR = os.path.join(_current_dir, '../templates')
OUTPUT_DIR = os.path.join(PROJECT_ROOT, 'docs/website')
ASSETS_DIR = os.path.join(OUTPUT_DIR, 'assets')


# Default configuration
DEFAULT_CONFIG = {
    "site": {"title": "Cyl", "description": "A programming language"},
    "branding": {"favicon_path": "icon.png"},
    "footer": {"copyright": "2025 Cyl Programming Language"},
    "navigation": {
        "main_nav": [
            {"name": "Home", "url": "index.html"},
            {"name": "Syntax", "url": "syntax.html"},
            {"name": "Examples", "url": "examples.html"},
            {"name": "Coverage", "url": "coverage.html"},
            {"name": "Changelog", "url": "changelog.html"},
            {"name": "Backends", "url": "backends.html"}
        ]
    },
    "content": {
        "hero": {
            "title": "Cyl Programming Language",
            "description": "A modern, efficient programming language designed for performance and simplicity.",
            "primary_button": {
                "text": "Get Started",
                "url": "syntax.html"
            },
            "secondary_button": {
                "text": "View Examples",
                "url": "examples.html"
            }
        }
    }
}


def load_config():
    """Load configuration from config.json"""
    config_path = os.path.join(_current_dir, '../config.json')
    try:
        with open(config_path, 'r') as f:
            return json.load(f)
    except FileNotFoundError:
        print("⚠️  config.json not found, using defaults")
        return DEFAULT_CONFIG
    except json.JSONDecodeError as e:
        print(f"⚠️  Error parsing config.json: {e}")
        return DEFAULT_CONFIG


def get_version():
    """Extract version from package.json"""
    try:
        package_path = os.path.join(PROJECT_ROOT, 'package.json')
        with open(package_path, 'r') as f:
            package = json.load(f)
            return package.get('version', '1.0.0')
    except Exception:
        return '1.0.0'


def determine_rule_type(category_id):
    """Determine the type of a syntax rule based on category and name"""
    type_mapping = {
        'declarations': 'declaration',
        'statements': 'statement', 
        'expressions': 'expression',
        'control_flow': 'control_structure',
        'types': 'primitive_type',
        'operators': 'operator',
        'comments': 'comment'
    }
    return type_mapping.get(category_id, 'syntax_rule')


def copy_coverage_files():
    """Copy coverage files to the website directory for accessibility"""
    print("📊 Copying coverage files...")
    
    coverage_source = os.path.join(PROJECT_ROOT, 'coverage')
    coverage_dest = os.path.join(OUTPUT_DIR, 'coverage')
    
    if os.path.exists(coverage_source):
        try:
            # Remove existing coverage directory if it exists
            if os.path.exists(coverage_dest):
                shutil.rmtree(coverage_dest)
            
            # Copy the entire coverage directory
            shutil.copytree(coverage_source, coverage_dest)
            print("✅ Coverage files copied to website directory")
        except Exception as e:
            print(f"⚠️  Could not copy coverage files: {e}")
    else:
        print(f"⚠️  Coverage source directory not found: {coverage_source}")


def create_assets():
    """Copy external CSS and JS assets and favicon"""
    print("🎨 Creating assets...")
    
    os.makedirs(ASSETS_DIR, exist_ok=True)
    
    # Copy favicon if it exists
    favicon_source = os.path.join(PROJECT_ROOT, 'icon.png')
    if os.path.exists(favicon_source):
        # Copy as both .ico and .png for better browser support
        favicon_ico_dest = os.path.join(OUTPUT_DIR, 'favicon.ico')
        favicon_png_dest = os.path.join(OUTPUT_DIR, 'favicon.png')
        try:
            shutil.copy2(favicon_source, favicon_ico_dest)
            shutil.copy2(favicon_source, favicon_png_dest)
            print("✅ Copied favicon (both .ico and .png)")
        except Exception as e:
            print(f"⚠️  Could not copy favicon: {e}")
    
    # Copy external CSS file from templates/assets
    css_source = os.path.join(TEMPLATE_DIR, 'assets', 'style.css')
    css_dest = os.path.join(ASSETS_DIR, 'style.css')
    
    if os.path.exists(css_source):
        try:
            shutil.copy2(css_source, css_dest)
            print("✅ Copied external CSS file")
        except Exception as e:
            print(f"⚠️  Could not copy CSS file: {e}")
    else:
        print(f"⚠️  CSS source file not found: {css_source}")
        print("Creating basic CSS file...")
        _create_basic_css(css_dest)
    
    # Copy external JavaScript file from templates/assets
    js_source = os.path.join(TEMPLATE_DIR, 'assets', 'script.js')
    js_dest = os.path.join(ASSETS_DIR, 'script.js')
    
    if os.path.exists(js_source):
        try:
            shutil.copy2(js_source, js_dest)
            print("✅ Copied external JavaScript file")
        except Exception as e:
            print(f"⚠️  Could not copy JavaScript file: {e}")
    else:
        print(f"⚠️  JavaScript source file not found: {js_source}")
        print("Creating basic JavaScript file...")
        _create_basic_js(js_dest)


def _create_basic_css(css_dest):
    """Create a basic CSS file if the source doesn't exist"""
    basic_css = """/* Basic Cyl Documentation Styles */
body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    line-height: 1.6;
    margin: 0;
    padding: 0;
    background: #fff;
    color: #333;
}

.container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 24px;
}

/* Header */
.header {
    background: #fff;
    border-bottom: 1px solid #eee;
    position: sticky;
    top: 0;
    z-index: 100;
}

.nav {
    display: flex;
    justify-content: space-between;
    align-items: center;
    height: 64px;
}

.logo {
    font-size: 20px;
    font-weight: 700;
    text-decoration: none;
    color: #333;
}

.nav-links {
    display: flex;
    gap: 32px;
}

.nav-links a {
    text-decoration: none;
    color: #666;
    font-weight: 500;
    transition: color 0.2s;
}

.nav-links a:hover,
.nav-links a.active {
    color: #0969da;
}

/* Main content */
.main {
    padding: 48px 0;
}

/* Code blocks */
.code,
pre {
    background: #f6f8fa;
    border: 1px solid #d1d9e0;
    border-radius: 6px;
    padding: 16px;
    overflow-x: auto;
    font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, 'Courier New', monospace;
    font-size: 14px;
}

/* Footer */
.footer {
    border-top: 1px solid #eee;
    padding: 24px 0;
    text-align: center;
    color: #666;
    font-size: 14px;
}
"""
    with open(css_dest, 'w') as f:
        f.write(basic_css)
    print("✅ Created basic CSS file")


def _create_basic_js(js_dest):
    """Create a basic JS file if the source doesn't exist"""
    basic_js = """// Basic Cyl Documentation JavaScript
document.addEventListener('DOMContentLoaded', function() {
    console.log('Cyl Documentation loaded');
    
    // Smooth scrolling for anchor links
    const anchorLinks = document.querySelectorAll('a[href^="#"]');
    
    anchorLinks.forEach(link => {
        link.addEventListener('click', function(e) {
            const href = this.getAttribute('href');
            if (href === '#') return;
            
            const targetElement = document.querySelector(href);
            if (targetElement) {
                e.preventDefault();
                targetElement.scrollIntoView({
                    behavior: 'smooth',
                    block: 'start'
                });
            }
        });
    });
    
    // Add copy functionality to code blocks
    const codeBlocks = document.querySelectorAll('pre code, .code');
    codeBlocks.forEach(block => {
        block.addEventListener('click', function() {
            if (navigator.clipboard) {
                navigator.clipboard.writeText(this.textContent);
                // Could add visual feedback here
            }
        });
    });
});
"""
    with open(js_dest, 'w') as f:
        f.write(basic_js)
    print("✅ Created basic JavaScript file")
