"""
Main documentation generator class.
"""

import os
import traceback
from datetime import datetime

# Add jinja2 import and setup
try:
    from jinja2 import Environment, FileSystemLoader
except ImportError:
    print("Error: jinja2 is not installed. Please run:")
    print("pip install -r requirements.txt")
    exit(1)

# Setup paths
_current_dir = os.path.dirname(__file__)
PROJECT_ROOT = os.path.abspath(os.path.join(_current_dir, '../../..'))
TEMPLATE_DIR = os.path.join(_current_dir, '../templates')
OUTPUT_DIR = os.path.join(PROJECT_ROOT, 'docs/website')

# Initialize Jinja2 environment
env = Environment(loader=FileSystemLoader(TEMPLATE_DIR))

from .utils import load_config, get_version, copy_coverage_files, create_assets
from .parsers import (
    SyntaxParser, ExampleParser, ChangelogParser, 
    StdlibParser, CoverageParser
)


class CylDocGenerator:
    def __init__(self):
        # Load configuration
        self.config = load_config()
        
        self.data = {
            'syntax': [],
            'features': [],
            'examples': [],
            'changelog': [],
            'backends': [],
            'stdlib': {},
            'meta': {
                'generated_at': datetime.now().isoformat(),
                'version': get_version()
            },
            'config': self.config
        }
        
        # Initialize parsers
        self.syntax_parser = SyntaxParser(self.config)
        self.example_parser = ExampleParser(self.config)
        self.changelog_parser = ChangelogParser(self.config)
        self.stdlib_parser = StdlibParser(self.config)
        self.coverage_parser = CoverageParser(self.config)

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

    def generate_all(self):
        """Generate all documentation"""
        print("🔧 Generating Cyl Language Documentation...")
        
        # Create output directories
        os.makedirs(OUTPUT_DIR, exist_ok=True)
        
        # Extract all data using parsers
        self.data['syntax'] = self.syntax_parser.parse_syntax()
        self.data['syntax_categories'] = self.config.get('syntax', {}).get('categories', [])
        self.data['examples'] = self.example_parser.parse_examples()
        self.data['changelog'] = self.changelog_parser.parse_changelog()
        self.data['stdlib'] = self.stdlib_parser.parse_stdlib()
        self.data['backends'] = self.get_backend_info()
        self.data['coverage'] = self.coverage_parser.parse_coverage()
        
        # Create assets first
        create_assets()
        
        # Copy coverage files for accessibility
        copy_coverage_files()
        
        # Render all templates with enhanced context
        self._render_all_templates()
        
        print("✅ Documentation generated successfully!")
        print(f"📁 Output directory: {OUTPUT_DIR}")
    
    def _render_all_templates(self):
        """Render all documentation templates"""
        # Syntax page with categories
        syntax_context = {
            'syntax': self.data['syntax'], 
            'syntax_categories': self.data['syntax_categories'],
            'meta': self.data['meta'], 
            'config': self.data['config']
        }
        self.render_template('syntax.html.j2', syntax_context, 'syntax.html')
        
        # Other pages
        pages = [
            ('examples.html.j2', {'examples': self.data['examples']}, 'examples.html'),
            ('coverage.html.j2', {'coverage': self.data['coverage']}, 'coverage.html'),
            ('changelog.html.j2', {'changelog': self.data['changelog']}, 'changelog.html'),
            ('backends.html.j2', {'backends': self.data['backends']}, 'backends.html'),
            ('index.html.j2', self.data, 'index.html')
        ]
        
        for template_name, context_data, output_name in pages:
            # Add meta and config to all contexts
            full_context = {
                **context_data,
                'meta': self.data['meta'],
                'config': self.data['config']
            }
            self.render_template(template_name, full_context, output_name)
