import { Component, ChangeDetectionStrategy } from '@angular/core';

@Component({
  selector: 'app-about',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: `
    <div class="page-container">
      <h1>About MCP Guard</h1>
      <div class="about-content">
        <p class="mission-statement">
          We are on a mission to secure the future of Agentic AI by providing robust, enterprise-grade security for the Model Context Protocol.
        </p>

        <h2>Our Story</h2>
        <p>MCP Guard started with a simple observation: as AI agents become more capable, the interfaces they use (MCP) need the same level of security and observability as traditional APIs. We built MCP Guard to bridge this gap, allowing developers to expose internal tools to AI agents safely and confidently.</p>

        <h2>Open Source</h2>
        <p>We believe that security tools should be open and transparent. That's why MCP Guard is open source under the AGPL-v3 license. You can inspect our code, contribute improvements, and run it on your own infrastructure.</p>

        <h2>The Team</h2>
        <p>We are a team of security engineers and AI enthusiasts passionate about building safe infrastructure for the next generation of software.</p>
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

    .mission-statement {
      font-size: 1.25rem;
      font-weight: 500;
      color: var(--text-primary, #111);
      margin-bottom: 2rem;
      line-height: 1.5;
    }

    h1 {
      font-size: 2.5rem;
      margin-bottom: 2rem;
      color: var(--text-primary, #111);
    }

    h2 {
      font-size: 1.5rem;
      margin-top: 2rem;
      margin-bottom: 1rem;
      color: var(--text-primary, #111);
    }

    p {
      line-height: 1.6;
      color: var(--text-secondary, #666);
      margin-bottom: 1rem;
    }
  `]
})
export class AboutComponent { }
