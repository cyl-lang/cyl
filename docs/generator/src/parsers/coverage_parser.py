"""
Coverage parser for extracting coverage information from coverage reports.
"""

import os
import json
import xml.etree.ElementTree as ET
from datetime import datetime

# Get PROJECT_ROOT relative to this file
PROJECT_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), '../../../..'))


class CoverageParser:
    def __init__(self, config):
        self.config = config
    
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
            
            import re
            # Extract overall percentages from the HTML
            statements_match = re.search(r'<span class="strong">([0-9.]+)%\s*</span>\s*<span class="quiet">Statements</span>', content)
            branches_match = re.search(r'<span class="strong">([0-9.]+)%\s*</span>\s*<span class="quiet">Branches</span>', content)
            functions_match = re.search(r'<span class="strong">([0-9.]+)%\s*</span>\s*<span class="quiet">Functions</span>', content)
            lines_match = re.search(r'<span class="strong">([0-9.]+)%\s*</span>\s*<span class="quiet">Lines</span>', content)
            
            # Extract fractions
            statements_fraction = re.search(r'<span class=\'fraction\'>(\d+)/(\d+)</span>', content)
            
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
