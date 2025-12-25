import re
import sys

def filter_report(input_path, output_path):
    with open(input_path, 'r') as f:
        content = f.read()

    # Split by the separator line "---"
    # The report structure seems to be:
    # Header
    # ---
    # Issues by Severity
    # ... chunks separated by ---
    
    # Let's try to identify issue blocks. 
    # Usually issues are separated by "---".
    
    parts = content.split('\n---')
    
    kept_parts = []
    
    magic_value_count = 0
    
    # The first part contains the header and summary. We need to handle it carefully.
    # Subsequent parts are likely issue blocks, but some might be section headers.
    
    # A safer approach might be to split, check if it looks like an issue block, and if so, check for "Magic value".
    
    for i, part in enumerate(parts):
        # Check if this part describes an issue that is a "Magic value"
        # The issue description usually starts with "#### Magic value" or contains it in the title line.
        
        if "#### Magic value" in part:
            magic_value_count += 1
            continue
            
        kept_parts.append(part)

    # Reassemble
    new_content = '\n---'.join(kept_parts)
    
    # Update the summary numbers if possible
    # Pattern: "- **Issues Found:** 1700"
    match = re.search(r'- \*\*Issues Found:\*\* (\d+)', new_content)
    if match:
        original_count = int(match.group(1))
        new_count = original_count - magic_value_count
        new_content = new_content.replace(f"- **Issues Found:** {original_count}", f"- **Issues Found:** {new_count}")
        
    # Also need to update "High" count or "Low" count if they are listed in the summary text (not the Table of Contents, but the section headers like "### 🟠 High (702 issues)")
    
    # Update section headers counts
    # The regex should look for "### \S+ \w+ \((\d+) issues\)" and we need to know how many we removed from which category.
    # NOTE: This is hard to do perfectly without parsing severity.
    # Magic values appeared in "High" and "Low" in the previous view.
    
    # For now, just updating the total count in the header is a good start. 
    # To do it properly, we should count how many we removed per severity.
    
    print(f"Removed {magic_value_count} magic value issues.")
    
    with open(output_path, 'w') as f:
        f.write(new_content)

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: python clean_report.py <input> <output>")
        sys.exit(1)
    
    filter_report(sys.argv[1], sys.argv[2])
