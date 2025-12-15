import {
  Component,
  signal,
  ChangeDetectionStrategy,
  OnInit,
  OnDestroy,
  inject,
  NgZone,
  ElementRef,
} from '@angular/core';
import { CommonModule } from '@angular/common';
import { IconComponent } from '../../shared/icon/icon.component';

interface Step {
  number: number;
  title: string;
  description: string;
  icon: 'terminal' | 'config' | 'rocket';
  code: string;
}

@Component({
  selector: 'app-how-it-works',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [CommonModule, IconComponent],
  template: `
    <section class="how-it-works" id="how-it-works">
      <div class="hiw-container">
        <div class="section-header">
          <span class="section-tag">// Quick Start</span>
          <h2 class="section-title">Up and running in <span class="gradient-text">3 steps</span></h2>
          <p class="section-subtitle">
            From zero to secure in under 5 minutes. No complex setup required.
          </p>
        </div>

        <div class="steps-container">
          <!-- Progress line -->
          <div class="progress-line">
            <div class="progress-fill" [style.width.%]="progressWidth()"></div>
          </div>

          @for (step of steps; track step.number; let i = $index) {
            <div
              class="step-card"
              [class.active]="activeStep() >= i"
              [class.current]="activeStep() === i"
              [attr.data-step]="i"
            >
              <div class="step-number">
                <span class="number-circle" [class.completed]="activeStep() > i">
                  @if (activeStep() > i) {
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
                      <polyline points="20 6 9 17 4 12"/>
                    </svg>
                  } @else {
                    {{ step.number }}
                  }
                </span>
              </div>

              <div class="step-content">
                <div class="step-header">
                  <div class="step-icon">
                    <app-icon [name]="step.icon" size="20px" />
                  </div>
                  <h3 class="step-title">{{ step.title }}</h3>
                </div>
                <p class="step-description">{{ step.description }}</p>

                <div class="step-code">
                  <div class="code-header">
                    <div class="code-dots">
                      <span></span>
                      <span></span>
                      <span></span>
                    </div>
                  </div>
                  <pre><code>{{ step.code }}</code></pre>
                </div>
              </div>
            </div>
          }
        </div>

        <!-- CTA after steps -->
        <div class="hiw-cta" [class.visible]="activeStep() >= 2">
          <p>Ready to secure your MCP servers?</p>
          <a href="/docs/quickstart" class="cta-btn">
            Start Now
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M5 12h14M12 5l7 7-7 7"/>
            </svg>
          </a>
        </div>
      </div>
    </section>
  `,
  styles: [`
    .how-it-works {
      position: relative;
      padding: 120px 0;
      background: var(--bg-secondary);

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

    .hiw-container {
      max-width: 900px;
      margin: 0 auto;
      padding: 0 24px;
    }

    .section-header {
      text-align: center;
      margin-bottom: 64px;
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
      font-family: var(--font-display);
      font-size: clamp(32px, 5vw, 52px);
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
      max-width: 500px;
      margin: 0 auto;
      line-height: 1.6;
    }

    .steps-container {
      position: relative;
      display: flex;
      flex-direction: column;
      gap: 32px;
    }

    .progress-line {
      position: absolute;
      left: 28px;
      top: 40px;
      bottom: 40px;
      width: 2px;
      background: var(--border-subtle);
      border-radius: 2px;

      @media (max-width: 640px) {
        left: 20px;
      }
    }

    .progress-fill {
      width: 100%;
      background: var(--gradient-brand);
      border-radius: 2px;
      transition: height 0.5s ease;
    }

    .step-card {
      display: grid;
      grid-template-columns: 60px 1fr;
      gap: 24px;
      opacity: 0.4;
      transform: translateX(-20px);
      transition: all 0.5s ease;

      @media (max-width: 640px) {
        grid-template-columns: 44px 1fr;
        gap: 16px;
      }

      &.active {
        opacity: 1;
        transform: translateX(0);
      }

      &.current {
        .number-circle {
          background: var(--gradient-brand);
          color: var(--bg-primary);
          box-shadow: 0 0 20px rgba(78, 205, 196, 0.4);
        }

        .step-content {
          background: linear-gradient(135deg, rgba(78, 205, 196, 0.05) 0%, transparent 50%);
        }
      }
    }

    .step-number {
      display: flex;
      justify-content: center;
      padding-top: 24px;
    }

    .number-circle {
      width: 56px;
      height: 56px;
      border-radius: 50%;
      background: var(--bg-elevated);
      border: 2px solid var(--border-subtle);
      display: flex;
      align-items: center;
      justify-content: center;
      font-family: var(--font-mono);
      font-size: 20px;
      font-weight: 700;
      color: var(--text-muted);
      transition: all 0.4s ease;

      @media (max-width: 640px) {
        width: 44px;
        height: 44px;
        font-size: 16px;
      }

      &.completed {
        background: var(--accent-cyan);
        border-color: var(--accent-cyan);
        color: var(--bg-primary);

        svg {
          width: 24px;
          height: 24px;

          @media (max-width: 640px) {
            width: 20px;
            height: 20px;
          }
        }
      }
    }

    .step-content {
      background: var(--bg-primary);
      border: 1px solid var(--border-subtle);
      border-radius: 16px;
      padding: 24px;
      transition: all 0.3s ease;
    }

    .step-header {
      display: flex;
      align-items: center;
      gap: 12px;
      margin-bottom: 12px;
    }

    .step-icon {
      width: 36px;
      height: 36px;
      background: var(--bg-elevated);
      border-radius: 10px;
      display: flex;
      align-items: center;
      justify-content: center;
      color: var(--accent-cyan);
    }

    .step-title {
      font-size: 18px;
      font-weight: 600;
      letter-spacing: -0.01em;
    }

    .step-description {
      color: var(--text-secondary);
      font-size: 14px;
      line-height: 1.6;
      margin-bottom: 16px;
    }

    .step-code {
      background: var(--bg-secondary);
      border: 1px solid var(--border-subtle);
      border-radius: 10px;
      overflow: hidden;
    }

    .code-header {
      padding: 10px 14px;
      border-bottom: 1px solid var(--border-subtle);
    }

    .code-dots {
      display: flex;
      gap: 6px;

      span {
        width: 10px;
        height: 10px;
        border-radius: 50%;
        background: var(--bg-elevated);
      }
    }

    .step-code pre {
      margin: 0;
      padding: 14px;
      font-family: var(--font-mono);
      font-size: 13px;
      line-height: 1.6;
      overflow-x: auto;
    }

    .step-code code {
      color: var(--accent-cyan);
    }

    .hiw-cta {
      display: flex;
      align-items: center;
      justify-content: center;
      gap: 20px;
      margin-top: 48px;
      padding: 24px;
      background: var(--bg-primary);
      border: 1px solid var(--border-subtle);
      border-radius: 16px;
      opacity: 0;
      transform: translateY(20px);
      transition: all 0.5s ease;

      &.visible {
        opacity: 1;
        transform: translateY(0);
      }

      p {
        color: var(--text-secondary);
        font-size: 15px;
      }

      @media (max-width: 640px) {
        flex-direction: column;
        gap: 16px;
      }
    }

    .cta-btn {
      display: inline-flex;
      align-items: center;
      gap: 8px;
      padding: 12px 24px;
      background: var(--gradient-brand);
      color: var(--bg-primary);
      text-decoration: none;
      font-size: 14px;
      font-weight: 600;
      border-radius: 10px;
      transition: all 0.3s;

      svg {
        width: 16px;
        height: 16px;
      }

      &:hover {
        transform: translateY(-2px);
        box-shadow: 0 8px 24px rgba(78, 205, 196, 0.3);
      }
    }
  `],
})
export class HowItWorksComponent implements OnInit, OnDestroy {
  private el = inject(ElementRef);
  private ngZone = inject(NgZone);
  private observer: IntersectionObserver | null = null;

  activeStep = signal(-1);
  progressWidth = signal(0);

  steps: Step[] = [
    {
      number: 1,
      title: 'Install',
      description: 'Single command installation. No dependencies, no containers.',
      icon: 'terminal',
      code: '$ cargo install mcp-guard',
    },
    {
      number: 2,
      title: 'Configure',
      description: 'Generate a config file with sensible defaults.',
      icon: 'config',
      code: '$ mcp-guard init',
    },
    {
      number: 3,
      title: 'Run',
      description: 'Start protecting your MCP server immediately.',
      icon: 'rocket',
      code: '$ mcp-guard run --upstream localhost:8080',
    },
  ];

  ngOnInit() {
    this.ngZone.runOutsideAngular(() => {
      this.observer = new IntersectionObserver(
        (entries) => {
          entries.forEach((entry) => {
            if (entry.isIntersecting) {
              const stepIndex = parseInt(
                (entry.target as HTMLElement).dataset['step'] || '0',
                10
              );
              this.ngZone.run(() => {
                if (stepIndex > this.activeStep()) {
                  this.activeStep.set(stepIndex);
                  this.progressWidth.set(((stepIndex + 1) / this.steps.length) * 100);
                }
              });
            }
          });
        },
        {
          threshold: 0.5,
          rootMargin: '0px 0px -100px 0px',
        }
      );

      setTimeout(() => {
        const cards = this.el.nativeElement.querySelectorAll('.step-card');
        cards.forEach((card: Element) => this.observer?.observe(card));
      }, 100);
    });
  }

  ngOnDestroy() {
    this.observer?.disconnect();
  }
}
