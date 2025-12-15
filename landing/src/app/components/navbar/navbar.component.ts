import { Component, signal, ChangeDetectionStrategy, NgZone, OnInit, OnDestroy, inject, HostListener } from '@angular/core';
import { CommonModule } from '@angular/common';
import { IconComponent } from '../../shared/icon/icon.component';

@Component({
  selector: 'app-navbar',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, IconComponent],
  template: `
    <nav [class.scrolled]="isScrolled()">
      <div class="nav-container">
        <a href="#" class="logo">
          <div class="logo-icon">
            <app-icon name="logo" size="22px" />
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
            <app-icon name="github" size="18px" />
            <span class="star-count">Star</span>
          </a>
          <a href="/docs/quickstart" class="btn-primary desktop-only">Get Started</a>

          <!-- Mobile menu button -->
          <button
            class="mobile-menu-btn"
            (click)="toggleMobileMenu()"
            [attr.aria-expanded]="isMobileMenuOpen()"
            aria-label="Toggle mobile menu"
          >
            <span class="hamburger" [class.open]="isMobileMenuOpen()">
              <span></span>
              <span></span>
              <span></span>
            </span>
          </button>
        </div>
      </div>

      <!-- Progress bar -->
      <div class="scroll-progress">
        <div class="scroll-progress-bar" [style.width.%]="scrollProgress()"></div>
      </div>
    </nav>

    <!-- Mobile menu drawer -->
    <div class="mobile-drawer" [class.open]="isMobileMenuOpen()">
      <div class="drawer-backdrop" (click)="closeMobileMenu()"></div>
      <div class="drawer-content">
        <nav class="drawer-nav">
          <a href="#features" class="drawer-link" (click)="closeMobileMenu()">
            <app-icon name="features" size="20px" />
            Features
          </a>
          <a href="#pricing" class="drawer-link" (click)="closeMobileMenu()">
            <app-icon name="pricing" size="20px" />
            Pricing
          </a>
          <a href="https://github.com/mcp-guard/mcp-guard" target="_blank" class="drawer-link" (click)="closeMobileMenu()">
            <app-icon name="github" size="20px" />
            GitHub
          </a>
          <a href="/docs" class="drawer-link" (click)="closeMobileMenu()">
            <app-icon name="docs" size="20px" />
            Documentation
          </a>
        </nav>
        <div class="drawer-cta">
          <a href="/docs/quickstart" class="btn-primary-large" (click)="closeMobileMenu()">
            Get Started
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M5 12h14M12 5l7 7-7 7"/>
            </svg>
          </a>
        </div>
      </div>
    </div>
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

    .desktop-only {
      @media (max-width: 900px) {
        display: none;
      }
    }

    // Mobile menu button
    .mobile-menu-btn {
      display: none;
      align-items: center;
      justify-content: center;
      width: 44px;
      height: 44px;
      background: transparent;
      border: 1px solid var(--border-subtle);
      border-radius: 10px;
      cursor: pointer;
      transition: all 0.2s;

      @media (max-width: 900px) {
        display: flex;
      }

      &:hover {
        background: var(--bg-elevated);
        border-color: var(--border-accent);
      }
    }

    .hamburger {
      display: flex;
      flex-direction: column;
      gap: 5px;
      width: 20px;

      span {
        display: block;
        height: 2px;
        background: var(--text-primary);
        border-radius: 2px;
        transition: all 0.3s ease;
      }

      &.open {
        span:nth-child(1) {
          transform: rotate(45deg) translate(5px, 5px);
        }
        span:nth-child(2) {
          opacity: 0;
        }
        span:nth-child(3) {
          transform: rotate(-45deg) translate(5px, -5px);
        }
      }
    }

    // Mobile drawer
    .mobile-drawer {
      position: fixed;
      inset: 0;
      z-index: 999;
      pointer-events: none;
      visibility: hidden;

      &.open {
        pointer-events: auto;
        visibility: visible;

        .drawer-backdrop {
          opacity: 1;
        }

        .drawer-content {
          transform: translateX(0);
        }
      }
    }

    .drawer-backdrop {
      position: absolute;
      inset: 0;
      background: rgba(0, 0, 0, 0.6);
      backdrop-filter: blur(4px);
      opacity: 0;
      transition: opacity 0.3s ease;
    }

    .drawer-content {
      position: absolute;
      top: 0;
      right: 0;
      bottom: 0;
      width: 280px;
      max-width: 80vw;
      background: var(--bg-secondary);
      border-left: 1px solid var(--border-subtle);
      transform: translateX(100%);
      transition: transform 0.3s ease;
      display: flex;
      flex-direction: column;
      padding: 80px 24px 24px;
    }

    .drawer-nav {
      display: flex;
      flex-direction: column;
      gap: 8px;
      flex: 1;
    }

    .drawer-link {
      display: flex;
      align-items: center;
      gap: 14px;
      padding: 14px 16px;
      color: var(--text-secondary);
      text-decoration: none;
      font-size: 15px;
      font-weight: 500;
      border-radius: 12px;
      transition: all 0.2s;

      &:hover {
        background: var(--bg-elevated);
        color: var(--text-primary);
      }
    }

    .drawer-cta {
      padding-top: 24px;
      border-top: 1px solid var(--border-subtle);
    }

    .btn-primary-large {
      display: flex;
      align-items: center;
      justify-content: center;
      gap: 10px;
      width: 100%;
      padding: 16px 24px;
      background: var(--gradient-brand);
      color: var(--bg-primary);
      text-decoration: none;
      font-size: 15px;
      font-weight: 600;
      border-radius: 12px;
      transition: all 0.2s;

      svg {
        width: 18px;
        height: 18px;
      }

      &:hover {
        transform: translateY(-2px);
        box-shadow: 0 8px 24px rgba(78, 205, 196, 0.3);
      }
    }
  `]
})
export class NavbarComponent implements OnInit, OnDestroy {
  isScrolled = signal(false);
  scrollProgress = signal(0);
  isMobileMenuOpen = signal(false);

  private ngZone = inject(NgZone);

  toggleMobileMenu() {
    this.isMobileMenuOpen.update((v) => !v);
    // Prevent body scroll when menu is open
    document.body.style.overflow = this.isMobileMenuOpen() ? 'hidden' : '';
  }

  closeMobileMenu() {
    this.isMobileMenuOpen.set(false);
    document.body.style.overflow = '';
  }

  @HostListener('window:keydown.escape')
  onEscapeKey() {
    if (this.isMobileMenuOpen()) {
      this.closeMobileMenu();
    }
  }

  ngOnInit() {
    this.ngZone.runOutsideAngular(() => {
      window.addEventListener('scroll', this.onScroll, { passive: true });
    });
  }

  ngOnDestroy() {
    window.removeEventListener('scroll', this.onScroll);
  }

  onScroll = () => {
    const scrollTop = window.scrollY;

    // Optimization: Calculate values first
    const isScrolled = scrollTop > 50;

    // Only re-enter zone if state needs to change (Signals handle equality checking but we want to avoid Zone entry if possible)
    if (this.isScrolled() !== isScrolled) {
      this.ngZone.run(() => {
        this.isScrolled.set(isScrolled);
      });
    }

    // Scroll progress runs frequently, so we might want to throttle this or accept it updates often.
    // Since it's outside zone, updating the signal *should* trigger view update if using standard change detection,
    // but with OnPush and signals, we typically need to be in zone or manually detect changes.
    // For smoothness, we can run this inside zone, or rely on the fact that signal updates schedule a microtask?
    // Let's wrapping it in run() for consistency with Angular's expected behavior.

    const docHeight = document.documentElement.scrollHeight - window.innerHeight;
    const progress = (scrollTop / docHeight) * 100;

    // To avoid excessive zone entries for progress bar (60fps), we could use requestAnimationFrame
    // But for now let's just use the zone run, effectively throttling by the browser's scroll event rate (which is high)
    // Actually, let's keep progress update lightweight.
    this.ngZone.run(() => {
      this.scrollProgress.set(progress);
    });
  }
}
