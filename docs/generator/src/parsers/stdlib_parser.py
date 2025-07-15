"""
Standard library parser for extracting stdlib information.
"""

import os
import re

# Get PROJECT_ROOT relative to this file
PROJECT_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), '../../../..'))


class StdlibParser:
    def __init__(self, config):
        self.config = config
    
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
