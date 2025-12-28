import { Component, ChangeDetectionStrategy } from '@angular/core';

@Component({
  selector: 'app-terms',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: `
    <div class="page-container">
      <h1>Terms of Service</h1>
      <div class="terms-content">
        <p>Last updated: December 28, 2025</p>

        <h2>1. Acceptance of Terms</h2>
        <p>By accessing and using MCP Guard, you accept and agree to be bound by the terms and provision of this agreement.</p>

        <h2>2. Description of Service</h2>
        <p>MCP Guard provides security proxies for Model Context Protocol (MCP) servers, including authentication, rate limiting, and observability features.</p>

        <h2>3. User Account</h2>
        <p>You are responsible for maintaining the security of your account and API keys. You are fully responsible for all activities that occur under the account and any other actions taken in connection with it.</p>

        <h2>4. Acceptable Use</h2>
        <p>You must not use the service to:</p>
        <ul>
          <li>Violate any laws or regulations</li>
          <li>Infringe upon the rights of others</li>
          <li>Interfere with or disrupt the service or servers</li>
          <li>Attempt to gain unauthorized access to any part of the service</li>
        </ul>

        <h2>5. Limitation of Liability</h2>
        <p>In no event shall MCP Guard, nor its directors, employees, partners, agents, suppliers, or affiliates, be liable for any indirect, incidental, special, consequential or punitive damages, including without limitation, loss of profits, data, use, goodwill, or other intangible losses.</p>

        <h2>6. Changes to Terms</h2>
        <p>We reserve the right, at our sole discretion, to modify or replace these Terms at any time. We will provide notice of any changes by posting the new Terms on this site.</p>
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
    
    .terms-content {
      line-height: 1.6;
      color: var(--text-secondary, #666);
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

    ul {
      margin-bottom: 1rem;
      padding-left: 1.5rem;
    }

    li {
      margin-bottom: 0.5rem;
    }
  `]
})
export class TermsComponent { }
