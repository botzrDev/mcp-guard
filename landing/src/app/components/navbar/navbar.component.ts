import { Component, signal, ChangeDetectionStrategy, NgZone, OnInit, OnDestroy, inject, HostListener } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterModule } from '@angular/router';
import { AuthService } from '../../core/auth';

@Component({
  selector: 'app-navbar',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, RouterModule],
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
            <a routerLink="/changelog" class="utility-link">
              <span class="version-tag">v1.0</span>
              Changelog
            </a>
            <span class="utility-divider">/</span>
            <a href="https://github.com/botzrdev/mcp-guard" target="_blank" class="utility-link">
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
          <a routerLink="/" class="logo-block">
            <img src="gateway.png" alt="MCP Guard" class="logo-img" />
            <div class="logo-stack">
              <span class="logo-name">mcp-guard</span>
              <span class="logo-tagline">Security Gateway</span>
            </div>
          </a>

          <!-- Keyboard-style navigation -->
          <div class="nav-keys">
            <a routerLink="/" fragment="features" class="nav-key" data-key="F">
              <span class="key-label">Features</span>
              <kbd>F</kbd>
            </a>
            <a routerLink="/" fragment="how-it-works" class="nav-key" data-key="H">
              <span class="key-label">How it works</span>
              <kbd>H</kbd>
            </a>
            <a routerLink="/" fragment="pricing" class="nav-key" data-key="P">
              <span class="key-label">Pricing</span>
              <kbd>P</kbd>
            </a>
            <a routerLink="/docs" class="nav-key" data-key="D">
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

            @if (authService.isAuthenticated()) {
              <a routerLink="/dashboard" class="dashboard-link">
                @if (authService.user()?.avatar_url) {
                  <img [src]="authService.user()?.avatar_url" alt="Avatar" class="user-avatar" />
                } @else {
                  <div class="user-avatar-placeholder">
                    {{ authService.user()?.name?.charAt(0) || 'U' }}
                  </div>
                }
                <span class="dashboard-text">Dashboard</span>
              </a>
            } @else {
              <a routerLink="/login" class="login-link">
                Sign in
              </a>
              <a routerLink="/docs/quickstart" class="cta-block">
                <span class="cta-text">Get Started</span>
                <span class="cta-arrow">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M5 12h14M12 5l7 7-7 7"/>
                  </svg>
                </span>
              </a>
            }
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
            <a href="https://github.com/botzrdev/mcp-guard" target="_blank" class="cmd-item" (click)="closeNav()">
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
      padding: var(--space-2) 0;
      font-size: var(--text-xs);
      transition: transform var(--duration-normal) var(--ease-out), opacity var(--duration-normal) var(--ease-out);

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
      padding: 0 var(--container-px);
      display: flex;
      justify-content: space-between;
      align-items: center;
    }

    .status-cluster {
      display: flex;
      align-items: center;
      gap: var(--space-2);
      color: var(--text-muted);
    }

    .status-dot {
      width: var(--space-1-5);
      height: var(--space-1-5);
      background: var(--accent-green);
      border-radius: var(--radius-full);
      box-shadow: 0 0 8px var(--accent-green);
      animation: pulse-glow 2s ease-in-out infinite;
    }

    @keyframes pulse-glow {
      0%, 100% { opacity: 1; box-shadow: 0 0 8px var(--accent-green); }
      50% { opacity: 0.7; box-shadow: 0 0 12px var(--accent-green); }
    }

    .status-text {
      font-family: var(--font-mono);
      letter-spacing: var(--tracking-wide);
      line-height: var(--leading-normal);
    }

    .utility-links {
      display: flex;
      align-items: center;
      gap: var(--space-3);
    }

    .utility-link {
      display: flex;
      align-items: center;
      gap: var(--space-1-5);
      color: var(--text-muted);
      text-decoration: none;
      line-height: var(--leading-normal);
      transition: color var(--duration-fast) var(--ease-out);

      &:hover {
        color: var(--text-primary);
      }

      svg {
        opacity: 0.7;
      }
    }

    .version-tag {
      font-family: var(--font-mono);
      font-size: var(--text-xs);
      padding: var(--space-0-5) var(--space-1-5);
      background: var(--bg-elevated);
      border: 1px solid var(--border-subtle);
      border-radius: var(--radius-sm);
      color: var(--accent-cyan);
      line-height: var(--leading-normal);
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
      z-index: var(--z-modal);
    }

    .main-nav {
      background: rgba(5, 5, 8, 0.6);
      backdrop-filter: blur(16px);
      border-bottom: 1px solid var(--border-subtle);
      transition: background var(--duration-normal) var(--ease-out);
    }

    header.scrolled .main-nav {
      background: rgba(5, 5, 8, 0.95);
    }

    .nav-content {
      max-width: 1280px;
      margin: 0 auto;
      padding: 0 var(--container-px);
      height: var(--navbar-height);
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: var(--space-8);
    }

    /* ============================================
       LOGO BLOCK
    ============================================ */
    .logo-block {
      display: flex;
      align-items: center;
      gap: var(--space-3);
      text-decoration: none;
      color: var(--text-primary);
      transition: opacity var(--duration-fast) var(--ease-out);
      position: relative;

      // Subtle orange glow under logo on hover
      &::after {
        content: '';
        position: absolute;
        left: 0;
        top: 50%;
        width: var(--space-11);
        height: var(--space-11);
        transform: translateY(-50%);
        background: radial-gradient(circle, rgba(255, 122, 48, 0.4) 0%, transparent 70%);
        border-radius: var(--radius-full);
        opacity: 0;
        filter: blur(8px);
        transition: opacity var(--duration-normal) var(--ease-out), filter var(--duration-fast) var(--ease-out);
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
      width: var(--space-11);
      height: var(--space-11);
      object-fit: contain;
      filter: drop-shadow(0 0 12px rgba(59, 130, 246, 0.3));
      transition: filter var(--duration-fast) var(--ease-out);
    }

    .logo-stack {
      display: flex;
      flex-direction: column;
      gap: var(--space-px);

      @media (max-width: 500px) {
        display: none;
      }
    }

    .logo-name {
      font-family: var(--font-mono);
      font-weight: var(--weight-bold);
      font-size: var(--text-base);
      letter-spacing: var(--tracking-tight);
      line-height: var(--leading-snug);
    }

    .logo-tagline {
      font-size: var(--text-xs);
      color: var(--text-muted);
      letter-spacing: var(--tracking-wider);
      text-transform: uppercase;
      line-height: var(--leading-normal);
    }

    /* ============================================
       KEYBOARD-STYLE NAVIGATION
    ============================================ */
    .nav-keys {
      display: flex;
      align-items: center;
      gap: var(--space-1);

      @media (max-width: 900px) {
        display: none;
      }
    }

    .nav-key {
      display: flex;
      align-items: center;
      gap: var(--space-2-5);
      padding: var(--space-2) var(--space-3-5);
      text-decoration: none;
      color: var(--text-secondary);
      border-radius: var(--radius-lg);
      transition: all var(--duration-fast) var(--ease-out);
      position: relative;

      &::before {
        content: '';
        position: absolute;
        inset: 0;
        background: var(--bg-elevated);
        border-radius: var(--radius-lg);
        opacity: 0;
        transition: opacity var(--duration-fast) var(--ease-out);
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
        font-size: var(--text-sm);
        font-weight: var(--weight-medium);
        line-height: var(--leading-normal);
      }

      kbd {
        position: relative;
        font-family: var(--font-mono);
        font-size: var(--text-xs);
        padding: var(--space-0-5) var(--space-1-5);
        background: var(--bg-secondary);
        border: 1px solid var(--border-medium);
        border-radius: var(--radius-sm);
        color: var(--text-muted);
        line-height: var(--leading-normal);
        transition: all var(--duration-fast) var(--ease-out);
      }
    }

    /* ============================================
       ACTION CLUSTER
    ============================================ */
    .nav-actions {
      display: flex;
      align-items: center;
      gap: var(--space-3);
    }

    .cmd-trigger {
      display: flex;
      align-items: center;
      gap: var(--space-2-5);
      padding: var(--space-2) var(--space-3);
      background: var(--bg-secondary);
      border: 1px solid var(--border-subtle);
      border-radius: var(--radius-lg);
      cursor: pointer;
      transition: all var(--duration-fast) var(--ease-out);
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
      width: var(--icon-sm);
      height: var(--icon-sm);

      svg {
        width: 100%;
        height: 100%;
      }

      path {
        transition: opacity var(--duration-fast) var(--ease-out);

        &.hidden {
          opacity: 0;
        }
      }
    }

    .cmd-shortcut {
      display: flex;
      gap: var(--space-0-5);

      kbd {
        font-family: var(--font-mono);
        font-size: var(--text-xs);
        padding: var(--space-0-5) var(--space-1);
        background: var(--bg-elevated);
        border: 1px solid var(--border-subtle);
        border-radius: var(--radius-sm);
        line-height: var(--leading-normal);
      }
    }

    .cta-block {
      display: flex;
      align-items: center;
      gap: var(--space-2);
      padding: var(--space-2-5) var(--space-5);
      background: var(--text-primary);
      color: var(--bg-primary);
      text-decoration: none;
      font-size: var(--text-sm);
      font-weight: var(--weight-semibold);
      line-height: var(--leading-normal);
      border-radius: var(--radius-lg);
      transition: all var(--duration-fast) var(--ease-out);
      overflow: hidden;
      position: relative;

      &::before {
        content: '';
        position: absolute;
        inset: 0;
        background: linear-gradient(90deg, transparent, rgba(78, 205, 196, 0.3), transparent);
        transform: translateX(-100%);
        transition: transform var(--duration-slower) var(--ease-out);
      }

      &:hover {
        transform: translateY(-1px);
        box-shadow: var(--shadow-lg);

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
      transition: transform var(--duration-fast) var(--ease-out);

      svg {
        width: var(--icon-sm);
        height: var(--icon-sm);
      }
    }

    /* ============================================
       AUTH LINKS
    ============================================ */
    .login-link {
      padding: var(--space-2) var(--space-4);
      color: var(--text-secondary);
      text-decoration: none;
      font-size: var(--text-sm);
      font-weight: var(--weight-medium);
      border-radius: var(--radius-lg);
      transition: all var(--duration-fast) var(--ease-out);

      &:hover {
        color: var(--text-primary);
        background: var(--bg-elevated);
      }

      @media (max-width: 600px) {
        display: none;
      }
    }

    .dashboard-link {
      display: flex;
      align-items: center;
      gap: var(--space-2);
      padding: var(--space-1-5) var(--space-3);
      background: var(--bg-secondary);
      border: 1px solid var(--border-subtle);
      border-radius: var(--radius-lg);
      text-decoration: none;
      color: var(--text-primary);
      font-size: var(--text-sm);
      font-weight: var(--weight-medium);
      transition: all var(--duration-fast) var(--ease-out);

      &:hover {
        background: var(--bg-elevated);
        border-color: var(--border-accent);
      }
    }

    .user-avatar {
      width: var(--space-7);
      height: var(--space-7);
      border-radius: var(--radius-full);
      object-fit: cover;
    }

    .user-avatar-placeholder {
      width: var(--space-7);
      height: var(--space-7);
      border-radius: var(--radius-full);
      background: var(--gradient-brand);
      color: var(--bg-primary);
      display: flex;
      align-items: center;
      justify-content: center;
      font-size: var(--text-sm);
      font-weight: var(--weight-bold);
    }

    .dashboard-text {
      @media (max-width: 768px) {
        display: none;
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
      z-index: var(--z-modal);
      background: rgba(3, 3, 5, 0.8);
      backdrop-filter: blur(8px);
      display: flex;
      justify-content: center;
      padding-top: var(--space-28);
      opacity: 0;
      visibility: hidden;
      transition: all var(--duration-normal) var(--ease-out);

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
      max-width: calc(100vw - var(--space-8));
      max-height: 480px;
      background: var(--bg-secondary);
      border: 1px solid var(--border-medium);
      border-radius: var(--radius-2xl);
      overflow: hidden;
      box-shadow: var(--shadow-xl);
      transform: translateY(-20px) scale(0.98);
      opacity: 0;
      transition: all var(--duration-normal) var(--ease-out);
    }

    .cmd-header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: var(--space-4) var(--space-5);
      border-bottom: 1px solid var(--border-subtle);
    }

    .cmd-search {
      display: flex;
      align-items: center;
      gap: var(--space-3);
      color: var(--text-muted);

      svg {
        width: var(--icon-sm);
        height: var(--icon-sm);
        opacity: 0.6;
      }
    }

    .cmd-placeholder {
      font-size: var(--text-base);
      line-height: var(--leading-normal);
    }

    .cmd-esc {
      font-family: var(--font-mono);
      font-size: var(--text-xs);
      padding: var(--space-1) var(--space-2);
      background: var(--bg-elevated);
      border: 1px solid var(--border-subtle);
      border-radius: var(--radius-sm);
      color: var(--text-muted);
      line-height: var(--leading-normal);
    }

    .cmd-list {
      padding: var(--space-3);
      max-height: 340px;
      overflow-y: auto;
    }

    .cmd-group {
      &:not(:last-child) {
        margin-bottom: var(--space-4);
      }
    }

    .cmd-group-label {
      display: block;
      font-size: var(--text-xs);
      font-weight: var(--weight-semibold);
      text-transform: uppercase;
      letter-spacing: var(--tracking-widest);
      color: var(--text-muted);
      padding: var(--space-2) var(--space-3);
      line-height: var(--leading-normal);
    }

    .cmd-item {
      display: flex;
      align-items: center;
      gap: var(--space-3);
      padding: var(--space-3);
      text-decoration: none;
      color: var(--text-secondary);
      border-radius: var(--radius-lg);
      transition: all var(--duration-fast) var(--ease-out);

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
        font-size: var(--text-xs);
        padding: var(--space-0-5) var(--space-2);
        background: var(--bg-card);
        border: 1px solid var(--border-subtle);
        border-radius: var(--radius-sm);
        color: var(--text-muted);
        line-height: var(--leading-normal);
        transition: all var(--duration-fast) var(--ease-out);
      }
    }

    .cmd-item-icon {
      width: var(--space-8);
      height: var(--space-8);
      display: flex;
      align-items: center;
      justify-content: center;
      background: var(--bg-card);
      border-radius: var(--radius-lg);

      svg {
        width: var(--icon-sm);
        height: var(--icon-sm);
      }

      &.accent {
        background: linear-gradient(135deg, rgba(78, 205, 196, 0.2), rgba(59, 130, 246, 0.2));

        svg {
          color: var(--accent-cyan);
        }
      }
    }

    .cmd-item-text {
      font-size: var(--text-sm);
      font-weight: var(--weight-medium);
      line-height: var(--leading-normal);
    }

    .cmd-item-badge {
      margin-left: auto;
      font-size: var(--text-xs);
      padding: var(--space-0-5) var(--space-2);
      background: rgba(78, 205, 196, 0.1);
      border: 1px solid rgba(78, 205, 196, 0.2);
      border-radius: var(--radius-full);
      color: var(--accent-cyan);
      line-height: var(--leading-normal);
    }

    .cmd-item-external {
      margin-left: auto;
      font-size: var(--text-sm);
      color: var(--text-muted);
    }

    .cmd-footer {
      display: flex;
      align-items: center;
      justify-content: center;
      gap: var(--space-6);
      padding: var(--space-3) var(--space-5);
      border-top: 1px solid var(--border-subtle);
      background: var(--bg-elevated);
    }

    .cmd-hint {
      display: flex;
      align-items: center;
      gap: var(--space-1);
      font-size: var(--text-xs);
      color: var(--text-muted);
      line-height: var(--leading-normal);

      kbd {
        font-family: var(--font-mono);
        font-size: var(--text-xs);
        padding: var(--space-0-5) var(--space-1);
        background: var(--bg-card);
        border: 1px solid var(--border-subtle);
        border-radius: var(--radius-sm);
        line-height: var(--leading-normal);
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
      z-index: var(--z-fixed);
      background: rgba(5, 5, 8, 0.95);
      backdrop-filter: blur(20px);
      border-top: 1px solid var(--border-subtle);
      padding: var(--space-2) var(--space-4) calc(var(--space-2) + env(safe-area-inset-bottom));
      justify-content: space-around;
      transition: transform var(--duration-normal) var(--ease-out);

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
      gap: var(--space-1);
      padding: var(--space-2) var(--space-4);
      text-decoration: none;
      color: var(--text-muted);
      font-size: var(--text-xs);
      font-weight: var(--weight-medium);
      line-height: var(--leading-normal);
      border-radius: var(--radius-lg);
      transition: all var(--duration-fast) var(--ease-out);

      svg {
        width: var(--icon-lg);
        height: var(--icon-lg);
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
  authService = inject(AuthService);

  isScrolled = signal(false);
  scrollProgress = signal(0);
  isNavExpanded = signal(false);
  isScrollingDown = signal(false);

  private ngZone = inject(NgZone);
  private lastScrollY = 0;
  private lastProgress = 0;
  private rafId: number | null = null;
  private pendingUpdate = false;

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
    if (this.rafId !== null) {
      cancelAnimationFrame(this.rafId);
    }
  }

  onScroll = () => {
    // Throttle updates using requestAnimationFrame
    if (this.pendingUpdate) return;
    this.pendingUpdate = true;

    this.rafId = requestAnimationFrame(() => {
      this.pendingUpdate = false;

      const scrollTop = window.scrollY;

      // Detect scroll direction for mobile dock
      const isScrollingDown = scrollTop > this.lastScrollY && scrollTop > 100;
      this.lastScrollY = scrollTop;

      // Calculate values
      const isScrolled = scrollTop > 50;
      const docHeight = document.documentElement.scrollHeight - window.innerHeight;
      const progress = docHeight > 0 ? Math.round((scrollTop / docHeight) * 100) : 0;

      // Batch all state changes into a single zone entry
      const needsScrolledUpdate = this.isScrolled() !== isScrolled;
      const needsDirectionUpdate = this.isScrollingDown() !== isScrollingDown;
      const needsProgressUpdate = Math.abs(this.lastProgress - progress) >= 1; // Only update if changed by 1%+

      if (needsScrolledUpdate || needsDirectionUpdate || needsProgressUpdate) {
        this.ngZone.run(() => {
          if (needsScrolledUpdate) this.isScrolled.set(isScrolled);
          if (needsDirectionUpdate) this.isScrollingDown.set(isScrollingDown);
          if (needsProgressUpdate) {
            this.scrollProgress.set(progress);
            this.lastProgress = progress;
          }
        });
      }
    });
  }
}
