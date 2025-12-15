import { Component, signal, ChangeDetectionStrategy, OnInit, OnDestroy, inject, NgZone, AfterViewInit, ViewChild, ElementRef } from '@angular/core';
import { CommonModule } from '@angular/common';
import { IconComponent, IconName } from '../../shared/icon/icon.component';

interface Feature {
  id: string;
  title: string;
  description: string;
  icon: IconName;
  tag: string;
  size: 'large' | 'small';
  code?: string;
}

@Component({
  selector: 'app-features',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, IconComponent],
  template: `
    <section class="features" id="features" #container>
      <div class="features-container">
        <!-- Clean header -->
        <div class="section-header">
          <div class="header-tag">
            <span class="tag-decorator">//</span>
            <span class="tag-text">Features</span>
          </div>
          <h2 class="section-title">
            Everything you need.<br>
            <span class="title-gradient">Nothing you don't.</span>
          </h2>
          <p class="section-subtitle">
            Enterprise-grade security in a single binary. No containers, no databases, no complexity.
          </p>
        </div>

        <!-- Clean Bento Grid -->
        <div class="bento-grid">
          <!-- Main feature - spans 2 columns -->
          <div class="feature-card feature-main" [class.visible]="visibleCards().has(0)">
            <div class="card-accent"></div>
            <div class="card-header">
              <div class="card-icon">
                <app-icon name="auth" />
              </div>
              <span class="card-tag">OAuth 2.1 • JWT • API Keys</span>
            </div>
            <h3 class="card-title">Multi-Provider Authentication</h3>
            <p class="card-description">
              API keys for simplicity. JWT for scale. OAuth 2.1 with PKCE for enterprise SSO. 
              Use one or combine them all.
            </p>
            <div class="card-code">
              <pre><code><span class="code-section">[auth]</span>
<span class="code-key">providers</span> = <span class="code-value">["api_key", "jwt", "oauth"]</span>

<span class="code-section">[auth.oauth]</span>
<span class="code-key">provider</span> = <span class="code-value">"github"</span>
<span class="code-key">client_id</span> = <span class="code-value">"your_client_id"</span></code></pre>
            </div>
          </div>

          <!-- Secondary features - smaller cards -->
          @for (feature of secondaryFeatures; track feature.id; let i = $index) {
            <div 
              class="feature-card feature-small" 
              [class.visible]="visibleCards().has(i + 1)"
            >
              <div class="card-icon-small">
                <app-icon [name]="feature.icon" />
              </div>
              <h4 class="card-title-small">{{ feature.title }}</h4>
              <p class="card-description-small">{{ feature.description }}</p>
              <span class="card-tag-small">{{ feature.tag }}</span>
            </div>
          }
        </div>
      </div>
    </section>
  `,
  styles: [`
    .features {
      position: relative;
      padding: 100px 0 120px;
      background: var(--bg-secondary);
    }

    .features-container {
      max-width: 1200px;
      margin: 0 auto;
      padding: 0 24px;
    }

    /* Header */
    .section-header {
      text-align: center;
      margin-bottom: 64px;
    }

    .header-tag {
      display: inline-flex;
      align-items: center;
      gap: 8px;
      margin-bottom: 16px;
      padding: 8px 16px;
      background: var(--bg-primary);
      border: 1px solid var(--border-subtle);
      border-radius: 6px;
    }

    .tag-decorator {
      font-family: var(--font-mono);
      font-size: 14px;
      color: var(--accent-cyan);
      font-weight: 600;
    }

    .tag-text {
      font-family: var(--font-mono);
      font-size: 13px;
      color: var(--text-secondary);
    }

    .section-title {
      font-family: var(--font-display);
      font-size: clamp(32px, 5vw, 48px);
      font-weight: 700;
      letter-spacing: -0.02em;
      line-height: 1.2;
      margin-bottom: 16px;
    }

    .title-gradient {
      background: var(--gradient-brand);
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
      background-clip: text;
    }

    .section-subtitle {
      font-size: 17px;
      color: var(--text-secondary);
      max-width: 520px;
      margin: 0 auto;
      line-height: 1.6;
    }

    /* Bento Grid */
    .bento-grid {
      display: grid;
      grid-template-columns: repeat(3, 1fr);
      gap: 20px;

      @media (max-width: 900px) {
        grid-template-columns: repeat(2, 1fr);
      }

      @media (max-width: 600px) {
        grid-template-columns: 1fr;
      }
    }

    /* Main feature card */
    .feature-main {
      grid-column: span 2;
      grid-row: span 2;

      @media (max-width: 900px) {
        grid-column: span 2;
        grid-row: span 1;
      }

      @media (max-width: 600px) {
        grid-column: span 1;
      }
    }

    .feature-card {
      position: relative;
      background: var(--bg-primary);
      border: 1px solid var(--border-subtle);
      border-radius: 16px;
      padding: 28px;
      opacity: 0;
      transform: translateY(20px);
      transition: opacity 0.4s ease, transform 0.4s ease, border-color 0.2s ease, box-shadow 0.2s ease;

      &.visible {
        opacity: 1;
        transform: translateY(0);
      }

      &:hover {
        border-color: var(--border-accent);
        box-shadow: 0 8px 32px -8px rgba(255, 122, 48, 0.12);

        .card-accent {
          transform: scaleY(1);
        }
      }
    }

    .card-accent {
      position: absolute;
      top: 0;
      left: 0;
      width: 3px;
      height: 100%;
      background: var(--gradient-brand);
      border-radius: 16px 0 0 16px;
      transform: scaleY(0);
      transform-origin: top;
      transition: transform 0.25s ease;
    }

    /* Main card styles */
    .card-header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      margin-bottom: 20px;
      flex-wrap: wrap;
      gap: 12px;
    }

    .card-icon {
      width: 48px;
      height: 48px;
      background: var(--gradient-brand);
      border-radius: 12px;
      display: flex;
      align-items: center;
      justify-content: center;
      color: var(--bg-primary);

      :host ::ng-deep svg {
        width: 24px;
        height: 24px;
      }
    }

    .card-tag {
      font-family: var(--font-mono);
      font-size: 11px;
      color: var(--accent-cyan);
      background: rgba(255, 122, 48, 0.08);
      padding: 6px 12px;
      border-radius: 6px;
      letter-spacing: 0.02em;
    }

    .card-title {
      font-family: var(--font-display);
      font-size: 24px;
      font-weight: 700;
      letter-spacing: -0.01em;
      margin-bottom: 12px;
    }

    .card-description {
      font-size: 15px;
      color: var(--text-secondary);
      line-height: 1.6;
      margin-bottom: 24px;
    }

    .card-code {
      background: var(--bg-secondary);
      border: 1px solid var(--border-subtle);
      border-radius: 10px;
      padding: 16px;
      overflow-x: auto;

      pre {
        margin: 0;
        font-family: var(--font-mono);
        font-size: 13px;
        line-height: 1.7;
      }

      .code-section {
        color: var(--accent-slate);
        font-weight: 600;
      }

      .code-key {
        color: var(--accent-cyan);
      }

      .code-value {
        color: var(--text-secondary);
      }
    }

    /* Small card styles */
    .feature-small {
      display: flex;
      flex-direction: column;
    }

    .card-icon-small {
      width: 40px;
      height: 40px;
      background: var(--bg-elevated);
      border-radius: 10px;
      display: flex;
      align-items: center;
      justify-content: center;
      color: var(--accent-cyan);
      margin-bottom: 16px;

      :host ::ng-deep svg {
        width: 20px;
        height: 20px;
      }
    }

    .card-title-small {
      font-size: 17px;
      font-weight: 600;
      letter-spacing: -0.01em;
      margin-bottom: 8px;
    }

    .card-description-small {
      font-size: 14px;
      color: var(--text-secondary);
      line-height: 1.5;
      flex: 1;
      margin-bottom: 16px;
    }

    .card-tag-small {
      display: inline-block;
      font-family: var(--font-mono);
      font-size: 10px;
      color: var(--text-muted);
      background: var(--bg-elevated);
      padding: 5px 10px;
      border-radius: 5px;
      letter-spacing: 0.02em;
      align-self: flex-start;
    }

    @media (max-width: 600px) {
      .features {
        padding: 80px 0 100px;
      }

      .section-header {
        margin-bottom: 48px;
      }

      .feature-card {
        padding: 24px;
      }

      .card-title {
        font-size: 20px;
      }
    }
  `]
})
export class FeaturesComponent implements OnInit, OnDestroy, AfterViewInit {
  @ViewChild('container') containerRef!: ElementRef<HTMLElement>;

  private ngZone = inject(NgZone);
  private observer: IntersectionObserver | null = null;

  visibleCards = signal<Set<number>>(new Set());

  secondaryFeatures: Feature[] = [
    {
      id: 'authz',
      title: 'Tool-Level Authorization',
      description: 'Define exactly which users can access which MCP tools with fine-grained ACLs.',
      icon: 'authz',
      tag: 'Per-tool ACLs',
      size: 'small'
    },
    {
      id: 'rate',
      title: 'Per-Identity Rate Limiting',
      description: 'Token bucket algorithm with configurable per-user limits and burst allowance.',
      icon: 'rate',
      tag: 'Token Bucket',
      size: 'small'
    },
    {
      id: 'audit',
      title: 'Audit Logging',
      description: 'Every request logged with automatic secret redaction. SOC 2 ready out of the box.',
      icon: 'audit',
      tag: 'SOC 2 Ready',
      size: 'small'
    },
    {
      id: 'metrics',
      title: 'Prometheus Metrics',
      description: 'Built-in /metrics endpoint for seamless observability integration.',
      icon: 'metrics',
      tag: 'Grafana Compatible',
      size: 'small'
    },
    {
      id: 'binary',
      title: 'Zero Infrastructure',
      description: 'Single static binary. No Docker, no databases, no external dependencies.',
      icon: 'binary',
      tag: 'Pure Rust',
      size: 'small'
    }
  ];

  ngOnInit() { }

  ngAfterViewInit() {
    this.initObserver();
  }

  ngOnDestroy() {
    this.observer?.disconnect();
  }

  private initObserver() {
    this.ngZone.runOutsideAngular(() => {
      this.observer = new IntersectionObserver(
        (entries) => {
          entries.forEach((entry) => {
            if (entry.isIntersecting) {
              this.ngZone.run(() => {
                this.revealCards();
              });
              this.observer?.unobserve(entry.target);
            }
          });
        },
        { threshold: 0.15 }
      );

      this.observer.observe(this.containerRef.nativeElement);
    });
  }

  private revealCards() {
    const totalCards = this.secondaryFeatures.length + 1;
    for (let i = 0; i < totalCards; i++) {
      setTimeout(() => {
        this.visibleCards.update(set => {
          const newSet = new Set(set);
          newSet.add(i);
          return newSet;
        });
      }, i * 80);
    }
  }
}
