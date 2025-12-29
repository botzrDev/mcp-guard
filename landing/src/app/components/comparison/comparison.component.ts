import { Component, signal, ChangeDetectionStrategy, OnInit, OnDestroy, ElementRef, inject, AfterViewInit, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ScrollAnimationService } from '../../shared/scroll-animation.service';
import { ScrollTrigger } from 'gsap/ScrollTrigger';

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
            <span class="section-tag">// How we compare (honestly)</span>
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
      padding: var(--section-py) 0;
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
      gap: var(--space-8);
      font-family: var(--font-display);
      font-size: clamp(60px, 12vw, 150px);
      font-weight: var(--weight-extrabold);
      color: transparent;
      -webkit-text-stroke: 1px rgba(255, 122, 48, 0.03);
      white-space: nowrap;
      pointer-events: none;
      user-select: none;
      letter-spacing: var(--tracking-tighter);
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
      font-size: clamp(150px, 30vw, 400px);
      font-weight: var(--weight-extrabold);
      color: transparent;
      -webkit-text-stroke: 1px var(--border-subtle);
      opacity: 0.15;
      pointer-events: none;
      user-select: none;
      letter-spacing: var(--tracking-tighter);
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
      padding: 0 var(--container-px);
      z-index: 1;
    }

    .section-header {
      display: flex;
      align-items: flex-start;
      gap: var(--space-8);
      margin-bottom: var(--space-16);
    }

    .section-number {
      font-family: var(--font-mono);
      font-size: var(--text-6xl);
      font-weight: var(--weight-extrabold);
      color: transparent;
      -webkit-text-stroke: 1px var(--border-subtle);
      line-height: var(--leading-none);
    }

    .header-text {
      padding-top: var(--space-4);
    }

    .section-tag {
      font-family: var(--font-mono);
      font-size: var(--text-xs);
      color: var(--accent-cyan);
      letter-spacing: var(--tracking-wider);
      margin-bottom: var(--space-3);
      display: block;
    }

    .section-title {
      display: flex;
      flex-direction: column;
      padding-bottom: 0.15em; /* Protect descenders from overflow clipping */
    }

    .title-line {
      font-family: var(--font-sans);
      font-size: clamp(24px, 4vw, 36px);
      font-weight: 400;
      color: var(--text-secondary);
    }

    .state-text {
      font-family: var(--font-display);
      font-size: var(--text-5xl);
      font-weight: var(--weight-extrabold);
      letter-spacing: var(--tracking-tighter);
      color: var(--accent-red);
      transition: color var(--duration-slow), text-shadow var(--duration-slow);

      &.secured {
        color: var(--accent-green);
        text-shadow: 0 0 40px var(--border-green);
      }
    }

    /* Split container */
    .split-container {
      display: grid;
      grid-template-columns: 1fr var(--space-20) 1fr;
      gap: 0;
      margin-bottom: var(--space-16);

      @media (max-width: 900px) {
        grid-template-columns: 1fr;
        gap: var(--space-8);
      }
    }

    .split-panel {
      position: relative;
      background: var(--bg-primary);
      border-radius: var(--radius-3xl);
      padding: var(--space-10);
      border: 1px solid var(--border-subtle);
      transition: all var(--duration-slower) var(--ease-in-out);
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
      gap: var(--space-3);
      padding: var(--space-3) var(--space-6);
      border-radius: var(--radius-full);
      font-family: var(--font-mono);
      font-size: var(--text-xs);
      font-weight: var(--weight-bold);
      letter-spacing: var(--tracking-widest);
      margin-bottom: var(--space-8);
      position: relative;

      svg {
        width: var(--icon-md);
        height: var(--icon-md);
      }

      &.danger {
        background: rgba(239, 68, 68, 0.1);
        color: var(--accent-red);
        border: 1px solid rgba(239, 68, 68, 0.2);
      }

      &.success {
        background: rgba(74, 222, 128, 0.1);
        color: var(--accent-green);
        border: 1px solid rgba(74, 222, 128, 0.2);
      }
    }

    .pulse-ring {
      position: absolute;
      top: 50%;
      left: var(--space-6);
      transform: translate(-50%, -50%);
      width: var(--space-10);
      height: var(--space-10);
      border-radius: var(--radius-full);
      animation: pulseRing 2s infinite;
    }

    .danger .pulse-ring {
      background: rgba(239, 68, 68, 0.3);
    }

    .success .pulse-ring {
      background: var(--border-green);
    }

    @keyframes pulseRing {
      0% { transform: translate(-50%, -50%) scale(0.5); opacity: 1; }
      100% { transform: translate(-50%, -50%) scale(2); opacity: 0; }
    }

    .attack-list, .protection-list {
      display: flex;
      flex-direction: column;
      gap: var(--space-4);
      margin-bottom: var(--space-8);
    }

    .attack-item, .protection-item {
      display: flex;
      align-items: center;
      gap: var(--space-4);
      padding: var(--space-4);
      background: var(--bg-secondary);
      border-radius: var(--radius-xl);
      transition: all var(--duration-slow);
    }

    .attack-item {
      border-left: 3px solid var(--accent-red);
    }

    .attack-icon {
      color: var(--accent-red);
      font-size: var(--text-lg);
    }

    .attack-text {
      color: var(--text-secondary);
      font-size: var(--text-sm);
    }

    .protection-item {
      border-left: 3px solid var(--accent-green);
      opacity: 0;
      transform: translateX(var(--space-5));
      transition: all var(--duration-slow) var(--ease-out);

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
      color: var(--accent-green);
      font-size: var(--text-lg);
    }

    .protection-text {
      color: var(--text-secondary);
      font-size: var(--text-sm);
    }

    .panel-terminal {
      background: var(--bg-secondary);
      border: 1px solid var(--border-subtle);
      border-radius: var(--radius-xl);
      padding: var(--space-5);

      pre {
        margin: 0;
        font-family: var(--font-mono);
        font-size: var(--text-xs);
        line-height: var(--leading-loose);
      }

      .comment { color: var(--text-dim); }
      .prompt { color: var(--accent-cyan); }
      .output-bad { color: var(--accent-red); }
      .output-good { color: var(--accent-green); }
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
      width: var(--space-20);
      height: var(--space-20);
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
      background: radial-gradient(circle, var(--border-green) 0%, transparent 70%);
    }

    .shield-icon {
      position: relative;
      z-index: 2;
      width: var(--space-14);
      height: var(--space-14);
      background: var(--bg-secondary);
      border: 2px solid var(--border-subtle);
      border-radius: var(--radius-full);
      display: flex;
      align-items: center;
      justify-content: center;
      color: var(--text-muted);
      transition: all var(--duration-slow);

      svg {
        width: var(--icon-xl);
        height: var(--icon-xl);
      }
    }

    .shield-container.active .shield-icon {
      border-color: var(--accent-green);
      color: var(--accent-green);
      background: rgba(74, 222, 128, 0.1);
    }

    .shield-ring {
      position: absolute;
      width: var(--space-18);
      height: var(--space-18);
      border: 2px solid transparent;
      border-top-color: var(--border-subtle);
      border-radius: var(--radius-full);
      animation: rotate 3s linear infinite;
    }

    .shield-container.active .shield-ring {
      border-top-color: var(--accent-green);
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
      gap: var(--space-4);
    }

    .mega-toggle {
      position: relative;
      width: 280px;
      height: var(--space-16);
      background: var(--bg-primary);
      border: 2px solid var(--border-subtle);
      border-radius: var(--radius-full);
      cursor: pointer;
      transition: all var(--duration-slow);
      overflow: hidden;

      &:hover {
        border-color: var(--border-accent);
      }

      &.secured {
        border-color: var(--border-green);

        .toggle-thumb {
          left: calc(100% - 56px);
          background: var(--accent-green);
          color: var(--bg-primary);
        }

        .toggle-label-off {
          opacity: 0.3;
        }

        .toggle-label-on {
          opacity: 1;
          color: var(--accent-green);
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
      padding: 0 var(--space-6);
    }

    .toggle-label {
      font-family: var(--font-mono);
      font-size: var(--text-xs);
      font-weight: var(--weight-bold);
      letter-spacing: var(--tracking-widest);
      color: var(--text-muted);
      transition: all var(--duration-slow);
    }

    .toggle-thumb {
      position: absolute;
      left: var(--space-1);
      width: var(--space-14);
      height: var(--space-14);
      background: var(--accent-red);
      border-radius: var(--radius-full);
      display: flex;
      align-items: center;
      justify-content: center;
      color: var(--bg-primary);
      transition: all var(--duration-slow) var(--ease-in-out);
      box-shadow: var(--shadow-lg);

      svg {
        width: var(--icon-lg);
        height: var(--icon-lg);
      }
    }

    .toggle-hint {
      font-size: var(--text-xs);
      color: var(--text-muted);
    }

    @media (max-width: 768px) {
      .section-header {
        flex-direction: column;
        gap: var(--space-4);
      }

      .section-number {
        font-size: var(--text-4xl);
      }

      .split-panel {
        padding: var(--space-6);
      }

      .mega-toggle {
        width: 240px;
        height: var(--space-14);
      }
    }
  `]
})
export class ComparisonComponent implements OnInit, OnDestroy, AfterViewInit {
  @ViewChild('container') containerRef!: ElementRef<HTMLElement>;
  @ViewChild('splitContainer') splitContainerRef!: ElementRef<HTMLElement>;

  private scrollService = inject(ScrollAnimationService);
  private scrollTrigger: ScrollTrigger | null = null;
  private toggleTimeout: ReturnType<typeof setTimeout> | null = null;

  isSecured = signal(false);

  ngOnInit() { }

  ngAfterViewInit() {
    this.initAutoToggle();
  }

  ngOnDestroy() {
    this.scrollService.killTrigger(this.scrollTrigger);
    if (this.toggleTimeout) {
      clearTimeout(this.toggleTimeout);
    }
  }

  toggleSecured() {
    this.isSecured.update(v => !v);
  }

  private initAutoToggle() {
    this.scrollTrigger = this.scrollService.createVisibilityTrigger(
      this.containerRef.nativeElement,
      {
        onEnter: () => {
          // Delay the toggle for dramatic effect
          this.toggleTimeout = setTimeout(() => {
            this.isSecured.set(true);
          }, 800);
        },
        onLeaveBack: () => {
          if (this.toggleTimeout) {
            clearTimeout(this.toggleTimeout);
          }
          this.isSecured.set(false);
        }
      },
      { start: 'top 40%' }
    );
  }
}
