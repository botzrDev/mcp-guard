import { Component, signal, OnInit, OnDestroy, ChangeDetectionStrategy } from '@angular/core';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-hero',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule],
  template: `
    <!-- Simplified background - just one subtle glow -->
    <div class="hero-bg">
      <div class="glow-orb"></div>
    </div>

    <section class="hero">
      <div class="hero-container">
        <!-- Clean centered layout -->
        <div class="hero-content">
          <!-- Badge -->
          <div class="hero-badge">
            <span class="badge-dot"></span>
            <span class="badge-text">Now with OAuth 2.1 + PKCE</span>
          </div>

          <!-- Title -->
          <h1 class="hero-title">
            <span class="title-line">Secure your MCP servers</span>
            <span class="title-line gradient-text">in 5 minutes</span>
          </h1>

          <!-- Subtitle -->
          <p class="hero-subtitle">
            A single Rust binary that adds OAuth, JWT, rate limiting, and audit logs to any MCP server.
            <span class="no-wrap">No Docker. No Kubernetes. No DevOps team.</span>
          </p>

          <!-- CTAs -->
          <div class="hero-cta">
            <a href="/docs/quickstart" class="btn-primary">
              Get Started
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M5 12h14M12 5l7 7-7 7"/>
              </svg>
            </a>
            <a href="https://github.com/mcp-guard/mcp-guard" target="_blank" class="btn-secondary">
              <svg viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
              </svg>
              Star on GitHub
            </a>
          </div>

          <!-- Simple install command -->
          <div class="install-bar">
            <code class="install-code">
              <span class="prompt">$</span>
              <span class="command">cargo install mcp-guard</span>
            </code>
            <button class="copy-btn" (click)="copyCommand()" [class.copied]="copied()">
              @if (copied()) {
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="20 6 9 17 4 12"/>
                </svg>
                <span>Copied</span>
              } @else {
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
                  <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/>
                </svg>
                <span>Copy</span>
              }
            </button>
          </div>
        </div>

        <!-- Scroll indicator -->
        <div class="scroll-indicator">
          <div class="scroll-line"></div>
        </div>
      </div>
    </section>
  `,
  styles: [`
    .hero-bg {
      position: absolute;
      inset: 0;
      overflow: hidden;
      pointer-events: none;
    }

    .glow-orb {
      position: absolute;
      top: 20%;
      left: 50%;
      transform: translateX(-50%);
      width: 800px;
      height: 600px;
      background: radial-gradient(ellipse, rgba(255, 122, 48, 0.08) 0%, transparent 60%);
      filter: blur(60px);
    }

    .hero {
      position: relative;
      min-height: 100vh;
      display: flex;
      align-items: center;
      padding: var(--space-28) 0 var(--space-20);
    }

    .hero-container {
      max-width: 900px;
      margin: 0 auto;
      padding: 0 var(--container-px);
      width: 100%;
    }

    .hero-content {
      text-align: center;
    }

    /* Badge */
    .hero-badge {
      display: inline-flex;
      align-items: center;
      gap: var(--space-2-5);
      padding: var(--space-2) var(--space-4) var(--space-2) var(--space-3);
      background: rgba(255, 122, 48, 0.08);
      border: 1px solid rgba(255, 122, 48, 0.15);
      border-radius: var(--radius-full);
      margin-bottom: var(--space-8);
    }

    .badge-dot {
      width: var(--space-2);
      height: var(--space-2);
      background: var(--accent-cyan);
      border-radius: var(--radius-full);
      animation: pulse 2s ease-in-out infinite;
    }

    .badge-text {
      font-family: var(--font-mono);
      font-size: var(--text-sm);
      color: var(--text-secondary);
      letter-spacing: var(--tracking-wide);
      line-height: var(--leading-normal);
    }

    @keyframes pulse {
      0%, 100% { opacity: 1; }
      50% { opacity: 0.5; }
    }

    /* Title */
    .hero-title {
      margin-bottom: var(--space-6);
    }

    .title-line {
      display: block;
      font-family: var(--font-display);
      font-size: var(--text-5xl);
      font-weight: var(--weight-bold);
      letter-spacing: var(--tracking-tighter);
      line-height: var(--leading-tight);
    }

    .gradient-text {
      background: var(--gradient-brand);
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
      background-clip: text;
    }

    /* Subtitle */
    .hero-subtitle {
      font-size: var(--text-lg);
      color: var(--text-secondary);
      line-height: var(--leading-relaxed);
      max-width: 600px;
      margin: 0 auto var(--space-10);
    }

    .no-wrap {
      display: block;
      color: var(--text-muted);
      margin-top: var(--space-2);
    }

    /* CTAs */
    .hero-cta {
      display: flex;
      justify-content: center;
      gap: var(--space-4);
      margin-bottom: var(--space-12);

      @media (max-width: 480px) {
        flex-direction: column;
        align-items: stretch;
      }
    }

    .btn-primary {
      display: inline-flex;
      align-items: center;
      justify-content: center;
      gap: var(--space-2-5);
      padding: var(--space-4) var(--space-8);
      background: var(--gradient-brand);
      color: var(--bg-primary);
      text-decoration: none;
      font-size: var(--text-base);
      font-weight: var(--weight-semibold);
      line-height: var(--leading-normal);
      border-radius: var(--radius-xl);
      transition: transform var(--duration-fast) var(--ease-out), box-shadow var(--duration-fast) var(--ease-out);

      svg {
        width: var(--icon-sm);
        height: var(--icon-sm);
        transition: transform var(--duration-fast) var(--ease-out);
      }

      &:hover {
        transform: translateY(-2px);
        box-shadow: var(--shadow-glow-orange);

        svg {
          transform: translateX(var(--space-1));
        }
      }

      &:active {
        transform: translateY(0);
      }
    }

    .btn-secondary {
      display: inline-flex;
      align-items: center;
      justify-content: center;
      gap: var(--space-2-5);
      padding: var(--space-4) var(--space-7);
      background: transparent;
      color: var(--text-primary);
      text-decoration: none;
      font-size: var(--text-base);
      font-weight: var(--weight-medium);
      line-height: var(--leading-normal);
      border-radius: var(--radius-xl);
      border: 1px solid var(--border-medium);
      transition: all var(--duration-fast) var(--ease-out);

      svg {
        width: var(--icon-md);
        height: var(--icon-md);
      }

      &:hover {
        background: var(--bg-elevated);
        border-color: var(--border-accent);
        transform: translateY(-1px);
      }

      &:active {
        transform: translateY(0);
      }
    }

    /* Install bar */
    .install-bar {
      display: inline-flex;
      align-items: center;
      gap: var(--space-3);
      padding: var(--space-1-5) var(--space-1-5) var(--space-1-5) var(--space-5);
      background: var(--bg-secondary);
      border: 1px solid var(--border-subtle);
      border-radius: var(--radius-xl);
    }

    .install-code {
      font-family: var(--font-mono);
      font-size: var(--text-sm);
      line-height: var(--leading-normal);
      display: flex;
      gap: var(--space-2);
    }

    .prompt {
      color: var(--accent-cyan);
    }

    .command {
      color: var(--text-primary);
    }

    .copy-btn {
      display: inline-flex;
      align-items: center;
      gap: var(--space-1-5);
      padding: var(--space-2-5) var(--space-4);
      background: var(--bg-elevated);
      border: 1px solid var(--border-subtle);
      border-radius: var(--radius-lg);
      color: var(--text-muted);
      font-size: var(--text-sm);
      font-weight: var(--weight-medium);
      line-height: var(--leading-normal);
      cursor: pointer;
      transition: all var(--duration-fast) var(--ease-out);

      svg {
        width: var(--icon-sm);
        height: var(--icon-sm);
      }

      &:hover {
        color: var(--text-primary);
        background: var(--bg-hover);
        border-color: var(--border-medium);
      }

      &:active {
        transform: scale(0.98);
      }

      &.copied {
        color: var(--accent-green);
        border-color: var(--border-green);
      }
    }

    /* Scroll indicator */
    .scroll-indicator {
      position: absolute;
      bottom: var(--space-10);
      left: 50%;
      transform: translateX(-50%);
    }

    .scroll-line {
      width: 2px;
      height: var(--space-12);
      background: linear-gradient(to bottom, var(--accent-cyan), transparent);
      border-radius: var(--radius-sm);
      animation: scrollPulse 2s ease-in-out infinite;
    }

    @keyframes scrollPulse {
      0%, 100% {
        opacity: 0.4;
        transform: scaleY(1);
      }
      50% {
        opacity: 1;
        transform: scaleY(1.2);
      }
    }

    @media (max-width: 640px) {
      .hero {
        padding: var(--space-24) 0 var(--space-16);
      }

      .hero-badge {
        margin-bottom: var(--space-6);
      }

      .hero-subtitle {
        font-size: var(--text-base);
        margin-bottom: var(--space-8);
      }

      .install-bar {
        flex-direction: column;
        padding: var(--space-4);
        gap: var(--space-3);
        width: 100%;
      }

      .copy-btn {
        width: 100%;
        justify-content: center;
      }

      .scroll-indicator {
        bottom: var(--space-6);
      }
    }
  `]
})
export class HeroComponent implements OnInit, OnDestroy {
  copied = signal(false);

  ngOnInit() { }

  ngOnDestroy() { }

  copyCommand() {
    navigator.clipboard.writeText('cargo install mcp-guard');
    this.copied.set(true);
    setTimeout(() => this.copied.set(false), 2000);
  }
}
