import { Component, ChangeDetectionStrategy } from '@angular/core';

@Component({
  selector: 'app-changelog',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: `
    <div class="page-container">
      <h1>Changelog</h1>
      <div class="changelog-list">
        
        <div class="release">
          <div class="release-header">
            <h2>v1.0.0</h2>
            <span class="release-date">December 26, 2025</span>
          </div>
          
          <div class="release-content">
            <h3>Authentication</h3>
            <ul>
              <li>API Key authentication with SHA256 hashing</li>
              <li>JWT authentication with JWKS support</li>
              <li>OAuth 2.1 support (GitHub, Google, Okta)</li>
              <li>mTLS client certificate authentication</li>
            </ul>

            <h3>Rate Limiting</h3>
            <ul>
              <li>Per-identity rate limiting</li>
              <li>Token bucket algorithm implementation</li>
              <li>Configurable burst sizes and TTLs</li>
            </ul>

            <h3>Observability</h3>
            <ul>
              <li>Prometheus metrics endpoint</li>
              <li>OpenTelemetry distributed tracing</li>
              <li>Structured audit logging</li>
            </ul>
          </div>
        </div>

        <div class="release">
          <div class="release-header">
            <h2>v0.1.0</h2>
            <span class="release-date">December 14, 2024</span>
          </div>
          <div class="release-content">
            <p>Initial development release.</p>
          </div>
        </div>

      </div>
    </div>
  `,
  styles: [`
    .page-container {
      max-width: 800px;
      margin: 0 auto;
      padding: 4rem 2rem;
      margin-top: 80px;
    }

    h1 {
      font-size: 2.5rem;
      margin-bottom: 3rem;
      color: var(--text-primary, #111);
    }

    .release {
      margin-bottom: 4rem;
    }

    .release-header {
      display: flex;
      align-items: baseline;
      gap: 1rem;
      margin-bottom: 1.5rem;
      padding-bottom: 1rem;
      border-bottom: 1px solid var(--border-color, #eee);
    }

    .release-date {
      color: var(--text-secondary, #666);
      font-family: monospace;
    }

    h3 {
      font-size: 1.1rem;
      margin-top: 1.5rem;
      margin-bottom: 0.75rem;
      color: var(--text-primary, #111);
    }

    ul {
      padding-left: 1.5rem;
      margin-bottom: 1rem;
      color: var(--text-secondary, #666);
      line-height: 1.6;
      
      li {
        margin-bottom: 0.25rem;
      }
    }
  `]
})
export class ChangelogComponent { }
