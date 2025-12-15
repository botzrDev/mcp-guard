import { Component, ChangeDetectionStrategy, OnInit, OnDestroy, AfterViewInit, ElementRef, inject, NgZone, signal, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';
import { gsap } from 'gsap';
import { ScrollTrigger } from 'gsap/ScrollTrigger';

gsap.registerPlugin(ScrollTrigger);

interface Stat {
  value: string;
  numericValue?: number;
  prefix?: string;
  suffix?: string;
  metric: string;
  label: string;
  color: string;
}

@Component({
  selector: 'app-stats',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule],
  template: `
    <section class="stats" #container>
      <!-- Massive branded background text -->
      <div class="stats-watermark">PERFORMANCE</div>
      
      <!-- Diagonal divider from hero -->
      <div class="diagonal-top"></div>
      
      <div class="stats-container">
        <!-- Off-grid section label -->
        <div class="section-label">
          <span class="label-number">01</span>
          <span class="label-line"></span>
          <span class="label-text">// Benchmarks</span>
        </div>

        <!-- Unconventional asymmetric grid -->
        <div class="stats-grid">
          @for (stat of stats; track stat.metric; let i = $index) {
            <div 
              class="stat-block" 
              [class.featured]="i === 0"
              [style.--stat-color]="stat.color"
              [attr.data-index]="i"
            >
              <!-- Decorative corner -->
              <div class="stat-corner"></div>
              
              <!-- Animated value -->
              <div class="stat-value-wrapper">
                <span class="stat-value" [attr.data-final]="stat.value" #statValue>
                  {{ stat.prefix || '' }}{{ animatedValues()[i] }}{{ stat.suffix || '' }}
                </span>
                <div class="value-underline"></div>
              </div>
              
              <!-- Metric name -->
              <div class="stat-metric">
                <span class="metric-flag">{{ stat.metric }}</span>
              </div>
              
              <!-- Label -->
              <p class="stat-label">{{ stat.label }}</p>
              
              <!-- Background glyph -->
              <div class="stat-glyph">{{ i + 1 }}</div>
            </div>
          }
        </div>

        <!-- Bottom connector line -->
        <div class="connector-visual">
          <svg viewBox="0 0 1200 60" preserveAspectRatio="none">
            <path 
              class="connector-path"
              d="M0,30 Q300,10 600,30 T1200,30" 
              fill="none" 
              stroke="url(#statsGradient)" 
              stroke-width="2"
              stroke-dasharray="8 4"
            />
            <defs>
              <linearGradient id="statsGradient" x1="0%" y1="0%" x2="100%" y2="0%">
                <stop offset="0%" stop-color="#FF7A30"/>
                <stop offset="50%" stop-color="#ff4d00"/>
                <stop offset="100%" stop-color="#465C88"/>
              </linearGradient>
            </defs>
          </svg>
        </div>
      </div>
      
      <!-- Diagonal divider to next section -->
      <div class="diagonal-bottom"></div>
    </section>
  `,
  styles: [`
    .stats {
      position: relative;
      padding: 100px 0 120px;
      background: var(--bg-primary);
      overflow: hidden;
    }

    /* Subtle watermark */
    .stats-watermark {
      position: absolute;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      font-family: var(--font-display);
      font-size: clamp(80px, 15vw, 200px);
      font-weight: 800;
      color: transparent;
      -webkit-text-stroke: 1px rgba(255, 122, 48, 0.02);
      white-space: nowrap;
      letter-spacing: -0.05em;
      pointer-events: none;
      user-select: none;
    }

    /* Diagonal dividers - softer angle */
    .diagonal-top {
      position: absolute;
      top: -2px;
      left: -5%;
      right: -5%;
      height: 60px;
      background: var(--bg-secondary);
      transform: skewY(-1deg);
      transform-origin: top left;
    }

    .diagonal-bottom {
      position: absolute;
      bottom: -2px;
      left: -5%;
      right: -5%;
      height: 60px;
      background: var(--bg-secondary);
      transform: skewY(1deg);
      transform-origin: bottom right;
    }

    .stats-container {
      position: relative;
      max-width: 1400px;
      margin: 0 auto;
      padding: 0 24px;
      z-index: 1;
    }

    .section-label {
      display: flex;
      align-items: center;
      gap: 16px;
      margin-bottom: 48px;
    }

    .label-number {
      font-family: var(--font-mono);
      font-size: 14px;
      font-weight: 700;
      color: var(--accent-cyan);
      background: rgba(255, 122, 48, 0.1);
      padding: 8px 16px;
      border-radius: 6px;
      border: 1px solid rgba(255, 122, 48, 0.2);
    }

    .label-line {
      width: 60px;
      height: 2px;
      background: var(--gradient-brand);
    }

    .label-text {
      font-family: var(--font-mono);
      font-size: 13px;
      color: var(--text-muted);
      letter-spacing: 0.05em;
    }

    /* Asymmetric grid - first item is larger */
    .stats-grid {
      display: grid;
      grid-template-columns: 1.5fr 1fr 1fr 1fr;
      gap: 24px;

      @media (max-width: 1024px) {
        grid-template-columns: 1fr 1fr;
      }

      @media (max-width: 600px) {
        grid-template-columns: 1fr;
      }
    }

    .stat-block {
      position: relative;
      background: var(--bg-secondary);
      border: 1px solid var(--border-subtle);
      border-radius: 16px;
      padding: 28px;
      overflow: hidden;
      transition: transform 0.25s ease, border-color 0.25s ease, box-shadow 0.25s ease;

      &:hover {
        transform: translateY(-4px);
        border-color: var(--stat-color, var(--accent-cyan));
        box-shadow: 0 16px 40px -16px var(--stat-color, var(--border-accent));

        .stat-corner {
          transform: scale(1.2);
        }

        .value-underline {
          transform: scaleX(1);
        }
      }

      &.featured {
        grid-row: span 1;
        background: linear-gradient(135deg, var(--bg-secondary) 0%, rgba(255, 122, 48, 0.05) 100%);
        border-color: rgba(255, 122, 48, 0.2);

        .stat-value {
          font-size: clamp(64px, 10vw, 96px);
        }

        @media (max-width: 1024px) {
          grid-column: span 2;
        }

        @media (max-width: 600px) {
          grid-column: span 1;
        }
      }
    }

    .stat-corner {
      position: absolute;
      top: 0;
      right: 0;
      width: 80px;
      height: 80px;
      background: linear-gradient(135deg, transparent 50%, var(--stat-color, rgba(255, 122, 48, 0.1)) 50%);
      transition: transform 0.4s;
    }

    .stat-value-wrapper {
      position: relative;
      display: inline-block;
      margin-bottom: 16px;
    }

    .stat-value {
      font-family: var(--font-display);
      font-size: clamp(48px, 8vw, 64px);
      font-weight: 800;
      letter-spacing: -0.03em;
      line-height: 1;
      background: var(--gradient-brand);
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
      background-clip: text;
    }

    .value-underline {
      position: absolute;
      bottom: -4px;
      left: 0;
      width: 100%;
      height: 4px;
      background: var(--gradient-brand);
      border-radius: 2px;
      transform: scaleX(0);
      transform-origin: left;
      transition: transform 0.4s;
    }

    .stat-metric {
      margin-bottom: 12px;
    }

    .metric-flag {
      display: inline-block;
      font-family: var(--font-mono);
      font-size: 11px;
      font-weight: 600;
      color: var(--stat-color, var(--accent-cyan));
      background: rgba(255, 122, 48, 0.1);
      padding: 6px 12px;
      border-radius: 4px;
      letter-spacing: 0.05em;
    }

    .stat-label {
      color: var(--text-secondary);
      font-size: 14px;
      line-height: 1.5;
    }

    .stat-glyph {
      position: absolute;
      bottom: -20px;
      right: 10px;
      font-family: var(--font-display);
      font-size: 160px;
      font-weight: 800;
      color: transparent;
      -webkit-text-stroke: 1px var(--border-subtle);
      opacity: 0.3;
      line-height: 1;
      pointer-events: none;
      user-select: none;
    }

    .connector-visual {
      margin-top: 64px;
      overflow: visible;

      svg {
        width: 100%;
        height: 60px;
      }

      .connector-path {
        stroke-dashoffset: 1200;
        animation: drawPath 2s ease forwards;
      }

      @media (max-width: 768px) {
        display: none;
      }
    }

    @keyframes drawPath {
      to {
        stroke-dashoffset: 0;
      }
    }

    @media (max-width: 768px) {
      .stats {
        padding: 100px 0 120px;
      }

      .diagonal-top,
      .diagonal-bottom {
        height: 40px;
      }

      .stat-block {
        padding: 24px;
      }

      .stat-glyph {
        font-size: 100px;
      }
    }
  `]
})
export class StatsComponent implements OnInit, OnDestroy, AfterViewInit {
  @ViewChild('container') containerRef!: ElementRef<HTMLElement>;

  private ngZone = inject(NgZone);
  private el = inject(ElementRef);
  private scrollTrigger: ScrollTrigger | null = null;
  private counterAnimations: gsap.core.Tween[] = [];

  animatedValues = signal<string[]>(['0', '0', '0', '0']);

  stats: Stat[] = [
    {
      value: '<2ms',
      numericValue: 2,
      prefix: '<',
      suffix: 'ms',
      metric: '--latency',
      label: 'P99 latency overhead. Your users won\'t notice we\'re there.',
      color: '#FF7A30'
    },
    {
      value: '5,000+',
      numericValue: 5000,
      suffix: '+',
      metric: '--throughput',
      label: 'Requests per second on commodity hardware.',
      color: '#ff4d00'
    },
    {
      value: '<15MB',
      numericValue: 15,
      prefix: '<',
      suffix: 'MB',
      metric: '--binary',
      label: 'Total binary size. No bloat, no containers.',
      color: '#465C88'
    },
    {
      value: 'Zero',
      metric: '--services',
      label: 'External services required. No databases, no Redis, no Docker.',
      color: '#4ade80'
    },
  ];

  ngOnInit() { }

  ngAfterViewInit() {
    this.initCounterAnimations();
  }

  ngOnDestroy() {
    this.scrollTrigger?.kill();
    this.counterAnimations.forEach(tween => tween.kill());
  }

  private initCounterAnimations() {
    this.ngZone.runOutsideAngular(() => {
      const statBlocks = this.el.nativeElement.querySelectorAll('.stat-block');

      this.scrollTrigger = ScrollTrigger.create({
        trigger: this.containerRef.nativeElement,
        start: 'top 70%',
        once: true,
        onEnter: () => {
          this.animateCounters();
        }
      });
    });
  }

  private animateCounters() {
    this.stats.forEach((stat, index) => {
      if (stat.numericValue !== undefined) {
        const obj = { value: 0 };
        const tween = gsap.to(obj, {
          value: stat.numericValue,
          duration: 2,
          delay: index * 0.15,
          ease: 'power2.out',
          onUpdate: () => {
            this.ngZone.run(() => {
              const newValues = [...this.animatedValues()];
              const displayValue = Math.round(obj.value);
              newValues[index] = displayValue.toLocaleString();
              this.animatedValues.set(newValues);
            });
          }
        });
        this.counterAnimations.push(tween);
      } else {
        // For non-numeric values like "Zero", just reveal after delay
        setTimeout(() => {
          this.ngZone.run(() => {
            const newValues = [...this.animatedValues()];
            newValues[index] = stat.value;
            this.animatedValues.set(newValues);
          });
        }, index * 150 + 500);
      }
    });
  }
}
