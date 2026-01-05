# Social Media Images Specification

This document outlines the requirements for creating social media preview images (Open Graph and Twitter Cards) for mcpg.botzr.com.

## Image Requirements

### Open Graph Image
**File:** `/public/og-image.png`

- **Dimensions:** 1200 x 630 pixels (exact)
- **Aspect Ratio:** 1.91:1
- **File Format:** PNG (preferred) or JPG
- **File Size:** < 8 MB (aim for < 300 KB)
- **Color Space:** RGB
- **Safe Zone:** Keep important text/elements within center 1200 x 600 pixels (15px padding on top/bottom)

### Twitter Card Image
**File:** `/public/twitter-card.png`

- **Dimensions:** 1200 x 600 pixels (exact)
- **Aspect Ratio:** 2:1
- **File Format:** PNG (preferred) or JPG
- **File Size:** < 5 MB (aim for < 300 KB)
- **Color Space:** RGB

---

## Design Specifications

### Brand Colors (from your theme)
```
Primary Background: #050508 (--bg-primary)
Secondary Background: #0A0C10 (--bg-secondary)
Accent Orange: #FF7A30 (--accent-orange)
Accent Cyan: #00E5FF (--accent-cyan)
Text Primary: #FAFAFA (--text-primary)
Text Secondary: #B0B0B0 (--text-secondary)
```

### Typography
- **Headline Font:** Inter (Display weight, bold)
- **Body Font:** Inter (Regular/Medium)
- **Mono Font:** JetBrains Mono (for code samples)

---

## Recommended Design Elements

### OG Image Layout (1200x630)

```
┌─────────────────────────────────────────────────┐
│  [Logo]                         mcpg.botzr.com  │ ← Header (60px)
│                                                 │
│         Secure Your MCP Servers                 │ ← Main Title (80px font)
│         in 5 Minutes                            │
│                                                 │
│  • OAuth 2.1  • JWT  • Rate Limiting            │ ← Features (32px)
│                                                 │
│  $ cargo install mcp-guard                      │ ← Code snippet (terminal style)
│                                                 │
│  [Shield Icon]    Production-Grade Security     │ ← Footer CTA
│                   Single Binary, No Docker      │
└─────────────────────────────────────────────────┘
```

### Key Visual Elements

1. **Background:**
   - Dark gradient (#050508 to #0A0C10)
   - Subtle grid pattern (optional, low opacity)
   - Security-themed accent (shield glow, lock icons)

2. **Logo Placement:**
   - Top left corner
   - 60-80px height
   - Use your existing logo: `/public/favicon-96x96.png` or create vector version

3. **Main Headline:**
   - "Secure Your MCP Servers in 5 Minutes"
   - Font size: 72-80px
   - Color: White (#FAFAFA)
   - Font weight: Bold (700-800)

4. **Subheadline/Features:**
   - Bullet points or badges
   - Highlight: OAuth 2.1, JWT, Rate Limiting, Audit Logs
   - Font size: 28-32px
   - Color: #B0B0B0 (text-secondary)

5. **Code Snippet:**
   - `$ cargo install mcp-guard`
   - Terminal-style background (#0A0C10)
   - Monospace font (JetBrains Mono)
   - Accent color for prompt ($) in cyan (#00E5FF)

6. **Visual Accent:**
   - Shield icon or lock icon
   - Gradient glow effect (orange to cyan)
   - Subtle security grid pattern

---

## Twitter Card Layout (1200x600)

Similar to OG image but with adjusted proportions:

```
┌─────────────────────────────────────────────────┐
│  [Logo]    Secure Your MCP Servers              │
│                                                 │
│  Production-grade security • Single binary      │
│  No Docker • No DevOps team                     │
│                                                 │
│  $ cargo install mcp-guard                      │
│                                                 │
│  [Shield Icon]          mcpg.botzr.com          │
└─────────────────────────────────────────────────┘
```

---

## Design Tools & Resources

### Option 1: Figma (Recommended)
1. Create artboard: 1200 x 630 px (OG) and 1200 x 600 px (Twitter)
2. Use Inter font from Google Fonts
3. Export as PNG with 2x resolution, then scale down for optimal file size

### Option 2: Canva
1. Use custom dimensions (1200 x 630 or 1200 x 600)
2. Upload your logo
3. Use provided brand colors
4. Download as PNG

### Option 3: Code-based (HTML/CSS Screenshot)
Create an HTML template with exact dimensions and screenshot using:
- Puppeteer
- Playwright
- Vercel OG Image API

---

## Current Assets Available

From `/landing/src/assets/`:
- Logo: `MCPG_LOGO1.png` (300KB PNG)
- SVG icons: `shield-logo.svg`, `lock-icon.svg`, `key-icon.svg`

---

## Testing Your Images

### Preview Tools:
1. **Open Graph Debugger:** https://www.opengraph.xyz/
2. **Twitter Card Validator:** https://cards-dev.twitter.com/validator
3. **Facebook Sharing Debugger:** https://developers.facebook.com/tools/debug/

### Checklist:
- [ ] Text is clearly readable at mobile sizes (shrink to 400px wide)
- [ ] Important content is within safe zones
- [ ] File size is optimized (< 300 KB)
- [ ] Colors match brand guidelines
- [ ] Logo is visible and crisp
- [ ] No text is cut off at edges

---

## Quick Start Template

If you need a quick start, use this Figma template structure:

1. **Background Layer:**
   - Linear gradient: #050508 → #0A0C10
   - Add subtle grid overlay (5% opacity)

2. **Content Layer:**
   - Logo (top-left, 70px height)
   - Main title: "Secure Your MCP Servers in 5 Minutes"
   - Features: OAuth 2.1 • JWT • Rate Limiting
   - Code snippet box with terminal styling
   - Shield icon with glow effect

3. **Export Settings:**
   - Format: PNG
   - Scale: 2x
   - Optimization: Medium compression

---

## Next Steps

1. Create OG image (1200 x 630) using design specifications above
2. Create Twitter card image (1200 x 600)
3. Save both images to `/landing/public/`
4. Test with debugging tools listed above
5. Update canonical URLs in `index.html` when domain is live

---

**Note:** The meta tags in `index.html` are already configured and ready. You just need to create the actual image files and place them in the `/public/` folder.
