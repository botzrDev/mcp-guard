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
              <div class="logo-icon">
                <svg viewBox="0 0 24 24" fill="currentColor">
                  <path d="M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4zm0 10.99h7c-.53 4.12-3.28 7.79-7 8.94V12H5V6.3l7-3.11v8.8z"/>
                </svg>
              </div>
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
              <a href="https://github.com/mcp-guard/mcp-guard" target="_blank">
                <svg viewBox="0 0 24 24" fill="currentColor">
                  <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                </svg>
                GitHub
              </a>
              <a href="https://twitter.com/mcp_guard" target="_blank">
                <svg viewBox="0 0 24 24" fill="currentColor">
                  <path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"/>
                </svg>
                Twitter
              </a>
              <a href="https://discord.gg/mcp-guard" target="_blank">
                <svg viewBox="0 0 24 24" fill="currentColor">
                  <path d="M20.317 4.3698a19.7913 19.7913 0 00-4.8851-1.5152.0741.0741 0 00-.0785.0371c-.211.3753-.4447.8648-.6083 1.2495-1.8447-.2762-3.68-.2762-5.4868 0-.1636-.3933-.4058-.8742-.6177-1.2495a.077.077 0 00-.0785-.037 19.7363 19.7363 0 00-4.8852 1.515.0699.0699 0 00-.0321.0277C.5334 9.0458-.319 13.5799.0992 18.0578a.0824.0824 0 00.0312.0561c2.0528 1.5076 4.0413 2.4228 5.9929 3.0294a.0777.0777 0 00.0842-.0276c.4616-.6304.8731-1.2952 1.226-1.9942a.076.076 0 00-.0416-.1057c-.6528-.2476-1.2743-.5495-1.8722-.8923a.077.077 0 01-.0076-.1277c.1258-.0943.2517-.1923.3718-.2914a.0743.0743 0 01.0776-.0105c3.9278 1.7933 8.18 1.7933 12.0614 0a.0739.0739 0 01.0785.0095c.1202.099.246.1981.3728.2924a.077.077 0 01-.0066.1276 12.2986 12.2986 0 01-1.873.8914.0766.0766 0 00-.0407.1067c.3604.698.7719 1.3628 1.225 1.9932a.076.076 0 00.0842.0286c1.961-.6067 3.9495-1.5219 6.0023-3.0294a.077.077 0 00.0313-.0552c.5004-5.177-.8382-9.6739-3.5485-13.6604a.061.061 0 00-.0312-.0286zM8.02 15.3312c-1.1825 0-2.1569-1.0857-2.1569-2.419 0-1.3332.9555-2.4189 2.157-2.4189 1.2108 0 2.1757 1.0952 2.1568 2.419 0 1.3332-.9555 2.4189-2.1569 2.4189zm7.9748 0c-1.1825 0-2.1569-1.0857-2.1569-2.419 0-1.3332.9554-2.4189 2.1569-2.4189 1.2108 0 2.1757 1.0952 2.1568 2.419 0 1.3332-.946 2.4189-2.1568 2.4189z"/>
                </svg>
                Discord
              </a>
            </div>
          </div>
        </div>

        <div class="footer-bottom">
          <p class="copyright">
            © {{ currentYear }} mcp-guard. Open source under AGPL-3.0.
          </p>
          <div class="footer-badges">
            <a href="https://github.com/mcp-guard/mcp-guard" target="_blank" class="badge">
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

      .logo-icon {
        width: var(--space-9);
        height: var(--space-9);
        background: var(--gradient-brand);
        border-radius: var(--radius-lg);
        display: flex;
        align-items: center;
        justify-content: center;

        svg {
          width: var(--icon-md);
          height: var(--icon-md);
          color: var(--bg-primary);
        }
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
