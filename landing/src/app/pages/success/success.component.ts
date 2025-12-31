import { Component, OnInit, signal } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ActivatedRoute } from '@angular/router';

@Component({
  selector: 'app-success',
  standalone: true,
  imports: [CommonModule],
  template: `
    <section class="success">
      <div class="success-container">
        <div class="success-card">
          <div class="icon-container">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
              <polyline points="22 4 12 14.01 9 11.01"></polyline>
            </svg>
          </div>

          <h1>Welcome to MCP-Guard Pro!</h1>
          <p class="subtitle">Your 7-day free trial has started</p>

          <div class="next-steps">
            <h3>What's next?</h3>
            <ol class="steps">
              <li>
                <div class="step-number">1</div>
                <div class="step-content">
                  <strong>Check your email</strong>
                  <p>We've sent your Pro license key to your inbox</p>
                </div>
              </li>
              <li>
                <div class="step-number">2</div>
                <div class="step-content">
                  <strong>Install MCP-Guard</strong>
                  <p>One command to get started:</p>
                  <pre class="install-command">curl -fsSL https://mcp-guard.io/install-pro.sh | \\
  MCP_GUARD_LICENSE_KEY=pro_xxx... bash</pre>
                </div>
              </li>
              <li>
                <div class="step-number">3</div>
                <div class="step-content">
                  <strong>Follow the quickstart guide</strong>
                  <p>Secure your first MCP server in 5 minutes</p>
                </div>
              </li>
            </ol>
          </div>

          <div class="actions">
            <a href="/docs/pro" class="btn btn-primary">
              View Documentation
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M5 12h14M12 5l7 7-7 7"/>
              </svg>
            </a>
            <a href="/docs/quickstart" class="btn btn-secondary">
              Quickstart Guide
            </a>
          </div>

          <div class="support-box">
            <h4>Need help?</h4>
            <p>Reply to your license email or contact <a href="mailto:support@mcp-guard.io">support&#64;mcp-guard.io</a></p>
            <p class="response-time">Average response time: 12 hours</p>
          </div>
        </div>

        <div class="pro-features">
          <h3>You now have access to:</h3>
          <ul class="feature-list">
            <li>
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"></path>
              </svg>
              <div>
                <strong>OAuth 2.1 Authentication</strong>
                <p>GitHub, Google, Okta, and custom providers</p>
              </div>
            </li>
            <li>
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path>
                <circle cx="9" cy="7" r="4"></circle>
                <path d="M23 21v-2a4 4 0 0 0-3-3.87"></path>
                <path d="M16 3.13a4 4 0 0 1 0 7.75"></path>
              </svg>
              <div>
                <strong>Per-Identity Rate Limiting</strong>
                <p>Fine-grained control for each user or service</p>
              </div>
            </li>
            <li>
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"></path>
                <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"></path>
              </svg>
              <div>
                <strong>HTTP & SSE Transports</strong>
                <p>Connect MCP servers over HTTP and Server-Sent Events</p>
              </div>
            </li>
            <li>
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"></path>
                <polyline points="22,6 12,13 2,6"></polyline>
              </svg>
              <div>
                <strong>Priority Email Support</strong>
                <p>48-hour SLA for all support inquiries</p>
              </div>
            </li>
          </ul>
        </div>
      </div>
    </section>
  `,
  styles: [`
    .success {
      min-height: 100vh;
      padding: var(--space-16) 0;
      background: var(--bg-primary);
      display: flex;
      align-items: center;
      justify-content: center;
    }

    .success-container {
      max-width: 1000px;
      margin: 0 auto;
      padding: 0 var(--container-px);
      display: grid;
      grid-template-columns: 1.2fr 0.8fr;
      gap: var(--space-12);

      @media (max-width: 900px) {
        grid-template-columns: 1fr;
      }
    }

    .success-card {
      background: var(--bg-secondary);
      border: 1px solid var(--border-subtle);
      border-radius: var(--radius-3xl);
      padding: var(--space-10);
      text-align: center;
    }

    .icon-container {
      width: var(--space-20);
      height: var(--space-20);
      background: rgba(78, 205, 196, 0.1);
      border-radius: var(--radius-full);
      display: flex;
      align-items: center;
      justify-content: center;
      margin: 0 auto var(--space-6);

      svg {
        width: var(--space-12);
        height: var(--space-12);
        color: var(--accent-cyan);
      }
    }

    h1 {
      font-family: var(--font-display);
      font-size: var(--text-4xl);
      font-weight: var(--weight-bold);
      margin-bottom: var(--space-2);
    }

    .subtitle {
      font-size: var(--text-lg);
      color: var(--text-secondary);
      margin-bottom: var(--space-10);
    }

    .next-steps {
      text-align: left;
      margin-bottom: var(--space-8);

      h3 {
        font-size: var(--text-xl);
        font-weight: var(--weight-semibold);
        margin-bottom: var(--space-6);
        text-align: center;
      }
    }

    .steps {
      list-style: none;
      display: flex;
      flex-direction: column;
      gap: var(--space-6);

      li {
        display: flex;
        gap: var(--space-4);
      }
    }

    .step-number {
      width: var(--space-8);
      height: var(--space-8);
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
      flex: 1;

      strong {
        display: block;
        margin-bottom: var(--space-1-5);
      }

      p {
        color: var(--text-muted);
        font-size: var(--text-sm);
        margin-bottom: var(--space-2);
      }
    }

    .install-command {
      background: var(--bg-primary);
      border: 1px solid var(--border-subtle);
      border-radius: var(--radius-lg);
      padding: var(--space-3) var(--space-4);
      font-family: var(--font-mono);
      font-size: var(--text-xs);
      color: var(--accent-cyan);
      overflow-x: auto;
      white-space: pre;
      margin-top: var(--space-2);
    }

    .actions {
      display: flex;
      gap: var(--space-3);
      margin-bottom: var(--space-8);

      @media (max-width: 640px) {
        flex-direction: column;
      }
    }

    .btn {
      flex: 1;
      display: flex;
      align-items: center;
      justify-content: center;
      gap: var(--space-2);
      padding: var(--space-3-5) var(--space-5);
      border-radius: var(--radius-xl);
      font-size: var(--text-base);
      font-weight: var(--weight-semibold);
      text-decoration: none;
      transition: all var(--duration-normal) var(--ease-out);

      svg {
        width: var(--icon-sm);
        height: var(--icon-sm);
      }
    }

    .btn-primary {
      background: var(--text-primary);
      color: var(--bg-primary);
      border: none;

      &:hover {
        transform: translateY(-2px);
        box-shadow: var(--shadow-lg);
      }
    }

    .btn-secondary {
      background: var(--bg-elevated);
      color: var(--text-primary);
      border: 1px solid var(--border-subtle);

      &:hover {
        background: var(--bg-hover);
        border-color: var(--border-accent);
      }
    }

    .support-box {
      padding: var(--space-5);
      background: var(--bg-elevated);
      border: 1px solid var(--border-subtle);
      border-radius: var(--radius-xl);

      h4 {
        font-size: var(--text-base);
        font-weight: var(--weight-semibold);
        margin-bottom: var(--space-2);
      }

      p {
        font-size: var(--text-sm);
        color: var(--text-secondary);
        margin-bottom: var(--space-1);

        a {
          color: var(--accent-cyan);
          text-decoration: none;

          &:hover {
            text-decoration: underline;
          }
        }
      }

      .response-time {
        color: var(--text-muted);
        font-size: var(--text-xs);
        margin-bottom: 0;
      }
    }

    .pro-features {
      h3 {
        font-size: var(--text-lg);
        font-weight: var(--weight-semibold);
        margin-bottom: var(--space-6);
      }
    }

    .feature-list {
      list-style: none;
      display: flex;
      flex-direction: column;
      gap: var(--space-5);

      li {
        display: flex;
        gap: var(--space-4);
        padding: var(--space-4);
        background: var(--bg-secondary);
        border: 1px solid var(--border-subtle);
        border-radius: var(--radius-xl);

        svg {
          width: var(--icon-lg);
          height: var(--icon-lg);
          color: var(--accent-cyan);
          flex-shrink: 0;
        }

        strong {
          display: block;
          font-size: var(--text-sm);
          margin-bottom: var(--space-0-5);
        }

        p {
          font-size: var(--text-xs);
          color: var(--text-muted);
        }
      }
    }
  `]
})
export class SuccessComponent implements OnInit {
  sessionId = signal<string | null>(null);

  constructor(private route: ActivatedRoute) {}

  ngOnInit() {
    this.route.queryParams.subscribe(params => {
      if (params['session_id']) {
        this.sessionId.set(params['session_id']);
      }
    });
  }
}
