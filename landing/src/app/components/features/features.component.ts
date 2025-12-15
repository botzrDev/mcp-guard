import { Component, signal, ChangeDetectionStrategy, OnInit, OnDestroy, inject, NgZone, ElementRef, AfterViewInit, ViewChild } from '@angular/core';
import { CommonModule } from '@angular/common';
import { IconComponent, IconName } from '../../shared/icon/icon.component';

interface Feature {
  id: string;
  title: string;
  description: string;
  icon: IconName;
  tag: string;
  code?: string;
}

@Component({
  selector: 'app-features',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, IconComponent],
  template: `
    <section class="features" id="features">
      <div class="features-container">
        <!-- Header -->
        <div class="section-header">
          <div class="header-tag">
            <span class="tag-dot"></span>
            <span class="tag-text">Features</span>
          </div>
          <h2 class="section-title">Built for security teams.</h2>
        </div>

        <!-- Horizontal scroll track -->
        <div class="cards-viewport" #viewport>
          <div class="cards-track" #track>
            @for (feature of features; track feature.id; let i = $index) {
              <div 
                class="feature-card" 
                [class.active]="activeIndex() === i"
                (click)="setActive(i)"
                (mouseenter)="setActive(i)"
              >
                <div class="card-inner">
                  <!-- Collapsed state -->
                  <div class="card-collapsed">
                    <div class="card-icon">
                      <app-icon [name]="feature.icon" />
                    </div>
                    <span class="card-number">0{{ i + 1 }}</span>
                  </div>

                  <!-- Expanded state -->
                  <div class="card-expanded">
                    <div class="expanded-header">
                      <div class="card-icon">
                        <app-icon [name]="feature.icon" />
                      </div>
                      <span class="card-tag">{{ feature.tag }}</span>
                    </div>
                    <h3 class="card-title">{{ feature.title }}</h3>
                    <p class="card-description">{{ feature.description }}</p>
                    @if (feature.code) {
                      <div class="card-code">
                        <pre><code>{{ feature.code }}</code></pre>
                      </div>
                    }
                  </div>
                </div>
              </div>
            }
          </div>
        </div>

        <!-- Scroll indicators -->
        <div class="scroll-controls">
          <div class="scroll-dots">
            @for (feature of features; track feature.id; let i = $index) {
              <button 
                class="scroll-dot" 
                [class.active]="activeIndex() === i"
                (click)="scrollToCard(i)"
              ></button>
            }
          </div>
          <div class="scroll-hint">
            <span>Drag to explore</span>
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M5 12h14M12 5l7 7-7 7"/>
            </svg>
          </div>
        </div>
      </div>
    </section>
  `,
  styles: [`
    .features {
      position: relative;
      padding: var(--section-py-sm) 0;
      background: var(--bg-secondary);
      overflow: hidden;
    }

    .features-container {
      max-width: 1400px;
      margin: 0 auto;
    }

    /* Header */
    .section-header {
      padding: 0 var(--container-px);
      margin-bottom: var(--space-12);
    }

    .header-tag {
      display: inline-flex;
      align-items: center;
      gap: var(--space-2);
      margin-bottom: var(--space-3);
    }

    .tag-dot {
      width: var(--space-2);
      height: var(--space-2);
      background: var(--accent-cyan);
      border-radius: var(--radius-full);
    }

    .tag-text {
      font-family: var(--font-mono);
      font-size: var(--text-sm);
      color: var(--text-muted);
      text-transform: uppercase;
      letter-spacing: var(--tracking-wider);
      line-height: var(--leading-normal);
    }

    .section-title {
      font-family: var(--font-display);
      font-size: var(--text-3xl);
      font-weight: var(--weight-bold);
      letter-spacing: var(--tracking-tight);
      line-height: var(--leading-snug);
    }

    /* Cards viewport */
    .cards-viewport {
      overflow-x: auto;
      overflow-y: hidden;
      padding: var(--space-5) 0 var(--space-10);
      cursor: grab;
      scrollbar-width: none;
      -ms-overflow-style: none;

      &::-webkit-scrollbar {
        display: none;
      }

      &:active {
        cursor: grabbing;
      }
    }

    .cards-track {
      display: flex;
      gap: var(--space-4);
      padding: 0 var(--container-px);
      width: max-content;
    }

    /* Feature cards */
    .feature-card {
      flex-shrink: 0;
      width: 120px;
      height: 400px;
      background: var(--bg-primary);
      border: 1px solid var(--border-subtle);
      border-radius: var(--radius-2xl);
      cursor: pointer;
      transition: width var(--duration-slow) var(--ease-in-out),
                  border-color var(--duration-fast) var(--ease-out),
                  box-shadow var(--duration-fast) var(--ease-out);
      overflow: hidden;

      &:hover {
        border-color: var(--border-accent);
      }

      &.active {
        width: 380px;
        border-color: var(--border-accent);
        box-shadow: var(--shadow-glow-orange);

        .card-collapsed {
          opacity: 0;
          pointer-events: none;
        }

        .card-expanded {
          opacity: 1;
          pointer-events: auto;
        }
      }

      @media (max-width: 600px) {
        width: 100px;
        height: 320px;

        &.active {
          width: 300px;
        }
      }
    }

    .card-inner {
      position: relative;
      width: 100%;
      height: 100%;
    }

    /* Collapsed state */
    .card-collapsed {
      position: absolute;
      inset: 0;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: space-between;
      padding: var(--space-6) var(--space-4);
      opacity: 1;
      transition: opacity var(--duration-normal) var(--ease-out);
    }

    .card-icon {
      width: var(--space-11);
      height: var(--space-11);
      background: var(--bg-elevated);
      border-radius: var(--radius-xl);
      display: flex;
      align-items: center;
      justify-content: center;
      color: var(--accent-cyan);

      :host ::ng-deep svg {
        width: var(--icon-lg);
        height: var(--icon-lg);
      }
    }

    .card-number {
      font-family: var(--font-mono);
      font-size: var(--text-sm);
      color: var(--text-muted);
      line-height: var(--leading-normal);
      writing-mode: vertical-rl;
      text-orientation: mixed;
      transform: rotate(180deg);
    }

    /* Expanded state */
    .card-expanded {
      position: absolute;
      inset: 0;
      padding: var(--space-7);
      display: flex;
      flex-direction: column;
      opacity: 0;
      pointer-events: none;
      transition: opacity var(--duration-normal) var(--ease-out) 0.1s;
    }

    .expanded-header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      margin-bottom: var(--space-5);
    }

    .card-tag {
      font-family: var(--font-mono);
      font-size: var(--text-xs);
      color: var(--accent-cyan);
      background: rgba(255, 122, 48, 0.08);
      padding: var(--space-1) var(--space-2-5);
      border-radius: var(--radius-md);
      line-height: var(--leading-normal);
    }

    .card-title {
      font-family: var(--font-display);
      font-size: var(--text-xl);
      font-weight: var(--weight-bold);
      letter-spacing: var(--tracking-tight);
      line-height: var(--leading-snug);
      margin-bottom: var(--space-2-5);
    }

    .card-description {
      font-size: var(--text-sm);
      color: var(--text-secondary);
      line-height: var(--leading-relaxed);
      flex: 1;
    }

    .card-code {
      margin-top: var(--space-4);
      background: var(--bg-secondary);
      border: 1px solid var(--border-subtle);
      border-radius: var(--radius-lg);
      padding: var(--space-3);
      overflow-x: auto;

      pre {
        margin: 0;
        font-family: var(--font-mono);
        font-size: var(--text-xs);
        line-height: var(--leading-relaxed);
        color: var(--text-secondary);
      }
    }

    /* Scroll controls */
    .scroll-controls {
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 0 var(--container-px);
    }

    .scroll-dots {
      display: flex;
      gap: var(--space-2);
    }

    .scroll-dot {
      width: var(--space-2);
      height: var(--space-2);
      background: var(--bg-hover);
      border: none;
      border-radius: var(--radius-full);
      cursor: pointer;
      transition: all var(--duration-fast) var(--ease-out);
      padding: 0;

      &:hover {
        background: var(--text-muted);
      }

      &.active {
        width: var(--space-6);
        border-radius: var(--radius-sm);
        background: var(--accent-cyan);
      }
    }

    .scroll-hint {
      display: flex;
      align-items: center;
      gap: var(--space-2);
      color: var(--text-muted);
      font-size: var(--text-xs);
      line-height: var(--leading-normal);

      svg {
        width: var(--icon-sm);
        height: var(--icon-sm);
      }

      @media (max-width: 600px) {
        display: none;
      }
    }

    @media (max-width: 768px) {
      .features {
        padding: var(--space-20) 0;
      }

      .section-header {
        margin-bottom: var(--space-8);
      }
    }
  `]
})
export class FeaturesComponent implements OnInit, OnDestroy, AfterViewInit {
  @ViewChild('viewport') viewportRef!: ElementRef<HTMLElement>;
  @ViewChild('track') trackRef!: ElementRef<HTMLElement>;

  private ngZone = inject(NgZone);

  activeIndex = signal(0);

  features: Feature[] = [
    {
      id: 'auth',
      title: 'Multi-Provider Auth',
      description: 'API keys for simplicity. JWT for scale. OAuth 2.1 with PKCE for enterprise SSO.',
      icon: 'auth',
      tag: 'OAuth 2.1 • JWT • API Keys',
      code: '[auth]\nproviders = ["api_key", "jwt", "oauth"]'
    },
    {
      id: 'authz',
      title: 'Tool Authorization',
      description: 'Define exactly which users can access which MCP tools with fine-grained ACLs.',
      icon: 'authz',
      tag: 'Per-tool ACLs'
    },
    {
      id: 'rate',
      title: 'Rate Limiting',
      description: 'Token bucket algorithm with configurable per-user limits and burst allowance.',
      icon: 'rate',
      tag: 'Token Bucket'
    },
    {
      id: 'audit',
      title: 'Audit Logging',
      description: 'Every request logged with automatic secret redaction. SOC 2 ready.',
      icon: 'audit',
      tag: 'SOC 2 Ready'
    },
    {
      id: 'metrics',
      title: 'Prometheus Metrics',
      description: 'Built-in /metrics endpoint for seamless observability integration.',
      icon: 'metrics',
      tag: 'Grafana Ready'
    },
    {
      id: 'binary',
      title: 'Zero Infrastructure',
      description: 'Single static binary. No Docker, no databases, no external dependencies.',
      icon: 'binary',
      tag: 'Pure Rust'
    }
  ];

  ngOnInit() { }

  ngAfterViewInit() {
    this.initDragScroll();
  }

  ngOnDestroy() { }

  setActive(index: number) {
    this.activeIndex.set(index);
  }

  scrollToCard(index: number) {
    this.activeIndex.set(index);
    const viewport = this.viewportRef.nativeElement;
    const cards = this.trackRef.nativeElement.querySelectorAll('.feature-card');
    const card = cards[index] as HTMLElement;

    if (card) {
      const scrollLeft = card.offsetLeft - 24; // Account for padding
      viewport.scrollTo({ left: scrollLeft, behavior: 'smooth' });
    }
  }

  private initDragScroll() {
    const viewport = this.viewportRef.nativeElement;
    let isDown = false;
    let startX: number;
    let scrollLeft: number;

    this.ngZone.runOutsideAngular(() => {
      viewport.addEventListener('mousedown', (e) => {
        isDown = true;
        startX = e.pageX - viewport.offsetLeft;
        scrollLeft = viewport.scrollLeft;
      });

      viewport.addEventListener('mouseleave', () => {
        isDown = false;
      });

      viewport.addEventListener('mouseup', () => {
        isDown = false;
      });

      viewport.addEventListener('mousemove', (e) => {
        if (!isDown) return;
        e.preventDefault();
        const x = e.pageX - viewport.offsetLeft;
        const walk = (x - startX) * 1.5;
        viewport.scrollLeft = scrollLeft - walk;
      });
    });
  }
}
