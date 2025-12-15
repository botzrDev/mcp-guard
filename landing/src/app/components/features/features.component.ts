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
      padding: 100px 0;
      background: var(--bg-secondary);
      overflow: hidden;
    }

    .features-container {
      max-width: 1400px;
      margin: 0 auto;
    }

    /* Header */
    .section-header {
      padding: 0 24px;
      margin-bottom: 48px;
    }

    .header-tag {
      display: inline-flex;
      align-items: center;
      gap: 8px;
      margin-bottom: 12px;
    }

    .tag-dot {
      width: 8px;
      height: 8px;
      background: #FF7A30;
      border-radius: 50%;
    }

    .tag-text {
      font-family: var(--font-mono);
      font-size: 13px;
      color: var(--text-muted);
      text-transform: uppercase;
      letter-spacing: 0.05em;
    }

    .section-title {
      font-family: var(--font-display);
      font-size: clamp(28px, 4vw, 40px);
      font-weight: 700;
      letter-spacing: -0.02em;
    }

    /* Cards viewport */
    .cards-viewport {
      overflow-x: auto;
      overflow-y: hidden;
      padding: 20px 0 40px;
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
      gap: 16px;
      padding: 0 24px;
      width: max-content;
    }

    /* Feature cards */
    .feature-card {
      flex-shrink: 0;
      width: 120px;
      height: 400px;
      background: var(--bg-primary);
      border: 1px solid var(--border-subtle);
      border-radius: 16px;
      cursor: pointer;
      transition: width 0.4s cubic-bezier(0.4, 0, 0.2, 1), 
                  border-color 0.2s ease,
                  box-shadow 0.2s ease;
      overflow: hidden;

      &:hover {
        border-color: var(--border-accent);
      }

      &.active {
        width: 380px;
        border-color: rgba(255, 122, 48, 0.3);
        box-shadow: 0 8px 32px -8px rgba(255, 122, 48, 0.15);

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
      padding: 24px 16px;
      opacity: 1;
      transition: opacity 0.3s ease;
    }

    .card-icon {
      width: 44px;
      height: 44px;
      background: var(--bg-elevated);
      border-radius: 12px;
      display: flex;
      align-items: center;
      justify-content: center;
      color: #FF7A30;

      :host ::ng-deep svg {
        width: 22px;
        height: 22px;
      }
    }

    .card-number {
      font-family: var(--font-mono);
      font-size: 14px;
      color: var(--text-muted);
      writing-mode: vertical-rl;
      text-orientation: mixed;
      transform: rotate(180deg);
    }

    /* Expanded state */
    .card-expanded {
      position: absolute;
      inset: 0;
      padding: 28px;
      display: flex;
      flex-direction: column;
      opacity: 0;
      pointer-events: none;
      transition: opacity 0.3s ease 0.1s;
    }

    .expanded-header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      margin-bottom: 20px;
    }

    .card-tag {
      font-family: var(--font-mono);
      font-size: 11px;
      color: #FF7A30;
      background: rgba(255, 122, 48, 0.08);
      padding: 5px 10px;
      border-radius: 5px;
    }

    .card-title {
      font-family: var(--font-display);
      font-size: 20px;
      font-weight: 700;
      letter-spacing: -0.01em;
      margin-bottom: 10px;
    }

    .card-description {
      font-size: 14px;
      color: var(--text-secondary);
      line-height: 1.6;
      flex: 1;
    }

    .card-code {
      margin-top: 16px;
      background: var(--bg-secondary);
      border: 1px solid var(--border-subtle);
      border-radius: 8px;
      padding: 12px;
      overflow-x: auto;

      pre {
        margin: 0;
        font-family: var(--font-mono);
        font-size: 11px;
        line-height: 1.6;
        color: var(--text-secondary);
      }
    }

    /* Scroll controls */
    .scroll-controls {
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 0 24px;
    }

    .scroll-dots {
      display: flex;
      gap: 8px;
    }

    .scroll-dot {
      width: 8px;
      height: 8px;
      background: var(--bg-hover);
      border: none;
      border-radius: 50%;
      cursor: pointer;
      transition: all 0.2s ease;
      padding: 0;

      &:hover {
        background: var(--text-muted);
      }

      &.active {
        width: 24px;
        border-radius: 4px;
        background: #FF7A30;
      }
    }

    .scroll-hint {
      display: flex;
      align-items: center;
      gap: 8px;
      color: var(--text-muted);
      font-size: 12px;

      svg {
        width: 16px;
        height: 16px;
      }

      @media (max-width: 600px) {
        display: none;
      }
    }

    @media (max-width: 768px) {
      .features {
        padding: 80px 0;
      }

      .section-header {
        margin-bottom: 32px;
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
