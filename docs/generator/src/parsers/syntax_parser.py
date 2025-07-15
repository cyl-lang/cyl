"""
Syntax parser for extracting syntax information from Rust parser files.
"""

import os
import re

# Get PROJECT_ROOT relative to this file
PROJECT_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), '../../../..'))


class SyntaxParser:
    def __init__(self, config):
        self.config = config
    
    def parse_syntax(self):
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
                            'type': self._determine_rule_type(cat_id),
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
    
    def _determine_rule_type(self, category_id):
        """Determine the type of a syntax rule based on category"""
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
