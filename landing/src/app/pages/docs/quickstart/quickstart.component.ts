import { Component, ChangeDetectionStrategy } from '@angular/core';

@Component({
    selector: 'app-quickstart',
    standalone: true,
    changeDetection: ChangeDetectionStrategy.OnPush,
    template: `
    <h1>Quick Start Guide</h1>
    <p>Get started with MCP Guard in 5 minutes.</p>
  `,
    styles: [`
    :host { display: block; }
  `]
})
export class QuickstartComponent { }
