import { Component, signal, ChangeDetectionStrategy, NgZone, OnInit, OnDestroy, inject, HostListener } from '@angular/core';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-navbar',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule],
  template: `
    <!-- Command bar style header -->
    <header [class.scrolled]="isScrolled()" [class.nav-expanded]="isNavExpanded()">
      <!-- Top utility bar -->
      <div class="utility-bar">
        <div class="utility-content">
          <div class="status-cluster">
            <span class="status-dot"></span>
            <span class="status-text">All systems operational</span>
          </div>
          <div class="utility-links">
            <a href="/changelog" class="utility-link">
              <span class="version-tag">v1.0</span>
              Changelog
            </a>
            <span class="utility-divider">/</span>
            <a href="https://github.com/mcp-guard/mcp-guard" target="_blank" class="utility-link">
              <svg viewBox="0 0 24 24" fill="currentColor" width="14" height="14">
                <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
              </svg>
              Star on GitHub
            </a>
          </div>
        </div>
      </div>

      <!-- Main navigation -->
      <nav class="main-nav">
        <div class="nav-content">
          <!-- Logo -->
          <a href="#" class="logo-block">
            <img src="assets/MCPG_LOGO1.png" alt="MCP Guard" class="logo-img" />
            <div class="logo-stack">
              <span class="logo-name">mcp-guard</span>
              <span class="logo-tagline">Security Gateway</span>
            </div>
          </a>

          <!-- Keyboard-style navigation -->
          <div class="nav-keys">
            <a href="#features" class="nav-key" data-key="F">
              <span class="key-label">Features</span>
              <kbd>F</kbd>
            </a>
            <a href="#how-it-works" class="nav-key" data-key="H">
              <span class="key-label">How it works</span>
              <kbd>H</kbd>
            </a>
            <a href="#pricing" class="nav-key" data-key="P">
              <span class="key-label">Pricing</span>
              <kbd>P</kbd>
            </a>
            <a href="/docs" class="nav-key" data-key="D">
              <span class="key-label">Docs</span>
              <kbd>D</kbd>
            </a>
          </div>

          <!-- Action cluster -->
          <div class="nav-actions">
            <!-- Command palette trigger -->
            <button class="cmd-trigger" (click)="toggleNav()" aria-label="Toggle navigation">
              <div class="cmd-icon">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                  <path d="M4 6h16M4 12h16M4 18h16" [class.hidden]="isNavExpanded()" />
                  <path d="M18 6L6 18M6 6l12 12" [class.hidden]="!isNavExpanded()" />
                </svg>
              </div>
              <span class="cmd-shortcut">
                <kbd>⌘</kbd><kbd>K</kbd>
              </span>
            </button>

            <a href="/docs/quickstart" class="cta-block">
              <span class="cta-text">Get Started</span>
              <span class="cta-arrow">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M5 12h14M12 5l7 7-7 7"/>
                </svg>
              </span>
            </a>
          </div>
        </div>
      </nav>

      <!-- Scroll indicator as thin accent line -->
      <div class="scroll-accent">
        <div class="scroll-fill" [style.width.%]="scrollProgress()"></div>
      </div>
    </header>

    <!-- Expanded command palette overlay -->
    <div class="cmd-overlay" [class.active]="isNavExpanded()" (click)="closeNav()">
      <div class="cmd-panel" (click)="$event.stopPropagation()">
        <div class="cmd-header">
          <div class="cmd-search">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="11" cy="11" r="8"/>
              <path d="M21 21l-4.35-4.35"/>
            </svg>
            <span class="cmd-placeholder">Navigate to...</span>
          </div>
          <kbd class="cmd-esc">ESC</kbd>
        </div>
        <div class="cmd-list">
          <div class="cmd-group">
            <span class="cmd-group-label">Pages</span>
            <a href="#features" class="cmd-item" (click)="closeNav()">
              <span class="cmd-item-icon">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
                </svg>
              </span>
              <span class="cmd-item-text">Features</span>
              <kbd>F</kbd>
            </a>
            <a href="#how-it-works" class="cmd-item" (click)="closeNav()">
              <span class="cmd-item-icon">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <circle cx="12" cy="12" r="10"/>
                  <path d="M12 16v-4M12 8h.01"/>
                </svg>
              </span>
              <span class="cmd-item-text">How it works</span>
              <kbd>H</kbd>
            </a>
            <a href="#pricing" class="cmd-item" (click)="closeNav()">
              <span class="cmd-item-icon">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M12 1v22M17 5H9.5a3.5 3.5 0 000 7h5a3.5 3.5 0 010 7H6"/>
                </svg>
              </span>
              <span class="cmd-item-text">Pricing</span>
              <kbd>P</kbd>
            </a>
            <a href="/docs" class="cmd-item" (click)="closeNav()">
              <span class="cmd-item-icon">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z"/>
                  <polyline points="14 2 14 8 20 8"/>
                  <line x1="16" y1="13" x2="8" y2="13"/>
                  <line x1="16" y1="17" x2="8" y2="17"/>
                </svg>
              </span>
              <span class="cmd-item-text">Documentation</span>
              <kbd>D</kbd>
            </a>
          </div>
          <div class="cmd-group">
            <span class="cmd-group-label">Quick actions</span>
            <a href="/docs/quickstart" class="cmd-item" (click)="closeNav()">
              <span class="cmd-item-icon accent">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"/>
                </svg>
              </span>
              <span class="cmd-item-text">Quick start guide</span>
              <span class="cmd-item-badge">5 min</span>
            </a>
            <a href="https://github.com/mcp-guard/mcp-guard" target="_blank" class="cmd-item" (click)="closeNav()">
              <span class="cmd-item-icon">
                <svg viewBox="0 0 24 24" fill="currentColor">
                  <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                </svg>
              </span>
              <span class="cmd-item-text">View on GitHub</span>
              <span class="cmd-item-external">↗</span>
            </a>
          </div>
        </div>
        <div class="cmd-footer">
          <span class="cmd-hint">
            <kbd>↑</kbd><kbd>↓</kbd> to navigate
          </span>
          <span class="cmd-hint">
            <kbd>↵</kbd> to select
          </span>
        </div>
      </div>
    </div>

    <!-- Mobile-only bottom navigation -->
    <nav class="mobile-dock" [class.hidden]="isScrollingDown()">
      <a href="#features" class="dock-item">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
        </svg>
        <span>Features</span>
      </a>
      <a href="#pricing" class="dock-item">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M12 1v22M17 5H9.5a3.5 3.5 0 000 7h5a3.5 3.5 0 010 7H6"/>
        </svg>
        <span>Pricing</span>
      </a>
      <a href="/docs/quickstart" class="dock-item primary">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"/>
        </svg>
        <span>Start</span>
      </a>
      <a href="/docs" class="dock-item">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8z"/>
          <polyline points="14 2 14 8 20 8"/>
        </svg>
        <span>Docs</span>
      </a>
    </nav>
  `,
  styles: [`
    /* ============================================
       UTILITY BAR - Top status strip
    ============================================ */
    .utility-bar {
      background: var(--bg-void);
      border-bottom: 1px solid var(--border-subtle);
      padding: 8px 0;
      font-size: 12px;
      transition: transform 0.3s var(--ease-out), opacity 0.3s var(--ease-out);

      @media (max-width: 768px) {
        display: none;
      }
    }

    header.scrolled .utility-bar {
      transform: translateY(-100%);
      opacity: 0;
      position: absolute;
    }

    .utility-content {
      max-width: 1280px;
      margin: 0 auto;
      padding: 0 24px;
      display: flex;
      justify-content: space-between;
      align-items: center;
    }

    .status-cluster {
      display: flex;
      align-items: center;
      gap: 8px;
      color: var(--text-muted);
    }

    .status-dot {
      width: 6px;
      height: 6px;
      background: #22c55e;
      border-radius: 50%;
      box-shadow: 0 0 8px #22c55e;
      animation: pulse-glow 2s ease-in-out infinite;
    }

    @keyframes pulse-glow {
      0%, 100% { opacity: 1; box-shadow: 0 0 8px #22c55e; }
      50% { opacity: 0.7; box-shadow: 0 0 12px #22c55e; }
    }

    .status-text {
      font-family: var(--font-mono);
      letter-spacing: 0.02em;
    }

    .utility-links {
      display: flex;
      align-items: center;
      gap: 12px;
    }

    .utility-link {
      display: flex;
      align-items: center;
      gap: 6px;
      color: var(--text-muted);
      text-decoration: none;
      transition: color 0.2s;

      &:hover {
        color: var(--text-primary);
      }

      svg {
        opacity: 0.7;
      }
    }

    .version-tag {
      font-family: var(--font-mono);
      font-size: 10px;
      padding: 2px 6px;
      background: var(--bg-elevated);
      border: 1px solid var(--border-subtle);
      border-radius: 4px;
      color: var(--accent-cyan);
    }

    .utility-divider {
      color: var(--border-medium);
    }

    /* ============================================
       MAIN NAVIGATION
    ============================================ */
    header {
      position: fixed;
      top: 0;
      left: 0;
      right: 0;
      z-index: 1000;
    }

    .main-nav {
      background: rgba(5, 5, 8, 0.6);
      backdrop-filter: blur(16px);
      border-bottom: 1px solid var(--border-subtle);
      transition: background 0.3s;
    }

    header.scrolled .main-nav {
      background: rgba(5, 5, 8, 0.95);
    }

    .nav-content {
      max-width: 1280px;
      margin: 0 auto;
      padding: 0 24px;
      height: 64px;
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: 32px;
    }

    /* ============================================
       LOGO BLOCK
    ============================================ */
    .logo-block {
      display: flex;
      align-items: center;
      gap: 12px;
      text-decoration: none;
      color: var(--text-primary);
      transition: opacity 0.2s;
      position: relative;

      // Subtle orange glow under logo on hover
      &::after {
        content: '';
        position: absolute;
        left: 0;
        top: 50%;
        width: 42px;
        height: 42px;
        transform: translateY(-50%);
        background: radial-gradient(circle, rgba(255, 122, 48, 0.4) 0%, transparent 70%);
        border-radius: 50%;
        opacity: 0;
        filter: blur(8px);
        transition: opacity 0.3s ease, filter 0.15s ease;
        pointer-events: none;
        z-index: -1;
      }

      &:hover {
        opacity: 1;

        &::after {
          opacity: 1;
        }
      }

      &:active {
        &::after {
          opacity: 1;
          background: radial-gradient(circle, rgba(255, 122, 48, 0.7) 0%, rgba(255, 122, 48, 0.2) 50%, transparent 70%);
          filter: blur(12px);
        }

        .logo-img {
          filter: drop-shadow(0 0 16px rgba(255, 122, 48, 0.6));
        }
      }
    }

    .logo-img {
      width: 42px;
      height: 42px;
      object-fit: contain;
      filter: drop-shadow(0 0 12px rgba(59, 130, 246, 0.3));
      transition: filter 0.15s ease;
    }

    .logo-stack {
      display: flex;
      flex-direction: column;
      gap: 1px;

      @media (max-width: 500px) {
        display: none;
      }
    }

    .logo-name {
      font-family: var(--font-mono);
      font-weight: 700;
      font-size: 15px;
      letter-spacing: -0.02em;
      line-height: 1.2;
    }

    .logo-tagline {
      font-size: 11px;
      color: var(--text-muted);
      letter-spacing: 0.04em;
      text-transform: uppercase;
    }

    /* ============================================
       KEYBOARD-STYLE NAVIGATION
    ============================================ */
    .nav-keys {
      display: flex;
      align-items: center;
      gap: 4px;

      @media (max-width: 900px) {
        display: none;
      }
    }

    .nav-key {
      display: flex;
      align-items: center;
      gap: 10px;
      padding: 8px 14px;
      text-decoration: none;
      color: var(--text-secondary);
      border-radius: 8px;
      transition: all 0.2s;
      position: relative;

      &::before {
        content: '';
        position: absolute;
        inset: 0;
        background: var(--bg-elevated);
        border-radius: 8px;
        opacity: 0;
        transition: opacity 0.2s;
      }

      &:hover {
        color: var(--text-primary);

        &::before {
          opacity: 1;
        }

        kbd {
          background: var(--accent-cyan);
          color: var(--bg-primary);
          border-color: var(--accent-cyan);
        }
      }

      .key-label {
        position: relative;
        font-size: 13px;
        font-weight: 500;
      }

      kbd {
        position: relative;
        font-family: var(--font-mono);
        font-size: 10px;
        padding: 3px 6px;
        background: var(--bg-secondary);
        border: 1px solid var(--border-medium);
        border-radius: 4px;
        color: var(--text-muted);
        transition: all 0.2s;
      }
    }

    /* ============================================
       ACTION CLUSTER
    ============================================ */
    .nav-actions {
      display: flex;
      align-items: center;
      gap: 12px;
    }

    .cmd-trigger {
      display: flex;
      align-items: center;
      gap: 10px;
      padding: 8px 12px;
      background: var(--bg-secondary);
      border: 1px solid var(--border-subtle);
      border-radius: 8px;
      cursor: pointer;
      transition: all 0.2s;
      color: var(--text-secondary);

      &:hover {
        background: var(--bg-elevated);
        border-color: var(--border-medium);
        color: var(--text-primary);
      }

      @media (max-width: 900px) {
        .cmd-shortcut {
          display: none;
        }
      }
    }

    .cmd-icon {
      width: 18px;
      height: 18px;

      svg {
        width: 100%;
        height: 100%;
      }

      path {
        transition: opacity 0.2s;

        &.hidden {
          opacity: 0;
        }
      }
    }

    .cmd-shortcut {
      display: flex;
      gap: 2px;

      kbd {
        font-family: var(--font-mono);
        font-size: 11px;
        padding: 2px 5px;
        background: var(--bg-elevated);
        border: 1px solid var(--border-subtle);
        border-radius: 3px;
      }
    }

    .cta-block {
      display: flex;
      align-items: center;
      gap: 8px;
      padding: 10px 18px;
      background: var(--text-primary);
      color: var(--bg-primary);
      text-decoration: none;
      font-size: 13px;
      font-weight: 600;
      border-radius: 8px;
      transition: all 0.2s;
      overflow: hidden;
      position: relative;

      &::before {
        content: '';
        position: absolute;
        inset: 0;
        background: linear-gradient(90deg, transparent, rgba(78, 205, 196, 0.3), transparent);
        transform: translateX(-100%);
        transition: transform 0.5s;
      }

      &:hover {
        transform: translateY(-1px);
        box-shadow: 0 8px 20px -8px rgba(248, 250, 252, 0.4);

        &::before {
          transform: translateX(100%);
        }

        .cta-arrow {
          transform: translateX(3px);
        }
      }

      @media (max-width: 600px) {
        display: none;
      }
    }

    .cta-text {
      position: relative;
    }

    .cta-arrow {
      display: flex;
      transition: transform 0.2s;

      svg {
        width: 16px;
        height: 16px;
      }
    }

    /* ============================================
       SCROLL ACCENT LINE
    ============================================ */
    .scroll-accent {
      height: 2px;
      background: transparent;
    }

    .scroll-fill {
      height: 100%;
      background: linear-gradient(90deg, var(--accent-cyan), var(--accent-blue), var(--accent-purple));
      transition: width 0.1s linear;
    }

    /* ============================================
       COMMAND PALETTE OVERLAY
    ============================================ */
    .cmd-overlay {
      position: fixed;
      inset: 0;
      z-index: 1100;
      background: rgba(3, 3, 5, 0.8);
      backdrop-filter: blur(8px);
      display: flex;
      justify-content: center;
      padding-top: 120px;
      opacity: 0;
      visibility: hidden;
      transition: all 0.25s var(--ease-out);

      &.active {
        opacity: 1;
        visibility: visible;

        .cmd-panel {
          transform: translateY(0) scale(1);
          opacity: 1;
        }
      }
    }

    .cmd-panel {
      width: 520px;
      max-width: calc(100vw - 32px);
      max-height: 480px;
      background: var(--bg-secondary);
      border: 1px solid var(--border-medium);
      border-radius: 16px;
      overflow: hidden;
      box-shadow:
        0 0 0 1px rgba(255, 255, 255, 0.05),
        0 24px 48px -12px rgba(0, 0, 0, 0.5),
        0 0 120px -40px rgba(78, 205, 196, 0.2);
      transform: translateY(-20px) scale(0.98);
      opacity: 0;
      transition: all 0.25s var(--ease-out);
    }

    .cmd-header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 16px 20px;
      border-bottom: 1px solid var(--border-subtle);
    }

    .cmd-search {
      display: flex;
      align-items: center;
      gap: 12px;
      color: var(--text-muted);

      svg {
        width: 18px;
        height: 18px;
        opacity: 0.6;
      }
    }

    .cmd-placeholder {
      font-size: 15px;
    }

    .cmd-esc {
      font-family: var(--font-mono);
      font-size: 11px;
      padding: 4px 8px;
      background: var(--bg-elevated);
      border: 1px solid var(--border-subtle);
      border-radius: 4px;
      color: var(--text-muted);
    }

    .cmd-list {
      padding: 12px;
      max-height: 340px;
      overflow-y: auto;
    }

    .cmd-group {
      &:not(:last-child) {
        margin-bottom: 16px;
      }
    }

    .cmd-group-label {
      display: block;
      font-size: 11px;
      font-weight: 600;
      text-transform: uppercase;
      letter-spacing: 0.08em;
      color: var(--text-muted);
      padding: 8px 12px 8px;
    }

    .cmd-item {
      display: flex;
      align-items: center;
      gap: 12px;
      padding: 12px;
      text-decoration: none;
      color: var(--text-secondary);
      border-radius: 10px;
      transition: all 0.15s;

      &:hover {
        background: var(--bg-elevated);
        color: var(--text-primary);

        kbd {
          background: var(--accent-cyan);
          color: var(--bg-primary);
          border-color: var(--accent-cyan);
        }
      }

      kbd {
        margin-left: auto;
        font-family: var(--font-mono);
        font-size: 11px;
        padding: 3px 7px;
        background: var(--bg-card);
        border: 1px solid var(--border-subtle);
        border-radius: 4px;
        color: var(--text-muted);
        transition: all 0.15s;
      }
    }

    .cmd-item-icon {
      width: 32px;
      height: 32px;
      display: flex;
      align-items: center;
      justify-content: center;
      background: var(--bg-card);
      border-radius: 8px;

      svg {
        width: 16px;
        height: 16px;
      }

      &.accent {
        background: linear-gradient(135deg, rgba(78, 205, 196, 0.2), rgba(59, 130, 246, 0.2));

        svg {
          color: var(--accent-cyan);
        }
      }
    }

    .cmd-item-text {
      font-size: 14px;
      font-weight: 500;
    }

    .cmd-item-badge {
      margin-left: auto;
      font-size: 11px;
      padding: 3px 8px;
      background: rgba(78, 205, 196, 0.1);
      border: 1px solid rgba(78, 205, 196, 0.2);
      border-radius: 100px;
      color: var(--accent-cyan);
    }

    .cmd-item-external {
      margin-left: auto;
      font-size: 14px;
      color: var(--text-muted);
    }

    .cmd-footer {
      display: flex;
      align-items: center;
      justify-content: center;
      gap: 24px;
      padding: 12px 20px;
      border-top: 1px solid var(--border-subtle);
      background: var(--bg-elevated);
    }

    .cmd-hint {
      display: flex;
      align-items: center;
      gap: 4px;
      font-size: 12px;
      color: var(--text-muted);

      kbd {
        font-family: var(--font-mono);
        font-size: 10px;
        padding: 2px 5px;
        background: var(--bg-card);
        border: 1px solid var(--border-subtle);
        border-radius: 3px;
      }
    }

    /* ============================================
       MOBILE DOCK NAVIGATION
    ============================================ */
    .mobile-dock {
      display: none;
      position: fixed;
      bottom: 0;
      left: 0;
      right: 0;
      z-index: 999;
      background: rgba(5, 5, 8, 0.95);
      backdrop-filter: blur(20px);
      border-top: 1px solid var(--border-subtle);
      padding: 8px 16px calc(8px + env(safe-area-inset-bottom));
      justify-content: space-around;
      transition: transform 0.3s var(--ease-out);

      @media (max-width: 900px) {
        display: flex;
      }

      &.hidden {
        transform: translateY(100%);
      }
    }

    .dock-item {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 4px;
      padding: 8px 16px;
      text-decoration: none;
      color: var(--text-muted);
      font-size: 11px;
      font-weight: 500;
      border-radius: 10px;
      transition: all 0.2s;

      svg {
        width: 22px;
        height: 22px;
      }

      &:hover, &:active {
        color: var(--text-primary);
        background: var(--bg-elevated);
      }

      &.primary {
        background: linear-gradient(135deg, var(--accent-cyan), var(--accent-blue));
        color: var(--bg-primary);

        &:hover {
          background: linear-gradient(135deg, var(--accent-cyan), var(--accent-blue));
        }
      }
    }
  `]
})
export class NavbarComponent implements OnInit, OnDestroy {
  isScrolled = signal(false);
  scrollProgress = signal(0);
  isNavExpanded = signal(false);
  isScrollingDown = signal(false);

  private ngZone = inject(NgZone);
  private lastScrollY = 0;

  toggleNav() {
    this.isNavExpanded.update((v) => !v);
    document.body.style.overflow = this.isNavExpanded() ? 'hidden' : '';
  }

  closeNav() {
    this.isNavExpanded.set(false);
    document.body.style.overflow = '';
  }

  @HostListener('window:keydown.escape')
  onEscapeKey() {
    if (this.isNavExpanded()) {
      this.closeNav();
    }
  }

  @HostListener('window:keydown', ['$event'])
  onKeyDown(event: KeyboardEvent) {
    // Command palette shortcut: Cmd/Ctrl + K
    if ((event.metaKey || event.ctrlKey) && event.key === 'k') {
      event.preventDefault();
      this.toggleNav();
    }

    // Quick nav shortcuts when not in input
    if (document.activeElement?.tagName !== 'INPUT' && !this.isNavExpanded()) {
      const shortcuts: Record<string, string> = {
        'f': '#features',
        'h': '#how-it-works',
        'p': '#pricing',
        'd': '/docs'
      };

      if (shortcuts[event.key.toLowerCase()]) {
        event.preventDefault();
        window.location.href = shortcuts[event.key.toLowerCase()];
      }
    }
  }

  ngOnInit() {
    this.ngZone.runOutsideAngular(() => {
      window.addEventListener('scroll', this.onScroll, { passive: true });
    });
  }

  ngOnDestroy() {
    window.removeEventListener('scroll', this.onScroll);
  }

  onScroll = () => {
    const scrollTop = window.scrollY;

    // Detect scroll direction for mobile dock
    const isScrollingDown = scrollTop > this.lastScrollY && scrollTop > 100;
    this.lastScrollY = scrollTop;

    // Optimization: Calculate values first
    const isScrolled = scrollTop > 50;

    // Only re-enter zone if state needs to change
    if (this.isScrolled() !== isScrolled || this.isScrollingDown() !== isScrollingDown) {
      this.ngZone.run(() => {
        this.isScrolled.set(isScrolled);
        this.isScrollingDown.set(isScrollingDown);
      });
    }

    const docHeight = document.documentElement.scrollHeight - window.innerHeight;
    const progress = (scrollTop / docHeight) * 100;

    this.ngZone.run(() => {
      this.scrollProgress.set(progress);
    });
  }
}
