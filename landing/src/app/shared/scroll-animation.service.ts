import { Injectable, NgZone, inject, OnDestroy } from '@angular/core';
import { gsap } from 'gsap';
import { ScrollTrigger } from 'gsap/ScrollTrigger';

gsap.registerPlugin(ScrollTrigger);

@Injectable({
    providedIn: 'root'
})
export class ScrollAnimationService implements OnDestroy {
    private ngZone = inject(NgZone);
    private initialized = false;
    private resizeTimeout: ReturnType<typeof setTimeout> | null = null;
    private resizeHandler: (() => void) | null = null;
    private trackedTriggers: ScrollTrigger[] = [];
    private trackedTweens: gsap.core.Tween[] = [];

    /**
     * Initialize GSAP ScrollTrigger defaults and resize handling
     */
    init() {
        if (this.initialized) return;

        this.ngZone.runOutsideAngular(() => {
            ScrollTrigger.defaults({
                toggleActions: 'play none none reverse',
                markers: false,
            });

            // Add debounced resize handler for ScrollTrigger refresh
            this.resizeHandler = () => {
                if (this.resizeTimeout) {
                    clearTimeout(this.resizeTimeout);
                }
                this.resizeTimeout = setTimeout(() => {
                    ScrollTrigger.refresh();
                }, 200);
            };

            window.addEventListener('resize', this.resizeHandler, { passive: true });

            // Also refresh after fonts load (can affect layout)
            if (document.fonts) {
                document.fonts.ready.then(() => {
                    ScrollTrigger.refresh();
                });
            }
        });

        this.initialized = true;
    }

    ngOnDestroy() {
        this.cleanup();
    }

    /**
     * Clean up all resources
     */
    private cleanup() {
        if (this.resizeTimeout) {
            clearTimeout(this.resizeTimeout);
        }
        if (this.resizeHandler) {
            window.removeEventListener('resize', this.resizeHandler);
        }
        this.killAll();
    }

    /**
     * Track a ScrollTrigger for cleanup
     */
    trackTrigger(trigger: ScrollTrigger): ScrollTrigger {
        this.trackedTriggers.push(trigger);
        return trigger;
    }

    /**
     * Track a tween for cleanup
     */
    trackTween(tween: gsap.core.Tween): gsap.core.Tween {
        this.trackedTweens.push(tween);
        return tween;
    }

    /**
     * Untrack and kill a specific trigger
     */
    killTrigger(trigger: ScrollTrigger | null) {
        if (!trigger) return;
        const index = this.trackedTriggers.indexOf(trigger);
        if (index > -1) {
            this.trackedTriggers.splice(index, 1);
        }
        trigger.kill();
    }

    /**
     * Untrack and kill a specific tween
     */
    killTween(tween: gsap.core.Tween | null) {
        if (!tween) return;
        const index = this.trackedTweens.indexOf(tween);
        if (index > -1) {
            this.trackedTweens.splice(index, 1);
        }
        tween.kill();
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
     * Create a simple scroll trigger for visibility-based animations
     */
    createVisibilityTrigger(
        trigger: HTMLElement,
        callbacks: {
            onEnter?: () => void;
            onLeave?: () => void;
            onEnterBack?: () => void;
            onLeaveBack?: () => void;
        },
        options?: {
            start?: string;
            end?: string;
            once?: boolean;
        }
    ): ScrollTrigger {
        this.init();

        const st = this.ngZone.runOutsideAngular(() => {
            return ScrollTrigger.create({
                trigger,
                start: options?.start ?? 'top 70%',
                end: options?.end,
                once: options?.once ?? false,
                onEnter: callbacks.onEnter ? () => this.ngZone.run(callbacks.onEnter!) : undefined,
                onLeave: callbacks.onLeave ? () => this.ngZone.run(callbacks.onLeave!) : undefined,
                onEnterBack: callbacks.onEnterBack ? () => this.ngZone.run(callbacks.onEnterBack!) : undefined,
                onLeaveBack: callbacks.onLeaveBack ? () => this.ngZone.run(callbacks.onLeaveBack!) : undefined,
            });
        });

        return this.trackTrigger(st);
    }

    /**
     * Create a horizontal scroll animation with proper cleanup
     */
    createHorizontalScrollWithTrigger(
        container: HTMLElement,
        track: HTMLElement,
        callbacks?: {
            onUpdate?: (progress: number, activeStep: number) => void;
            onEnter?: () => void;
            onLeave?: () => void;
            onEnterBack?: () => void;
            onLeaveBack?: () => void;
        },
        options?: {
            scrub?: number | boolean;
        }
    ): { tween: gsap.core.Tween; trigger: ScrollTrigger } {
        this.init();

        return this.ngZone.runOutsideAngular(() => {
            const panels = track.querySelectorAll('.step-panel');
            const totalWidth = track.scrollWidth - container.offsetWidth;

            const tween = gsap.to(track, {
                x: -totalWidth,
                ease: 'none',
            });

            const trigger = ScrollTrigger.create({
                trigger: container,
                start: 'top top',
                end: () => `+=${track.scrollWidth - container.offsetWidth}`, // Dynamic recalculation
                pin: true,
                animation: tween,
                scrub: options?.scrub ?? 1,
                invalidateOnRefresh: true, // Recalculate on refresh
                onUpdate: callbacks?.onUpdate ? (self) => {
                    const progress = self.progress;
                    const stepProgress = progress * panels.length;
                    this.ngZone.run(() => callbacks.onUpdate!(progress, Math.floor(stepProgress)));
                } : undefined,
                onEnter: callbacks?.onEnter ? () => this.ngZone.run(callbacks.onEnter!) : undefined,
                onLeave: callbacks?.onLeave ? () => this.ngZone.run(callbacks.onLeave!) : undefined,
                onEnterBack: callbacks?.onEnterBack ? () => this.ngZone.run(callbacks.onEnterBack!) : undefined,
                onLeaveBack: callbacks?.onLeaveBack ? () => this.ngZone.run(callbacks.onLeaveBack!) : undefined,
            });

            this.trackTween(tween);
            this.trackTrigger(trigger);

            return { tween, trigger };
        });
    }

    /**
     * Animate a counter outside Angular zone with final zone update
     */
    animateCounterOutsideZone(
        targetValue: number,
        onUpdate: (value: number) => void,
        onComplete?: () => void,
        options?: {
            duration?: number;
            delay?: number;
        }
    ): gsap.core.Tween {
        this.init();

        return this.ngZone.runOutsideAngular(() => {
            const obj = { value: 0 };
            const tween = gsap.to(obj, {
                value: targetValue,
                duration: options?.duration ?? 2,
                delay: options?.delay ?? 0,
                ease: 'power2.out',
                onUpdate: () => {
                    // Direct DOM update outside zone - caller handles DOM manipulation
                    onUpdate(Math.round(obj.value));
                },
                onComplete: onComplete ? () => this.ngZone.run(onComplete) : undefined,
            });

            this.trackTween(tween);
            return tween;
        });
    }

    /**
     * Kill all tracked ScrollTriggers and tweens
     */
    killAll() {
        this.trackedTweens.forEach(tween => tween.kill());
        this.trackedTweens = [];
        this.trackedTriggers.forEach(trigger => trigger.kill());
        this.trackedTriggers = [];
    }

    /**
     * Kill all global ScrollTriggers (use with caution)
     */
    killAllGlobal() {
        ScrollTrigger.getAll().forEach(trigger => trigger.kill());
    }

    /**
     * Refresh ScrollTrigger calculations
     */
    refresh() {
        ScrollTrigger.refresh();
    }

    /**
     * Refresh with a delay (useful after layout changes)
     */
    refreshDelayed(delay: number = 100) {
        setTimeout(() => ScrollTrigger.refresh(), delay);
    }
}
