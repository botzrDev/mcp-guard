import { Component, signal, OnInit, OnDestroy, ChangeDetectionStrategy } from '@angular/core';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-hero',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule],
  template: `
    <!-- Animated background -->
    <div class="hero-bg">
      <div class="radar-container">
        <div class="radar-sweep"></div>
        <div class="radar-ring ring-1"></div>
        <div class="radar-ring ring-2"></div>
        <div class="radar-ring ring-3"></div>
      </div>
      <div class="grid-overlay"></div>
      <div class="glow-orb orb-1"></div>
      <div class="glow-orb orb-2"></div>
    </div>

    <section class="hero">
      <div class="hero-container">
        <!-- Asymmetric split layout -->
        <div class="hero-content">
          <!-- Left side - Text content -->
          <div class="hero-left">
            <!-- Annotation-style badge -->
            <div class="hero-badge">
              <span class="badge-line"></span>
              <div class="badge-content">
                <span class="badge-dot"></span>
                <span>Now with OAuth 2.1 + PKCE</span>
              </div>
            </div>

            <h1 class="hero-title">
              <span class="title-line line-1">Secure your</span>
              <span class="title-line line-2">MCP servers</span>
              <span class="title-line line-3 gradient-text">in 5 minutes</span>
            </h1>

            <p class="hero-subtitle">
              No Docker. No Kubernetes. No DevOps team.<br>
              <span class="subtitle-highlight">A single Rust binary</span> that adds OAuth, JWT,
              rate limiting, and audit logs to any MCP server.
            </p>

            <div class="hero-cta">
              <a href="/docs/quickstart" class="btn-glow">
                Get Started
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M5 12h14M12 5l7 7-7 7"/>
                </svg>
              </a>
              <a href="https://github.com/mcp-guard/mcp-guard" target="_blank" class="btn-outline">
                <svg viewBox="0 0 24 24" fill="currentColor">
                  <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                </svg>
                View on GitHub
              </a>
            </div>
          </div>

          <!-- Right side - Terminal (tilted) -->
          <div class="hero-right">
            <div class="terminal-wrapper">
              <div class="terminal">
                <div class="terminal-header">
                  <div class="terminal-dots">
                    <span class="dot red"></span>
                    <span class="dot yellow"></span>
                    <span class="dot green"></span>
                  </div>
                  <span class="terminal-title">mcp-guard</span>
                </div>
                <div class="terminal-body">
                  @for (line of terminalLines(); track $index) {
                    <div class="terminal-line" [class.visible]="$index <= currentLine()">
                      @if (line.type === 'command') {
                        <span class="prompt">$</span>
                        <span class="command">{{ line.text }}</span>
                      }
                      @if (line.type === 'output') {
                        <span class="output" [class]="line.class || ''">{{ line.text }}</span>
                      }
                    </div>
                  }
                  <div class="cursor" [class.visible]="showCursor()"></div>
                </div>
              </div>

              <!-- Floating decorations -->
              <div class="floating-badge badge-rust">
                <svg viewBox="0 0 24 24" fill="currentColor">
                  <path d="M23.836 11.562l-.727-.463-.036-.034a3.723 3.723 0 00-.091-1.013l.605-.584-.006-.073a9.453 9.453 0 00-.352-1.675l-.019-.062-.789-.286a3.668 3.668 0 00-.448-.911l.344-.723-.04-.062a9.439 9.439 0 00-.975-1.31l-.05-.056-.82.12a3.67 3.67 0 00-.764-.66l.061-.803-.053-.05a9.43 9.43 0 00-1.429-.825l-.06-.029-.687.43a3.694 3.694 0 00-.929-.293l-.224-.787-.064-.025a9.413 9.413 0 00-1.642-.353l-.069-.008-.447.679a3.807 3.807 0 00-.967.017L14.03.155 13.964.15a9.418 9.418 0 00-1.675.352l-.062.02-.286.788a3.678 3.678 0 00-.911.448l-.723-.343-.062.04a9.463 9.463 0 00-1.31.975l-.057.05.121.82a3.657 3.657 0 00-.66.764l-.804-.061-.05.053a9.403 9.403 0 00-.824 1.429l-.03.06.43.687a3.69 3.69 0 00-.293.929l-.787.223-.025.065a9.42 9.42 0 00-.353 1.642l-.008.069.679.447a3.807 3.807 0 00.017.967l-.76.214-.006.073a9.446 9.446 0 00.352 1.675l.02.062.788.286c.107.32.258.62.448.91l-.344.724.04.062c.266.467.592.905.975 1.31l.05.056.82-.12c.21.257.465.48.764.66l-.06.803.053.05c.441.328.921.603 1.428.824l.06.029.688-.43c.298.137.614.232.929.293l.223.787.064.025c.546.133 1.1.24 1.642.353l.069.008.447-.679c.323.01.646-.008.967-.017l.214.761.067.004a9.452 9.452 0 001.675-.352l.062-.02.286-.788c.32-.107.62-.258.911-.448l.723.343.062-.04a9.456 9.456 0 001.31-.975l.056-.05-.12-.82c.256-.21.48-.464.66-.764l.804.061.05-.053c.328-.441.602-.921.824-1.428l.029-.06-.43-.688c.137-.298.233-.614.293-.929l.787-.223.025-.065a9.41 9.41 0 00.353-1.642l.008-.069-.679-.447a3.807 3.807 0 00-.017-.967l.76-.214zm-6.805 7.9a.618.618 0 01-.592-.043l-1.267-.854a.306.306 0 00-.395.055l-.326.373a.306.306 0 00.02.419l1.044 1.075a.617.617 0 01.143.576c-.43 1.396-1.193 2.46-2.225 3.197a.617.617 0 01-.59.054l-1.367-.552a.306.306 0 00-.382.132l-.216.423a.306.306 0 00.11.398l1.293.815a.617.617 0 01.27.54c-.012.264-.032.52-.068.757a.617.617 0 01-.445.5l-1.463.332a.306.306 0 00-.237.333l.045.476a.306.306 0 00.309.271l1.498-.069a.617.617 0 01.531.317c.117.22.248.428.39.622a.617.617 0 01-.02.74l-.926 1.147a.306.306 0 00.035.41l.349.31a.306.306 0 00.413-.025l1.065-1.054a.617.617 0 01.598-.153c.232.063.472.111.718.145a.617.617 0 01.473.403l.497 1.416a.306.306 0 00.362.19l.461-.113a.306.306 0 00.223-.337l-.202-1.49a.617.617 0 01.296-.586c.2-.117.388-.248.563-.391a.617.617 0 01.74-.016l1.228.813a.306.306 0 00.414-.077l.273-.379a.306.306 0 00-.054-.408l-1.158-.958a.617.617 0 01-.195-.663 5.56 5.56 0 00.15-.715.617.617 0 01.407-.49l1.398-.482a.306.306 0 00.183-.373l-.152-.45a.306.306 0 00-.353-.2l-1.475.272a.617.617 0 01-.622-.263z"/>
                </svg>
                <span>Rust</span>
              </div>

              <div class="floating-badge badge-perf">
                <span class="perf-value">&lt;2ms</span>
                <span class="perf-label">p99</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Install command bar - spans full width -->
        <div class="install-bar">
          <div class="install-content">
            <code>
              <span class="prompt">$</span>
              <span class="command">cargo install mcp-guard</span>
            </code>
            <button class="copy-btn" (click)="copyCommand()" [class.copied]="copied()">
              @if (copied()) {
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="20 6 9 17 4 12"/>
                </svg>
              } @else {
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
                  <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/>
                </svg>
              }
            </button>
          </div>
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

    .radar-container {
      position: absolute;
      top: 50%;
      right: -20%;
      width: 800px;
      height: 800px;
      transform: translateY(-50%);
      opacity: 0.4;
    }

    .radar-sweep {
      position: absolute;
      inset: 0;
      background: conic-gradient(from 0deg, transparent 0deg, rgba(78, 205, 196, 0.3) 30deg, transparent 60deg);
      border-radius: 50%;
      animation: radar-sweep 4s linear infinite;
    }

    @keyframes radar-sweep {
      from { transform: rotate(0deg); }
      to { transform: rotate(360deg); }
    }

    .radar-ring {
      position: absolute;
      border: 1px solid rgba(78, 205, 196, 0.15);
      border-radius: 50%;
    }

    .ring-1 {
      inset: 20%;
    }

    .ring-2 {
      inset: 35%;
    }

    .ring-3 {
      inset: 50%;
    }

    .grid-overlay {
      position: absolute;
      inset: 0;
      background-image:
        linear-gradient(rgba(78, 205, 196, 0.03) 1px, transparent 1px),
        linear-gradient(90deg, rgba(78, 205, 196, 0.03) 1px, transparent 1px);
      background-size: 50px 50px;
    }

    .glow-orb {
      position: absolute;
      border-radius: 50%;
      filter: blur(100px);
    }

    .orb-1 {
      width: 600px;
      height: 600px;
      background: rgba(78, 205, 196, 0.15);
      top: -200px;
      left: -200px;
    }

    .orb-2 {
      width: 400px;
      height: 400px;
      background: rgba(139, 92, 246, 0.1);
      bottom: -100px;
      right: 10%;
    }

    .hero {
      position: relative;
      min-height: 100vh;
      display: flex;
      align-items: center;
      padding: 140px 0 80px;
    }

    .hero-container {
      max-width: 1280px;
      margin: 0 auto;
      padding: 0 24px;
      width: 100%;
    }

    .hero-content {
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 80px;
      align-items: center;

      @media (max-width: 1024px) {
        grid-template-columns: 1fr;
        gap: 60px;
      }
    }

    .hero-left {
      animation: slideInLeft 0.8s ease-out;
    }

    .hero-badge {
      display: flex;
      align-items: center;
      gap: 16px;
      margin-bottom: 32px;
    }

    .badge-line {
      width: 40px;
      height: 2px;
      background: var(--gradient-brand);
    }

    .badge-content {
      display: flex;
      align-items: center;
      gap: 8px;
      padding: 8px 16px;
      background: var(--bg-elevated);
      border: 1px solid var(--border-subtle);
      border-radius: 100px;
      font-size: 13px;
      color: var(--text-secondary);
    }

    .badge-dot {
      width: 8px;
      height: 8px;
      background: var(--accent-cyan);
      border-radius: 50%;
      animation: pulse 2s infinite;
    }

    .hero-title {
      font-size: clamp(48px, 7vw, 72px);
      font-weight: 700;
      line-height: 1.05;
      letter-spacing: -0.03em;
      margin-bottom: 24px;
    }

    .title-line {
      display: block;
    }

    .gradient-text {
      background: var(--gradient-brand);
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
      background-clip: text;
    }

    .hero-subtitle {
      font-size: 18px;
      color: var(--text-secondary);
      line-height: 1.7;
      margin-bottom: 40px;
      max-width: 520px;
    }

    .subtitle-highlight {
      color: var(--accent-cyan);
      font-weight: 500;
    }

    .hero-cta {
      display: flex;
      gap: 16px;
      flex-wrap: wrap;
    }

    .btn-glow {
      display: inline-flex;
      align-items: center;
      gap: 10px;
      padding: 14px 28px;
      background: var(--gradient-brand);
      color: var(--bg-primary);
      text-decoration: none;
      font-size: 15px;
      font-weight: 600;
      border-radius: 10px;
      box-shadow: 0 0 30px rgba(78, 205, 196, 0.4);
      transition: all 0.3s;

      svg {
        width: 18px;
        height: 18px;
      }

      &:hover {
        transform: translateY(-2px);
        box-shadow: 0 0 40px rgba(78, 205, 196, 0.6);
      }
    }

    .btn-outline {
      display: inline-flex;
      align-items: center;
      gap: 10px;
      padding: 14px 28px;
      background: transparent;
      color: var(--text-primary);
      text-decoration: none;
      font-size: 15px;
      font-weight: 500;
      border-radius: 10px;
      border: 1px solid var(--border-medium);
      transition: all 0.3s;

      svg {
        width: 20px;
        height: 20px;
      }

      &:hover {
        background: var(--bg-elevated);
        border-color: var(--border-accent);
      }
    }

    .hero-right {
      animation: slideInRight 0.8s ease-out 0.2s both;

      @media (max-width: 1024px) {
        display: flex;
        justify-content: center;
      }
    }

    .terminal-wrapper {
      position: relative;
      transform: perspective(1000px) rotateY(-5deg) rotateX(2deg);
      transition: transform 0.5s ease;

      &:hover {
        transform: perspective(1000px) rotateY(-2deg) rotateX(1deg);
      }

      @media (max-width: 1024px) {
        transform: none;

        &:hover {
          transform: none;
        }
      }
    }

    .terminal {
      width: 480px;
      max-width: 100%;
      background: var(--bg-secondary);
      border: 1px solid var(--border-subtle);
      border-radius: 16px;
      overflow: hidden;
      box-shadow:
        0 0 0 1px rgba(255, 255, 255, 0.05),
        0 25px 50px -12px rgba(0, 0, 0, 0.5),
        0 0 80px -20px rgba(78, 205, 196, 0.3);
    }

    .terminal-header {
      display: flex;
      align-items: center;
      padding: 14px 20px;
      background: var(--bg-elevated);
      border-bottom: 1px solid var(--border-subtle);
    }

    .terminal-dots {
      display: flex;
      gap: 8px;
    }

    .dot {
      width: 12px;
      height: 12px;
      border-radius: 50%;

      &.red { background: #ff5f57; }
      &.yellow { background: #febc2e; }
      &.green { background: #28c840; }
    }

    .terminal-title {
      flex: 1;
      text-align: center;
      font-family: var(--font-mono);
      font-size: 13px;
      color: var(--text-muted);
    }

    .terminal-body {
      padding: 20px;
      font-family: var(--font-mono);
      font-size: 13px;
      line-height: 1.8;
      min-height: 280px;
    }

    .terminal-line {
      display: flex;
      gap: 8px;
      opacity: 0;
      transform: translateY(10px);
      transition: all 0.3s ease;

      &.visible {
        opacity: 1;
        transform: translateY(0);
      }
    }

    .prompt {
      color: var(--accent-cyan);
      user-select: none;
    }

    .command {
      color: var(--text-primary);
    }

    .output {
      color: var(--text-muted);

      &.success {
        color: #28c840;
      }

      &.highlight {
        color: var(--accent-cyan);
      }
    }

    .cursor {
      display: inline-block;
      width: 8px;
      height: 18px;
      background: var(--accent-cyan);
      margin-left: 4px;
      opacity: 0;

      &.visible {
        animation: blink 1s step-end infinite;
      }
    }

    .floating-badge {
      position: absolute;
      display: flex;
      align-items: center;
      gap: 8px;
      padding: 10px 16px;
      background: var(--bg-card);
      border: 1px solid var(--border-subtle);
      border-radius: 10px;
      font-size: 13px;
      font-weight: 500;
      box-shadow: 0 10px 30px -10px rgba(0, 0, 0, 0.3);

      svg {
        width: 18px;
        height: 18px;
      }
    }

    .badge-rust {
      top: -20px;
      right: -30px;
      color: #dea584;
      animation: float 4s ease-in-out infinite;

      @media (max-width: 1024px) {
        right: 0;
      }
    }

    .badge-perf {
      bottom: 40px;
      left: -40px;
      flex-direction: column;
      gap: 2px;
      animation: float 4s ease-in-out infinite 1s;

      @media (max-width: 1024px) {
        left: 0;
      }
    }

    .perf-value {
      font-family: var(--font-mono);
      font-size: 18px;
      font-weight: 700;
      color: var(--accent-cyan);
    }

    .perf-label {
      font-size: 11px;
      color: var(--text-muted);
      text-transform: uppercase;
      letter-spacing: 0.05em;
    }

    .install-bar {
      margin-top: 80px;
      display: flex;
      justify-content: center;
      animation: fadeInUp 0.8s ease-out 0.4s both;
    }

    .install-content {
      display: flex;
      align-items: center;
      gap: 20px;
      padding: 16px 24px;
      background: var(--bg-secondary);
      border: 1px solid var(--border-subtle);
      border-radius: 14px;
      position: relative;
      overflow: hidden;

      &::before {
        content: '';
        position: absolute;
        inset: 0;
        background: linear-gradient(90deg, transparent, rgba(78, 205, 196, 0.05), transparent);
        animation: shimmer 3s infinite;
      }

      code {
        font-family: var(--font-mono);
        font-size: 15px;
        display: flex;
        gap: 10px;
        position: relative;
        z-index: 1;

        .prompt {
          color: var(--accent-cyan);
        }

        .command {
          color: var(--text-primary);
        }
      }
    }

    .copy-btn {
      display: flex;
      align-items: center;
      justify-content: center;
      width: 40px;
      height: 40px;
      background: var(--bg-elevated);
      border: 1px solid var(--border-subtle);
      border-radius: 8px;
      cursor: pointer;
      color: var(--text-muted);
      transition: all 0.2s;
      position: relative;
      z-index: 1;

      svg {
        width: 18px;
        height: 18px;
      }

      &:hover {
        color: var(--text-primary);
        background: var(--bg-hover);
        border-color: var(--border-accent);
      }

      &.copied {
        color: #28c840;
        border-color: rgba(40, 200, 64, 0.3);
      }
    }

    @keyframes pulse {
      0%, 100% { opacity: 1; transform: scale(1); }
      50% { opacity: 0.6; transform: scale(0.95); }
    }

    @keyframes float {
      0%, 100% { transform: translateY(0); }
      50% { transform: translateY(-8px); }
    }

    @keyframes blink {
      0%, 50% { opacity: 1; }
      51%, 100% { opacity: 0; }
    }

    @keyframes fadeInUp {
      from {
        opacity: 0;
        transform: translateY(30px);
      }
      to {
        opacity: 1;
        transform: translateY(0);
      }
    }

    @keyframes slideInLeft {
      from {
        opacity: 0;
        transform: translateX(-50px);
      }
      to {
        opacity: 1;
        transform: translateX(0);
      }
    }

    @keyframes slideInRight {
      from {
        opacity: 0;
        transform: translateX(50px);
      }
      to {
        opacity: 1;
        transform: translateX(0);
      }
    }

    @keyframes shimmer {
      0% { transform: translateX(-100%); }
      100% { transform: translateX(100%); }
    }
  `]
})
export class HeroComponent implements OnInit, OnDestroy {
  terminalLines = signal([
    { type: 'command', text: 'mcp-guard init' },
    { type: 'output', text: '✓ Created mcp-guard.toml', class: 'success' },
    { type: 'command', text: 'mcp-guard run --upstream localhost:8080' },
    { type: 'output', text: '✓ Auth: API Key + JWT + OAuth 2.1', class: 'success' },
    { type: 'output', text: '✓ Rate limit: 1000 req/min', class: 'success' },
    { type: 'output', text: '✓ Audit logging: enabled', class: 'success' },
    { type: 'output', text: '→ Proxy listening on 0.0.0.0:3000', class: 'highlight' },
  ]);

  currentLine = signal(-1);
  showCursor = signal(true);
  copied = signal(false);

  private animationInterval: any;

  ngOnInit() {
    this.animateTerminal();
  }

  ngOnDestroy() {
    if (this.animationInterval) {
      clearInterval(this.animationInterval);
    }
  }

  animateTerminal() {
    let line = 0;
    this.animationInterval = setInterval(() => {
      if (line <= this.terminalLines().length) {
        this.currentLine.set(line);
        line++;
      } else {
        clearInterval(this.animationInterval);
      }
    }, 400);
  }

  copyCommand() {
    navigator.clipboard.writeText('cargo install mcp-guard');
    this.copied.set(true);
    setTimeout(() => this.copied.set(false), 2000);
  }
}
