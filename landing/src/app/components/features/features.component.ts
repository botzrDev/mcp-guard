import { Component, signal } from '@angular/core';
import { CommonModule } from '@angular/common';

interface Feature {
  id: string;
  title: string;
  description: string;
  icon: string;
  tag: string;
  size: 'large' | 'small';
  code?: string;
}

@Component({
  selector: 'app-features',
  standalone: true,
  imports: [CommonModule],
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
          @for (feature of features; track feature.id) {
            <div
              class="bento-card"
              [class.large]="feature.size === 'large'"
              [attr.data-feature]="feature.id"
            >
              <div class="card-header">
                <div class="card-icon" [innerHTML]="feature.icon"></div>
                <span class="card-tag">{{ feature.tag }}</span>
              </div>

              <h3 class="card-title">{{ feature.title }}</h3>
              <p class="card-description">{{ feature.description }}</p>

              @if (feature.code) {
                <div class="card-code">
                  <pre><code>{{ feature.code }}</code></pre>
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
      font-size: clamp(32px, 5vw, 48px);
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
      transition: all 0.4s ease;

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
      background: radial-gradient(ellipse at top, rgba(78, 205, 196, 0.1) 0%, transparent 70%);
      opacity: 0;
      transition: opacity 0.4s;
      pointer-events: none;
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
      background: rgba(78, 205, 196, 0.1);
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
        line-height: 1.6;
      }

      code {
        color: var(--text-secondary);
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
      background: rgba(78, 205, 196, 0.15);
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
export class FeaturesComponent {
  animatedCount = signal(12847);

  features: Feature[] = [
    {
      id: 'auth',
      title: 'Multi-Provider Authentication',
      description: 'API keys for simplicity. JWT for scale. OAuth 2.1 with PKCE for enterprise SSO. Use one or combine them all.',
      icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect><path d="M7 11V7a5 5 0 0 1 10 0v4"></path></svg>',
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
      icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"></path></svg>',
      tag: 'Per-tool ACLs',
      size: 'small'
    },
    {
      id: 'rate',
      title: 'Per-Identity Rate Limiting',
      description: 'Token bucket algorithm with per-user limits. Custom rates per identity. Automatic Retry-After headers.',
      icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"></circle><polyline points="12 6 12 12 16 14"></polyline></svg>',
      tag: 'Token Bucket',
      size: 'small'
    },
    {
      id: 'audit',
      title: 'Audit Logging',
      description: 'Every request logged with identity, tool, timestamp, and outcome. Automatic secret redaction. Export-ready for compliance.',
      icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path><polyline points="14 2 14 8 20 8"></polyline><line x1="16" y1="13" x2="8" y2="13"></line><line x1="16" y1="17" x2="8" y2="17"></line></svg>',
      tag: 'SOC 2 Ready',
      size: 'small'
    },
    {
      id: 'metrics',
      title: 'Prometheus Metrics',
      description: 'Built-in /metrics endpoint with request counts, latency histograms, auth outcomes, and rate limit tracking.',
      icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="22 12 18 12 15 21 9 3 6 12 2 12"></polyline></svg>',
      tag: 'Grafana Compatible',
      size: 'small'
    },
    {
      id: 'binary',
      title: 'Zero Infrastructure',
      description: 'Single static binary. No Docker, no Kubernetes, no databases. Compiles to WASM for edge deployment.',
      icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="12 2 2 7 12 12 22 7 12 2"></polygon><polyline points="2 17 12 22 22 17"></polyline><polyline points="2 12 12 17 22 12"></polyline></svg>',
      tag: 'Rust • WASM Ready',
      size: 'large'
    }
  ];

  constructor() {
    // Animate the metrics counter
    setInterval(() => {
      this.animatedCount.update(v => v + Math.floor(Math.random() * 10));
    }, 2000);
  }
}
