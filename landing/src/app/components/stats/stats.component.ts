import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-stats',
  standalone: true,
  imports: [CommonModule],
  template: `
    <section class="stats">
      <div class="stats-container">
        <div class="stats-grid">
          @for (stat of stats; track stat.label; let i = $index) {
            <div class="stat-card" [style.--delay]="i * 100 + 'ms'">
              <div class="stat-terminal">
                <div class="stat-header">
                  <span class="stat-prompt">mcp-guard</span>
                  <span class="stat-metric">{{ stat.metric }}</span>
                </div>
                <div class="stat-value">{{ stat.value }}</div>
              </div>
              <div class="stat-label">{{ stat.label }}</div>
            </div>
          }
        </div>

        <!-- Connecting lines -->
        <svg class="connector-lines" viewBox="0 0 1200 100" preserveAspectRatio="none">
          <path class="connector-path" d="M150,50 Q300,20 450,50 T750,50 T1050,50" />
        </svg>
      </div>
    </section>
  `,
  styles: [`
    .stats {
      position: relative;
      padding: 80px 0;
      background: var(--bg-secondary);
      border-top: 1px solid var(--border-subtle);
      border-bottom: 1px solid var(--border-subtle);
      overflow: hidden;

      &::before {
        content: '';
        position: absolute;
        top: 0;
        left: 50%;
        transform: translateX(-50%);
        width: 600px;
        height: 300px;
        background: radial-gradient(ellipse, rgba(78, 205, 196, 0.08) 0%, transparent 70%);
        pointer-events: none;
      }
    }

    .stats-container {
      max-width: 1280px;
      margin: 0 auto;
      padding: 0 24px;
      position: relative;
    }

    .stats-grid {
      display: grid;
      grid-template-columns: repeat(4, 1fr);
      gap: 24px;

      @media (max-width: 900px) {
        grid-template-columns: repeat(2, 1fr);
      }

      @media (max-width: 500px) {
        grid-template-columns: 1fr;
      }
    }

    .stat-card {
      position: relative;
      animation: fadeInUp 0.6s ease-out var(--delay, 0ms) both;

      &:nth-child(1) { transform: translateY(0); }
      &:nth-child(2) { transform: translateY(-10px); }
      &:nth-child(3) { transform: translateY(5px); }
      &:nth-child(4) { transform: translateY(-5px); }
    }

    .stat-terminal {
      background: var(--bg-primary);
      border: 1px solid var(--border-subtle);
      border-radius: 12px;
      padding: 20px;
      transition: all 0.3s ease;

      &:hover {
        border-color: var(--border-accent);
        box-shadow: 0 10px 40px -10px rgba(78, 205, 196, 0.2);
        transform: translateY(-4px);
      }
    }

    .stat-header {
      display: flex;
      align-items: center;
      gap: 8px;
      margin-bottom: 12px;
      font-family: var(--font-mono);
      font-size: 11px;
    }

    .stat-prompt {
      color: var(--text-muted);
    }

    .stat-metric {
      color: var(--accent-cyan);
    }

    .stat-value {
      font-family: var(--font-mono);
      font-size: 36px;
      font-weight: 700;
      letter-spacing: -0.02em;
      background: var(--gradient-brand);
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
      background-clip: text;
    }

    .stat-label {
      margin-top: 12px;
      font-size: 13px;
      color: var(--text-muted);
      text-align: center;
    }

    .connector-lines {
      position: absolute;
      bottom: 60px;
      left: 0;
      right: 0;
      height: 100px;
      pointer-events: none;
      opacity: 0.3;

      @media (max-width: 900px) {
        display: none;
      }
    }

    .connector-path {
      fill: none;
      stroke: url(#connector-gradient);
      stroke-width: 2;
      stroke-dasharray: 8 4;
    }

    @keyframes fadeInUp {
      from {
        opacity: 0;
        transform: translateY(30px);
      }
      to {
        opacity: 1;
      }
    }
  `]
})
export class StatsComponent {
  stats = [
    { value: '<2ms', metric: '--latency', label: 'P99 Latency Overhead' },
    { value: '5,000+', metric: '--throughput', label: 'Requests Per Second' },
    { value: '<15MB', metric: '--size', label: 'Binary Size' },
    { value: 'Zero', metric: '--deps', label: 'External Dependencies' },
  ];
}
