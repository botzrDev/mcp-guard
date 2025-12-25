import { Component, ChangeDetectionStrategy } from '@angular/core';

@Component({
    selector: 'app-contact',
    standalone: true,
    changeDetection: ChangeDetectionStrategy.OnPush,
    template: `
    <div class="page-container">
      <h1>Contact Us</h1>
      <p>Get in touch with us.</p>
    </div>
  `,
    styles: [`
    .page-container {
      max-width: 800px;
      margin: 0 auto;
      padding: 2rem;
      margin-top: 80px;
    }
  `]
})
export class ContactComponent { }
