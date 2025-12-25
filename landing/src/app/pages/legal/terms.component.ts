import { Component, ChangeDetectionStrategy } from '@angular/core';

@Component({
    selector: 'app-terms',
    standalone: true,
    changeDetection: ChangeDetectionStrategy.OnPush,
    template: `
    <div class="page-container">
      <h1>Terms of Service</h1>
      <p>Our terms of service.</p>
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
export class TermsComponent { }
