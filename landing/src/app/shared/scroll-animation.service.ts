import { Injectable, NgZone, inject } from '@angular/core';
import { gsap } from 'gsap';
import { ScrollTrigger } from 'gsap/ScrollTrigger';

gsap.registerPlugin(ScrollTrigger);

@Injectable({
    providedIn: 'root'
})
export class ScrollAnimationService {
    private ngZone = inject(NgZone);
    private initialized = false;

    /**
     * Initialize GSAP ScrollTrigger defaults
     */
    init() {
        if (this.initialized) return;

        this.ngZone.runOutsideAngular(() => {
            ScrollTrigger.defaults({
                toggleActions: 'play none none reverse',
                markers: false,
            });
        });

        this.initialized = true;
    }

    /**
     * Create a horizontal scroll section that scrolls content horizontally while user scrolls vertically
     */
    createHorizontalScroll(container: HTMLElement, panels: HTMLElement[], options?: {
        pinSpacing?: boolean;
        scrub?: number | boolean;
    }) {
        this.init();

        return this.ngZone.runOutsideAngular(() => {
            const totalWidth = panels.reduce((acc, panel) => acc + panel.offsetWidth, 0);

            return gsap.to(panels, {
                xPercent: -100 * (panels.length - 1),
                ease: 'none',
                scrollTrigger: {
                    trigger: container,
                    pin: true,
                    pinSpacing: options?.pinSpacing ?? true,
                    scrub: options?.scrub ?? 1,
                    end: () => `+=${totalWidth}`,
                }
            });
        });
    }

    /**
     * Create a pinned section with scroll-linked progress
     */
    createPinnedProgress(trigger: HTMLElement, onProgress: (progress: number) => void, options?: {
        start?: string;
        end?: string;
    }) {
        this.init();

        return this.ngZone.runOutsideAngular(() => {
            return ScrollTrigger.create({
                trigger,
                pin: true,
                start: options?.start ?? 'top top',
                end: options?.end ?? '+=200%',
                scrub: true,
                onUpdate: (self) => {
                    this.ngZone.run(() => onProgress(self.progress));
                }
            });
        });
    }

    /**
     * Create a parallax effect with custom depth
     */
    createParallax(element: HTMLElement, speed: number, options?: {
        direction?: 'vertical' | 'horizontal';
        start?: string;
        end?: string;
    }) {
        this.init();

        return this.ngZone.runOutsideAngular(() => {
            const direction = options?.direction ?? 'vertical';
            const property = direction === 'vertical' ? 'y' : 'x';

            return gsap.to(element, {
                [property]: speed * 100,
                ease: 'none',
                scrollTrigger: {
                    trigger: element,
                    start: options?.start ?? 'top bottom',
                    end: options?.end ?? 'bottom top',
                    scrub: true,
                }
            });
        });
    }

    /**
     * Create a reveal animation triggered on scroll
     */
    createReveal(elements: HTMLElement | HTMLElement[], options?: {
        from?: gsap.TweenVars;
        to?: gsap.TweenVars;
        stagger?: number;
        start?: string;
    }) {
        this.init();

        return this.ngZone.runOutsideAngular(() => {
            return gsap.from(elements, {
                opacity: 0,
                y: 60,
                rotation: 3,
                scale: 0.95,
                ...options?.from,
                scrollTrigger: {
                    trigger: Array.isArray(elements) ? elements[0] : elements,
                    start: options?.start ?? 'top 80%',
                    toggleActions: 'play none none reverse',
                },
                stagger: options?.stagger ?? 0.1,
                duration: 0.8,
                ease: 'power3.out',
                ...options?.to,
            });
        });
    }

    /**
     * Animate counter from 0 to target value
     */
    animateCounter(element: HTMLElement, targetValue: number, options?: {
        prefix?: string;
        suffix?: string;
        duration?: number;
        delay?: number;
    }) {
        this.init();

        return this.ngZone.runOutsideAngular(() => {
            const obj = { value: 0 };
            const prefix = options?.prefix ?? '';
            const suffix = options?.suffix ?? '';

            return gsap.to(obj, {
                value: targetValue,
                duration: options?.duration ?? 2,
                delay: options?.delay ?? 0,
                ease: 'power2.out',
                scrollTrigger: {
                    trigger: element,
                    start: 'top 80%',
                    once: true,
                },
                onUpdate: () => {
                    element.textContent = prefix + Math.round(obj.value).toLocaleString() + suffix;
                }
            });
        });
    }

    /**
     * Create a morph/transform animation between two states
     */
    createMorph(element: HTMLElement, fromState: gsap.TweenVars, toState: gsap.TweenVars) {
        this.init();

        return this.ngZone.runOutsideAngular(() => {
            gsap.set(element, fromState);

            return gsap.to(element, {
                ...toState,
                scrollTrigger: {
                    trigger: element,
                    start: 'top 60%',
                    end: 'bottom 40%',
                    scrub: 1,
                }
            });
        });
    }

    /**
     * Create orbital/circular motion
     */
    createOrbit(element: HTMLElement, radius: number, options?: {
        duration?: number;
        direction?: 'clockwise' | 'counter-clockwise';
    }) {
        this.init();

        return this.ngZone.runOutsideAngular(() => {
            const direction = options?.direction === 'counter-clockwise' ? -1 : 1;

            return gsap.to(element, {
                rotation: 360 * direction,
                duration: options?.duration ?? 20,
                repeat: -1,
                ease: 'none',
                transformOrigin: `center ${radius}px`,
            });
        });
    }

    /**
     * Kill all ScrollTriggers for cleanup
     */
    killAll() {
        ScrollTrigger.getAll().forEach(trigger => trigger.kill());
    }

    /**
     * Refresh ScrollTrigger calculations
     */
    refresh() {
        ScrollTrigger.refresh();
    }
}
