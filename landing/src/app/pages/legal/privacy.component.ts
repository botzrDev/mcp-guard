import { Component, ChangeDetectionStrategy } from '@angular/core';

@Component({
    selector: 'app-privacy',
    standalone: true,
    changeDetection: ChangeDetectionStrategy.OnPush,
    template: `
    <div class="page-container">
      <h1>Privacy Policy</h1>
      <p>Our privacy policy.</p>
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
export class PrivacyComponent { }
