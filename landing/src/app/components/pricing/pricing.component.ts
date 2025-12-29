import { Component, signal, ChangeDetectionStrategy } from '@angular/core';
import { CommonModule } from '@angular/common';

interface PricingTier {
  name: string;
  description: string;
  price: number;
  originalPrice?: number;
  period: string;
  features: string[];
  cta: string;
  ctaLink: string;
  featured?: boolean;
  founderPricing?: boolean;
}

interface ComparisonFeature {
  name: string;
  values: (boolean | string)[];
}

@Component({
  selector: 'app-pricing',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule],
  template: `
    <section class="pricing" id="pricing">
      <div class="pricing-container">
        <div class="section-header">
          <span class="section-tag">// Pricing</span>
          <h2 class="section-title">Pricing that <span class="gradient-text">respects</span> your budget</h2>
          <p class="section-subtitle">
            Free for side projects. $12/mo when you're serious. No "contact sales" games.
          </p>

          <!-- View toggle -->
          <div class="view-toggle">
            <button
              class="toggle-btn"
              [class.active]="viewMode() === 'cards'"
              (click)="viewMode.set('cards')"
            >
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <rect x="3" y="3" width="7" height="7"></rect>
                <rect x="14" y="3" width="7" height="7"></rect>
                <rect x="3" y="14" width="7" height="7"></rect>
                <rect x="14" y="14" width="7" height="7"></rect>
              </svg>
              Cards
            </button>
            <button
              class="toggle-btn"
              [class.active]="viewMode() === 'table'"
              (click)="viewMode.set('table')"
            >
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <line x1="3" y1="6" x2="21" y2="6"></line>
                <line x1="3" y1="12" x2="21" y2="12"></line>
                <line x1="3" y1="18" x2="21" y2="18"></line>
              </svg>
              Compare
            </button>
          </div>
        </div>

        <!-- Founder pricing countdown -->
        <div class="founder-banner">
          <div class="founder-icon">
            <svg viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/>
            </svg>
          </div>
          <div class="founder-text">
            <strong>Founder Pricing Active</strong>
            <span>Lock in 40% off forever — limited time only</span>
          </div>
        </div>

        <!-- Cards View -->
        <div class="pricing-levels" [class.hidden]="viewMode() === 'table'">
          @for (tier of tiers; track tier.name; let i = $index) {
            <div
              class="pricing-card"
              [class.featured]="tier.featured"
              [style.--level]="i"
            >
              @if (tier.founderPricing) {
                <div class="founder-badge">
                  <svg viewBox="0 0 24 24" fill="currentColor">
                    <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/>
                  </svg>
                  40% OFF
                </div>
              }

              @if (tier.featured) {
                <div class="featured-badge">Most Popular</div>
              }

              <div class="card-content">
                <div class="tier-header">
                  <h3 class="tier-name">{{ tier.name }}</h3>
                  <p class="tier-description">{{ tier.description }}</p>
                </div>

                <div class="tier-price">
                  <span class="currency">$</span>
                  <span class="amount">{{ tier.price }}</span>
                  <span class="period">{{ tier.period }}</span>
                  @if (tier.originalPrice) {
                    <span class="original-price">\${{ tier.originalPrice }}</span>
                  }
                </div>

                <ul class="tier-features">
                  @for (feature of tier.features; track feature) {
                    <li>
                      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <polyline points="20 6 9 17 4 12"/>
                      </svg>
                      {{ feature }}
                    </li>
                  }
                </ul>

                <a [href]="tier.ctaLink" class="tier-cta" [class.primary]="tier.featured">
                  {{ tier.cta }}
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M5 12h14M12 5l7 7-7 7"/>
                  </svg>
                </a>
              </div>

              <!-- Level indicator -->
              <div class="level-indicator">
                <span class="level-number">{{ i + 1 }}</span>
              </div>
            </div>
          }
        </div>

        <!-- Table View -->
        <div class="comparison-table" [class.hidden]="viewMode() === 'cards'">
          <table>
            <thead>
              <tr>
                <th class="feature-col">Feature</th>
                @for (tier of tiers; track tier.name) {
                  <th [class.featured]="tier.featured">
                    <span class="tier-name">{{ tier.name }}</span>
                    <span class="tier-price">
                      @if (tier.price === 0) {
                        Free
                      } @else {
                        \${{ tier.price }}{{ tier.period }}
                      }
                    </span>
                  </th>
                }
              </tr>
            </thead>
            <tbody>
              @for (feature of comparisonFeatures; track feature.name) {
                <tr>
                  <td class="feature-name">{{ feature.name }}</td>
                  @for (tier of tiers; track tier.name; let i = $index) {
                    <td [class.featured]="tier.featured">
                      @if (feature.values[i] === true) {
                        <svg class="check-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                          <polyline points="20 6 9 17 4 12"/>
                        </svg>
                      } @else if (feature.values[i] === false) {
                        <span class="dash">—</span>
                      } @else {
                        <span class="value">{{ feature.values[i] }}</span>
                      }
                    </td>
                  }
                </tr>
              }
            </tbody>
          </table>
        </div>

        <!-- Enterprise slider -->
        <div class="team-calculator">
          <div class="calculator-header">
            <h4>Calculate your Enterprise plan cost</h4>
            <p>Adjust the slider to see pricing for your team size</p>
          </div>
          <div class="calculator-body">
            <div class="slider-container">
              <input
                type="range"
                min="2"
                max="50"
                [value]="teamSize()"
                (input)="onTeamSizeChange($event)"
                class="team-slider"
              />
              <div class="slider-labels">
                <span>2 users</span>
                <span>50 users</span>
              </div>
            </div>
            <div class="calculator-result">
              <div class="result-users">
                <span class="result-number">{{ teamSize() }}</span>
                <span class="result-label">team members</span>
              </div>
              <div class="result-price">
                <span class="result-currency">$</span>
                <span class="result-amount">{{ calculateTeamPrice() }}</span>
                <span class="result-period">/month</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  `,
  styles: [`
    .pricing {
      position: relative;
      padding: var(--section-py-lg) 0;
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

    .pricing-container {
      max-width: 1200px;
      margin: 0 auto;
      padding: 0 var(--container-px);
    }

    .section-header {
      text-align: center;
      margin-bottom: var(--space-12);
    }

    .section-tag {
      font-family: var(--font-mono);
      font-size: var(--text-sm);
      color: var(--accent-cyan);
      letter-spacing: var(--tracking-wider);
      line-height: var(--leading-normal);
      margin-bottom: var(--space-4);
      display: block;
    }

    .section-title {
      font-family: var(--font-display);
      font-size: var(--text-4xl);
      font-weight: var(--weight-bold);
      letter-spacing: var(--tracking-tight);
      line-height: var(--leading-snug);
      margin-bottom: var(--space-4);
    }

    .gradient-text {
      background: var(--gradient-brand);
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
      background-clip: text;
    }

    .section-subtitle {
      font-size: var(--text-lg);
      color: var(--text-secondary);
      max-width: 600px;
      margin: 0 auto var(--space-6);
      line-height: var(--leading-relaxed);
    }

    .view-toggle {
      display: flex;
      justify-content: center;
      gap: var(--space-2);
    }

    .view-toggle .toggle-btn {
      display: flex;
      align-items: center;
      gap: var(--space-2);
      padding: var(--space-2-5) var(--space-4);
      background: transparent;
      border: 1px solid var(--border-subtle);
      border-radius: var(--radius-lg);
      color: var(--text-muted);
      font-size: var(--text-sm);
      font-weight: var(--weight-medium);
      line-height: var(--leading-normal);
      cursor: pointer;
      transition: all var(--duration-fast) var(--ease-out);

      svg {
        width: var(--icon-sm);
        height: var(--icon-sm);
      }

      &.active {
        background: var(--bg-elevated);
        border-color: var(--border-accent);
        color: var(--text-primary);
      }

      &:hover:not(.active) {
        border-color: var(--border-medium);
        color: var(--text-secondary);
      }
    }

    .founder-banner {
      display: flex;
      align-items: center;
      justify-content: center;
      gap: var(--space-4);
      padding: var(--space-4) var(--space-6);
      background: linear-gradient(90deg, rgba(239, 68, 68, 0.1) 0%, rgba(249, 115, 22, 0.1) 100%);
      border: 1px solid rgba(239, 68, 68, 0.2);
      border-radius: var(--radius-xl);
      margin-bottom: var(--space-12);
    }

    .founder-icon {
      width: var(--space-10);
      height: var(--space-10);
      background: rgba(239, 68, 68, 0.15);
      border-radius: var(--radius-lg);
      display: flex;
      align-items: center;
      justify-content: center;
      color: var(--accent-red);

      svg {
        width: var(--icon-lg);
        height: var(--icon-lg);
      }
    }

    .founder-text {
      display: flex;
      flex-direction: column;
      gap: var(--space-0-5);

      strong {
        color: var(--text-primary);
        font-size: var(--text-base);
        line-height: var(--leading-normal);
      }

      span {
        color: var(--text-muted);
        font-size: var(--text-sm);
        line-height: var(--leading-normal);
      }
    }

    .hidden {
      display: none !important;
    }

    .pricing-levels {
      display: grid;
      grid-template-columns: repeat(3, 1fr);
      gap: var(--space-6);
      margin-bottom: var(--space-16);

      @media (max-width: 900px) {
        grid-template-columns: 1fr;
        max-width: 400px;
        margin-left: auto;
        margin-right: auto;
      }
    }

    .pricing-card {
      position: relative;
      background: var(--bg-secondary);
      border: 1px solid var(--border-subtle);
      border-radius: var(--radius-3xl);
      overflow: hidden;
      transition: all var(--duration-slow) var(--ease-out);

      // Staggered heights for visual progression
      &:nth-child(1) { transform: translateY(var(--space-5)); }
      &:nth-child(2) { transform: translateY(0); }
      &:nth-child(3) { transform: translateY(var(--space-2-5)); }

      @media (max-width: 900px) {
        transform: translateY(0) !important;
      }

      &:hover {
        transform: translateY(-4px) !important;
        border-color: var(--border-accent);
      }

      &.featured {
        border-color: var(--accent-cyan);
        background: linear-gradient(180deg, rgba(78, 205, 196, 0.05) 0%, var(--bg-secondary) 30%);
        box-shadow: var(--shadow-glow-orange);

        &::before {
          content: '';
          position: absolute;
          top: 0;
          left: 0;
          right: 0;
          height: 3px;
          background: var(--gradient-brand);
        }
      }
    }

    .founder-badge {
      position: absolute;
      top: var(--space-4);
      right: var(--space-4);
      display: flex;
      align-items: center;
      gap: var(--space-1-5);
      padding: var(--space-1-5) var(--space-3);
      background: rgba(239, 68, 68, 0.1);
      border: 1px solid rgba(239, 68, 68, 0.2);
      border-radius: var(--radius-full);
      color: var(--accent-red);
      font-size: var(--text-xs);
      font-weight: var(--weight-bold);
      letter-spacing: var(--tracking-wide);
      line-height: var(--leading-normal);

      svg {
        width: var(--icon-xs);
        height: var(--icon-xs);
      }
    }

    .featured-badge {
      position: absolute;
      top: -1px;
      left: 50%;
      transform: translateX(-50%);
      padding: var(--space-1-5) var(--space-5);
      background: var(--gradient-brand);
      color: var(--bg-primary);
      font-size: var(--text-xs);
      font-weight: var(--weight-semibold);
      line-height: var(--leading-normal);
      border-radius: 0 0 var(--radius-lg) var(--radius-lg);
    }

    .card-content {
      padding: var(--space-8);
    }

    .tier-header {
      margin-bottom: var(--space-6);
    }

    .tier-name {
      font-size: var(--text-xl);
      font-weight: var(--weight-semibold);
      line-height: var(--leading-snug);
      margin-bottom: var(--space-1-5);
    }

    .tier-description {
      color: var(--text-muted);
      font-size: var(--text-sm);
      line-height: var(--leading-normal);
    }

    .tier-price {
      display: flex;
      align-items: baseline;
      gap: var(--space-1);
      margin-bottom: var(--space-7);
    }

    .currency {
      font-size: var(--text-2xl);
      font-weight: var(--weight-semibold);
      color: var(--text-secondary);
      line-height: var(--leading-none);
    }

    .amount {
      font-size: var(--text-5xl);
      font-weight: var(--weight-bold);
      letter-spacing: var(--tracking-tighter);
      line-height: var(--leading-none);
    }

    .period {
      font-size: var(--text-base);
      color: var(--text-muted);
      margin-left: var(--space-1);
      line-height: var(--leading-normal);
    }

    .original-price {
      font-size: var(--text-xl);
      color: var(--text-dim);
      text-decoration: line-through;
      margin-left: var(--space-3);
      line-height: var(--leading-normal);
    }

    .tier-features {
      list-style: none;
      margin-bottom: var(--space-7);

      li {
        display: flex;
        align-items: center;
        gap: var(--space-3);
        padding: var(--space-2-5) 0;
        font-size: var(--text-sm);
        color: var(--text-secondary);
        line-height: var(--leading-normal);

        &:not(:last-child) {
          border-bottom: 1px solid var(--border-subtle);
        }

        svg {
          width: var(--icon-sm);
          height: var(--icon-sm);
          color: var(--accent-cyan);
          flex-shrink: 0;
        }
      }
    }

    .tier-cta {
      display: flex;
      align-items: center;
      justify-content: center;
      gap: var(--space-2-5);
      width: 100%;
      padding: var(--space-3-5) var(--space-6);
      background: var(--bg-elevated);
      border: 1px solid var(--border-subtle);
      border-radius: var(--radius-xl);
      color: var(--text-primary);
      font-size: var(--text-base);
      font-weight: var(--weight-semibold);
      line-height: var(--leading-normal);
      text-decoration: none;
      transition: all var(--duration-normal) var(--ease-out);

      svg {
        width: var(--icon-sm);
        height: var(--icon-sm);
      }

      &:hover {
        background: var(--bg-hover);
        border-color: var(--border-accent);
      }

      &.primary {
        background: var(--text-primary);
        color: var(--bg-primary);
        border-color: transparent;

        &:hover {
          transform: translateY(-2px);
          box-shadow: var(--shadow-lg);
        }
      }
    }

    .level-indicator {
      position: absolute;
      bottom: var(--space-4);
      left: var(--space-4);
      width: var(--space-7);
      height: var(--space-7);
      background: var(--bg-elevated);
      border: 1px solid var(--border-subtle);
      border-radius: var(--radius-lg);
      display: flex;
      align-items: center;
      justify-content: center;
    }

    .level-number {
      font-family: var(--font-mono);
      font-size: var(--text-xs);
      color: var(--text-muted);
      line-height: var(--leading-normal);
    }

    // Comparison table
    .comparison-table {
      margin-bottom: var(--space-16);
      overflow-x: auto;
      background: var(--bg-secondary);
      border: 1px solid var(--border-subtle);
      border-radius: var(--radius-2xl);
    }

    .comparison-table table {
      width: 100%;
      border-collapse: collapse;
      min-width: 600px;
    }

    .comparison-table thead th {
      padding: var(--space-6) var(--space-5);
      text-align: center;
      border-bottom: 1px solid var(--border-subtle);
      vertical-align: bottom;

      &.feature-col {
        text-align: left;
        width: 200px;
        color: var(--text-muted);
        font-size: var(--text-sm);
        font-weight: var(--weight-medium);
        text-transform: uppercase;
        letter-spacing: var(--tracking-wider);
        line-height: var(--leading-normal);
      }

      &.featured {
        background: linear-gradient(180deg, rgba(78, 205, 196, 0.08) 0%, transparent 100%);
        position: relative;

        &::before {
          content: 'Popular';
          position: absolute;
          top: var(--space-2);
          left: 50%;
          transform: translateX(-50%);
          padding: var(--space-1) var(--space-2-5);
          background: var(--gradient-brand);
          color: var(--bg-primary);
          font-size: var(--text-xs);
          font-weight: var(--weight-semibold);
          line-height: var(--leading-normal);
          border-radius: var(--radius-full);
        }
      }
    }

    .comparison-table .tier-name {
      display: block;
      font-size: var(--text-lg);
      font-weight: var(--weight-semibold);
      line-height: var(--leading-snug);
      margin-bottom: var(--space-1);
    }

    .comparison-table .tier-price {
      display: block;
      font-size: var(--text-sm);
      color: var(--text-muted);
      line-height: var(--leading-normal);
    }

    .comparison-table tbody tr {
      border-bottom: 1px solid var(--border-subtle);

      &:last-child {
        border-bottom: none;
      }

      &:hover {
        background: rgba(255, 255, 255, 0.02);
      }
    }

    .comparison-table tbody td {
      padding: var(--space-4) var(--space-5);
      text-align: center;
      font-size: var(--text-sm);
      line-height: var(--leading-normal);

      &.feature-name {
        text-align: left;
        color: var(--text-secondary);
      }

      &.featured {
        background: rgba(78, 205, 196, 0.03);
      }
    }

    .comparison-table .check-icon {
      width: var(--icon-md);
      height: var(--icon-md);
      color: var(--accent-cyan);
      margin: 0 auto;
    }

    .comparison-table .dash {
      color: var(--text-dim);
    }

    .comparison-table .value {
      color: var(--text-primary);
      font-weight: var(--weight-medium);
    }

    // Team calculator
    .team-calculator {
      background: var(--bg-secondary);
      border: 1px solid var(--border-subtle);
      border-radius: var(--radius-2xl);
      padding: var(--space-8);
    }

    .calculator-header {
      text-align: center;
      margin-bottom: var(--space-8);

      h4 {
        font-size: var(--text-xl);
        font-weight: var(--weight-semibold);
        line-height: var(--leading-snug);
        margin-bottom: var(--space-2);
      }

      p {
        color: var(--text-muted);
        font-size: var(--text-sm);
        line-height: var(--leading-normal);
      }
    }

    .calculator-body {
      display: grid;
      grid-template-columns: 1fr auto;
      gap: var(--space-12);
      align-items: center;

      @media (max-width: 640px) {
        grid-template-columns: 1fr;
        gap: var(--space-8);
      }
    }

    .slider-container {
      width: 100%;
    }

    .team-slider {
      width: 100%;
      height: var(--space-2);
      background: var(--bg-elevated);
      border-radius: var(--radius-full);
      outline: none;
      -webkit-appearance: none;
      cursor: pointer;

      &::-webkit-slider-thumb {
        -webkit-appearance: none;
        width: var(--space-6);
        height: var(--space-6);
        background: var(--gradient-brand);
        border-radius: var(--radius-full);
        cursor: pointer;
        box-shadow: var(--shadow-glow-orange);
        transition: transform var(--duration-fast) var(--ease-out);

        &:hover {
          transform: scale(1.1);
        }
      }
    }

    .slider-labels {
      display: flex;
      justify-content: space-between;
      margin-top: var(--space-3);
      font-size: var(--text-xs);
      color: var(--text-muted);
      line-height: var(--leading-normal);
    }

    .calculator-result {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: var(--space-2);
      padding: var(--space-6) var(--space-8);
      background: var(--bg-primary);
      border: 1px solid var(--border-subtle);
      border-radius: var(--radius-2xl);
      min-width: 200px;
    }

    .result-users {
      display: flex;
      align-items: baseline;
      gap: var(--space-2);
    }

    .result-number {
      font-size: var(--text-3xl);
      font-weight: var(--weight-bold);
      color: var(--accent-cyan);
      line-height: var(--leading-tight);
    }

    .result-label {
      font-size: var(--text-sm);
      color: var(--text-muted);
      line-height: var(--leading-normal);
    }

    .result-price {
      display: flex;
      align-items: baseline;
      gap: var(--space-0-5);
    }

    .result-currency {
      font-size: var(--text-lg);
      color: var(--text-secondary);
      line-height: var(--leading-tight);
    }

    .result-amount {
      font-size: var(--text-3xl);
      font-weight: var(--weight-bold);
      line-height: var(--leading-tight);
    }

    .result-period {
      font-size: var(--text-sm);
      color: var(--text-muted);
      line-height: var(--leading-normal);
    }
  `]
})
export class PricingComponent {
  teamSize = signal(5);
  viewMode = signal<'cards' | 'table'>('cards');

  comparisonFeatures: ComparisonFeature[] = [
    // Authentication
    { name: 'API Key Auth', values: [true, true, true] },
    { name: 'JWT HS256', values: [true, true, true] },
    { name: 'JWT JWKS (RS256/ES256)', values: [false, true, true] },
    { name: 'OAuth 2.1 + PKCE', values: [false, true, true] },
    { name: 'mTLS Client Certs', values: [false, false, true] },
    // Transport
    { name: 'Stdio Transport', values: [true, true, true] },
    { name: 'HTTP/SSE Transport', values: [false, true, true] },
    { name: 'Multi-Server Routing', values: [false, false, true] },
    // Rate Limiting
    { name: 'Global Rate Limiting', values: [true, true, true] },
    { name: 'Per-Identity Rate Limiting', values: [false, true, true] },
    { name: 'Per-Tool Rate Limiting', values: [false, false, true] },
    // Observability
    { name: 'Prometheus Metrics', values: [true, true, true] },
    { name: 'Health Endpoints', values: [true, true, true] },
    { name: 'OpenTelemetry Tracing', values: [false, false, true] },
    // Audit
    { name: 'File/Console Audit Logs', values: [true, true, true] },
    { name: 'SIEM Log Shipping', values: [false, false, true] },
    // Admin
    { name: 'Guard Tools API', values: [false, false, true] },
    { name: 'Support', values: ['Community', 'Email (48h)', 'Priority (4h)'] },
  ];

  tiers: PricingTier[] = [
    {
      name: 'Free',
      description: 'Open source, forever free',
      price: 0,
      period: '',
      features: [
        'API key + JWT HS256 auth',
        'Stdio transport',
        'Global rate limiting',
        'Prometheus metrics',
        'File/console audit logs',
        'Community support'
      ],
      cta: 'Get Started',
      ctaLink: '/docs/quickstart'
    },
    {
      name: 'Pro',
      description: 'Founder pricing (normally $20)',
      price: 12,
      originalPrice: 20,
      period: '/month',
      features: [
        'Everything in Free, plus:',
        'OAuth 2.1 + PKCE auth',
        'JWT JWKS (RS256/ES256)',
        'HTTP & SSE transports',
        'Per-identity rate limiting',
        'Email support (48h)'
      ],
      cta: 'Start Free Trial',
      ctaLink: '/signup?plan=pro',
      featured: true,
      founderPricing: true
    },
    {
      name: 'Enterprise',
      description: 'For teams with compliance needs',
      price: 29,
      period: ' + $8/seat',
      features: [
        'Everything in Pro, plus:',
        'mTLS client certificates',
        'Multi-server routing',
        'OpenTelemetry tracing',
        'SIEM log shipping',
        'Per-tool rate limiting',
        'Guard tools API',
        'Priority support (4h)'
      ],
      cta: 'Contact Sales',
      ctaLink: '/contact?plan=enterprise',
      founderPricing: false
    }
  ];

  onTeamSizeChange(event: Event) {
    const target = event.target as HTMLInputElement;
    this.teamSize.set(parseInt(target.value, 10));
  }

  calculateTeamPrice(): number {
    return 29 + (this.teamSize() * 8);
  }
}
