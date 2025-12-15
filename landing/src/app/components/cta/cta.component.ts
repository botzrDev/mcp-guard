import { Component, signal, OnInit, OnDestroy, ChangeDetectionStrategy, AfterViewInit, ElementRef, inject, NgZone, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';
import { gsap } from 'gsap';
import { ScrollTrigger } from 'gsap/ScrollTrigger';

gsap.registerPlugin(ScrollTrigger);

@Component({
  selector: 'app-cta',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule],
  template: `
    <section class="cta" #container>
      <!-- Bold diagonal brand stripes -->
      <div class="brand-stripes">
        <div class="stripe stripe-1"></div>
        <div class="stripe stripe-2"></div>
        <div class="stripe stripe-3"></div>
      </div>

      <!-- Massive floating text -->
      <div class="floating-cta-text">START</div>

      <div class="cta-container">
        <!-- Convergent layout -->
        <div class="cta-content" [class.converged]="isVisible()">
          <!-- Left side: Text -->
          <div class="cta-text" #leftContent>
            <div class="section-marker">
              <span class="marker-line"></span>
              <span class="marker-text">// Ready?</span>
            </div>
            
            <h2 class="cta-title">
              <span class="title-line">Secure your</span>
              <span class="title-line title-gradient">MCP servers</span>
              <span class="title-line">in minutes.</span>
            </h2>
            
            <p class="cta-subtitle">
              Join developers who trust mcp-guard to protect their AI infrastructure.
            </p>
            
            <div class="cta-actions">
              <a href="/docs/quickstart" class="btn-primary">
                <span class="btn-text">Read the Docs</span>
                <span class="btn-icon">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M5 12h14M12 5l7 7-7 7"/>
                  </svg>
                </span>
                <span class="btn-glow"></span>
              </a>
              <a href="https://github.com/mcp-guard/mcp-guard" target="_blank" class="btn-secondary">
                <svg viewBox="0 0 24 24" fill="currentColor">
                  <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                </svg>
                Star on GitHub
              </a>
            </div>

            <!-- Social proof -->
            <div class="social-proof">
              <div class="proof-avatars">
                <div class="avatar"></div>
                <div class="avatar"></div>
                <div class="avatar"></div>
                <div class="avatar avatar-more">+50</div>
              </div>
              <span class="proof-text">Developers already using mcp-guard</span>
            </div>
          </div>

          <!-- Right side: Interactive Terminal -->
          <div class="cta-terminal" #rightContent>
            <div class="terminal-wrapper">
              <div class="terminal">
                <div class="terminal-header">
                  <div class="terminal-dots">
                    <span class="dot red"></span>
                    <span class="dot yellow"></span>
                    <span class="dot green"></span>
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

              <!-- Orbiting elements -->
              <div class="orbit-element orbit-1">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
                </svg>
              </div>
              <div class="orbit-element orbit-2">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <polyline points="20 6 9 17 4 12"/>
                </svg>
              </div>
              <div class="orbit-element orbit-3">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
                  <path d="M7 11V7a5 5 0 0 1 10 0v4"/>
                </svg>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Bottom glow -->
      <div class="bottom-glow"></div>
    </section>
  `,
  styles: [`
    .cta {
      position: relative;
      padding: 120px 0;
      background: var(--bg-primary);
      overflow: hidden;
    }

    /* Subtle diagonal stripes */
    .brand-stripes {
      position: absolute;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      pointer-events: none;
      overflow: hidden;
    }

    .stripe {
      position: absolute;
      height: 200%;
      width: 80px;
      transform: rotate(-35deg);
    }

    .stripe-1 {
      left: 5%;
      background: linear-gradient(180deg, transparent, rgba(255, 122, 48, 0.02), transparent);
    }

    .stripe-2 {
      left: 20%;
      background: linear-gradient(180deg, transparent, rgba(70, 92, 136, 0.03), transparent);
    }

    .stripe-3 {
      right: 10%;
      background: linear-gradient(180deg, transparent, rgba(255, 122, 48, 0.02), transparent);
    }

    .floating-cta-text {
      position: absolute;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%) rotate(-10deg);
      font-family: var(--font-display);
      font-size: clamp(150px, 25vw, 400px);
      font-weight: 800;
      color: transparent;
      -webkit-text-stroke: 1px rgba(255, 122, 48, 0.03);
      white-space: nowrap;
      letter-spacing: -0.05em;
      pointer-events: none;
      user-select: none;
    }

    .cta-container {
      position: relative;
      max-width: 1400px;
      margin: 0 auto;
      padding: 0 24px;
      z-index: 1;
    }

    .cta-content {
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 80px;
      align-items: center;

      @media (max-width: 1024px) {
        grid-template-columns: 1fr;
        gap: 60px;
      }
    }

    /* Convergent animation - starts spread apart */
    .cta-text {
      transform: translateX(-40px);
      opacity: 0;
      transition: all 0.8s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .cta-terminal {
      transform: translateX(40px);
      opacity: 0;
      transition: all 0.8s cubic-bezier(0.4, 0, 0.2, 1);
      transition-delay: 0.1s;
    }

    .cta-content.converged {
      .cta-text {
        transform: translateX(0);
        opacity: 1;
      }

      .cta-terminal {
        transform: translateX(0);
        opacity: 1;
      }
    }

    .section-marker {
      display: flex;
      align-items: center;
      gap: 16px;
      margin-bottom: 24px;
    }

    .marker-line {
      width: 40px;
      height: 3px;
      background: var(--gradient-brand);
      border-radius: 2px;
    }

    .marker-text {
      font-family: var(--font-mono);
      font-size: 13px;
      color: #FF7A30;
      letter-spacing: 0.05em;
    }

    .cta-title {
      margin-bottom: 24px;
    }

    .title-line {
      display: block;
      font-family: var(--font-display);
      font-size: clamp(36px, 6vw, 56px);
      font-weight: 700;
      letter-spacing: -0.03em;
      line-height: 1.1;
    }

    .title-gradient {
      background: var(--gradient-brand);
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
      background-clip: text;
    }

    .cta-subtitle {
      font-size: 18px;
      color: var(--text-secondary);
      margin-bottom: 32px;
      line-height: 1.6;
      max-width: 500px;
    }

    .cta-actions {
      display: flex;
      gap: 16px;
      flex-wrap: wrap;
      margin-bottom: 40px;
    }

    .btn-primary {
      position: relative;
      display: inline-flex;
      align-items: center;
      gap: 12px;
      padding: 16px 32px;
      background: var(--gradient-brand);
      color: var(--bg-primary);
      text-decoration: none;
      font-size: 16px;
      font-weight: 600;
      border-radius: 12px;
      overflow: hidden;
      transition: all 0.3s;

      .btn-icon svg {
        width: 20px;
        height: 20px;
      }

      .btn-glow {
        position: absolute;
        inset: -2px;
        background: var(--gradient-brand);
        filter: blur(20px);
        opacity: 0;
        transition: opacity 0.3s;
        z-index: -1;
      }

      &:hover {
        transform: translateY(-3px);

        .btn-glow {
          opacity: 0.5;
        }
      }
    }

    .btn-secondary {
      display: inline-flex;
      align-items: center;
      gap: 10px;
      padding: 16px 28px;
      background: transparent;
      color: var(--text-primary);
      text-decoration: none;
      font-size: 16px;
      font-weight: 500;
      border-radius: 12px;
      border: 1px solid var(--border-medium);
      transition: all 0.3s;

      svg {
        width: 22px;
        height: 22px;
      }

      &:hover {
        background: var(--bg-elevated);
        border-color: var(--border-accent);
        transform: translateY(-2px);
      }
    }

    .social-proof {
      display: flex;
      align-items: center;
      gap: 16px;
    }

    .proof-avatars {
      display: flex;
    }

    .avatar {
      width: 36px;
      height: 36px;
      background: var(--bg-elevated);
      border: 2px solid var(--bg-primary);
      border-radius: 50%;
      margin-left: -8px;

      &:first-child {
        margin-left: 0;
      }

      &.avatar-more {
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 11px;
        font-weight: 600;
        color: var(--text-muted);
        background: var(--bg-secondary);
      }
    }

    .proof-text {
      font-size: 14px;
      color: var(--text-muted);
    }

    /* Terminal */
    .cta-terminal {
      position: relative;

      @media (max-width: 1024px) {
        display: flex;
        justify-content: center;
      }
    }

    .terminal-wrapper {
      position: relative;
    }

    .terminal {
      width: 100%;
      max-width: 520px;
      background: var(--bg-secondary);
      border: 1px solid var(--border-subtle);
      border-radius: 20px;
      overflow: hidden;
      box-shadow:
        0 0 0 1px rgba(255, 255, 255, 0.05),
        0 40px 80px -20px rgba(0, 0, 0, 0.5),
        0 0 100px -30px rgba(255, 122, 48, 0.2);
    }

    .terminal-header {
      display: flex;
      align-items: center;
      padding: 16px 20px;
      background: var(--bg-elevated);
      border-bottom: 1px solid var(--border-subtle);
    }

    .terminal-dots {
      display: flex;
      gap: 8px;

      .dot {
        width: 12px;
        height: 12px;
        border-radius: 50%;

        &.red { background: #ff5f57; }
        &.yellow { background: #febc2e; }
        &.green { background: #28c840; }
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
      padding: 24px;
      font-family: var(--font-mono);
      font-size: 14px;
      line-height: 1.9;
      min-height: 220px;
    }

    .terminal-line {
      display: flex;
      align-items: center;
      gap: 10px;
      opacity: 0;
      transform: translateY(10px);
      transition: all 0.3s ease;
      padding: 6px 0;
      border-radius: 8px;
      margin: 0 -10px;
      padding-left: 10px;
      padding-right: 10px;

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
      color: #FF7A30;
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
        color: #FF7A30;
      }
    }

    .copy-icon {
      display: flex;
      align-items: center;
      justify-content: center;
      width: 30px;
      height: 30px;
      background: var(--bg-card);
      border: 1px solid var(--border-subtle);
      border-radius: 8px;
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
      width: 10px;
      height: 20px;
      background: #FF7A30;
      margin-left: 4px;
      opacity: 0;

      &.visible {
        animation: blink 1s step-end infinite;
      }
    }

    /* Orbiting elements */
    .orbit-element {
      position: absolute;
      width: 52px;
      height: 52px;
      background: var(--bg-card);
      border: 1px solid var(--border-subtle);
      border-radius: 14px;
      display: flex;
      align-items: center;
      justify-content: center;
      color: #FF7A30;
      box-shadow: 0 10px 40px -10px rgba(0, 0, 0, 0.3);

      svg {
        width: 24px;
        height: 24px;
      }

      @media (max-width: 1024px) {
        display: none;
      }
    }

    .orbit-1 {
      top: -20px;
      right: -20px;
      animation: orbit 8s linear infinite;
      animation-delay: 0s;
    }

    .orbit-2 {
      bottom: 60px;
      left: -30px;
      animation: orbit 8s linear infinite;
      animation-delay: -2.5s;
    }

    .orbit-3 {
      bottom: -10px;
      right: 40px;
      animation: orbit 8s linear infinite;
      animation-delay: -5s;
    }

    @keyframes orbit {
      0%, 100% { transform: translateY(0) rotate(0deg); }
      25% { transform: translateY(-10px) rotate(5deg); }
      50% { transform: translateY(0) rotate(0deg); }
      75% { transform: translateY(10px) rotate(-5deg); }
    }

    @keyframes blink {
      0%, 50% { opacity: 1; }
      51%, 100% { opacity: 0; }
    }

    .bottom-glow {
      position: absolute;
      bottom: -200px;
      left: 50%;
      transform: translateX(-50%);
      width: 1000px;
      height: 500px;
      background: radial-gradient(ellipse, rgba(255, 122, 48, 0.08) 0%, transparent 70%);
      pointer-events: none;
    }

    @media (max-width: 768px) {
      .cta {
        padding: 100px 0;
      }

      .cta-content {
        gap: 40px;
      }

      .cta-actions {
        flex-direction: column;

        .btn-primary,
        .btn-secondary {
          width: 100%;
          justify-content: center;
        }
      }

      .social-proof {
        flex-direction: column;
        align-items: flex-start;
        gap: 12px;
      }
    }
  `]
})
export class CtaComponent implements OnInit, OnDestroy, AfterViewInit {
  @ViewChild('container') containerRef!: ElementRef<HTMLElement>;
  @ViewChild('leftContent') leftContentRef!: ElementRef<HTMLElement>;
  @ViewChild('rightContent') rightContentRef!: ElementRef<HTMLElement>;

  private ngZone = inject(NgZone);
  private scrollTrigger: ScrollTrigger | null = null;

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
  isVisible = signal(false);

  private animationInterval: any;

  ngOnInit() { }

  ngAfterViewInit() {
    this.initScrollTrigger();
  }

  ngOnDestroy() {
    this.scrollTrigger?.kill();
    if (this.animationInterval) {
      clearInterval(this.animationInterval);
    }
  }

  private initScrollTrigger() {
    this.ngZone.runOutsideAngular(() => {
      this.scrollTrigger = ScrollTrigger.create({
        trigger: this.containerRef.nativeElement,
        start: 'top 70%',
        onEnter: () => {
          this.ngZone.run(() => {
            this.isVisible.set(true);
            // Start terminal animation after convergence
            setTimeout(() => this.animateTerminal(), 400);
          });
        },
        onLeaveBack: () => {
          this.ngZone.run(() => {
            this.isVisible.set(false);
            this.currentLine.set(-1);
          });
        }
      });
    });
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

  copyLine(text: string) {
    navigator.clipboard.writeText(text);
    this.copiedLine.set(text);
    setTimeout(() => this.copiedLine.set(null), 2000);
  }
}
