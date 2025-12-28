import { Component, ChangeDetectionStrategy } from '@angular/core';

@Component({
  selector: 'app-privacy',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: `
    <div class="page-container">
      <h1>Privacy Policy</h1>
      <div class="privacy-content">
        <p>Last updated: December 28, 2025</p>
        
        <h2>1. Introduction</h2>
        <p>Welcome to MCP Guard ("we," "our," or "us"). We are committed to protecting your personal information and your right to privacy. This Privacy Policy explains how we collect, use, disclosure, and safeguard your information when you use our service.</p>

        <h2>2. Information We Collect</h2>
        <p>We collect information that you provide directly to us when you:</p>
        <ul>
          <li>Register for an account or API keys</li>
          <li>Configure your MCP servers</li>
          <li>Contact our support team</li>
          <li>Subscribe to our newsletter</li>
        </ul>

        <h2>3. How We Use Your Information</h2>
        <p>We use the information we collect to:</p>
        <ul>
          <li>Provide, maintain, and improve our services</li>
          <li>Process transactions and manage your account</li>
          <li>Send you technical notices, updates, and security alerts</li>
          <li>Respond to your comments and questions</li>
        </ul>

        <h2>4. Data Security</h2>
        <p>We implement appropriate technical and organizational security measures designed to protect the security of any personal information we process. However, please also look after your own data security by protecting your API keys and configuration secrets.</p>

        <h2>5. Contact Us</h2>
        <p>If you have questions or comments about this policy, you may email us at privacy@mcp-guard.com.</p>
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
    
    .privacy-content {
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
export class PrivacyComponent { }
