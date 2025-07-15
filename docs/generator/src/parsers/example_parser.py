"""
Example parser for extracting examples from the examples directory.
"""

import os

# Get PROJECT_ROOT relative to this file
PROJECT_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), '../../../..'))


class ExampleParser:
    def __init__(self, config):
        self.config = config
    
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
                            'description': self._generate_description(filename, code)
                        })

            print(f"✅ Found {len(examples)} examples")
            return examples

        except Exception as e:
            print(f"❌ Error parsing examples: {e}")
            return []

    def _generate_description(self, filename, content):
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
