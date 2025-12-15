import { Component, signal, ChangeDetectionStrategy, OnInit, OnDestroy, inject, NgZone, ElementRef, QueryList, ViewChildren } from '@angular/core';
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
    <section class="features" id="features">
      <div class="features-container">
        <div class="section-header">
          <span class="section-tag">// Features</span>
          <h2 class="section-title">Everything you need to <span class="gradient-text">secure MCP</span></h2>
          <p class="section-subtitle">
            Production-grade security without the infrastructure complexity.
            Drop-in protection for any MCP server.
          </p>
        </div>

        <!-- Bento Grid -->
        <div class="bento-grid">
          @for (feature of features; track feature.id; let i = $index) {
            <div
              class="bento-card"
              [class.large]="feature.size === 'large'"
              [class.is-visible]="visibleCards().has(i)"
              [attr.data-feature]="feature.id"
              [attr.data-index]="i"
            >
              <div class="card-header">
                <div class="card-icon">
                  <app-icon [name]="feature.icon" />
                </div>
                <span class="card-tag">{{ feature.tag }}</span>
              </div>

              <h3 class="card-title">{{ feature.title }}</h3>
              <p class="card-description">{{ feature.description }}</p>

              @if (feature.code) {
                <div class="card-code">
                  <pre><code [innerHTML]="highlightCode(feature.code)"></code></pre>
                </div>
              }

              <!-- Visual element for specific cards -->
              @if (feature.id === 'binary') {
                <div class="binary-visual">
                  <div class="size-bar">
                    <div class="size-fill" style="width: 15%">
                      <span class="size-label">mcp-guard</span>
                      <span class="size-value">14MB</span>
                    </div>
                  </div>
                  <div class="size-bar comparison">
                    <div class="size-fill full">
                      <span class="size-label">Docker image</span>
                      <span class="size-value">~500MB</span>
                    </div>
                  </div>
                </div>
              }

              @if (feature.id === 'metrics') {
                <div class="metrics-visual">
                  <div class="metric-row">
                    <span class="metric-name">requests_total</span>
                    <span class="metric-value">{{ animatedCount() }}</span>
                  </div>
                  <div class="metric-row">
                    <span class="metric-name">auth_success</span>
                    <span class="metric-value">99.8%</span>
                  </div>
                  <div class="metric-row">
                    <span class="metric-name">p99_latency_ms</span>
                    <span class="metric-value">1.2</span>
                  </div>
                </div>
              }

              <div class="card-glow"></div>
            </div>
          }
        </div>
      </div>
    </section>
  `,
  styles: [`
    .features {
      position: relative;
      padding: 120px 0;
      background: var(--bg-primary);

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

    .features-container {
      max-width: 1280px;
      margin: 0 auto;
      padding: 0 24px;
    }

    .section-header {
      text-align: center;
      margin-bottom: 64px;
    }

    .section-tag {
      font-family: var(--font-mono);
      font-size: 13px;
      color: var(--accent-cyan);
      letter-spacing: 0.05em;
      margin-bottom: 16px;
      display: block;
    }

    .section-title {
      font-family: var(--font-display);
      font-size: clamp(32px, 5vw, 52px);
      font-weight: 700;
      letter-spacing: -0.02em;
      margin-bottom: 16px;
    }

    .gradient-text {
      background: var(--gradient-brand);
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
      background-clip: text;
    }

    .section-subtitle {
      font-size: 18px;
      color: var(--text-secondary);
      max-width: 600px;
      margin: 0 auto;
      line-height: 1.6;
    }

    .bento-grid {
      display: grid;
      grid-template-columns: repeat(3, 1fr);
      grid-auto-rows: minmax(200px, auto);
      gap: 20px;

      @media (max-width: 1024px) {
        grid-template-columns: repeat(2, 1fr);
      }

      @media (max-width: 640px) {
        grid-template-columns: 1fr;
      }
    }

    .bento-card {
      position: relative;
      background: var(--bg-secondary);
      border: 1px solid var(--border-subtle);
      border-radius: 20px;
      padding: 28px;
      overflow: hidden;
      transition: all 0.4s ease, opacity 0.6s ease, transform 0.6s ease;
      opacity: 0;
      transform: translateY(30px);

      // Staggered animation delays
      &[data-index="0"] { transition-delay: 0s; }
      &[data-index="1"] { transition-delay: 0.1s; }
      &[data-index="2"] { transition-delay: 0.2s; }
      &[data-index="3"] { transition-delay: 0.15s; }
      &[data-index="4"] { transition-delay: 0.25s; }
      &[data-index="5"] { transition-delay: 0.1s; }

      &.is-visible {
        opacity: 1;
        transform: translateY(0);
      }

      &::before {
        content: '';
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        height: 2px;
        background: var(--gradient-brand);
        opacity: 0;
        transition: opacity 0.3s;
      }

      &:hover {
        border-color: var(--border-accent);
        transform: translateY(-4px);

        &::before {
          opacity: 1;
        }

        .card-glow {
          opacity: 1;
        }
      }

      &.large {
        grid-column: span 2;

        @media (max-width: 640px) {
          grid-column: span 1;
        }
      }
    }

    .card-glow {
      position: absolute;
      top: 0;
      left: 0;
      right: 0;
      height: 150px;
      background: radial-gradient(ellipse at top, rgba(255, 122, 48, 0.1) 0%, transparent 70%);
      opacity: 0;
      transition: opacity 0.4s;
      pointer-events: none;
    }

    // Alternate blue glow for visual variety
    .bento-card[data-index="1"] .card-glow,
    .bento-card[data-index="3"] .card-glow {
      background: radial-gradient(ellipse at top, rgba(59, 130, 246, 0.08) 0%, transparent 70%);
    }

    .bento-card[data-index="1"]:hover,
    .bento-card[data-index="3"]:hover {
      border-color: var(--border-blue);
    }

    .card-header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      margin-bottom: 20px;
    }

    .card-icon {
      width: 48px;
      height: 48px;
      background: var(--bg-elevated);
      border-radius: 12px;
      display: flex;
      align-items: center;
      justify-content: center;
      color: var(--accent-cyan);

      :host ::ng-deep svg {
        width: 24px;
        height: 24px;
      }
    }

    .card-tag {
      font-family: var(--font-mono);
      font-size: 11px;
      color: var(--accent-cyan);
      background: rgba(255, 122, 48, 0.1);
      padding: 5px 10px;
      border-radius: 6px;
      letter-spacing: 0.02em;
    }

    .card-title {
      font-size: 20px;
      font-weight: 600;
      margin-bottom: 10px;
      letter-spacing: -0.01em;
    }

    .card-description {
      color: var(--text-secondary);
      font-size: 14px;
      line-height: 1.6;
    }

    .card-code {
      margin-top: 20px;
      background: var(--bg-primary);
      border: 1px solid var(--border-subtle);
      border-radius: 10px;
      padding: 16px;
      overflow-x: auto;

      pre {
        margin: 0;
        font-family: var(--font-mono);
        font-size: 12px;
        line-height: 1.7;
      }

      code {
        color: var(--text-secondary);

        :host ::ng-deep {
          .token-comment { color: #6a737d; font-style: italic; }
          .token-section { color: #3B82F6; font-weight: bold; }
          .token-key { color: #FF7A30; }
          .token-operator { color: var(--text-muted); }
          .token-string { color: #94a3b8; }
          .token-value { color: #E9E3DF; }
        }
      }
    }

    // Binary size visual
    .binary-visual {
      margin-top: 24px;
    }

    .size-bar {
      margin-bottom: 12px;

      &.comparison .size-fill {
        background: var(--bg-elevated);
        border-color: var(--border-subtle);

        .size-label, .size-value {
          color: var(--text-muted);
        }
      }
    }

    .size-fill {
      height: 40px;
      background: rgba(255, 122, 48, 0.15);
      border: 1px solid var(--border-accent);
      border-radius: 8px;
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 0 12px;
      transition: width 1s ease-out;

      &.full {
        width: 100% !important;
      }
    }

    .size-label {
      font-size: 12px;
      color: var(--text-primary);
    }

    .size-value {
      font-family: var(--font-mono);
      font-size: 12px;
      color: var(--accent-cyan);
    }

    // Metrics visual
    .metrics-visual {
      margin-top: 24px;
      background: var(--bg-primary);
      border: 1px solid var(--border-subtle);
      border-radius: 10px;
      padding: 16px;
    }

    .metric-row {
      display: flex;
      justify-content: space-between;
      padding: 8px 0;
      font-family: var(--font-mono);
      font-size: 12px;

      &:not(:last-child) {
        border-bottom: 1px solid var(--border-subtle);
      }
    }

    .metric-name {
      color: var(--text-muted);
    }

    .metric-value {
      color: var(--accent-cyan);
      font-weight: 600;
    }
  `]
})
export class FeaturesComponent implements OnInit, OnDestroy {
  private el = inject(ElementRef);
  private ngZone = inject(NgZone);
  private observer: IntersectionObserver | null = null;

  animatedCount = signal(12847);
  visibleCards = signal<Set<number>>(new Set());

  features: Feature[] = [
    {
      id: 'auth',
      title: 'Multi-Provider Authentication',
      description: 'API keys for simplicity. JWT for scale. OAuth 2.1 with PKCE for enterprise SSO. Use one or combine them all.',
      icon: 'auth',
      tag: 'OAuth 2.1 • JWT • API Keys',
      size: 'large',
      code: `[auth]
providers = ["api_key", "jwt", "oauth"]

[auth.oauth]
provider = "github"
client_id = "your_client_id"`
    },
    {
      id: 'authz',
      title: 'Tool-Level Authorization',
      description: 'Define exactly which users can access which MCP tools. Map OAuth scopes or JWT claims to granular permissions.',
      icon: 'authz',
      tag: 'Per-tool ACLs',
      size: 'small'
    },
    {
      id: 'rate',
      title: 'Per-Identity Rate Limiting',
      description: 'Token bucket algorithm with per-user limits. Custom rates per identity. Automatic Retry-After headers.',
      icon: 'rate',
      tag: 'Token Bucket',
      size: 'small'
    },
    {
      id: 'audit',
      title: 'Audit Logging',
      description: 'Every request logged with identity, tool, timestamp, and outcome. Automatic secret redaction. Export-ready for compliance.',
      icon: 'audit',
      tag: 'SOC 2 Ready',
      size: 'small'
    },
    {
      id: 'metrics',
      title: 'Prometheus Metrics',
      description: 'Built-in /metrics endpoint with request counts, latency histograms, auth outcomes, and rate limit tracking.',
      icon: 'metrics',
      tag: 'Grafana Compatible',
      size: 'small'
    },
    {
      id: 'binary',
      title: 'Zero Infrastructure',
      description: 'Single static binary. No Docker, no Kubernetes, no databases. Compiles to WASM for edge deployment.',
      icon: 'binary',
      tag: 'Rust • WASM Ready',
      size: 'large'
    }
  ];

  private counterInterval: ReturnType<typeof setInterval> | null = null;

  highlightCode(code: string): string {
    return code
      // Comments (lines starting with #)
      .replace(/(#[^\n]*)/g, '<span class="token-comment">$1</span>')
      // Section headers [section]
      .replace(/(\[[^\]]+\])/g, '<span class="token-section">$1</span>')
      // Keys (word before =)
      .replace(/^(\s*)(\w+)(\s*=)/gm, '$1<span class="token-key">$2</span><span class="token-operator">$3</span>')
      // Strings in quotes
      .replace(/"([^"]*)"/g, '<span class="token-string">"$1"</span>');
  }

  ngOnInit() {
    // Animate the metrics counter
    this.counterInterval = setInterval(() => {
      this.animatedCount.update(v => v + Math.floor(Math.random() * 10));
    }, 2000);

    // Set up intersection observer for card animations
    this.ngZone.runOutsideAngular(() => {
      this.observer = new IntersectionObserver(
        (entries) => {
          entries.forEach((entry) => {
            if (entry.isIntersecting) {
              const index = parseInt(
                (entry.target as HTMLElement).dataset['index'] || '0',
                10
              );
              this.ngZone.run(() => {
                this.visibleCards.update((set) => {
                  const newSet = new Set(set);
                  newSet.add(index);
                  return newSet;
                });
              });
              this.observer?.unobserve(entry.target);
            }
          });
        },
        {
          threshold: 0.1,
          rootMargin: '0px 0px -50px 0px',
        }
      );

      // Observe all bento cards after view init
      setTimeout(() => {
        const cards = this.el.nativeElement.querySelectorAll('.bento-card');
        cards.forEach((card: Element) => this.observer?.observe(card));
      }, 100);
    });
  }

  ngOnDestroy() {
    this.observer?.disconnect();
    if (this.counterInterval) {
      clearInterval(this.counterInterval);
    }
  }
}
