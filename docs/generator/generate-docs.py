#!/usr/bin/env python3
"""
Cyl Language Documentation Generator

This script automatically generates documentation for the Cyl programming language
by analyzing the compiler source code, examples, and changelog information.

Features:
- Extracts syntax rules from Rust parser
- Parses examples from the examples directory
- Integrates changelog information
- Generates backend documentation
- Creates clean HTML pages using Jinja2 templates
"""

import os
import re
import json
import sys
import shutil
import xml.etree.ElementTree as ET
import traceback
from datetime import datetime

try:
    from jinja2 import Environment, FileSystemLoader
except ImportError:
    print("Error: jinja2 is not installed. Please run:")
    print("pip install -r requirements.txt")
    sys.exit(1)

# Configuration
PROJECT_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), '../..'))
TEMPLATE_DIR = os.path.join(os.path.dirname(__file__), 'templates')
OUTPUT_DIR = os.path.join(PROJECT_ROOT, 'docs/website')
ASSETS_DIR = os.path.join(OUTPUT_DIR, 'assets')

# Initialize Jinja2 environment
env = Environment(loader=FileSystemLoader(TEMPLATE_DIR))

class CylDocGenerator:
    def __init__(self):
        # Load configuration
        self.config = self.load_config()
        
        self.data = {
            'syntax': [],
            'features': [],
            'examples': [],
            'changelog': [],
            'backends': [],
            'stdlib': {},
            'meta': {
                'generated_at': datetime.now().isoformat(),
                'version': self.get_version()
            },
            'config': self.config
        }

    def get_version(self):
        """Extract version from package.json"""
        try:
            package_path = os.path.join(PROJECT_ROOT, 'package.json')
            with open(package_path, 'r') as f:
                package = json.load(f)
                return package.get('version', '1.0.0')
        except:
            return '1.0.0'

    def parse_rust_syntax(self):
        """Extract syntax information from Rust parser files with comprehensive categorization"""
        print("📝 Extracting syntax information...")
        
        # Get categories and rules from config
        syntax_categories = self.config.get('syntax', {}).get('categories', [])
        syntax_rules_config = self.config.get('syntax', {}).get('rules', {})
        
        # Define comprehensive syntax rules with categories, examples, and descriptions
        syntax_rules = []
        
        # Check multiple possible parser locations for additional context
        parser_locations = [
            os.path.join(PROJECT_ROOT, 'compiler/src/parser.rs'),
            os.path.join(PROJECT_ROOT, 'compiler/src/parser/mod.rs'),
            os.path.join(PROJECT_ROOT, 'compiler/src/parser/statements.rs'),
            os.path.join(PROJECT_ROOT, 'compiler/src/parser/expressions.rs')
        ]
        
        parser_functions = []
        for parser_path in parser_locations:
            if os.path.exists(parser_path):
                try:
                    with open(parser_path, 'r') as f:
                        content = f.read()
                    
                    print(f"✅ Found parser file: {os.path.basename(parser_path)}")
                    
                    # Extract function names that represent parsing rules
                    function_matches = re.findall(r'fn\s+(parse_\w+)', content)
                    parser_functions.extend(function_matches)
                        
                except Exception as e:
                    print(f"⚠️  Error reading {parser_path}: {e}")
        
        # Create a mapping of category IDs to category info 
        category_map = {}
        for cat in syntax_categories:
            if isinstance(cat, dict) and 'id' in cat:
                category_map[cat['id']] = cat
        
        # Build syntax rules from config dynamically
        all_categories = []
        
        # Process each category from config
        for cat in syntax_categories:
            if isinstance(cat, dict):
                cat_id = cat.get('id', '')
                cat_name = cat.get('name', '')
                
                # Get rules for this category from config
                category_rules_config = syntax_rules_config.get(cat_id, [])
                category_rules = []
                
                for rule_config in category_rules_config:
                    if isinstance(rule_config, dict):
                        rule = {
                            'name': rule_config.get('name', 'Unknown Rule'),
                            'category': cat_id,
                            'type': self._determine_rule_type(cat_id, rule_config.get('name', '')),
                            'syntax': rule_config.get('syntax', ''),
                            'description': rule_config.get('description', ''),
                            'examples': rule_config.get('examples', []),
                            'heritage': rule_config.get('heritage', '')
                        }
                        category_rules.append(rule)
                
                if category_rules:
                    all_categories.append((cat_name, category_rules))
        
        # Flatten into a single list with category information
        for category_name, category_rules in all_categories:
            for rule in category_rules:
                rule['category_name'] = category_name
                # Add category metadata from config
                cat_info = category_map.get(rule['category'], {})
                rule['category_icon'] = cat_info.get('icon', 'file-text')
                rule['category_description'] = cat_info.get('description', '')
                syntax_rules.append(rule)
        
        # Add any parser functions we found as additional context
        for function in parser_functions:
            rule_name = function.replace('parse_', '').replace('_', ' ').title()
            # Only add if we don't already have this rule
            if not any(rule['name'].lower() == rule_name.lower() for rule in syntax_rules):
                syntax_rules.append({
                    'name': rule_name,
                    'category': 'parser_internals',
                    'category_name': 'Parser Internals',
                    'type': 'parser_rule',
                    'syntax': f'// Parsed by {function}()',
                    'description': f'Internal parser rule for {rule_name.lower()}.',
                    'examples': ['// Implementation specific'],
                    'heritage': 'Internal compiler parsing logic.'
                })

        print(f"✅ Generated {len(syntax_rules)} comprehensive syntax rules across {len(all_categories)} categories")
        return syntax_rules

    def parse_examples(self):
        """Extract examples from the examples directory"""
        print("📚 Extracting examples...")
        examples_dir = os.path.join(PROJECT_ROOT, 'examples')
        
        if not os.path.exists(examples_dir):
            print(f"⚠️  Examples directory not found: {examples_dir}")
            return []

        examples = []
        try:
            for filename in os.listdir(examples_dir):
                if filename.endswith('.cyl'):
                    filepath = os.path.join(examples_dir, filename)
                    with open(filepath, 'r') as f:
                        code = f.read()
                    
                    if code.strip():  # Only include non-empty files
                        examples.append({
                            'name': filename.replace('.cyl', '').replace('_', ' ').title(),
                            'filename': filename,
                            'code': code.strip(),
                            'description': self.generate_example_description(filename, code)
                        })

            print(f"✅ Found {len(examples)} examples")
            return examples

        except Exception as e:
            print(f"❌ Error parsing examples: {e}")
            return []

    def generate_example_description(self, filename, content):
        """Generate description based on example content"""
        name = filename.replace('.cyl', '').replace('_', ' ')
        
        if 'print' in content:
            return f"{name} - Demonstrates printing and basic output"
        elif 'let' in content:
            return f"{name} - Shows variable declaration and usage"
        elif 'if' in content:
            return f"{name} - Illustrates conditional logic and control flow"
        elif 'fn' in content:
            return f"{name} - Example of function definition and usage"
        else:
            return f"{name} - Example Cyl program"

    def parse_changelog(self):
        """Extract changelog information with improved markdown parsing"""
        print("📋 Extracting changelog...")
        changelog_path = os.path.join(PROJECT_ROOT, 'CHANGELOG.md')
        
        if not os.path.exists(changelog_path):
            print(f"⚠️  Changelog not found: {changelog_path}")
            return {'versions': [], 'toc': []}

        try:
            with open(changelog_path, 'r') as f:
                content = f.read()

            # Parse changelog sections
            version_sections = re.split(r'^##\s+', content, flags=re.MULTILINE)[1:]  # Skip first empty split
            
            versions = []
            toc = []
            
            for i, version_section in enumerate(version_sections):
                lines = version_section.strip().split('\n')
                if not lines:
                    continue
                    
                # Extract version number and date
                version_line = lines[0].strip()
                
                # First try to extract semver directly from the line
                semver_match = re.search(r'(\d+\.\d+\.\d+)', version_line)
                if semver_match:
                    version_number = semver_match.group(1)
                    date = "Recent"  # Default date
                else:
                    # Fallback to extracting non-semver versions or other formats
                    version_match = re.match(r'([^\(]+)', version_line)
                    if version_match:
                        version_text = version_match.group(1).strip()
                        version_number = version_text
                    else:
                        version_number = version_line
                    date = "Recent"
                
                # Create anchor for linking (use semantic version for cleaner anchors)
                anchor = f"v{version_number.replace('.', '-').replace(' ', '-').lower()}"
                
                # Parse content with markdown formatting
                content_html = self.parse_markdown_content('\n'.join(lines[1:]))
                
                # Parse changes for summary
                changes = []
                current_category = None
                
                for line in lines[1:]:
                    line = line.strip()
                    if not line:
                        continue
                        
                    # Check for category headers
                    if line.startswith('### '):
                        current_category = line[4:].strip()
                    elif line.startswith('- ') or line.startswith('* '):
                        change_text = line[2:].strip()
                        changes.append({
                            'category': current_category or 'Changes',
                            'text': change_text
                        })
                    elif line and not line.startswith('#'):
                        # Handle lines that might be part of the description
                        changes.append({
                            'category': 'Notes',
                            'text': line
                        })
                
                version_data = {
                    'version': version_number,
                    'date': date,
                    'anchor': anchor,
                    'content_html': content_html,
                    'changes': changes,
                    'summary': f"{len(changes)} changes" if changes else "Initial release"
                }
                
                versions.append(version_data)
                toc.append({
                    'version': version_number,
                    'date': date,
                    'anchor': anchor,
                    'summary': version_data['summary']
                })

            print(f"✅ Found {len(versions)} changelog entries")
            return {'versions': versions, 'toc': toc}

        except Exception as e:
            print(f"❌ Error parsing changelog: {e}")
            return {'versions': [], 'toc': []}

    def parse_markdown_content(self, content):
        """Enhanced markdown parser that preserves rich formatting from the original markdown"""
        if not content.strip():
            return ""
        
        # Split content into lines for processing
        lines = content.split('\n')
        html_lines = []
        
        i = 0
        while i < len(lines):
            line = lines[i]
            stripped = line.strip()
            
            # Skip empty lines (will be handled later)
            if not stripped:
                html_lines.append('')
                i += 1
                continue
            
            # Handle code blocks first (multi-line)
            if stripped.startswith('```'):
                lang_match = re.match(r'^```(\w+)?', stripped)
                lang = lang_match.group(1) if lang_match and lang_match.group(1) else ''
                
                code_lines = []
                i += 1
                while i < len(lines) and not lines[i].strip().startswith('```'):
                    code_lines.append(lines[i])
                    i += 1
                
                code_content = '\n'.join(code_lines)
                html_lines.append(f'<div class="code-block"><pre><code class="language-{lang}">{code_content}</code></pre></div>')
                i += 1  # Skip the closing ```
                continue
            
            # Handle headers (h1 through h6)
            if stripped.startswith('#'):
                header_match = re.match(r'^(#{1,6})\s+(.+)$', stripped)
                if header_match:
                    level = len(header_match.group(1))
                    text = header_match.group(2)
                    html_lines.append(f'<h{level}>{text}</h{level}>')
                    i += 1
                    continue
            
            # Handle blockquotes
            if stripped.startswith('>'):
                quote_text = re.sub(r'^>\s*', '', stripped)
                html_lines.append(f'<blockquote>{quote_text}</blockquote>')
                i += 1
                continue
            
            # Handle horizontal rules
            if re.match(r'^(-{3,}|\*{3,})$', stripped):
                html_lines.append('<hr>')
                i += 1
                continue
            
            # Handle lists (both ordered and unordered)
            list_match = re.match(r'^(\s*)([-*+]|\d+\.)\s+(.+)$', line)
            if list_match:
                indent = list_match.group(1)
                list_type = 'ol' if list_match.group(2).endswith('.') else 'ul'
                item_content = list_match.group(3)
                
                # Start collecting list items
                list_items = []
                current_indent = len(indent)
                
                # Add current item
                list_items.append(self._process_inline_formatting(item_content))
                i += 1
                
                # Collect consecutive list items with same or deeper indentation
                while i < len(lines):
                    next_line = lines[i]
                    next_stripped = next_line.strip()
                    
                    if not next_stripped:
                        i += 1
                        continue
                    
                    next_list_match = re.match(r'^(\s*)([-*+]|\d+\.)\s+(.+)$', next_line)
                    if next_list_match and len(next_list_match.group(1)) == current_indent:
                        list_items.append(self._process_inline_formatting(next_list_match.group(3)))
                        i += 1
                    else:
                        break
                
                # Generate the list HTML
                list_html = f'<{list_type}>'
                for item in list_items:
                    list_html += f'<li>{item}</li>'
                list_html += f'</{list_type}>'
                
                html_lines.append(list_html)
                continue
            
            # Handle regular paragraphs with inline formatting
            formatted_line = self._process_inline_formatting(line)
            html_lines.append(f'<p>{formatted_line}</p>')
            i += 1
        
        # Join all lines and clean up
        html_content = '\n'.join(html_lines)
        
        # Clean up excessive whitespace
        html_content = re.sub(r'\n{3,}', '\n\n', html_content)
        
        return html_content
    
    def _process_inline_formatting(self, text):
        """Process inline markdown formatting like bold, italic, code, links"""
        # Handle bold and italic (order matters)
        text = re.sub(r'\*\*\*([^*]+)\*\*\*', r'<strong><em>\1</em></strong>', text)  # Bold + italic
        text = re.sub(r'\*\*([^*]+)\*\*', r'<strong>\1</strong>', text)  # Bold
        text = re.sub(r'\*([^*\n]+)\*', r'<em>\1</em>', text)  # Italic
        
        # Handle strikethrough
        text = re.sub(r'~~([^~]+)~~', r'<del>\1</del>', text)
        
        # Handle inline code
        text = re.sub(r'`([^`\n]+)`', r'<code>\1</code>', text)
        
        # Handle links
        text = re.sub(r'\[([^\]]+)\]\(([^)]+)\)', r'<a href="\2">\1</a>', text)
        
        # Handle images
        text = re.sub(r'!\[([^\]]*)\]\(([^)]+)\)', r'<img src="\2" alt="\1">', text)
        
        return text

    def parse_stdlib(self):
        """Extract standard library information"""
        print("📖 Extracting standard library...")
        stdlib_path = os.path.join(PROJECT_ROOT, 'STDLIB.md')
        
        if not os.path.exists(stdlib_path):
            print(f"⚠️  Standard library docs not found: {stdlib_path}")
            return {}

        try:
            with open(stdlib_path, 'r') as f:
                content = f.read()

            # Extract function headings
            function_matches = re.findall(r'^##\s+(.+)$', content, re.MULTILINE)
            
            return {
                'content': content,
                'functions': [match.strip() for match in function_matches]
            }

        except Exception as e:
            print(f"❌ Error parsing stdlib: {e}")
            return {}

    def get_backend_info(self):
        """Get information about compilation backends from config"""
        print("⚙️  Extracting backend information...")
        
        # Use config if available, otherwise fallback to defaults
        backends_config = self.config.get('backends', [])
        
        if backends_config:
            return backends_config
        
        # Fallback defaults if not in config
        return [
            {
                'name': 'Cranelift',
                'description': 'Pure Rust code generation backend (default)',
                'features': ['Fast compilation', 'No external dependencies', 'Object file generation'],
                'usage': '--backend cranelift'
            },
            {
                'name': 'LLVM',
                'description': 'High-performance optimized code generation',
                'features': ['Advanced optimizations', 'Cross-platform support', 'Production-ready'],
                'usage': '--backend llvm'
            },
            {
                'name': 'Interpreter',
                'description': 'Direct execution engine for development and testing',
                'features': ['Immediate execution', 'No compilation step', 'Educational purposes'],
                'usage': '--backend interpreter'
            }
        ]

    def render_template(self, template_name, context, output_name):
        """Render a Jinja2 template with context"""
        try:
            template = env.get_template(template_name)
            html = template.render(**context)
            
            output_path = os.path.join(OUTPUT_DIR, output_name)
            with open(output_path, 'w') as f:
                f.write(html)
            
            print(f"✅ Generated {output_name}")
            
        except Exception as e:
            print(f"❌ Error rendering {template_name}: {e}")
            traceback.print_exc()

    def create_assets(self):
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
            # Create a basic CSS file if the source doesn't exist
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
            # Create a basic JS file if the source doesn't exist
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

    def copy_coverage_files(self):
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

    def load_config(self):
        """Load configuration from config.json"""
        config_path = os.path.join(os.path.dirname(__file__), 'config.json')
        try:
            with open(config_path, 'r') as f:
                return json.load(f)
        except FileNotFoundError:
            print("⚠️  config.json not found, using defaults")
            return {
                "site": {"title": "Cyl", "description": "A programming language"},
                "branding": {"favicon_path": "icon.png"},
                "footer": {"copyright": "2025 Cyl Programming Language"}
            }
        except json.JSONDecodeError as e:
            print(f"⚠️  Error parsing config.json: {e}")
            return {}

    def _determine_rule_type(self, category_id, rule_name):
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

    def generate_all(self):
        """Generate all documentation"""
        print("🔧 Generating Cyl Language Documentation...")
        
        # Create output directories
        os.makedirs(OUTPUT_DIR, exist_ok=True)
        
        # Extract all data
        self.data['syntax'] = self.parse_rust_syntax()
        self.data['syntax_categories'] = self.config.get('syntax', {}).get('categories', [])
        self.data['examples'] = self.parse_examples()
        self.data['changelog'] = self.parse_changelog()
        self.data['stdlib'] = self.parse_stdlib()
        self.data['backends'] = self.get_backend_info()
        self.data['coverage'] = self.parse_coverage()
        
        # Create assets first
        self.create_assets()
        
        # Copy coverage files for accessibility
        self.copy_coverage_files()
        
        # Render all templates with enhanced context
        syntax_context = {
            'syntax': self.data['syntax'], 
            'syntax_categories': self.data['syntax_categories'],
            'meta': self.data['meta'], 
            'config': self.data['config']
        }
        self.render_template('syntax.html.j2', syntax_context, 'syntax.html')
        self.render_template('examples.html.j2', {'examples': self.data['examples'], 'meta': self.data['meta'], 'config': self.data['config']}, 'examples.html')
        self.render_template('coverage.html.j2', {'coverage': self.data['coverage'], 'meta': self.data['meta'], 'config': self.data['config']}, 'coverage.html')
        self.render_template('changelog.html.j2', {'changelog': self.data['changelog'], 'meta': self.data['meta'], 'config': self.data['config']}, 'changelog.html')
        self.render_template('backends.html.j2', {'backends': self.data['backends'], 'meta': self.data['meta'], 'config': self.data['config']}, 'backends.html')
        self.render_template('index.html.j2', self.data, 'index.html')
        
        # Copy coverage files to website directory
        self.copy_coverage_files()
        
        print("✅ Documentation generated successfully!")
        print(f"📁 Output directory: {OUTPUT_DIR}")

    def parse_coverage(self):
        """Extract coverage information from coverage reports"""
        print("📊 Extracting coverage information...")
        coverage_data = {
            'last_updated': None,
            'typescript': None,
            'rust': None,
            'files': []
        }
        
        coverage_dir = os.path.join(PROJECT_ROOT, 'coverage')
        
        if not os.path.exists(coverage_dir):
            print(f"⚠️  Coverage directory not found: {coverage_dir}")
            return coverage_data
        
        # Parse TypeScript coverage from JSON summary
        typescript_summary_path = os.path.join(coverage_dir, 'coverage-summary.json')
        if os.path.exists(typescript_summary_path):
            coverage_data['typescript'] = self._parse_typescript_coverage(typescript_summary_path)
        
        # If no JSON summary, try parsing HTML index
        if not coverage_data['typescript']:
            html_index_path = os.path.join(coverage_dir, 'index.html')
            if os.path.exists(html_index_path):
                coverage_data['typescript'] = self._parse_typescript_html_coverage(html_index_path)
        
        # Parse Rust coverage from Cobertura XML
        cobertura_path = os.path.join(coverage_dir, 'cobertura.xml')
        if os.path.exists(cobertura_path):
            coverage_data['rust'] = self._parse_rust_coverage(cobertura_path)
        
        # Get last updated timestamp
        coverage_data['last_updated'] = self._get_coverage_timestamp(coverage_dir)
        
        print(f"✅ Coverage data extracted - TypeScript: {'✓' if coverage_data['typescript'] else '✗'}, Rust: {'✓' if coverage_data['rust'] else '✗'}")
        return coverage_data
    
    def _parse_typescript_coverage(self, summary_path):
        """Parse TypeScript coverage from JSON summary"""
        try:
            with open(summary_path, 'r') as f:
                data = json.load(f)
            
            total = data.get('total', {})
            files = []
            
            # Extract file-level data
            for file_path, file_data in data.items():
                if file_path != 'total' and isinstance(file_data, dict):
                    files.append({
                        'name': file_path,
                        'statements': self._format_coverage_metric(file_data.get('statements', {})),
                        'branches': self._format_coverage_metric(file_data.get('branches', {})),
                        'functions': self._format_coverage_metric(file_data.get('functions', {})),
                        'lines': self._format_coverage_metric(file_data.get('lines', {})),
                        'overall_pct': file_data.get('lines', {}).get('pct', 0)
                    })
            
            return {
                'statements': self._format_coverage_metric(total.get('statements', {})),
                'branches': self._format_coverage_metric(total.get('branches', {})),
                'functions': self._format_coverage_metric(total.get('functions', {})),
                'lines': self._format_coverage_metric(total.get('lines', {})),
                'files': files
            }
        except Exception as e:
            print(f"❌ Error parsing TypeScript coverage JSON: {e}")
            return None
    
    def _parse_typescript_html_coverage(self, html_path):
        """Parse TypeScript coverage from HTML index file"""
        try:
            with open(html_path, 'r') as f:
                content = f.read()
            
            # Extract overall percentages from the HTML
            statements_match = re.search(r'<span class="strong">([0-9.]+)%\s*</span>\s*<span class="quiet">Statements</span>', content)
            branches_match = re.search(r'<span class="strong">([0-9.]+)%\s*</span>\s*<span class="quiet">Branches</span>', content)
            functions_match = re.search(r'<span class="strong">([0-9.]+)%\s*</span>\s*<span class="quiet">Functions</span>', content)
            lines_match = re.search(r'<span class="strong">([0-9.]+)%\s*</span>\s*<span class="quiet">Lines</span>', content)
            
            # Extract fractions
            statements_fraction = re.search(r'<span class=\'fraction\'>(\d+)/(\d+)</span>', content)
            branches_fraction = re.search(r'<span class=\'fraction\'>(\d+)/(\d+)</span>', content)
            
            if statements_match and lines_match:
                return {
                    'statements': {
                        'pct': float(statements_match.group(1)),
                        'covered': int(statements_fraction.group(1)) if statements_fraction else 0,
                        'total': int(statements_fraction.group(2)) if statements_fraction else 0,
                        'color': self._get_coverage_color(float(statements_match.group(1)))
                    },
                    'branches': {
                        'pct': float(branches_match.group(1)) if branches_match else 0,
                        'covered': 0,
                        'total': 0,
                        'color': self._get_coverage_color(float(branches_match.group(1)) if branches_match else 0)
                    },
                    'functions': {
                        'pct': float(functions_match.group(1)) if functions_match else 0,
                        'covered': 0,
                        'total': 0,
                        'color': self._get_coverage_color(float(functions_match.group(1)) if functions_match else 0)
                    },
                    'lines': {
                        'pct': float(lines_match.group(1)),
                        'covered': 0,
                        'total': 0,
                        'color': self._get_coverage_color(float(lines_match.group(1)))
                    },
                    'files': []
                }
        except Exception as e:
            print(f"❌ Error parsing TypeScript coverage HTML: {e}")
            return None
    
    def _parse_rust_coverage(self, cobertura_path):
        """Parse Rust coverage from Cobertura XML"""
        try:
            tree = ET.parse(cobertura_path)
            root = tree.getroot()
            
            # Get overall line coverage
            total_lines = 0
            covered_lines = 0
            files = []
            
            for package in root.findall('.//package'):
                for class_elem in package.findall('.//class'):
                    filename = class_elem.get('filename', '')
                    
                    # Get lines covered for this file
                    file_lines = class_elem.findall('.//line')
                    file_total = len(file_lines)
                    file_covered = len([line for line in file_lines if line.get('hits', '0') != '0'])
                    
                    if file_total > 0:
                        file_pct = (file_covered / file_total) * 100
                        files.append({
                            'name': filename,
                            'lines': {
                                'pct': round(file_pct, 2),
                                'covered': file_covered,
                                'total': file_total,
                                'color': self._get_coverage_color(file_pct)
                            }
                        })
                    
                    total_lines += file_total
                    covered_lines += file_covered
            
            overall_pct = (covered_lines / total_lines * 100) if total_lines > 0 else 0
            
            return {
                'lines': {
                    'pct': round(overall_pct, 2),
                    'covered': covered_lines,
                    'total': total_lines,
                    'color': self._get_coverage_color(overall_pct)
                },
                'files': files
            }
        except Exception as e:
            print(f"❌ Error parsing Rust coverage XML: {e}")
            return None
    
    def _format_coverage_metric(self, metric_data):
        """Format a coverage metric with color coding"""
        if not isinstance(metric_data, dict):
            return {'pct': 0, 'covered': 0, 'total': 0, 'color': '#dc3545'}
        
        pct = metric_data.get('pct', 0)
        return {
            'pct': pct,
            'covered': metric_data.get('covered', 0),
            'total': metric_data.get('total', 0),
            'color': self._get_coverage_color(pct)
        }
    
    def _get_coverage_color(self, percentage):
        """Get color based on coverage percentage"""
        if percentage >= 80:
            return '#28a745'  # Green
        elif percentage >= 60:
            return '#ffc107'  # Yellow
        else:
            return '#dc3545'  # Red
    
    def _get_coverage_timestamp(self, coverage_dir):
        """Get the timestamp of the most recent coverage file formatted as YYYY-MM-DD"""
        try:
            files_to_check = [
                os.path.join(coverage_dir, 'index.html'),
                os.path.join(coverage_dir, 'cobertura.xml'),
                os.path.join(coverage_dir, 'coverage-summary.json')
            ]
            
            latest_time = None
            for file_path in files_to_check:
                if os.path.exists(file_path):
                    mtime = os.path.getmtime(file_path)
                    if latest_time is None or mtime > latest_time:
                        latest_time = mtime
            
            if latest_time:
                # Format as YYYY-MM-DD only
                return datetime.fromtimestamp(latest_time).strftime('%Y-%m-%d')
        except Exception:
            pass
        
        return None
def main():
    generator = CylDocGenerator()
    generator.generate_all()

if __name__ == '__main__':
    main()
