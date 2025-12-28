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
          <h2 class="section-title">Everything you need. Nothing you don't.</h2>
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
      title: 'Authentication',
      description: 'API keys, OAuth 2.1, JWT, or OIDC. Pick what fits. Switch without code changes.',
      icon: 'auth',
      tag: 'Multi-Provider',
      code: '[auth]\nproviders = ["api_key", "jwt", "oauth"]'
    },
    {
      id: 'rate',
      title: 'Rate Limiting',
      description: 'Per-client, per-tool, or global. Your rules. Protect your upstream.',
      icon: 'rate',
      tag: 'Configurable'
    },
    {
      id: 'audit',
      title: 'Audit Logging',
      description: 'Every request. Every tool. JSON logs ready for your SIEM or compliance officer.',
      icon: 'audit',
      tag: 'Compliance Ready'
    },
    {
      id: 'authz',
      title: 'Tool Permissions',
      description: 'Control which clients access which tools. Principle of least privilege.',
      icon: 'authz',
      tag: 'Fine-Grained'
    },
    {
      id: 'metrics',
      title: 'Metrics',
      description: 'Prometheus-compatible. Grafana-ready dashboards. See what\'s happening.',
      icon: 'metrics',
      tag: 'Observable'
    },
    {
      id: 'binary',
      title: 'Any Transport',
      description: 'HTTP, SSE, stdio. Works with your existing MCP servers unchanged.',
      icon: 'binary',
      tag: 'Flexible'
    }
  ];

  // Store drag state and listener references for proper cleanup
  private dragState = {
    isDown: false,
    startX: 0,
    scrollLeft: 0
  };
  private dragListeners: {
    mousedown?: (e: MouseEvent) => void;
    mouseleave?: () => void;
    mouseup?: () => void;
    mousemove?: (e: MouseEvent) => void;
  } = {};

  ngOnInit() { }

  ngAfterViewInit() {
    this.initDragScroll();
  }

  ngOnDestroy() {
    // Clean up drag scroll listeners
    const viewport = this.viewportRef?.nativeElement;
    if (viewport && this.dragListeners.mousedown) {
      viewport.removeEventListener('mousedown', this.dragListeners.mousedown);
      viewport.removeEventListener('mouseleave', this.dragListeners.mouseleave!);
      viewport.removeEventListener('mouseup', this.dragListeners.mouseup!);
      viewport.removeEventListener('mousemove', this.dragListeners.mousemove!);
    }
  }

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

    // Create bound listener functions for proper cleanup
    this.dragListeners.mousedown = (e: MouseEvent) => {
      this.dragState.isDown = true;
      this.dragState.startX = e.pageX - viewport.offsetLeft;
      this.dragState.scrollLeft = viewport.scrollLeft;
      viewport.style.cursor = 'grabbing';
    };

    this.dragListeners.mouseleave = () => {
      this.dragState.isDown = false;
      viewport.style.cursor = 'grab';
    };

    this.dragListeners.mouseup = () => {
      this.dragState.isDown = false;
      viewport.style.cursor = 'grab';
    };

    this.dragListeners.mousemove = (e: MouseEvent) => {
      if (!this.dragState.isDown) return;
      e.preventDefault();
      const x = e.pageX - viewport.offsetLeft;
      const walk = (x - this.dragState.startX) * 1.5;
      viewport.scrollLeft = this.dragState.scrollLeft - walk;
    };

    this.ngZone.runOutsideAngular(() => {
      viewport.addEventListener('mousedown', this.dragListeners.mousedown!, { passive: true });
      viewport.addEventListener('mouseleave', this.dragListeners.mouseleave!, { passive: true });
      viewport.addEventListener('mouseup', this.dragListeners.mouseup!, { passive: true });
      // Mark as non-passive since we call preventDefault()
      viewport.addEventListener('mousemove', this.dragListeners.mousemove!, { passive: false });
    });

    // Set initial cursor style
    viewport.style.cursor = 'grab';
  }
}
