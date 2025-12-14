import { Component, signal, HostListener } from '@angular/core';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-navbar',
  standalone: true,
  imports: [CommonModule],
  template: `
    <nav [class.scrolled]="isScrolled()">
      <div class="nav-container">
        <a href="#" class="logo">
          <div class="logo-icon">
            <svg viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4zm0 10.99h7c-.53 4.12-3.28 7.79-7 8.94V12H5V6.3l7-3.11v8.8z"/>
            </svg>
          </div>
          <span class="logo-text">mcp-guard</span>
        </a>

        <div class="nav-center">
          <a href="#features" class="nav-link">Features</a>
          <a href="#pricing" class="nav-link">Pricing</a>
          <a href="https://github.com/mcp-guard/mcp-guard" target="_blank" class="nav-link">GitHub</a>
          <a href="/docs" class="nav-link">Docs</a>
        </div>

        <div class="nav-actions">
          <a href="https://github.com/mcp-guard/mcp-guard" target="_blank" class="btn-github">
            <svg viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
            </svg>
            <span class="star-count">Star</span>
          </a>
          <a href="/docs/quickstart" class="btn-primary">Get Started</a>
        </div>
      </div>

      <!-- Progress bar -->
      <div class="scroll-progress">
        <div class="scroll-progress-bar" [style.width.%]="scrollProgress()"></div>
      </div>
    </nav>
  `,
  styles: [`
    nav {
      position: fixed;
      top: 0;
      left: 0;
      right: 0;
      z-index: 1000;
      padding: 16px 0;
      transition: all 0.3s ease;

      &::before {
        content: '';
        position: absolute;
        inset: 0;
        background: rgba(5, 5, 8, 0);
        backdrop-filter: blur(0);
        transition: all 0.3s ease;
        z-index: -1;
      }

      &.scrolled::before {
        background: rgba(5, 5, 8, 0.85);
        backdrop-filter: blur(20px);
        border-bottom: 1px solid var(--border-subtle);
      }
    }

    .nav-container {
      max-width: 1280px;
      margin: 0 auto;
      padding: 0 24px;
      display: flex;
      align-items: center;
      justify-content: space-between;
    }

    .logo {
      display: flex;
      align-items: center;
      gap: 10px;
      text-decoration: none;
      color: var(--text-primary);
    }

    .logo-icon {
      width: 38px;
      height: 38px;
      background: var(--gradient-brand);
      border-radius: 10px;
      display: flex;
      align-items: center;
      justify-content: center;
      box-shadow: 0 0 20px rgba(78, 205, 196, 0.3);

      svg {
        width: 22px;
        height: 22px;
        color: var(--bg-primary);
      }
    }

    .logo-text {
      font-family: var(--font-mono);
      font-weight: 600;
      font-size: 18px;
      letter-spacing: -0.02em;
    }

    .nav-center {
      display: flex;
      align-items: center;
      gap: 32px;

      @media (max-width: 900px) {
        display: none;
      }
    }

    .nav-link {
      color: var(--text-secondary);
      text-decoration: none;
      font-size: 14px;
      font-weight: 500;
      transition: color 0.2s;
      position: relative;

      &::after {
        content: '';
        position: absolute;
        bottom: -4px;
        left: 0;
        width: 0;
        height: 2px;
        background: var(--gradient-brand);
        transition: width 0.3s ease;
      }

      &:hover {
        color: var(--text-primary);

        &::after {
          width: 100%;
        }
      }
    }

    .nav-actions {
      display: flex;
      align-items: center;
      gap: 12px;
    }

    .btn-github {
      display: flex;
      align-items: center;
      gap: 8px;
      padding: 8px 14px;
      background: transparent;
      border: 1px solid var(--border-medium);
      border-radius: 8px;
      color: var(--text-secondary);
      text-decoration: none;
      font-size: 13px;
      font-weight: 500;
      transition: all 0.2s;

      svg {
        width: 18px;
        height: 18px;
      }

      &:hover {
        background: var(--bg-elevated);
        color: var(--text-primary);
        border-color: var(--border-accent);
      }

      @media (max-width: 600px) {
        .star-count {
          display: none;
        }
      }
    }

    .btn-primary {
      display: flex;
      align-items: center;
      gap: 8px;
      padding: 10px 20px;
      background: var(--text-primary);
      color: var(--bg-primary);
      text-decoration: none;
      font-size: 14px;
      font-weight: 600;
      border-radius: 8px;
      transition: all 0.2s;

      &:hover {
        transform: translateY(-1px);
        box-shadow: 0 8px 24px -8px rgba(248, 250, 252, 0.3);
      }

      @media (max-width: 600px) {
        padding: 8px 14px;
        font-size: 13px;
      }
    }

    .scroll-progress {
      position: absolute;
      bottom: 0;
      left: 0;
      right: 0;
      height: 2px;
      background: transparent;
    }

    .scroll-progress-bar {
      height: 100%;
      background: var(--gradient-brand);
      transition: width 0.1s linear;
    }
  `]
})
export class NavbarComponent {
  isScrolled = signal(false);
  scrollProgress = signal(0);

  @HostListener('window:scroll')
  onScroll() {
    const scrollTop = window.scrollY;
    const docHeight = document.documentElement.scrollHeight - window.innerHeight;

    this.isScrolled.set(scrollTop > 50);
    this.scrollProgress.set((scrollTop / docHeight) * 100);
  }
}
