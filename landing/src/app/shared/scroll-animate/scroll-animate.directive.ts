import {
  Directive,
  ElementRef,
  Input,
  OnInit,
  OnDestroy,
  inject,
  NgZone,
} from '@angular/core';

@Directive({
  selector: '[appScrollAnimate]',
  standalone: true,
})
export class ScrollAnimateDirective implements OnInit, OnDestroy {
  @Input() animationClass = 'animate-on-scroll';
  @Input() threshold = 0.1;
  @Input() rootMargin = '0px 0px -50px 0px';

  private el = inject(ElementRef);
  private ngZone = inject(NgZone);
  private observer: IntersectionObserver | null = null;

  ngOnInit() {
    this.el.nativeElement.classList.add(this.animationClass);

    this.ngZone.runOutsideAngular(() => {
      this.observer = new IntersectionObserver(
        (entries) => {
          entries.forEach((entry) => {
            if (entry.isIntersecting) {
              entry.target.classList.add('is-visible');
              // Once visible, stop observing (animate only once)
              this.observer?.unobserve(entry.target);
            }
          });
        },
        {
          threshold: this.threshold,
          rootMargin: this.rootMargin,
        }
      );

      this.observer.observe(this.el.nativeElement);
    });
  }

  ngOnDestroy() {
    this.observer?.disconnect();
  }
}
