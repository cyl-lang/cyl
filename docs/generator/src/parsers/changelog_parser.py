"""
Changelog parser for extracting changelog information.
"""

import os
import re

# Get PROJECT_ROOT relative to this file
PROJECT_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), '../../../..'))


def parse_markdown_content(content):
    """Enhanced markdown parser that preserves rich formatting from the original markdown"""
    if not content.strip():
        return ""
    
    # For now, simple implementation - can be enhanced later
    # Convert basic markdown to HTML
    content = re.sub(r'\*\*([^*]+)\*\*', r'<strong>\1</strong>', content)  # Bold
    content = re.sub(r'\*([^*\n]+)\*', r'<em>\1</em>', content)  # Italic
    content = re.sub(r'`([^`\n]+)`', r'<code>\1</code>', content)  # Code
    content = re.sub(r'^###\s+(.+)$', r'<h3>\1</h3>', content, flags=re.MULTILINE)  # H3
    content = re.sub(r'^##\s+(.+)$', r'<h2>\1</h2>', content, flags=re.MULTILINE)  # H2
    content = re.sub(r'^#\s+(.+)$', r'<h1>\1</h1>', content, flags=re.MULTILINE)  # H1
    content = re.sub(r'^[-*+]\s+(.+)$', r'<li>\1</li>', content, flags=re.MULTILINE)  # List items
    
    # Wrap consecutive list items in ul tags
    content = re.sub(r'(<li>.*?</li>\n?)(?=<li>|$)', r'<ul>\1</ul>', content, flags=re.DOTALL)
    
    return content


class ChangelogParser:
    def __init__(self, config):
        self.config = config
    
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
                content_html = parse_markdown_content('\n'.join(lines[1:]))
                
                # Parse changes for summary
                changes = self._parse_changes(lines[1:])
                
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
    
    def _parse_changes(self, lines):
        """Parse changes for summary"""
        changes = []
        current_category = None
        
        for line in lines:
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
        
        return changes
