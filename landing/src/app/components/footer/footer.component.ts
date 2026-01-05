import { Component, ChangeDetectionStrategy } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterModule } from '@angular/router';

@Component({
  selector: 'app-footer',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, RouterModule],
  template: `
    <footer class="footer">
      <div class="footer-container">
        <div class="footer-main">
          <div class="footer-brand">
            <a routerLink="/" class="logo">
              <img src="gateway.png" alt="MCP Guard" class="logo-img" />
              <span class="logo-text">mcp-guard</span>
            </a>
            <p class="brand-tagline">
              Secure your MCP servers in 5 minutes.<br>
              No Docker. No Kubernetes. No DevOps team.
            </p>
          </div>

          <div class="footer-links">
            <div class="link-group">
              <h4>Product</h4>
              <a routerLink="/" fragment="features">Features</a>
              <a routerLink="/" fragment="pricing">Pricing</a>
              <a routerLink="/docs">Documentation</a>
              <a routerLink="/changelog">Changelog</a>
            </div>

            <div class="link-group">
              <h4>Resources</h4>
              <a routerLink="/docs/quickstart">Quick Start</a>
              <a routerLink="/docs/configuration">Configuration</a>
              <a routerLink="/docs/api">API Reference</a>
              <a routerLink="/blog">Blog</a>
            </div>

            <div class="link-group">
              <h4>Company</h4>
              <a routerLink="/about">About</a>
              <a routerLink="/contact">Contact</a>
              <a routerLink="/privacy">Privacy</a>
              <a routerLink="/terms">Terms</a>
            </div>

            <div class="link-group">
              <h4>Community</h4>
              <a href="https://github.com/botzrdev/mcp-guard" target="_blank">
                <svg viewBox="0 0 24 24" fill="currentColor">
                  <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                </svg>
                GitHub
              </a>
              <a href="https://discord.gg/krRa4GVx" target="_blank">
                <svg viewBox="0 0 24 24" fill="currentColor">
                  <path d="M20.317 4.37a19.791 19.791 0 0 0-4.885-1.515.074.074 0 0 0-.079.037c-.21.375-.444.864-.608 1.25a18.27 18.27 0 0 0-5.487 0 12.64 12.64 0 0 0-.617-1.25.077.077 0 0 0-.079-.037A19.736 19.736 0 0 0 3.677 4.37a.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.057a.082.082 0 0 0 .031.057 19.9 19.9 0 0 0 5.993 3.03.078.078 0 0 0 .084-.028 14.09 14.09 0 0 0 1.226-1.994.076.076 0 0 0-.041-.106 13.107 13.107 0 0 1-1.872-.892.077.077 0 0 1-.008-.128 10.2 10.2 0 0 0 .372-.292.074.074 0 0 1 .077-.01c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 0 1 .078.01c.12.098.246.198.373.292a.077.077 0 0 1-.006.127 12.299 12.299 0 0 1-1.873.892.077.077 0 0 0-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 0 0 .084.028 19.839 19.839 0 0 0 6.002-3.03.077.077 0 0 0 .032-.054c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 0 0-.031-.03zM8.02 15.33c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.956-2.419 2.157-2.419 1.21 0 2.176 1.086 2.157 2.419 0 1.334-.956 2.419-2.157 2.419zm7.975 0c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.955-2.419 2.157-2.419 1.21 0 2.176 1.086 2.157 2.419 0 1.334-.946 2.419-2.157 2.419z"/>
                </svg>
                Discord
              </a>
              <a href="https://twitter.com/mcp_guard" target="_blank">
                <svg viewBox="0 0 24 24" fill="currentColor">
                  <path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"/>
                </svg>
                Twitter
              </a>
            </div>
          </div>
        </div>

        <div class="footer-bottom">
          <p class="copyright">
            © {{ currentYear }} mcp-guard. Open source under AGPL-3.0.
          </p>
          <div class="footer-badges">
            <a href="https://github.com/botzrdev/mcp-guard" target="_blank" class="badge">
              <svg viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"/>
              </svg>
              Built with Rust
            </a>
          </div>
        </div>
      </div>
    </footer>
  `,
  styles: [`
    .footer {
      position: relative;
      padding: var(--space-20) 0 var(--space-10);
      background: var(--bg-primary);
      border-top: 1px solid var(--border-subtle);
    }

    .footer-container {
      max-width: 1280px;
      margin: 0 auto;
      padding: 0 var(--container-px);
    }

    .footer-main {
      display: grid;
      grid-template-columns: 1.5fr 2fr;
      gap: var(--space-16);
      margin-bottom: var(--space-12);

      @media (max-width: 900px) {
        grid-template-columns: 1fr;
        gap: var(--space-12);
      }
    }

    .footer-brand {
      .logo {
        display: flex;
        align-items: center;
        gap: var(--space-2-5);
        text-decoration: none;
        color: var(--text-primary);
        margin-bottom: var(--space-4);
      }

      .logo-img {
        width: var(--space-9);
        height: var(--space-9);
        object-fit: contain;
      }

      .logo-text {
        font-family: var(--font-mono);
        font-weight: var(--weight-semibold);
        font-size: var(--text-lg);
        letter-spacing: var(--tracking-tight);
        line-height: var(--leading-normal);
      }
    }

    .brand-tagline {
      color: var(--text-muted);
      font-size: var(--text-sm);
      line-height: var(--leading-relaxed);
      max-width: 280px;
    }

    .footer-links {
      display: grid;
      grid-template-columns: repeat(4, 1fr);
      gap: var(--space-8);

      @media (max-width: 768px) {
        grid-template-columns: repeat(2, 1fr);
      }

      @media (max-width: 500px) {
        grid-template-columns: 1fr;
      }
    }

    .link-group {
      h4 {
        font-size: var(--text-xs);
        font-weight: var(--weight-semibold);
        color: var(--text-primary);
        margin-bottom: var(--space-4);
        text-transform: uppercase;
        letter-spacing: var(--tracking-wider);
        line-height: var(--leading-normal);
      }

      a {
        display: flex;
        align-items: center;
        gap: var(--space-2);
        color: var(--text-muted);
        text-decoration: none;
        font-size: var(--text-sm);
        line-height: var(--leading-normal);
        padding: var(--space-1-5) 0;
        transition: color var(--duration-fast) var(--ease-out);

        svg {
          width: var(--icon-sm);
          height: var(--icon-sm);
        }

        &:hover {
          color: var(--text-primary);
        }
      }
    }

    .footer-bottom {
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding-top: var(--space-8);
      border-top: 1px solid var(--border-subtle);

      @media (max-width: 640px) {
        flex-direction: column;
        gap: var(--space-4);
        text-align: center;
      }
    }

    .copyright {
      color: var(--text-muted);
      font-size: var(--text-xs);
      line-height: var(--leading-normal);
    }

    .footer-badges {
      display: flex;
      gap: var(--space-3);
    }

    .badge {
      display: flex;
      align-items: center;
      gap: var(--space-1-5);
      padding: var(--space-1-5) var(--space-3);
      background: var(--bg-elevated);
      border: 1px solid var(--border-subtle);
      border-radius: var(--radius-md);
      color: var(--text-muted);
      text-decoration: none;
      font-size: var(--text-xs);
      line-height: var(--leading-normal);
      transition: all var(--duration-fast) var(--ease-out);

      svg {
        width: var(--icon-xs);
        height: var(--icon-xs);
      }

      &:hover {
        border-color: var(--border-accent);
        color: var(--text-primary);
      }
    }
  `]
})
export class FooterComponent {
  currentYear = new Date().getFullYear();
}
