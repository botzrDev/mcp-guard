import { Component, ChangeDetectionStrategy } from '@angular/core';

@Component({
    selector: 'app-api',
    standalone: true,
    changeDetection: ChangeDetectionStrategy.OnPush,
    template: `
    <h1>API Reference</h1>
    <p>Complete API documentation.</p>
  `,
    styles: [`
    :host { display: block; }
  `]
})
export class ApiComponent { }
