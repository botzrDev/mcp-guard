import { Component, signal, OnInit, OnDestroy, ChangeDetectionStrategy } from '@angular/core';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-cta',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule],
  template: `
    <section class="cta">
      <div class="cta-container">
        <div class="cta-content">
          <!-- Left side: Text -->
          <div class="cta-text">
            <h2>Ready to secure your <span class="gradient-text">MCP servers</span>?</h2>
            <p>Join developers who trust mcp-guard to protect their AI infrastructure.</p>
            <div class="cta-actions">
              <a href="/docs/quickstart" class="btn-glow">
                Read the Docs
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M5 12h14M12 5l7 7-7 7"/>
                </svg>
              </a>
              <a href="https://github.com/mcp-guard/mcp-guard" target="_blank" class="btn-outline">
                <svg viewBox="0 0 24 24" fill="currentColor">
                  <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                </svg>
                Star on GitHub
              </a>
            </div>
          </div>

          <!-- Right side: Interactive Terminal -->
          <div class="cta-terminal">
            <div class="terminal">
              <div class="terminal-header">
                <div class="terminal-dots">
                  <span class="dot"></span>
                  <span class="dot"></span>
                  <span class="dot"></span>
                </div>
                <span class="terminal-title">Quick Start</span>
              </div>
              <div class="terminal-body">
                @for (line of terminalLines(); track $index) {
                  <div
                    class="terminal-line"
                    [class.visible]="$index <= currentLine()"
                    [class.clickable]="line.copyable"
                    (click)="line.copyable && copyLine(line.text)"
                  >
                    @if (line.type === 'command') {
                      <span class="prompt">$</span>
                      <span class="command">{{ line.text }}</span>
                      @if (line.copyable) {
                        <button class="copy-icon" [class.copied]="copiedLine() === line.text">
                          @if (copiedLine() === line.text) {
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
                      }
                    }
                    @if (line.type === 'output') {
                      <span class="output" [class]="line.class || ''">{{ line.text }}</span>
                    }
                    @if (line.type === 'empty') {
                      <span>&nbsp;</span>
                    }
                  </div>
                }
                <div class="cursor" [class.visible]="showCursor()"></div>
              </div>
            </div>

            <!-- Floating elements -->
            <div class="floating-element element-1">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
              </svg>
            </div>
            <div class="floating-element element-2">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="20 6 9 17 4 12"/>
              </svg>
            </div>
          </div>
        </div>

        <!-- Bottom wave decoration -->
        <div class="cta-glow"></div>
      </div>
    </section>
  `,
  styles: [`
    .cta {
      position: relative;
      padding: 120px 0;
      background: var(--bg-secondary);
      overflow: hidden;

      &::before {
        content: '';
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        height: 1px;
        background: linear-gradient(90deg, transparent, var(--border-subtle), transparent);
      }
    }

    .cta-container {
      max-width: 1200px;
      margin: 0 auto;
      padding: 0 24px;
      position: relative;
    }

    .cta-content {
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 64px;
      align-items: center;

      @media (max-width: 900px) {
        grid-template-columns: 1fr;
        gap: 48px;
      }
    }

    .cta-text {
      h2 {
        font-size: clamp(32px, 4vw, 44px);
        font-weight: 700;
        letter-spacing: -0.02em;
        line-height: 1.2;
        margin-bottom: 20px;
      }

      p {
        font-size: 18px;
        color: var(--text-secondary);
        margin-bottom: 32px;
        line-height: 1.6;
      }
    }

    .gradient-text {
      background: var(--gradient-brand);
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
      background-clip: text;
    }

    .cta-actions {
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

    .cta-terminal {
      position: relative;

      @media (max-width: 900px) {
        display: flex;
        justify-content: center;
      }
    }

    .terminal {
      width: 100%;
      max-width: 500px;
      background: var(--bg-primary);
      border: 1px solid var(--border-subtle);
      border-radius: 16px;
      overflow: hidden;
      box-shadow:
        0 0 0 1px rgba(255, 255, 255, 0.05),
        0 25px 50px -12px rgba(0, 0, 0, 0.5),
        0 0 80px -20px rgba(78, 205, 196, 0.2);
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

      .dot {
        width: 12px;
        height: 12px;
        background: var(--bg-card);
        border-radius: 50%;
      }
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
      min-height: 200px;
    }

    .terminal-line {
      display: flex;
      align-items: center;
      gap: 8px;
      opacity: 0;
      transform: translateY(10px);
      transition: all 0.3s ease;
      padding: 4px 0;
      border-radius: 6px;
      margin: 0 -8px;
      padding-left: 8px;
      padding-right: 8px;

      &.visible {
        opacity: 1;
        transform: translateY(0);
      }

      &.clickable {
        cursor: pointer;

        &:hover {
          background: var(--bg-elevated);

          .copy-icon {
            opacity: 1;
          }
        }
      }
    }

    .prompt {
      color: var(--accent-cyan);
      user-select: none;
    }

    .command {
      color: var(--text-primary);
      flex: 1;
    }

    .output {
      color: var(--text-muted);

      &.success {
        color: #4ade80;
      }

      &.highlight {
        color: var(--accent-cyan);
      }
    }

    .copy-icon {
      display: flex;
      align-items: center;
      justify-content: center;
      width: 28px;
      height: 28px;
      background: var(--bg-card);
      border: 1px solid var(--border-subtle);
      border-radius: 6px;
      cursor: pointer;
      opacity: 0;
      transition: all 0.2s;
      color: var(--text-muted);

      svg {
        width: 14px;
        height: 14px;
      }

      &:hover {
        color: var(--text-primary);
        border-color: var(--border-accent);
      }

      &.copied {
        opacity: 1;
        color: #4ade80;
        border-color: rgba(74, 222, 128, 0.3);
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

    .floating-element {
      position: absolute;
      width: 48px;
      height: 48px;
      background: var(--bg-card);
      border: 1px solid var(--border-subtle);
      border-radius: 12px;
      display: flex;
      align-items: center;
      justify-content: center;
      color: var(--accent-cyan);
      box-shadow: 0 10px 30px -10px rgba(0, 0, 0, 0.3);
      animation: float 4s ease-in-out infinite;

      svg {
        width: 24px;
        height: 24px;
      }

      @media (max-width: 900px) {
        display: none;
      }
    }

    .element-1 {
      top: -20px;
      right: -20px;
      animation-delay: 0s;
    }

    .element-2 {
      bottom: 20px;
      left: -30px;
      animation-delay: 1s;
    }

    .cta-glow {
      position: absolute;
      bottom: -200px;
      left: 50%;
      transform: translateX(-50%);
      width: 800px;
      height: 400px;
      background: radial-gradient(ellipse, rgba(78, 205, 196, 0.1) 0%, transparent 70%);
      pointer-events: none;
    }

    @keyframes float {
      0%, 100% { transform: translateY(0); }
      50% { transform: translateY(-10px); }
    }

    @keyframes blink {
      0%, 50% { opacity: 1; }
      51%, 100% { opacity: 0; }
    }
  `]
})
export class CtaComponent implements OnInit, OnDestroy {
  terminalLines = signal([
    { type: 'command', text: 'cargo install mcp-guard', copyable: true },
    { type: 'empty', text: '' },
    { type: 'command', text: 'mcp-guard init', copyable: true },
    { type: 'output', text: '✓ Created mcp-guard.toml', class: 'success' },
    { type: 'empty', text: '' },
    { type: 'command', text: 'mcp-guard run', copyable: true },
    { type: 'output', text: '→ Proxy listening on 0.0.0.0:3000', class: 'highlight' },
  ]);

  currentLine = signal(-1);
  showCursor = signal(true);
  copiedLine = signal<string | null>(null);

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
    }, 500);
  }

  copyLine(text: string) {
    navigator.clipboard.writeText(text);
    this.copiedLine.set(text);
    setTimeout(() => this.copiedLine.set(null), 2000);
  }
}
