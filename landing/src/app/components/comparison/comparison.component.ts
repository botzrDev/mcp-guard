import { Component, signal, ChangeDetectionStrategy } from '@angular/core';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-comparison',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule],
  template: `
    <section class="comparison">
      <div class="comparison-container">
        <div class="section-header">
          <span class="section-tag">// Before & After</span>
          <h2 class="section-title">From exposed to <span class="gradient-text">enterprise-secure</span></h2>
          <p class="section-subtitle">
            Most MCP servers are deployed with zero authentication. Toggle to see the difference.
          </p>
        </div>

        <!-- Toggle Switch -->
        <div class="toggle-container">
          <button
            class="toggle-btn"
            [class.active]="!isSecured()"
            (click)="isSecured.set(false)"
          >
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="10"></circle>
              <line x1="15" y1="9" x2="9" y2="15"></line>
              <line x1="9" y1="9" x2="15" y2="15"></line>
            </svg>
            Exposed
          </button>
          <div class="toggle-switch" (click)="toggleSecured()">
            <div class="toggle-thumb" [class.secured]="isSecured()"></div>
          </div>
          <button
            class="toggle-btn"
            [class.active]="isSecured()"
            (click)="isSecured.set(true)"
          >
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
              <polyline points="22 4 12 14.01 9 11.01"></polyline>
            </svg>
            Secured
          </button>
        </div>

        <!-- Code Display with Diagonal Split -->
        <div class="code-display" [class.secured]="isSecured()">
          <div class="code-panel exposed-panel">
            <div class="panel-header">
              <div class="status-indicator danger">
                <span class="pulse-dot"></span>
                Vulnerable
              </div>
            </div>
            <div class="code-content">
              <pre><code><span class="comment"># Anyone can connect - no authentication</span>
<span class="prompt">$</span> <span class="cmd">curl</span> http://your-mcp-server:8080

<span class="comment"># Direct access to all tools</span>
<span class="output">{{"{"}} "method": "tools/call",</span>
<span class="output">  "params": {{"{"}} "name": "database_query" {{"}}"}} {{"}"}}</span>

<span class="error">⚠ No auth required</span>
<span class="error">⚠ No rate limiting</span>
<span class="error">⚠ No audit trail</span>
<span class="error">⚠ All tools exposed</span></code></pre>
            </div>
          </div>

          <div class="diagonal-divider">
            <div class="shield-icon" [class.active]="isSecured()">
              <svg viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4zm0 10.99h7c-.53 4.12-3.28 7.79-7 8.94V12H5V6.3l7-3.11v8.8z"/>
              </svg>
            </div>
          </div>

          <div class="code-panel secured-panel">
            <div class="panel-header">
              <div class="status-indicator success">
                <span class="pulse-dot"></span>
                Protected
              </div>
            </div>
            <div class="code-content">
              <pre><code><span class="comment"># OAuth 2.1 authentication required</span>
<span class="prompt">$</span> <span class="cmd">curl</span> -H <span class="string">"Authorization: Bearer &lt;token&gt;"</span> \
    http://mcp-guard:3000

<span class="success">✓ Identity verified: user@company.com</span>
<span class="success">✓ Rate limit: 847/1000 remaining</span>
<span class="success">✓ Audit logged: req_abc123</span>
<span class="success">✓ Tool access: authorized</span></code></pre>
            </div>
          </div>
        </div>

        <!-- Security checklist -->
        <div class="security-checklist" [class.visible]="isSecured()">
          <div class="checklist-item">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="20 6 9 17 4 12"></polyline>
            </svg>
            <span>OAuth 2.1 / JWT / API Key authentication</span>
          </div>
          <div class="checklist-item">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="20 6 9 17 4 12"></polyline>
            </svg>
            <span>Per-identity rate limiting with Retry-After</span>
          </div>
          <div class="checklist-item">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="20 6 9 17 4 12"></polyline>
            </svg>
            <span>Full audit logging with secret redaction</span>
          </div>
          <div class="checklist-item">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="20 6 9 17 4 12"></polyline>
            </svg>
            <span>Tool-level authorization by role/scope</span>
          </div>
        </div>
      </div>
    </section>
  `,
  styles: [`
    .comparison {
      position: relative;
      padding: 120px 0;
      background: var(--bg-secondary);
      overflow: hidden;

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

    .comparison-container {
      max-width: 1100px;
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

    .toggle-container {
      display: flex;
      align-items: center;
      justify-content: center;
      gap: 20px;
      margin-bottom: 48px;
    }

    .toggle-btn {
      display: flex;
      align-items: center;
      gap: 8px;
      padding: 10px 20px;
      background: transparent;
      border: 1px solid var(--border-subtle);
      border-radius: 10px;
      color: var(--text-muted);
      font-size: 14px;
      font-weight: 500;
      cursor: pointer;
      transition: all 0.3s;

      svg {
        width: 18px;
        height: 18px;
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

    .toggle-switch {
      width: 56px;
      height: 28px;
      background: var(--bg-elevated);
      border: 1px solid var(--border-subtle);
      border-radius: 100px;
      cursor: pointer;
      position: relative;
      transition: all 0.3s;

      &:hover {
        border-color: var(--border-accent);
      }
    }

    .toggle-thumb {
      position: absolute;
      top: 3px;
      left: 3px;
      width: 20px;
      height: 20px;
      background: var(--accent-red);
      border-radius: 50%;
      transition: all 0.3s ease;
      box-shadow: 0 2px 8px rgba(239, 68, 68, 0.4);

      &.secured {
        left: calc(100% - 23px);
        background: var(--accent-cyan);
        box-shadow: 0 2px 8px rgba(78, 205, 196, 0.4);
      }
    }

    .code-display {
      position: relative;
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 0;
      background: var(--bg-primary);
      border: 1px solid var(--border-subtle);
      border-radius: 20px;
      overflow: hidden;

      @media (max-width: 768px) {
        grid-template-columns: 1fr;
      }
    }

    .code-panel {
      padding: 24px;
      transition: opacity 0.5s ease;
    }

    .exposed-panel {
      border-right: 1px solid var(--border-subtle);
      background: linear-gradient(135deg, rgba(239, 68, 68, 0.03) 0%, transparent 50%);

      @media (max-width: 768px) {
        border-right: none;
        border-bottom: 1px solid var(--border-subtle);
      }
    }

    .secured-panel {
      background: linear-gradient(135deg, transparent 50%, rgba(78, 205, 196, 0.03) 100%);
    }

    .code-display.secured {
      .exposed-panel {
        opacity: 0.4;
      }

      .secured-panel {
        opacity: 1;
      }
    }

    .code-display:not(.secured) {
      .exposed-panel {
        opacity: 1;
      }

      .secured-panel {
        opacity: 0.4;
      }
    }

    .panel-header {
      margin-bottom: 16px;
    }

    .status-indicator {
      display: inline-flex;
      align-items: center;
      gap: 8px;
      padding: 6px 14px;
      border-radius: 100px;
      font-size: 12px;
      font-weight: 600;

      &.danger {
        background: rgba(239, 68, 68, 0.1);
        color: var(--accent-red);
        border: 1px solid rgba(239, 68, 68, 0.2);
      }

      &.success {
        background: rgba(78, 205, 196, 0.1);
        color: var(--accent-cyan);
        border: 1px solid rgba(78, 205, 196, 0.2);
      }
    }

    .pulse-dot {
      width: 8px;
      height: 8px;
      border-radius: 50%;
      background: currentColor;
      animation: pulse 2s infinite;
    }

    .code-content {
      pre {
        margin: 0;
        font-family: var(--font-mono);
        font-size: 13px;
        line-height: 1.8;
        white-space: pre-wrap;
      }

      .comment { color: var(--text-dim); }
      .prompt { color: var(--accent-cyan); }
      .cmd { color: var(--text-primary); }
      .string { color: #a5d6a7; }
      .output { color: var(--text-muted); }
      .error { color: var(--accent-red); }
      .success { color: #4ade80; }
    }

    .diagonal-divider {
      position: absolute;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      z-index: 10;

      @media (max-width: 768px) {
        display: none;
      }
    }

    .shield-icon {
      width: 56px;
      height: 56px;
      background: var(--bg-secondary);
      border: 2px solid var(--border-subtle);
      border-radius: 50%;
      display: flex;
      align-items: center;
      justify-content: center;
      color: var(--text-muted);
      transition: all 0.4s ease;

      svg {
        width: 28px;
        height: 28px;
      }

      &.active {
        border-color: var(--accent-cyan);
        color: var(--accent-cyan);
        box-shadow: 0 0 30px rgba(78, 205, 196, 0.3);
      }
    }

    .security-checklist {
      display: grid;
      grid-template-columns: repeat(2, 1fr);
      gap: 16px;
      margin-top: 32px;
      opacity: 0.3;
      transition: opacity 0.5s ease;

      &.visible {
        opacity: 1;
      }

      @media (max-width: 640px) {
        grid-template-columns: 1fr;
      }
    }

    .checklist-item {
      display: flex;
      align-items: center;
      gap: 12px;
      padding: 14px 18px;
      background: var(--bg-primary);
      border: 1px solid var(--border-subtle);
      border-radius: 10px;
      font-size: 14px;
      color: var(--text-secondary);

      svg {
        width: 20px;
        height: 20px;
        color: var(--accent-cyan);
        flex-shrink: 0;
      }
    }

    @keyframes pulse {
      0%, 100% { opacity: 1; transform: scale(1); }
      50% { opacity: 0.5; transform: scale(0.9); }
    }
  `]
})
export class ComparisonComponent {
  isSecured = signal(true);

  toggleSecured() {
    this.isSecured.update(v => !v);
  }
}
