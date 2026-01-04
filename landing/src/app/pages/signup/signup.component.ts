import { Component, OnInit, signal } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ActivatedRoute, Router } from '@angular/router';
import { FormsModule } from '@angular/forms';
import { environment } from '../../../environments/environment';

declare const Stripe: any;

@Component({
  selector: 'app-signup',
  standalone: true,
  imports: [CommonModule, FormsModule],
  template: `
    <section class="signup">
      <div class="signup-container">
        <div class="signup-card">
          <div class="card-header">
            <h1>Start Your Pro Trial</h1>
            <p>7-day free trial, no credit card required</p>
          </div>

          @if (error()) {
            <div class="error-banner">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="10"></circle>
                <line x1="12" y1="8" x2="12" y2="12"></line>
                <line x1="12" y1="16" x2="12.01" y2="16"></line>
              </svg>
              {{ error() }}
            </div>
          }

          <form class="signup-form" (submit)="handleSubmit($event)">
            <div class="form-group">
              <label for="email">Email</label>
              <input
                type="email"
                id="email"
                name="email"
                [(ngModel)]="email"
                placeholder="you@company.com"
                required
                [disabled]="loading()"
              />
            </div>

            <div class="plan-summary">
              <div class="plan-header">
                <h3>MCP-Guard Pro</h3>
                <span class="founder-badge">Founder Pricing</span>
              </div>
              <div class="plan-details">
                <div class="price-row">
                  <span>Monthly subscription</span>
                  <span class="price">
                    <s class="original">$20</s>
                    $12/month
                  </span>
                </div>
                <ul class="features">
                  <li>OAuth 2.1 authentication</li>
                  <li>HTTP & SSE transports</li>
                  <li>Per-identity rate limiting</li>
                  <li>Email support (48h SLA)</li>
                </ul>
              </div>
            </div>

            <button
              type="submit"
              class="submit-btn"
              [disabled]="loading()"
            >
              @if (loading()) {
                <span class="spinner"></span>
                Processing...
              } @else {
                Start Free Trial →
              }
            </button>

            <p class="terms">
              By continuing, you agree to our
              <a href="/terms">Terms of Service</a> and
              <a href="/privacy">Privacy Policy</a>
            </p>
          </form>
        </div>

        <div class="trial-info">
          <h3>What happens next?</h3>
          <ol class="steps">
            <li>
              <div class="step-icon">1</div>
              <div class="step-content">
                <strong>7-day free trial</strong>
                <p>Try Pro features with no commitment</p>
              </div>
            </li>
            <li>
              <div class="step-icon">2</div>
              <div class="step-content">
                <strong>Receive your license</strong>
                <p>Instant email with installation instructions</p>
              </div>
            </li>
            <li>
              <div class="step-icon">3</div>
              <div class="step-content">
                <strong>Install in 5 minutes</strong>
                <p>One command to secure your MCP servers</p>
              </div>
            </li>
          </ol>

          <div class="testimonial">
            <p>"MCP-Guard saved us hours of security implementation. The Pro tier is absolutely worth it."</p>
            <div class="author">
              <strong>Alex Chen</strong>
              <span>Engineering Lead, TechCorp</span>
            </div>
          </div>
        </div>
      </div>
    </section>
  `,
  styles: [`
    .signup {
      min-height: 100vh;
      padding: var(--space-16) 0;
      background: var(--bg-primary);
      display: flex;
      align-items: center;
      justify-content: center;
    }

    .signup-container {
      max-width: 1000px;
      margin: 0 auto;
      padding: 0 var(--container-px);
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: var(--space-16);

      @media (max-width: 900px) {
        grid-template-columns: 1fr;
      }
    }

    .signup-card {
      background: var(--bg-secondary);
      border: 1px solid var(--border-subtle);
      border-radius: var(--radius-3xl);
      padding: var(--space-10);
    }

    .card-header {
      text-align: center;
      margin-bottom: var(--space-8);

      h1 {
        font-family: var(--font-display);
        font-size: var(--text-3xl);
        font-weight: var(--weight-bold);
        margin-bottom: var(--space-2);
      }

      p {
        color: var(--text-muted);
        font-size: var(--text-base);
      }
    }

    .error-banner {
      display: flex;
      align-items: center;
      gap: var(--space-3);
      padding: var(--space-4);
      background: rgba(239, 68, 68, 0.1);
      border: 1px solid rgba(239, 68, 68, 0.2);
      border-radius: var(--radius-lg);
      color: var(--accent-red);
      margin-bottom: var(--space-6);

      svg {
        width: var(--icon-md);
        height: var(--icon-md);
        flex-shrink: 0;
      }
    }

    .signup-form {
      display: flex;
      flex-direction: column;
      gap: var(--space-6);
    }

    .form-group {
      label {
        display: block;
        font-size: var(--text-sm);
        font-weight: var(--weight-medium);
        margin-bottom: var(--space-2);
        color: var(--text-secondary);
      }

      input {
        width: 100%;
        padding: var(--space-3-5) var(--space-4);
        background: var(--bg-elevated);
        border: 1px solid var(--border-subtle);
        border-radius: var(--radius-lg);
        font-size: var(--text-base);
        color: var(--text-primary);
        transition: all var(--duration-fast) var(--ease-out);

        &:focus {
          outline: none;
          border-color: var(--accent-cyan);
          box-shadow: 0 0 0 3px rgba(78, 205, 196, 0.1);
        }

        &:disabled {
          opacity: 0.5;
          cursor: not-allowed;
        }

        &::placeholder {
          color: var(--text-dim);
        }
      }
    }

    .plan-summary {
      background: var(--bg-elevated);
      border: 1px solid var(--border-subtle);
      border-radius: var(--radius-xl);
      overflow: hidden;
    }

    .plan-header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: var(--space-4) var(--space-5);
      background: var(--bg-secondary);
      border-bottom: 1px solid var(--border-subtle);

      h3 {
        font-size: var(--text-lg);
        font-weight: var(--weight-semibold);
      }
    }

    .founder-badge {
      padding: var(--space-1-5) var(--space-3);
      background: rgba(239, 68, 68, 0.1);
      border: 1px solid rgba(239, 68, 68, 0.2);
      border-radius: var(--radius-full);
      color: var(--accent-red);
      font-size: var(--text-xs);
      font-weight: var(--weight-bold);
    }

    .plan-details {
      padding: var(--space-5);
    }

    .price-row {
      display: flex;
      align-items: center;
      justify-content: space-between;
      margin-bottom: var(--space-4);
      padding-bottom: var(--space-4);
      border-bottom: 1px solid var(--border-subtle);

      .price {
        font-size: var(--text-xl);
        font-weight: var(--weight-bold);

        .original {
          color: var(--text-dim);
          margin-right: var(--space-2);
        }
      }
    }

    .features {
      list-style: none;
      display: flex;
      flex-direction: column;
      gap: var(--space-2);

      li {
        display: flex;
        align-items: center;
        font-size: var(--text-sm);
        color: var(--text-secondary);

        &::before {
          content: '✓';
          color: var(--accent-cyan);
          font-weight: var(--weight-bold);
          margin-right: var(--space-3);
        }
      }
    }

    .submit-btn {
      width: 100%;
      padding: var(--space-4) var(--space-6);
      background: var(--text-primary);
      color: var(--bg-primary);
      border: none;
      border-radius: var(--radius-xl);
      font-size: var(--text-base);
      font-weight: var(--weight-semibold);
      cursor: pointer;
      transition: all var(--duration-normal) var(--ease-out);
      display: flex;
      align-items: center;
      justify-content: center;
      gap: var(--space-2);

      &:hover:not(:disabled) {
        transform: translateY(-2px);
        box-shadow: var(--shadow-lg);
      }

      &:disabled {
        opacity: 0.7;
        cursor: not-allowed;
        transform: none;
      }
    }

    .spinner {
      width: var(--icon-sm);
      height: var(--icon-sm);
      border: 2px solid var(--bg-primary);
      border-top-color: transparent;
      border-radius: 50%;
      animation: spin 1s linear infinite;
    }

    @keyframes spin {
      to { transform: rotate(360deg); }
    }

    .terms {
      text-align: center;
      font-size: var(--text-xs);
      color: var(--text-muted);

      a {
        color: var(--accent-cyan);
        text-decoration: none;

        &:hover {
          text-decoration: underline;
        }
      }
    }

    .trial-info {
      display: flex;
      flex-direction: column;
      gap: var(--space-8);

      h3 {
        font-size: var(--text-xl);
        font-weight: var(--weight-semibold);
      }
    }

    .steps {
      list-style: none;
      display: flex;
      flex-direction: column;
      gap: var(--space-6);
      counter-reset: step;

      li {
        display: flex;
        gap: var(--space-4);
      }
    }

    .step-icon {
      width: var(--space-10);
      height: var(--space-10);
      background: var(--bg-elevated);
      border: 1px solid var(--border-subtle);
      border-radius: var(--radius-lg);
      display: flex;
      align-items: center;
      justify-content: center;
      font-weight: var(--weight-bold);
      color: var(--accent-cyan);
      flex-shrink: 0;
    }

    .step-content {
      strong {
        display: block;
        margin-bottom: var(--space-1);
      }

      p {
        color: var(--text-muted);
        font-size: var(--text-sm);
      }
    }

    .testimonial {
      padding: var(--space-6);
      background: var(--bg-secondary);
      border: 1px solid var(--border-subtle);
      border-radius: var(--radius-xl);

      p {
        font-style: italic;
        color: var(--text-secondary);
        margin-bottom: var(--space-4);
      }

      .author {
        display: flex;
        flex-direction: column;
        gap: var(--space-0-5);

        strong {
          font-size: var(--text-sm);
        }

        span {
          font-size: var(--text-xs);
          color: var(--text-muted);
        }
      }
    }
  `]
})
export class SignupComponent implements OnInit {
  email = '';
  loading = signal(false);
  error = signal<string | null>(null);
  plan = signal<string>('pro');

  // Stripe configuration
  private stripePublishableKey = environment.stripePublishableKey;
  private stripePriceId = environment.stripePriceId;

  constructor(
    private route: ActivatedRoute,
    private router: Router
  ) { }

  ngOnInit() {
    // Get plan from query params
    this.route.queryParams.subscribe(params => {
      if (params['plan']) {
        this.plan.set(params['plan']);
      }
    });

    // Load Stripe.js
    this.loadStripe();
  }

  private loadStripe() {
    if (typeof Stripe !== 'undefined') {
      return;
    }

    const script = document.createElement('script');
    script.src = 'https://js.stripe.com/v3/';
    script.async = true;
    document.head.appendChild(script);
  }

  async handleSubmit(event: Event) {
    event.preventDefault();

    if (!this.email || this.loading()) {
      return;
    }

    this.loading.set(true);
    this.error.set(null);

    try {
      // Initialize Stripe
      const stripe = Stripe(this.stripePublishableKey);

      // Create checkout session via your backend API
      // For now, redirect directly to Stripe checkout
      const { error } = await stripe.redirectToCheckout({
        lineItems: [{ price: this.stripePriceId, quantity: 1 }],
        mode: 'subscription',
        successUrl: `${window.location.origin}/success?session_id={CHECKOUT_SESSION_ID}`,
        cancelUrl: `${window.location.origin}/pricing`,
        customerEmail: this.email,
        clientReferenceId: this.plan(),
        metadata: {
          tier: this.plan()
        },
        // Trial period
        subscriptionData: {
          trialPeriodDays: 7
        }
      });

      if (error) {
        this.error.set(error.message || 'Failed to start checkout');
        this.loading.set(false);
      }
    } catch (err) {
      console.error('Checkout error:', err);
      this.error.set('An unexpected error occurred. Please try again.');
      this.loading.set(false);
    }
  }
}
