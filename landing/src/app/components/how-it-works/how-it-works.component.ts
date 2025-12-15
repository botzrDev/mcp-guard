import {
  Component,
  signal,
  ChangeDetectionStrategy,
  OnInit,
  OnDestroy,
  inject,
  NgZone,
  ElementRef,
  AfterViewInit,
  ViewChild,
} from '@angular/core';
import { CommonModule } from '@angular/common';
import { IconComponent } from '../../shared/icon/icon.component';
import { gsap } from 'gsap';
import { ScrollTrigger } from 'gsap/ScrollTrigger';

gsap.registerPlugin(ScrollTrigger);

interface Step {
  number: number;
  title: string;
  description: string;
  icon: 'terminal' | 'config' | 'rocket';
  code: string;
  brandColor: string;
}

@Component({
  selector: 'app-how-it-works',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, IconComponent],
  template: `
    <section class="how-it-works" id="how-it-works" #container>
      <!-- Brand diagonal stripe -->
      <div class="brand-stripe"></div>
      
      <!-- Floating brand marks -->
      <div class="brand-watermark">mcp-guard</div>
      
      <div class="hiw-header">
        <div class="header-content">
          <span class="section-number">02</span>
          <div class="section-tag-wrapper">
            <span class="section-tag">// Quick Start</span>
            <div class="tag-line"></div>
          </div>
          <h2 class="section-title">
            <span class="title-small">Three commands.</span>
            <span class="title-big gradient-text">That's it.</span>
          </h2>
        </div>
        <div class="scroll-hint">
          <span>Scroll to explore</span>
          <div class="scroll-arrow"></div>
        </div>
      </div>

      <!-- Horizontal scroll panels -->
      <div class="panels-container" #panelsContainer>
        <div class="panels-track" #panelsTrack>
          @for (step of steps; track step.number; let i = $index) {
            <div 
              class="step-panel" 
              [attr.data-step]="i"
              [style.--brand-color]="step.brandColor"
            >
              <!-- Large background number -->
              <div class="panel-bg-number">{{ step.number }}</div>
              
              <!-- Brand accent corner -->
              <div class="panel-corner"></div>
              
              <div class="panel-content">
                <div class="step-indicator">
                  <div class="indicator-ring" [class.active]="activeStep() >= i">
                    <div class="indicator-fill"></div>
                  </div>
                  <span class="indicator-label">Step {{ step.number }}</span>
                </div>

                <div class="step-main">
                  <div class="step-icon-wrapper">
                    <div class="icon-glow"></div>
                    <div class="step-icon">
                      <app-icon [name]="step.icon" size="32px" />
                    </div>
                  </div>
                  
                  <h3 class="step-title">{{ step.title }}</h3>
                  <p class="step-description">{{ step.description }}</p>
                </div>

                <div class="step-terminal">
                  <div class="terminal-chrome">
                    <div class="terminal-dots">
                      <span></span><span></span><span></span>
                    </div>
                    <span class="terminal-path">~/project</span>
                  </div>
                  <div class="terminal-body">
                    <div class="terminal-line" [class.typed]="activeStep() >= i">
                      <span class="prompt">$</span>
                      <span class="command">{{ step.code }}</span>
                      <span class="cursor"></span>
                    </div>
                    @if (i === 0 && activeStep() >= 0) {
                      <div class="terminal-output">
                        <span class="output-success">✓ Installed mcp-guard v0.5.0</span>
                      </div>
                    }
                    @if (i === 1 && activeStep() >= 1) {
                      <div class="terminal-output">
                        <span class="output-success">✓ Created mcp-guard.toml</span>
                      </div>
                    }
                    @if (i === 2 && activeStep() >= 2) {
                      <div class="terminal-output">
                        <span class="output-highlight">→ Proxy listening on 0.0.0.0:3000</span>
                        <span class="output-success">✓ Auth: OAuth 2.1 + JWT + API Key</span>
                        <span class="output-success">✓ Rate limiting: enabled</span>
                        <span class="output-success">✓ Audit logging: enabled</span>
                      </div>
                    }
                  </div>
                </div>
              </div>

              <!-- Connection line to next panel -->
              @if (i < steps.length - 1) {
                <div class="panel-connector">
                  <svg viewBox="0 0 200 4" preserveAspectRatio="none">
                    <line x1="0" y1="2" x2="200" y2="2" 
                      stroke="url(#connector-grad)" 
                      stroke-width="2"
                      stroke-dasharray="8 4"
                      [class.animated]="activeStep() >= i"
                    />
                    <defs>
                      <linearGradient id="connector-grad" x1="0%" y1="0%" x2="100%" y2="0%">
                        <stop offset="0%" stop-color="#FF7A30" stop-opacity="0.6"/>
                        <stop offset="100%" stop-color="#465C88" stop-opacity="0.2"/>
                      </linearGradient>
                    </defs>
                  </svg>
                </div>
              }
            </div>
          }
          
          <!-- Final success panel -->
          <div class="step-panel success-panel">
            <div class="success-content">
              <div class="success-icon">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
                  <polyline points="22 4 12 14.01 9 11.01"></polyline>
                </svg>
              </div>
              <h3 class="success-title">You're protected.</h3>
              <p class="success-subtitle">Your MCP server now has enterprise-grade security.</p>
              <a href="/docs/quickstart" class="success-cta">
                Read Full Documentation
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M5 12h14M12 5l7 7-7 7"/>
                </svg>
              </a>
            </div>
          </div>
        </div>
      </div>

      <!-- Progress indicator -->
      <div class="progress-track">
        <div class="progress-fill" [style.width.%]="progressPercent()"></div>
        <div class="progress-markers">
          @for (step of steps; track step.number; let i = $index) {
            <div class="progress-marker" [class.active]="activeStep() >= i">
              <span class="marker-dot"></span>
              <span class="marker-label">{{ step.title }}</span>
            </div>
          }
          <div class="progress-marker" [class.active]="activeStep() >= 3">
            <span class="marker-dot"></span>
            <span class="marker-label">Done</span>
          </div>
        </div>
      </div>
    </section>
  `,
  styles: [`
    .how-it-works {
      position: relative;
      background: var(--bg-primary);
      overflow: hidden;
      min-height: 100vh;
    }

    /* Brand stripe - subtle */
    .brand-stripe {
      position: absolute;
      top: 0;
      left: -10%;
      width: 120%;
      height: 3px;
      background: var(--gradient-brand);
      transform: rotate(-0.5deg);
    }

    .brand-watermark {
      position: absolute;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%) rotate(-10deg);
      font-family: var(--font-display);
      font-size: clamp(100px, 16vw, 220px);
      font-weight: var(--weight-extrabold);
      color: rgba(255, 122, 48, 0.015);
      white-space: nowrap;
      pointer-events: none;
      user-select: none;
      letter-spacing: var(--tracking-tighter);
    }

    .hiw-header {
      position: relative;
      padding: var(--space-16) var(--container-px) var(--space-8);
      max-width: 1400px;
      margin: 0 auto;
      display: flex;
      justify-content: space-between;
      align-items: flex-end;
    }

    .section-number {
      font-family: var(--font-mono);
      font-size: var(--text-6xl);
      font-weight: var(--weight-extrabold);
      color: transparent;
      -webkit-text-stroke: 1px var(--border-subtle);
      line-height: var(--leading-none);
      margin-bottom: var(--space-5);
      display: block;
    }

    .section-tag-wrapper {
      display: flex;
      align-items: center;
      gap: var(--space-4);
      margin-bottom: var(--space-4);
    }

    .section-tag {
      font-family: var(--font-mono);
      font-size: var(--text-sm);
      color: var(--accent-cyan);
      letter-spacing: var(--tracking-wider);
      line-height: var(--leading-normal);
    }

    .tag-line {
      width: 60px;
      height: 2px;
      background: var(--gradient-brand);
    }

    .section-title {
      display: flex;
      flex-direction: column;
    }

    .title-small {
      font-family: var(--font-sans);
      font-size: var(--text-xl);
      font-weight: var(--weight-normal);
      color: var(--text-secondary);
      line-height: var(--leading-normal);
      margin-bottom: var(--space-2);
    }

    .title-big {
      font-family: var(--font-display);
      font-size: var(--text-6xl);
      font-weight: var(--weight-extrabold);
      letter-spacing: var(--tracking-tighter);
      line-height: var(--leading-none);
    }

    .gradient-text {
      background: var(--gradient-brand);
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
      background-clip: text;
    }

    .scroll-hint {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: var(--space-3);
      color: var(--text-muted);
      font-size: var(--text-xs);
      text-transform: uppercase;
      letter-spacing: var(--tracking-widest);
      line-height: var(--leading-normal);

      @media (max-width: 768px) {
        display: none;
      }
    }

    .scroll-arrow {
      width: var(--space-6);
      height: var(--space-10);
      border: 2px solid var(--border-subtle);
      border-radius: var(--radius-xl);
      position: relative;

      &::after {
        content: '';
        position: absolute;
        top: var(--space-2);
        left: 50%;
        transform: translateX(-50%);
        width: var(--space-1);
        height: var(--space-2);
        background: var(--accent-cyan);
        border-radius: var(--radius-sm);
        animation: scrollBounce 1.5s infinite;
      }
    }

    @keyframes scrollBounce {
      0%, 100% { transform: translateX(-50%) translateY(0); opacity: 1; }
      50% { transform: translateX(-50%) translateY(var(--space-3)); opacity: 0.3; }
    }

    /* Horizontal scroll panels */
    .panels-container {
      position: relative;
      overflow: hidden;
      padding: var(--space-10) 0 var(--space-24);
    }

    .panels-track {
      display: flex;
      gap: var(--space-10);
      padding: 0 max(var(--space-6), calc((100vw - 1400px) / 2));
      width: max-content;
    }

    .step-panel {
      position: relative;
      width: min(500px, 80vw);
      min-height: 500px;
      background: var(--bg-secondary);
      border: 1px solid var(--border-subtle);
      border-radius: var(--radius-3xl);
      padding: var(--space-10);
      flex-shrink: 0;
      overflow: hidden;
      transition: border-color var(--duration-slow) var(--ease-out), box-shadow var(--duration-slow) var(--ease-out);

      &:hover {
        border-color: var(--brand-color, #FF7A30);
        box-shadow: var(--shadow-glow-orange);
      }
    }

    .panel-bg-number {
      position: absolute;
      top: -40px;
      right: -20px;
      font-family: var(--font-display);
      font-size: 280px;
      font-weight: var(--weight-extrabold);
      color: transparent;
      -webkit-text-stroke: 1px var(--border-subtle);
      opacity: 0.3;
      line-height: var(--leading-none);
      user-select: none;
      pointer-events: none;
    }

    .panel-corner {
      position: absolute;
      top: 0;
      right: 0;
      width: 120px;
      height: 120px;
      background: linear-gradient(135deg, transparent 50%, var(--brand-color, rgba(255, 122, 48, 0.1)) 50%);
      pointer-events: none;
    }

    .panel-content {
      position: relative;
      z-index: 1;
      height: 100%;
      display: flex;
      flex-direction: column;
    }

    .step-indicator {
      display: flex;
      align-items: center;
      gap: var(--space-3);
      margin-bottom: var(--space-8);
    }

    .indicator-ring {
      width: var(--space-10);
      height: var(--space-10);
      border-radius: var(--radius-full);
      border: 2px solid var(--border-subtle);
      display: flex;
      align-items: center;
      justify-content: center;
      transition: all var(--duration-slow) var(--ease-out);

      &.active {
        border-color: var(--accent-cyan);

        .indicator-fill {
          transform: scale(1);
        }
      }
    }

    .indicator-fill {
      width: var(--space-4);
      height: var(--space-4);
      background: var(--gradient-brand);
      border-radius: var(--radius-full);
      transform: scale(0);
      transition: transform var(--duration-slow) var(--ease-bounce);
    }

    .indicator-label {
      font-family: var(--font-mono);
      font-size: var(--text-xs);
      color: var(--text-muted);
      text-transform: uppercase;
      letter-spacing: var(--tracking-widest);
      line-height: var(--leading-normal);
    }

    .step-main {
      flex: 1;
    }

    .step-icon-wrapper {
      position: relative;
      width: var(--space-18);
      height: var(--space-18);
      margin-bottom: var(--space-6);
    }

    .icon-glow {
      position: absolute;
      inset: -20px;
      background: radial-gradient(circle, var(--brand-color, rgba(255, 122, 48, 0.2)) 0%, transparent 70%);
      opacity: 0;
      transition: opacity var(--duration-slow) var(--ease-out);
    }

    .step-panel:hover .icon-glow {
      opacity: 1;
    }

    .step-icon {
      width: var(--space-18);
      height: var(--space-18);
      background: var(--bg-elevated);
      border: 1px solid var(--border-subtle);
      border-radius: var(--radius-2xl);
      display: flex;
      align-items: center;
      justify-content: center;
      color: var(--accent-cyan);
    }

    .step-title {
      font-family: var(--font-display);
      font-size: var(--text-3xl);
      font-weight: var(--weight-bold);
      letter-spacing: var(--tracking-tight);
      line-height: var(--leading-snug);
      margin-bottom: var(--space-3);
    }

    .step-description {
      color: var(--text-secondary);
      font-size: var(--text-base);
      line-height: var(--leading-relaxed);
      margin-bottom: var(--space-8);
    }

    .step-terminal {
      background: var(--bg-primary);
      border: 1px solid var(--border-subtle);
      border-radius: var(--radius-xl);
      overflow: hidden;
    }

    .terminal-chrome {
      display: flex;
      align-items: center;
      padding: var(--space-3) var(--space-4);
      background: var(--bg-elevated);
      border-bottom: 1px solid var(--border-subtle);
    }

    .terminal-dots {
      display: flex;
      gap: var(--space-1-5);
      margin-right: var(--space-4);

      span {
        width: var(--space-2-5);
        height: var(--space-2-5);
        border-radius: var(--radius-full);
        background: var(--bg-hover);
      }
    }

    .terminal-path {
      font-family: var(--font-mono);
      font-size: var(--text-xs);
      color: var(--text-muted);
      line-height: var(--leading-normal);
    }

    .terminal-body {
      padding: var(--space-4);
      font-family: var(--font-mono);
      font-size: var(--text-sm);
      line-height: var(--leading-loose);
    }

    .terminal-line {
      display: flex;
      align-items: center;
      gap: var(--space-2);
    }

    .prompt {
      color: var(--accent-cyan);
    }

    .command {
      color: var(--text-primary);
    }

    .cursor {
      width: var(--space-2);
      height: var(--icon-sm);
      background: var(--accent-cyan);
      animation: blink 1s step-end infinite;
    }

    .terminal-line.typed .cursor {
      display: none;
    }

    .terminal-output {
      display: flex;
      flex-direction: column;
      gap: var(--space-1);
      margin-top: var(--space-3);
      padding-top: var(--space-3);
      border-top: 1px dashed var(--border-subtle);
    }

    .output-success {
      color: var(--accent-green);
      line-height: var(--leading-normal);
    }

    .output-highlight {
      color: var(--accent-cyan);
      line-height: var(--leading-normal);
    }

    @keyframes blink {
      0%, 50% { opacity: 1; }
      51%, 100% { opacity: 0; }
    }

    .panel-connector {
      position: absolute;
      right: calc(var(--space-10) * -1);
      top: 50%;
      transform: translateY(-50%);
      width: var(--space-10);
      z-index: 2;

      svg {
        width: 100%;
        height: var(--space-1);
      }

      line {
        stroke-dashoffset: 200;
        transition: stroke-dashoffset 1s var(--ease-out);

        &.animated {
          stroke-dashoffset: 0;
        }
      }
    }

    /* Success panel */
    .success-panel {
      background: linear-gradient(135deg, var(--bg-secondary) 0%, rgba(255, 122, 48, 0.05) 100%);
      display: flex;
      align-items: center;
      justify-content: center;
      text-align: center;
    }

    .success-content {
      max-width: 320px;
    }

    .success-icon {
      width: var(--space-20);
      height: var(--space-20);
      margin: 0 auto var(--space-6);
      background: rgba(74, 222, 128, 0.1);
      border: 2px solid var(--border-green);
      border-radius: var(--radius-full);
      display: flex;
      align-items: center;
      justify-content: center;
      color: var(--accent-green);

      svg {
        width: var(--space-10);
        height: var(--space-10);
      }
    }

    .success-title {
      font-family: var(--font-display);
      font-size: var(--text-3xl);
      font-weight: var(--weight-bold);
      line-height: var(--leading-snug);
      margin-bottom: var(--space-3);
    }

    .success-subtitle {
      color: var(--text-secondary);
      font-size: var(--text-base);
      line-height: var(--leading-relaxed);
      margin-bottom: var(--space-8);
    }

    .success-cta {
      display: inline-flex;
      align-items: center;
      gap: var(--space-2-5);
      padding: var(--space-3-5) var(--space-7);
      background: var(--gradient-brand);
      color: var(--bg-primary);
      text-decoration: none;
      font-size: var(--text-base);
      font-weight: var(--weight-semibold);
      line-height: var(--leading-normal);
      border-radius: var(--radius-lg);
      transition: all var(--duration-normal) var(--ease-out);

      svg {
        width: var(--icon-sm);
        height: var(--icon-sm);
      }

      &:hover {
        transform: translateY(-2px);
        box-shadow: var(--shadow-glow-orange);
      }
    }

    /* Progress track */
    .progress-track {
      position: fixed;
      bottom: var(--space-10);
      left: 50%;
      transform: translateX(-50%);
      width: min(600px, 90vw);
      height: var(--space-1);
      background: var(--bg-elevated);
      border-radius: var(--radius-sm);
      z-index: var(--z-fixed);
      opacity: 0;
      pointer-events: none;
      transition: opacity var(--duration-normal) var(--ease-out);

      &.visible {
        opacity: 1;
      }
    }

    .progress-fill {
      height: 100%;
      background: var(--gradient-brand);
      border-radius: var(--radius-sm);
      transition: width 0.1s linear;
    }

    .progress-markers {
      position: absolute;
      top: calc(var(--space-8) * -1);
      left: 0;
      right: 0;
      display: flex;
      justify-content: space-between;
    }

    .progress-marker {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: var(--space-2);
    }

    .marker-dot {
      width: var(--space-3);
      height: var(--space-3);
      background: var(--bg-elevated);
      border: 2px solid var(--border-subtle);
      border-radius: var(--radius-full);
      transition: all var(--duration-normal) var(--ease-out);
    }

    .progress-marker.active .marker-dot {
      background: var(--accent-cyan);
      border-color: var(--accent-cyan);
      box-shadow: var(--shadow-glow-orange);
    }

    .marker-label {
      font-family: var(--font-mono);
      font-size: var(--text-xs);
      color: var(--text-muted);
      text-transform: uppercase;
      letter-spacing: var(--tracking-wider);
      line-height: var(--leading-normal);
      white-space: nowrap;

      @media (max-width: 640px) {
        display: none;
      }
    }

    @media (max-width: 768px) {
      .hiw-header {
        padding: var(--space-16) var(--container-px) var(--space-6);
      }

      .section-number {
        font-size: var(--text-6xl);
      }

      .step-panel {
        width: 85vw;
        min-height: 450px;
        padding: var(--space-6);
      }

      .panel-bg-number {
        font-size: 180px;
      }
    }
  `],
})
export class HowItWorksComponent implements OnInit, OnDestroy, AfterViewInit {
  @ViewChild('container') containerRef!: ElementRef<HTMLElement>;
  @ViewChild('panelsContainer') panelsContainerRef!: ElementRef<HTMLElement>;
  @ViewChild('panelsTrack') panelsTrackRef!: ElementRef<HTMLElement>;

  private el = inject(ElementRef);
  private ngZone = inject(NgZone);
  private scrollTrigger: ScrollTrigger | null = null;
  private tween: gsap.core.Tween | null = null;

  activeStep = signal(-1);
  progressPercent = signal(0);

  steps: Step[] = [
    {
      number: 1,
      title: 'Install',
      description: 'One command. No dependencies. No containers. Just a single Rust binary.',
      icon: 'terminal',
      code: 'cargo install mcp-guard',
      brandColor: '#FF7A30',
    },
    {
      number: 2,
      title: 'Configure',
      description: 'Generate a config file with sensible defaults. Customize as needed.',
      icon: 'config',
      code: 'mcp-guard init',
      brandColor: '#ff4d00',
    },
    {
      number: 3,
      title: 'Run',
      description: 'Start the gateway. It reads your config and secures your server instantly.',
      icon: 'rocket',
      code: 'mcp-guard run',
      brandColor: '#465C88',
    },
  ];

  ngOnInit() { }

  ngAfterViewInit() {
    this.initHorizontalScroll();
  }

  ngOnDestroy() {
    this.scrollTrigger?.kill();
    this.tween?.kill();
  }

  private initHorizontalScroll() {
    this.ngZone.runOutsideAngular(() => {
      const container = this.panelsContainerRef.nativeElement;
      const track = this.panelsTrackRef.nativeElement;
      const panels = track.querySelectorAll('.step-panel');
      const progressTrack = this.el.nativeElement.querySelector('.progress-track');

      // Calculate total scroll distance
      const totalWidth = track.scrollWidth - container.offsetWidth;

      // Create the horizontal scroll animation
      this.tween = gsap.to(track, {
        x: -totalWidth,
        ease: 'none',
      });

      // Create ScrollTrigger
      this.scrollTrigger = ScrollTrigger.create({
        trigger: container,
        start: 'top top',
        end: () => `+=${totalWidth}`,
        pin: true,
        animation: this.tween,
        scrub: 1,
        onUpdate: (self) => {
          this.ngZone.run(() => {
            const progress = self.progress;
            this.progressPercent.set(progress * 100);

            // Determine active step based on progress
            const stepProgress = progress * (panels.length);
            this.activeStep.set(Math.floor(stepProgress));
          });
        },
        onEnter: () => {
          progressTrack?.classList.add('visible');
        },
        onLeave: () => {
          progressTrack?.classList.remove('visible');
        },
        onEnterBack: () => {
          progressTrack?.classList.add('visible');
        },
        onLeaveBack: () => {
          progressTrack?.classList.remove('visible');
        },
      });
    });
  }
}
