import { Component, ChangeDetectionStrategy } from '@angular/core';
import { NavbarComponent } from './components/navbar/navbar.component';
import { HeroComponent } from './components/hero/hero.component';
import { StatsComponent } from './components/stats/stats.component';
import { FeaturesComponent } from './components/features/features.component';
import { ComparisonComponent } from './components/comparison/comparison.component';
import { PricingComponent } from './components/pricing/pricing.component';
import { CtaComponent } from './components/cta/cta.component';
import { FooterComponent } from './components/footer/footer.component';
import { BackgroundComponent } from './components/background/background.component';

@Component({
  selector: 'app-root',
  standalone: true,
  changeDetection: ChangeDetectionStrategy.OnPush,
  imports: [
    NavbarComponent,
    HeroComponent,
    StatsComponent,
    FeaturesComponent,
    ComparisonComponent,
    PricingComponent,
    CtaComponent,
    FooterComponent,
    BackgroundComponent
  ],
  template: `
    <app-background />
    <app-navbar />
    <main>
      <app-hero />
      <app-stats />
      <app-features />
      <app-comparison />
      <app-pricing />
      <app-cta />
    </main>
    <app-footer />
  `,
  styles: [`
    :host {
      display: block;
      min-height: 100vh;
    }

    main {
      position: relative;
    }
  `]
})
export class App { }
