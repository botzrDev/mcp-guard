import { Component, ChangeDetectionStrategy } from '@angular/core';

@Component({
  selector: 'app-contact',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  template: `
    <div class="page-container">
      <h1>Contact Us</h1>
      <div class="contact-content">
        <p class="intro">
          We'd love to hear from you. Whether you have a question about features, pricing, or need technical support, we're here to help.
        </p>

        <div class="contact-grid">
          <div class="contact-card">
            <h3>Support & Inquiries</h3>
            <p>For general questions and support:</p>
            <a href="mailto:support@botzr.com" class="contact-link">support@botzr.com</a>
          </div>

          <div class="contact-card">
            <h3>GitHub Issues</h3>
            <p>Found a bug or have a feature request? Open an issue on GitHub:</p>
            <a href="https://github.com/botzrdev/mcp-guard/issues" target="_blank" class="contact-link">GitHub Issues</a>
          </div>
        </div>
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

    .intro {
      font-size: 1.1rem;
      color: var(--text-secondary, #666);
      margin-bottom: 3rem;
      line-height: 1.6;
    }

    h1 {
      font-size: 2.5rem;
      margin-bottom: 1rem;
      color: var(--text-primary, #111);
    }

    .contact-grid {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
      gap: 2rem;
    }

    .contact-card {
      padding: 2rem;
      background: var(--bg-secondary, #f9fafb);
      border-radius: 8px;
      border: 1px solid var(--border-color, #e5e7eb);

      h3 {
        font-size: 1.25rem;
        font-weight: 600;
        margin-bottom: 0.5rem;
        color: var(--text-primary, #111);
      }

      p {
        margin-bottom: 1rem;
        font-size: 0.9rem;
        color: var(--text-secondary, #666);
      }
    }

    .contact-link {
      color: var(--primary-color, #007bff);
      text-decoration: none;
      font-weight: 500;
      
      &:hover {
        text-decoration: underline;
      }
    }
  `]
})
export class ContactComponent { }
