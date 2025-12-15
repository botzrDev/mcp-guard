import { Component, signal, ChangeDetectionStrategy, OnInit, OnDestroy, ElementRef, inject, NgZone, AfterViewInit, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';
import { gsap } from 'gsap';
import { ScrollTrigger } from 'gsap/ScrollTrigger';

gsap.registerPlugin(ScrollTrigger);

@Component({
  selector: 'app-comparison',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule],
  template: `
    <section class="comparison" #container>
      <!-- Bold diagonal brand element -->
      <div class="diagonal-brand">
        <span>BEFORE</span>
        <span class="brand-divider">/</span>
        <span class="after-text">AFTER</span>
      </div>

      <!-- Floating brand marks -->
      <div class="brand-float brand-float-1">mcp</div>
      <div class="brand-float brand-float-2">guard</div>

      <div class="comparison-container">
        <!-- Unconventional header - off-grid -->
        <div class="section-header">
          <span class="section-number">03</span>
          <div class="header-text">
            <span class="section-tag">// The Transformation</span>
            <h2 class="section-title">
              <span class="title-line">Your server is</span>
              <span class="state-text" [class.secured]="isSecured()">
                {{ isSecured() ? 'protected' : 'exposed' }}
              </span>
            </h2>
          </div>
        </div>

        <!-- DRAMATIC SPLIT SCREEN -->
        <div class="split-container" [class.secured]="isSecured()" #splitContainer>
          <!-- Exposed state (left) -->
          <div class="split-panel exposed-panel">
            <div class="panel-overlay"></div>
            <div class="panel-content">
              <div class="status-badge danger">
                <div class="pulse-ring"></div>
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <circle cx="12" cy="12" r="10"></circle>
                  <line x1="15" y1="9" x2="9" y2="15"></line>
                  <line x1="9" y1="9" x2="15" y2="15"></line>
                </svg>
                <span>VULNERABLE</span>
              </div>

              <div class="attack-list">
                <div class="attack-item">
                  <span class="attack-icon">⚠</span>
                  <span class="attack-text">Direct access to all MCP tools</span>
                </div>
                <div class="attack-item">
                  <span class="attack-icon">⚠</span>
                  <span class="attack-text">No identity verification</span>
                </div>
                <div class="attack-item">
                  <span class="attack-icon">⚠</span>
                  <span class="attack-text">Unlimited requests allowed</span>
                </div>
                <div class="attack-item">
                  <span class="attack-icon">⚠</span>
                  <span class="attack-text">No audit trail</span>
                </div>
              </div>

              <div class="panel-terminal">
                <pre><code><span class="comment"># Anyone can connect</span>
<span class="prompt">$</span> curl http://your-server:8080
<span class="output-bad">→ Full access granted</span>
<span class="output-bad">→ No authentication required</span></code></pre>
              </div>
            </div>
          </div>

          <!-- Center divider with animated shield -->
          <div class="center-divider">
            <div class="shield-container" [class.active]="isSecured()">
              <div class="shield-glow"></div>
              <div class="shield-icon">
                <svg viewBox="0 0 24 24" fill="currentColor">
                  <path d="M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4zm0 10.99h7c-.53 4.12-3.28 7.79-7 8.94V12H5V6.3l7-3.11v8.8z"/>
                </svg>
              </div>
              <div class="shield-ring"></div>
            </div>
            <div class="divider-line"></div>
          </div>

          <!-- Secured state (right) -->
          <div class="split-panel secured-panel">
            <div class="panel-overlay"></div>
            <div class="panel-content">
              <div class="status-badge success">
                <div class="pulse-ring"></div>
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
                  <polyline points="22 4 12 14.01 9 11.01"></polyline>
                </svg>
                <span>PROTECTED</span>
              </div>

              <div class="protection-list">
                <div class="protection-item" [class.visible]="isSecured()">
                  <span class="protection-icon">✓</span>
                  <span class="protection-text">OAuth 2.1 + JWT + API Key auth</span>
                </div>
                <div class="protection-item" [class.visible]="isSecured()">
                  <span class="protection-icon">✓</span>
                  <span class="protection-text">Identity verified on every request</span>
                </div>
                <div class="protection-item" [class.visible]="isSecured()">
                  <span class="protection-icon">✓</span>
                  <span class="protection-text">Per-user rate limiting</span>
                </div>
                <div class="protection-item" [class.visible]="isSecured()">
                  <span class="protection-icon">✓</span>
                  <span class="protection-text">Full audit logging with redaction</span>
                </div>
              </div>

              <div class="panel-terminal">
                <pre><code><span class="comment"># mcp-guard enforces security</span>
<span class="prompt">$</span> curl -H "Authorization: Bearer ..." \\
    http://mcp-guard:3000

<span class="output-good">✓ Identity: user@company.com</span>
<span class="output-good">✓ Rate limit: 847/1000 remaining</span>
<span class="output-good">✓ Audit logged: req_abc123</span></code></pre>
              </div>
            </div>
          </div>
        </div>

        <!-- Big toggle -->
        <div class="toggle-section">
          <button 
            class="mega-toggle" 
            (click)="toggleSecured()"
            [class.secured]="isSecured()"
          >
            <div class="toggle-track">
              <span class="toggle-label toggle-label-off">EXPOSED</span>
              <span class="toggle-label toggle-label-on">SECURED</span>
              <div class="toggle-thumb">
                <svg viewBox="0 0 24 24" fill="currentColor">
                  <path d="M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4z"/>
                </svg>
              </div>
            </div>
          </button>
          <p class="toggle-hint">Click to toggle protection</p>
        </div>
      </div>
    </section>
  `,
  styles: [`
    .comparison {
      position: relative;
      padding: 100px 0;
      background: var(--bg-secondary);
      overflow: hidden;
    }

    /* Subtle diagonal brand element */
    .diagonal-brand {
      position: absolute;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%) rotate(-45deg);
      display: flex;
      align-items: center;
      gap: 32px;
      font-family: var(--font-display);
      font-size: clamp(60px, 12vw, 150px);
      font-weight: 800;
      color: transparent;
      -webkit-text-stroke: 1px rgba(255, 122, 48, 0.03);
      white-space: nowrap;
      pointer-events: none;
      user-select: none;
      letter-spacing: -0.05em;
      z-index: 0;
    }

    .brand-divider {
      color: rgba(255, 122, 48, 0.1);
      -webkit-text-stroke: none;
    }

    .after-text {
      -webkit-text-stroke-color: rgba(70, 92, 136, 0.08);
    }

    .brand-float {
      position: absolute;
      font-family: var(--font-display);
      font-size: 400px;
      font-weight: 800;
      color: transparent;
      -webkit-text-stroke: 1px var(--border-subtle);
      opacity: 0.15;
      pointer-events: none;
      user-select: none;
      letter-spacing: -0.05em;
    }

    .brand-float-1 {
      top: -100px;
      left: -100px;
    }

    .brand-float-2 {
      bottom: -100px;
      right: -100px;
    }

    .comparison-container {
      position: relative;
      max-width: 1400px;
      margin: 0 auto;
      padding: 0 24px;
      z-index: 1;
    }

    .section-header {
      display: flex;
      align-items: flex-start;
      gap: 32px;
      margin-bottom: 64px;
    }

    .section-number {
      font-family: var(--font-mono);
      font-size: 100px;
      font-weight: 800;
      color: transparent;
      -webkit-text-stroke: 1px var(--border-subtle);
      line-height: 0.8;
    }

    .header-text {
      padding-top: 16px;
    }

    .section-tag {
      font-family: var(--font-mono);
      font-size: 13px;
      color: #FF7A30;
      letter-spacing: 0.05em;
      margin-bottom: 12px;
      display: block;
    }

    .section-title {
      display: flex;
      flex-direction: column;
    }

    .title-line {
      font-family: var(--font-sans);
      font-size: clamp(24px, 4vw, 36px);
      font-weight: 400;
      color: var(--text-secondary);
    }

    .state-text {
      font-family: var(--font-display);
      font-size: clamp(48px, 8vw, 80px);
      font-weight: 800;
      letter-spacing: -0.03em;
      color: var(--accent-red);
      transition: color 0.5s, text-shadow 0.5s;

      &.secured {
        color: #4ade80;
        text-shadow: 0 0 40px rgba(74, 222, 128, 0.3);
      }
    }

    /* Split container */
    .split-container {
      display: grid;
      grid-template-columns: 1fr 80px 1fr;
      gap: 0;
      margin-bottom: 64px;

      @media (max-width: 900px) {
        grid-template-columns: 1fr;
        gap: 32px;
      }
    }

    .split-panel {
      position: relative;
      background: var(--bg-primary);
      border-radius: 24px;
      padding: 40px;
      border: 1px solid var(--border-subtle);
      transition: all 0.6s cubic-bezier(0.4, 0, 0.2, 1);
      overflow: hidden;
    }

    .panel-overlay {
      position: absolute;
      inset: 0;
      pointer-events: none;
      transition: opacity 0.6s;
    }

    .exposed-panel {
      transform-origin: right center;

      .panel-overlay {
        background: linear-gradient(135deg, rgba(239, 68, 68, 0.05) 0%, transparent 50%);
      }
    }

    .secured-panel {
      transform-origin: left center;

      .panel-overlay {
        background: linear-gradient(135deg, transparent 50%, rgba(74, 222, 128, 0.05) 100%);
      }
    }

    .split-container.secured {
      .exposed-panel {
        opacity: 0.4;
        transform: scale(0.98) translateX(-10px);
      }

      .secured-panel {
        transform: scale(1.02);
        border-color: rgba(74, 222, 128, 0.3);
        box-shadow: 0 0 60px -20px rgba(74, 222, 128, 0.3);
      }
    }

    .split-container:not(.secured) {
      .exposed-panel {
        transform: scale(1.02);
        border-color: rgba(239, 68, 68, 0.3);
        box-shadow: 0 0 60px -20px rgba(239, 68, 68, 0.3);
      }

      .secured-panel {
        opacity: 0.4;
        transform: scale(0.98) translateX(10px);
      }
    }

    .panel-content {
      position: relative;
      z-index: 1;
    }

    .status-badge {
      display: inline-flex;
      align-items: center;
      gap: 12px;
      padding: 12px 24px;
      border-radius: 100px;
      font-family: var(--font-mono);
      font-size: 12px;
      font-weight: 700;
      letter-spacing: 0.1em;
      margin-bottom: 32px;
      position: relative;

      svg {
        width: 20px;
        height: 20px;
      }

      &.danger {
        background: rgba(239, 68, 68, 0.1);
        color: var(--accent-red);
        border: 1px solid rgba(239, 68, 68, 0.2);
      }

      &.success {
        background: rgba(74, 222, 128, 0.1);
        color: #4ade80;
        border: 1px solid rgba(74, 222, 128, 0.2);
      }
    }

    .pulse-ring {
      position: absolute;
      top: 50%;
      left: 24px;
      transform: translate(-50%, -50%);
      width: 40px;
      height: 40px;
      border-radius: 50%;
      animation: pulseRing 2s infinite;
    }

    .danger .pulse-ring {
      background: rgba(239, 68, 68, 0.3);
    }

    .success .pulse-ring {
      background: rgba(74, 222, 128, 0.3);
    }

    @keyframes pulseRing {
      0% { transform: translate(-50%, -50%) scale(0.5); opacity: 1; }
      100% { transform: translate(-50%, -50%) scale(2); opacity: 0; }
    }

    .attack-list, .protection-list {
      display: flex;
      flex-direction: column;
      gap: 16px;
      margin-bottom: 32px;
    }

    .attack-item, .protection-item {
      display: flex;
      align-items: center;
      gap: 16px;
      padding: 16px;
      background: var(--bg-secondary);
      border-radius: 12px;
      transition: all 0.4s;
    }

    .attack-item {
      border-left: 3px solid var(--accent-red);
    }

    .attack-icon {
      color: var(--accent-red);
      font-size: 18px;
    }

    .attack-text {
      color: var(--text-secondary);
      font-size: 14px;
    }

    .protection-item {
      border-left: 3px solid #4ade80;
      opacity: 0;
      transform: translateX(20px);
      transition: all 0.4s ease;

      &.visible {
        opacity: 1;
        transform: translateX(0);
      }

      @for $i from 1 through 4 {
        &:nth-child(#{$i}).visible {
          transition-delay: #{($i - 1) * 0.1}s;
        }
      }
    }

    .protection-icon {
      color: #4ade80;
      font-size: 18px;
    }

    .protection-text {
      color: var(--text-secondary);
      font-size: 14px;
    }

    .panel-terminal {
      background: var(--bg-secondary);
      border: 1px solid var(--border-subtle);
      border-radius: 12px;
      padding: 20px;

      pre {
        margin: 0;
        font-family: var(--font-mono);
        font-size: 13px;
        line-height: 1.8;
      }

      .comment { color: var(--text-dim); }
      .prompt { color: #FF7A30; }
      .output-bad { color: var(--accent-red); }
      .output-good { color: #4ade80; }
    }

    /* Center divider */
    .center-divider {
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      position: relative;
      z-index: 10;

      @media (max-width: 900px) {
        flex-direction: row;
        order: -1;
      }
    }

    .shield-container {
      position: relative;
      width: 80px;
      height: 80px;
      display: flex;
      align-items: center;
      justify-content: center;
    }

    .shield-glow {
      position: absolute;
      inset: -20px;
      background: radial-gradient(circle, rgba(255, 122, 48, 0.3) 0%, transparent 70%);
      opacity: 0;
      transition: opacity 0.5s;
    }

    .shield-container.active .shield-glow {
      opacity: 1;
      background: radial-gradient(circle, rgba(74, 222, 128, 0.4) 0%, transparent 70%);
    }

    .shield-icon {
      position: relative;
      z-index: 2;
      width: 56px;
      height: 56px;
      background: var(--bg-secondary);
      border: 2px solid var(--border-subtle);
      border-radius: 50%;
      display: flex;
      align-items: center;
      justify-content: center;
      color: var(--text-muted);
      transition: all 0.5s;

      svg {
        width: 28px;
        height: 28px;
      }
    }

    .shield-container.active .shield-icon {
      border-color: #4ade80;
      color: #4ade80;
      background: rgba(74, 222, 128, 0.1);
    }

    .shield-ring {
      position: absolute;
      width: 70px;
      height: 70px;
      border: 2px solid transparent;
      border-top-color: var(--border-subtle);
      border-radius: 50%;
      animation: rotate 3s linear infinite;
    }

    .shield-container.active .shield-ring {
      border-top-color: #4ade80;
    }

    @keyframes rotate {
      from { transform: rotate(0deg); }
      to { transform: rotate(360deg); }
    }

    .divider-line {
      flex: 1;
      width: 2px;
      background: linear-gradient(to bottom, transparent, var(--border-subtle), transparent);

      @media (max-width: 900px) {
        width: auto;
        height: 2px;
        flex: 1;
        background: linear-gradient(to right, transparent, var(--border-subtle), transparent);
      }
    }

    /* Mega toggle */
    .toggle-section {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 16px;
    }

    .mega-toggle {
      position: relative;
      width: 280px;
      height: 64px;
      background: var(--bg-primary);
      border: 2px solid var(--border-subtle);
      border-radius: 100px;
      cursor: pointer;
      transition: all 0.4s;
      overflow: hidden;

      &:hover {
        border-color: var(--border-accent);
      }

      &.secured {
        border-color: rgba(74, 222, 128, 0.4);

        .toggle-thumb {
          left: calc(100% - 56px);
          background: #4ade80;
          color: var(--bg-primary);
        }

        .toggle-label-off {
          opacity: 0.3;
        }

        .toggle-label-on {
          opacity: 1;
          color: #4ade80;
        }
      }

      &:not(.secured) {
        border-color: rgba(239, 68, 68, 0.4);

        .toggle-label-off {
          opacity: 1;
          color: var(--accent-red);
        }

        .toggle-label-on {
          opacity: 0.3;
        }
      }
    }

    .toggle-track {
      position: relative;
      width: 100%;
      height: 100%;
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 0 24px;
    }

    .toggle-label {
      font-family: var(--font-mono);
      font-size: 12px;
      font-weight: 700;
      letter-spacing: 0.1em;
      color: var(--text-muted);
      transition: all 0.4s;
    }

    .toggle-thumb {
      position: absolute;
      left: 4px;
      width: 52px;
      height: 52px;
      background: var(--accent-red);
      border-radius: 50%;
      display: flex;
      align-items: center;
      justify-content: center;
      color: var(--bg-primary);
      transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
      box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);

      svg {
        width: 24px;
        height: 24px;
      }
    }

    .toggle-hint {
      font-size: 13px;
      color: var(--text-muted);
    }

    @media (max-width: 768px) {
      .section-header {
        flex-direction: column;
        gap: 16px;
      }

      .section-number {
        font-size: 60px;
      }

      .split-panel {
        padding: 24px;
      }

      .mega-toggle {
        width: 240px;
        height: 56px;
      }
    }
  `]
})
export class ComparisonComponent implements OnInit, OnDestroy, AfterViewInit {
  @ViewChild('container') containerRef!: ElementRef<HTMLElement>;
  @ViewChild('splitContainer') splitContainerRef!: ElementRef<HTMLElement>;

  private ngZone = inject(NgZone);
  private scrollTrigger: ScrollTrigger | null = null;

  isSecured = signal(false);

  ngOnInit() { }

  ngAfterViewInit() {
    this.initAutoToggle();
  }

  ngOnDestroy() {
    this.scrollTrigger?.kill();
  }

  toggleSecured() {
    this.isSecured.update(v => !v);
  }

  private initAutoToggle() {
    this.ngZone.runOutsideAngular(() => {
      // Auto-toggle when section is 60% in view
      this.scrollTrigger = ScrollTrigger.create({
        trigger: this.containerRef.nativeElement,
        start: 'top 40%',
        onEnter: () => {
          // Delay the toggle for dramatic effect
          setTimeout(() => {
            this.ngZone.run(() => {
              this.isSecured.set(true);
            });
          }, 800);
        },
        onLeaveBack: () => {
          this.ngZone.run(() => {
            this.isSecured.set(false);
          });
        }
      });
    });
  }
}
