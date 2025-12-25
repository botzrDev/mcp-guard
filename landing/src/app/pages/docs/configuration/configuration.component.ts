import { Component, ChangeDetectionStrategy } from '@angular/core';

@Component({
    selector: 'app-configuration',
    standalone: true,
    changeDetection: ChangeDetectionStrategy.OnPush,
    template: `
    <h1>Configuration</h1>
    <p>Configure MCP Guard for your needs.</p>
  `,
    styles: [`
    :host { display: block; }
  `]
})
export class ConfigurationComponent { }
