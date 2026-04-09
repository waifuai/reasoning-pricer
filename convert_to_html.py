#!/usr/bin/env python3
"""Convert markdown files to standalone HTML with KaTeX math rendering."""

import os
import re
import markdown

BASE_DIR = os.path.dirname(os.path.abspath(__file__))
HTML_DIR = os.path.join(BASE_DIR, "html")

def protect_math(text):
    """Extract math blocks before markdown processing to prevent mangling."""
    placeholders = []
    
    # Protect display math ($$...$$) first, then inline ($...$)
    def replacer(match):
        placeholders.append(match.group(0))
        return f"@@MATH{len(placeholders)-1}@@"
    
    # Display math first
    text = re.sub(r'\$\$(.+?)\$\$', replacer, text, flags=re.DOTALL)
    # Inline math
    text = re.sub(r'\$(.+?)\$', replacer, text)
    
    return text, placeholders

def restore_math(html, placeholders):
    """Restore math placeholders with proper KaTeX spans."""
    for i, math in enumerate(placeholders):
        # Strip $ or $$ delimiters
        if math.startswith('$$') and math.endswith('$$'):
            content = math[2:-2].strip()
            html = html.replace(f"@@MATH{i}@@", 
                f'<span class="katex-display" data-katex="{content}"></span>')
        else:
            content = math[1:-1].strip()
            html = html.replace(f"@@MATH{i}@@",
                f'<span class="katex-inline" data-katex="{content}"></span>')
    return html

HTML_TEMPLATE = """<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{title}</title>
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.css">
<script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/katex.min.js"></script>
<script defer src="https://cdn.jsdelivr.net/npm/katex@0.16.9/dist/contrib/auto-render.min.js"
    onload="renderMathInElement(document.body, {{
        delimiters: [
            {{left: '$$', right: '$$', display: true}},
            {{left: '$', right: '$', display: false}}
        ]
    }});"></script>
<style>
body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; max-width: 900px; margin: 0 auto; padding: 2rem; line-height: 1.6; color: #333; }}
pre {{ background: #f5f5f5; padding: 1rem; border-radius: 4px; overflow-x: auto; }}
code {{ background: #f0f0f0; padding: 0.2em 0.4em; border-radius: 3px; font-size: 0.9em; }}
pre code {{ background: none; padding: 0; }}
table {{ border-collapse: collapse; width: 100%; margin: 1rem 0; }}
th, td {{ border: 1px solid #ddd; padding: 8px; text-align: left; }}
th {{ background: #f5f5f5; }}
a {{ color: #0066cc; }}
</style>
</head>
<body>
{body}
</body>
</html>"""

def convert_file(md_path):
    """Convert a single markdown file to HTML with KaTeX."""
    with open(md_path, 'r', encoding='utf-8') as f:
        md_text = f.read()
    
    # Protect math before markdown processing
    protected, placeholders = protect_math(md_text)
    
    # Convert markdown to HTML
    html_body = markdown.markdown(protected, extensions=['tables', 'fenced_code', 'codehilite'])
    
    # Fix .md -> .html links
    html_body = html_body.replace('.md"', '.html"')
    
    # Note: auto-render.min.js handles KaTeX rendering via delimiters,
    # so we don't need restore_math here. But if math was protected,
    # we need to restore them back as raw $ delimiters for KaTeX.
    # Actually, since we protected them and restore them, auto-render
    # won't find the delimiters. Better approach: just pass through
    # without protection if using auto-render.
    
    # Actually, the issue is markdown mangles things like _ inside $...$.
    # Let's use a hybrid: protect, convert, restore with delimiters intact.
    
    # Restore math
    for i, math in enumerate(placeholders):
        html_body = html_body.replace(f"@@MATH{i}@@", math)
    
    # Get title from first heading or filename
    title_match = re.search(r'^#\s+(.+)$', md_text, re.MULTILINE)
    title = title_match.group(1) if title_match else os.path.basename(md_path)
    # Strip any remaining markdown from title
    title = re.sub(r'[*_`]', '', title)
    
    return HTML_TEMPLATE.format(title=title, body=html_body)

def main():
    md_files = []
    for root, dirs, files in os.walk(BASE_DIR):
        # Skip .git, html, and target directories
        dirs[:] = [d for d in dirs if d not in ('.git', 'html', 'target', '__pycache__')]
        for f in files:
            if f.endswith('.md'):
                md_files.append(os.path.join(root, f))
    
    print(f"Found {len(md_files)} markdown files")
    
    for md_path in sorted(md_files):
        # Compute relative path and output path
        rel_path = os.path.relpath(md_path, BASE_DIR)
        html_rel = os.path.splitext(rel_path)[0] + '.html'
        html_path = os.path.join(HTML_DIR, html_rel)
        
        # Create output directory
        os.makedirs(os.path.dirname(html_path), exist_ok=True)
        
        try:
            html_content = convert_file(md_path)
            with open(html_path, 'w', encoding='utf-8') as f:
                f.write(html_content)
            print(f"  {rel_path} -> html/{html_rel}")
        except Exception as e:
            print(f"  ERROR {rel_path}: {e}")
    
    print(f"\nDone! HTML files in: {HTML_DIR}")

if __name__ == '__main__':
    main()
