import { Component, signal, OnInit, OnDestroy, ChangeDetectionStrategy } from '@angular/core';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-hero',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule],
  template: `
    <!-- Security Grid Background -->
    <div class="hero-bg">
      <!-- Animated grid -->
      <div class="security-grid">
        <div class="grid-lines"></div>
        <div class="scan-line"></div>
      </div>
      
      <!-- Shield pulse effect -->
      <div class="shield-pulse">
        <div class="pulse-ring pulse-1"></div>
        <div class="pulse-ring pulse-2"></div>
        <div class="pulse-ring pulse-3"></div>
        <svg class="shield-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1">
          <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
        </svg>
      </div>
      
      <!-- Floating security nodes -->
      <div class="security-nodes">
        <div class="node node-1">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
            <path d="M7 11V7a5 5 0 0 1 10 0v4"/>
          </svg>
        </div>
        <div class="node node-2">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/>
            <polyline points="22 4 12 14.01 9 11.01"/>
          </svg>
        </div>
        <div class="node node-3">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
          </svg>
        </div>
      </div>
      
      <!-- Subtle gradient accent -->
      <div class="gradient-accent"></div>
    </div>

    <section class="hero">
      <div class="hero-container">
        <!-- Clean centered layout -->
        <div class="hero-content">
          <!-- Badge -->
          <div class="hero-badge">
            <span class="badge-dot"></span>
            <span class="badge-text">Lightweight MCP Gateway</span>
          </div>

          <!-- Title -->
          <h1 class="hero-title">
            <span class="title-line">MCP security without the</span>
            <span class="title-line gradient-text">infrastructure tax.</span>
          </h1>

          <!-- Subtitle -->
          <p class="hero-subtitle">
            One binary. One config file. Production-ready in 5 minutes.
            <span class="no-wrap">While others ship Kubernetes clusters, you ship features.</span>
          </p>

          <!-- CTAs -->
          <div class="hero-cta">
            <a href="/docs/quickstart" class="btn-primary">
              Get Started Free
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M5 12h14M12 5l7 7-7 7"/>
              </svg>
            </a>
            <a href="https://github.com/mcp-guard/mcp-guard" target="_blank" class="btn-secondary">
              <svg viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
              </svg>
              View on GitHub ⭐
            </a>
          </div>

          <!-- Simple install command -->
          <div class="install-bar">
            <code class="install-code">
              <span class="prompt">$</span>
              <span class="command">curl -fsSL https://mcp.guard/install.sh | sh</span>
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

    /* Security Grid */
    .security-grid {
      position: absolute;
      inset: 0;
      opacity: 0.4;
    }

    .grid-lines {
      position: absolute;
      inset: 0;
      background-image: 
        linear-gradient(rgba(255, 122, 48, 0.03) 1px, transparent 1px),
        linear-gradient(90deg, rgba(255, 122, 48, 0.03) 1px, transparent 1px);
      background-size: 60px 60px;
      mask-image: radial-gradient(ellipse 80% 60% at 50% 40%, black 20%, transparent 70%);
    }

    .scan-line {
      position: absolute;
      left: 0;
      right: 0;
      height: 2px;
      background: linear-gradient(90deg, transparent, rgba(255, 122, 48, 0.4), var(--accent-cyan), rgba(255, 122, 48, 0.4), transparent);
      animation: scan 4s ease-in-out infinite;
      opacity: 0.6;
    }

    @keyframes scan {
      0%, 100% {
        top: 15%;
        opacity: 0;
      }
      10% {
        opacity: 0.6;
      }
      50% {
        top: 55%;
        opacity: 0.6;
      }
      90% {
        opacity: 0.6;
      }
      100% {
        top: 85%;
        opacity: 0;
      }
    }

    /* Shield Pulse Effect */
    .shield-pulse {
      position: absolute;
      top: 35%;
      left: 50%;
      transform: translate(-50%, -50%);
    }

    .pulse-ring {
      position: absolute;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      border-radius: 50%;
      border: 1px solid rgba(255, 122, 48, 0.15);
    }

    .pulse-1 {
      width: 200px;
      height: 200px;
      animation: pulse-expand 4s ease-out infinite;
    }

    .pulse-2 {
      width: 350px;
      height: 350px;
      animation: pulse-expand 4s ease-out infinite 1.3s;
    }

    .pulse-3 {
      width: 500px;
      height: 500px;
      animation: pulse-expand 4s ease-out infinite 2.6s;
    }

    @keyframes pulse-expand {
      0% {
        opacity: 0.5;
        transform: translate(-50%, -50%) scale(0.8);
      }
      100% {
        opacity: 0;
        transform: translate(-50%, -50%) scale(1.3);
      }
    }

    .shield-icon {
      position: relative;
      width: 80px;
      height: 80px;
      color: rgba(255, 122, 48, 0.15);
      animation: shield-glow 3s ease-in-out infinite;
    }

    @keyframes shield-glow {
      0%, 100% {
        opacity: 0.3;
        filter: drop-shadow(0 0 10px rgba(255, 122, 48, 0.2));
      }
      50% {
        opacity: 0.6;
        filter: drop-shadow(0 0 20px rgba(255, 122, 48, 0.4));
      }
    }

    /* Floating Security Nodes */
    .security-nodes {
      position: absolute;
      inset: 0;
    }

    .node {
      position: absolute;
      width: 40px;
      height: 40px;
      background: rgba(10, 12, 16, 0.8);
      border: 1px solid rgba(255, 122, 48, 0.2);
      border-radius: var(--radius-lg);
      display: flex;
      align-items: center;
      justify-content: center;
      color: var(--accent-cyan);
      animation: float-node 6s ease-in-out infinite;

      svg {
        width: 20px;
        height: 20px;
      }
    }

    .node-1 {
      top: 25%;
      left: 12%;
      animation-delay: 0s;
    }

    .node-2 {
      top: 40%;
      right: 15%;
      animation-delay: -2s;
    }

    .node-3 {
      bottom: 30%;
      left: 18%;
      animation-delay: -4s;
    }

    @keyframes float-node {
      0%, 100% {
        transform: translateY(0) rotate(0deg);
        opacity: 0.5;
      }
      50% {
        transform: translateY(-15px) rotate(5deg);
        opacity: 0.8;
      }
    }

    /* Gradient Accent */
    .gradient-accent {
      position: absolute;
      top: 30%;
      left: 50%;
      transform: translateX(-50%);
      width: 600px;
      height: 400px;
      background: radial-gradient(ellipse, rgba(255, 122, 48, 0.06) 0%, transparent 60%);
      filter: blur(40px);
    }

    .hero {
      position: relative;
      min-height: 100vh;
      display: flex;
      align-items: center;
      padding: var(--space-32) 0 var(--space-24);
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
      margin-bottom: var(--space-10);
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
      margin-bottom: var(--space-8);
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
      margin: 0 auto var(--space-12);
    }

    .no-wrap {
      display: block;
      color: var(--text-muted);
      margin-top: var(--space-3);
    }

    /* CTAs */
    .hero-cta {
      display: flex;
      justify-content: center;
      gap: var(--space-4);
      margin-bottom: var(--space-14);

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
      bottom: var(--space-12);
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

    /* Hide background elements on mobile for performance */
    @media (max-width: 768px) {
      .security-nodes {
        display: none;
      }

      .pulse-3 {
        display: none;
      }

      .grid-lines {
        background-size: 40px 40px;
      }
    }

    @media (max-width: 640px) {
      .hero {
        padding: var(--space-28) 0 var(--space-20);
      }

      .hero-badge {
        margin-bottom: var(--space-8);
      }

      .hero-title {
        margin-bottom: var(--space-6);
      }

      .hero-subtitle {
        font-size: var(--text-base);
        margin-bottom: var(--space-10);
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
        bottom: var(--space-8);
      }

      .shield-pulse {
        top: 30%;
      }

      .shield-icon {
        width: 60px;
        height: 60px;
      }

      .pulse-1 {
        width: 120px;
        height: 120px;
      }

      .pulse-2 {
        width: 200px;
        height: 200px;
      }
    }
  `]
})
export class HeroComponent implements OnInit, OnDestroy {
  copied = signal(false);

  ngOnInit() { }

  ngOnDestroy() { }

  copyCommand() {
    navigator.clipboard.writeText('curl -fsSL https://mcp.guard/install.sh | sh');
    this.copied.set(true);
    setTimeout(() => this.copied.set(false), 2000);
  }
}
