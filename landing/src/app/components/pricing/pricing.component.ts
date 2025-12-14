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
          <h2 class="section-title">Simple, <span class="gradient-text">transparent</span> pricing</h2>
          <p class="section-subtitle">
            Start free, upgrade when you need it. Lock in founder pricing — 40% off forever.
          </p>
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

        <!-- Pricing tiers as levels -->
        <div class="pricing-levels">
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

        <!-- Team slider -->
        <div class="team-calculator">
          <div class="calculator-header">
            <h4>Calculate your Team plan cost</h4>
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

    .pricing-container {
      max-width: 1200px;
      margin: 0 auto;
      padding: 0 24px;
    }

    .section-header {
      text-align: center;
      margin-bottom: 48px;
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

    .founder-banner {
      display: flex;
      align-items: center;
      justify-content: center;
      gap: 16px;
      padding: 16px 24px;
      background: linear-gradient(90deg, rgba(239, 68, 68, 0.1) 0%, rgba(249, 115, 22, 0.1) 100%);
      border: 1px solid rgba(239, 68, 68, 0.2);
      border-radius: 12px;
      margin-bottom: 48px;
    }

    .founder-icon {
      width: 40px;
      height: 40px;
      background: rgba(239, 68, 68, 0.15);
      border-radius: 10px;
      display: flex;
      align-items: center;
      justify-content: center;
      color: var(--accent-red);

      svg {
        width: 22px;
        height: 22px;
      }
    }

    .founder-text {
      display: flex;
      flex-direction: column;
      gap: 2px;

      strong {
        color: var(--text-primary);
        font-size: 15px;
      }

      span {
        color: var(--text-muted);
        font-size: 13px;
      }
    }

    .pricing-levels {
      display: grid;
      grid-template-columns: repeat(3, 1fr);
      gap: 24px;
      margin-bottom: 64px;

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
      border-radius: 24px;
      overflow: hidden;
      transition: all 0.4s ease;

      // Staggered heights for visual progression
      &:nth-child(1) { transform: translateY(20px); }
      &:nth-child(2) { transform: translateY(0); }
      &:nth-child(3) { transform: translateY(10px); }

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
        box-shadow: 0 0 40px rgba(78, 205, 196, 0.15);

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
      top: 16px;
      right: 16px;
      display: flex;
      align-items: center;
      gap: 6px;
      padding: 6px 12px;
      background: rgba(239, 68, 68, 0.1);
      border: 1px solid rgba(239, 68, 68, 0.2);
      border-radius: 100px;
      color: var(--accent-red);
      font-size: 11px;
      font-weight: 700;
      letter-spacing: 0.02em;

      svg {
        width: 14px;
        height: 14px;
      }
    }

    .featured-badge {
      position: absolute;
      top: -1px;
      left: 50%;
      transform: translateX(-50%);
      padding: 6px 20px;
      background: var(--gradient-brand);
      color: var(--bg-primary);
      font-size: 12px;
      font-weight: 600;
      border-radius: 0 0 10px 10px;
    }

    .card-content {
      padding: 32px;
    }

    .tier-header {
      margin-bottom: 24px;
    }

    .tier-name {
      font-size: 22px;
      font-weight: 600;
      margin-bottom: 6px;
    }

    .tier-description {
      color: var(--text-muted);
      font-size: 14px;
    }

    .tier-price {
      display: flex;
      align-items: baseline;
      gap: 4px;
      margin-bottom: 28px;
    }

    .currency {
      font-size: 24px;
      font-weight: 600;
      color: var(--text-secondary);
    }

    .amount {
      font-size: 56px;
      font-weight: 700;
      letter-spacing: -0.03em;
      line-height: 1;
    }

    .period {
      font-size: 16px;
      color: var(--text-muted);
      margin-left: 4px;
    }

    .original-price {
      font-size: 20px;
      color: var(--text-dim);
      text-decoration: line-through;
      margin-left: 12px;
    }

    .tier-features {
      list-style: none;
      margin-bottom: 28px;

      li {
        display: flex;
        align-items: center;
        gap: 12px;
        padding: 10px 0;
        font-size: 14px;
        color: var(--text-secondary);

        &:not(:last-child) {
          border-bottom: 1px solid var(--border-subtle);
        }

        svg {
          width: 18px;
          height: 18px;
          color: var(--accent-cyan);
          flex-shrink: 0;
        }
      }
    }

    .tier-cta {
      display: flex;
      align-items: center;
      justify-content: center;
      gap: 10px;
      width: 100%;
      padding: 14px 24px;
      background: var(--bg-elevated);
      border: 1px solid var(--border-subtle);
      border-radius: 12px;
      color: var(--text-primary);
      font-size: 15px;
      font-weight: 600;
      text-decoration: none;
      transition: all 0.3s;

      svg {
        width: 18px;
        height: 18px;
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
          box-shadow: 0 8px 24px rgba(248, 250, 252, 0.2);
        }
      }
    }

    .level-indicator {
      position: absolute;
      bottom: 16px;
      left: 16px;
      width: 28px;
      height: 28px;
      background: var(--bg-elevated);
      border: 1px solid var(--border-subtle);
      border-radius: 8px;
      display: flex;
      align-items: center;
      justify-content: center;
    }

    .level-number {
      font-family: var(--font-mono);
      font-size: 12px;
      color: var(--text-muted);
    }

    // Team calculator
    .team-calculator {
      background: var(--bg-secondary);
      border: 1px solid var(--border-subtle);
      border-radius: 20px;
      padding: 32px;
    }

    .calculator-header {
      text-align: center;
      margin-bottom: 32px;

      h4 {
        font-size: 20px;
        font-weight: 600;
        margin-bottom: 8px;
      }

      p {
        color: var(--text-muted);
        font-size: 14px;
      }
    }

    .calculator-body {
      display: grid;
      grid-template-columns: 1fr auto;
      gap: 48px;
      align-items: center;

      @media (max-width: 640px) {
        grid-template-columns: 1fr;
        gap: 32px;
      }
    }

    .slider-container {
      width: 100%;
    }

    .team-slider {
      width: 100%;
      height: 8px;
      background: var(--bg-elevated);
      border-radius: 100px;
      outline: none;
      -webkit-appearance: none;
      cursor: pointer;

      &::-webkit-slider-thumb {
        -webkit-appearance: none;
        width: 24px;
        height: 24px;
        background: var(--gradient-brand);
        border-radius: 50%;
        cursor: pointer;
        box-shadow: 0 2px 10px rgba(78, 205, 196, 0.4);
        transition: transform 0.2s;

        &:hover {
          transform: scale(1.1);
        }
      }
    }

    .slider-labels {
      display: flex;
      justify-content: space-between;
      margin-top: 12px;
      font-size: 12px;
      color: var(--text-muted);
    }

    .calculator-result {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 8px;
      padding: 24px 32px;
      background: var(--bg-primary);
      border: 1px solid var(--border-subtle);
      border-radius: 16px;
      min-width: 200px;
    }

    .result-users {
      display: flex;
      align-items: baseline;
      gap: 8px;
    }

    .result-number {
      font-size: 32px;
      font-weight: 700;
      color: var(--accent-cyan);
    }

    .result-label {
      font-size: 14px;
      color: var(--text-muted);
    }

    .result-price {
      display: flex;
      align-items: baseline;
      gap: 2px;
    }

    .result-currency {
      font-size: 18px;
      color: var(--text-secondary);
    }

    .result-amount {
      font-size: 36px;
      font-weight: 700;
    }

    .result-period {
      font-size: 14px;
      color: var(--text-muted);
    }
  `]
})
export class PricingComponent {
  teamSize = signal(5);

  tiers: PricingTier[] = [
    {
      name: 'Free',
      description: 'Perfect for side projects',
      price: 0,
      period: '/month',
      features: [
        '1 MCP server',
        'API key authentication',
        '1,000 requests/day',
        '7-day log retention',
        'Community support'
      ],
      cta: 'Get Started Free',
      ctaLink: '/docs/quickstart'
    },
    {
      name: 'Pro',
      description: 'For production workloads',
      price: 12,
      originalPrice: 19,
      period: '/month',
      features: [
        'Unlimited MCP servers',
        'OAuth 2.1 + JWT + API keys',
        'Unlimited requests',
        '90-day log retention',
        'Email support (48h)'
      ],
      cta: 'Start Pro Trial',
      ctaLink: '/signup?plan=pro',
      featured: true,
      founderPricing: true
    },
    {
      name: 'Team',
      description: 'For growing organizations',
      price: 29,
      originalPrice: 49,
      period: '+ $8/user',
      features: [
        'Everything in Pro',
        'Team management dashboard',
        'SSO (SAML/OIDC)',
        '1-year log retention',
        'Priority support (24h)'
      ],
      cta: 'Contact Sales',
      ctaLink: '/contact?plan=team',
      founderPricing: true
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
