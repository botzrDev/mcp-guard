import { Component, signal, ChangeDetectionStrategy, NgZone, OnInit, OnDestroy, inject } from '@angular/core';
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
export class NavbarComponent implements OnInit, OnDestroy {
  isScrolled = signal(false);
  scrollProgress = signal(0);

  private ngZone = inject(NgZone);

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
