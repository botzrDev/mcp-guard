import { Component, ChangeDetectionStrategy } from '@angular/core';
import { HeroComponent } from '../../components/hero/hero.component';
import { StatsComponent } from '../../components/stats/stats.component';
import { FeaturesComponent } from '../../components/features/features.component';
import { HowItWorksComponent } from '../../components/how-it-works/how-it-works.component';
import { ComparisonComponent } from '../../components/comparison/comparison.component';
import { PricingComponent } from '../../components/pricing/pricing.component';
import { CtaComponent } from '../../components/cta/cta.component';
import { BackgroundComponent } from '../../components/background/background.component';

@Component({
    selector: 'app-home',
    standalone: true,
    changeDetection: ChangeDetectionStrategy.OnPush,
    imports: [
        HeroComponent,
        StatsComponent,
        FeaturesComponent,
        HowItWorksComponent,
        ComparisonComponent,
        PricingComponent,
        CtaComponent,
        BackgroundComponent
    ],
    template: `
    <app-background />
    <main>
      <app-hero />
      <app-stats />
      <app-features />
      <app-how-it-works />
      <app-comparison />
      <app-pricing />
      <app-cta />
    </main>
  `,
    styles: [`
    :host {
      display: block;
    }

    main {
      position: relative;
    }
  `]
})
export class HomeComponent { }
