"""
Markdown parsing utilities for the documentation generator.
"""

import re


def parse_markdown_content(content):
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
            list_items.append(process_inline_formatting(item_content))
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
                    list_items.append(process_inline_formatting(next_list_match.group(3)))
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
        formatted_line = process_inline_formatting(line)
        html_lines.append(f'<p>{formatted_line}</p>')
        i += 1
    
    # Join all lines and clean up
    html_content = '\n'.join(html_lines)
    
    # Clean up excessive whitespace
    html_content = re.sub(r'\n{3,}', '\n\n', html_content)
    
    return html_content


def process_inline_formatting(text):
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
