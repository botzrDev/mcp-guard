import { ApplicationConfig, provideBrowserGlobalErrorListeners, APP_INITIALIZER, inject } from '@angular/core';
import { provideHttpClient } from '@angular/common/http';
import { provideRouter, withInMemoryScrolling, withViewTransitions, Router, NavigationEnd } from '@angular/router';
import { routes } from './app.routes';
import { ScrollAnimationService } from './shared/scroll-animation.service';
import { filter } from 'rxjs';

/**
 * Initialize ScrollAnimationService and set up router integration
 * This ensures GSAP ScrollTrigger is refreshed after navigation to recalculate positions
 */
function initializeScrollService() {
  const scrollService = inject(ScrollAnimationService);
  const router = inject(Router);

  return () => {
    // Initialize the scroll service (sets up resize handling, etc.)
    scrollService.init();

    // Refresh ScrollTrigger after each navigation to recalculate positions
    // This prevents race conditions between Angular's scroll restoration and GSAP
    router.events.pipe(
      filter(event => event instanceof NavigationEnd)
    ).subscribe(() => {
      // Delay refresh to allow DOM to settle after navigation
      scrollService.refreshDelayed(100);
    });
  };
}

export const appConfig: ApplicationConfig = {
  providers: [
    provideBrowserGlobalErrorListeners(),
    provideHttpClient(),
    provideRouter(
      routes,
      withInMemoryScrolling({
        // Disable automatic scroll position restoration - conflicts with GSAP ScrollTrigger
        // GSAP handles scroll positioning during pinned animations
        scrollPositionRestoration: 'disabled',
        // Keep anchor scrolling for in-page navigation
        anchorScrolling: 'enabled',
      }),
      withViewTransitions()
    ),
    // Initialize scroll service and router integration
    {
      provide: APP_INITIALIZER,
      useFactory: initializeScrollService,
      multi: true
    }
  ]
};
