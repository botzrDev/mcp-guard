import { ComponentFixture, TestBed, fakeAsync, tick } from '@angular/core/testing';
import { ActivatedRoute } from '@angular/router';
import { BehaviorSubject } from 'rxjs';
import { LoginComponent } from './login.component';
import { AuthService } from '../../../core/auth';

describe('LoginComponent', () => {
    let component: LoginComponent;
    let fixture: ComponentFixture<LoginComponent>;
    let authServiceMock: jest.Mocked<Partial<AuthService>>;
    let queryParamsSubject: BehaviorSubject<Record<string, string>>;

    beforeEach(async () => {
        queryParamsSubject = new BehaviorSubject<Record<string, string>>({});

        authServiceMock = {
            isLoading: jest.fn().mockReturnValue(false),
            loginWithGitHub: jest.fn(),
            loginWithGoogle: jest.fn(),
            sendMagicLink: jest.fn().mockResolvedValue(true),
            resetMagicLinkState: jest.fn(),
            magicLinkSent: jest.fn().mockReturnValue(false),
            magicLinkEmail: jest.fn().mockReturnValue(null),
        };

        await TestBed.configureTestingModule({
            imports: [LoginComponent],
            providers: [
                { provide: AuthService, useValue: authServiceMock },
                {
                    provide: ActivatedRoute,
                    useValue: { queryParams: queryParamsSubject.asObservable() },
                },
            ],
        }).compileComponents();

        fixture = TestBed.createComponent(LoginComponent);
        component = fixture.componentInstance;
    });

    afterEach(() => {
        jest.clearAllMocks();
    });

    it('should create', () => {
        expect(component).toBeTruthy();
    });

    it('should have no error message initially', fakeAsync(() => {
        fixture.detectChanges();
        tick();

        expect(component.errorMessage()).toBeNull();
    }));

    describe('error handling', () => {
        it('should display session_expired error', fakeAsync(() => {
            queryParamsSubject.next({ error: 'session_expired' });
            fixture.detectChanges();
            tick();

            expect(component.errorMessage()).toBe(
                'Your session has expired. Please sign in again.'
            );
        }));

        it('should display access_denied error', fakeAsync(() => {
            queryParamsSubject.next({ error: 'access_denied' });
            fixture.detectChanges();
            tick();

            expect(component.errorMessage()).toBe(
                'Access was denied. Please try again.'
            );
        }));

        it('should display invalid_state error', fakeAsync(() => {
            queryParamsSubject.next({ error: 'invalid_state' });
            fixture.detectChanges();
            tick();

            expect(component.errorMessage()).toBe(
                'Authentication failed. Please try again.'
            );
        }));

        it('should display generic error for unknown error codes', fakeAsync(() => {
            queryParamsSubject.next({ error: 'unknown_error' });
            fixture.detectChanges();
            tick();

            expect(component.errorMessage()).toBe(
                'An error occurred. Please try again.'
            );
        }));
    });

    describe('login actions', () => {
        beforeEach(() => {
            fixture.detectChanges();
        });

        it('should call loginWithGitHub when GitHub button clicked', () => {
            component.loginWithGitHub();

            expect(authServiceMock.loginWithGitHub).toHaveBeenCalledTimes(1);
        });

        it('should call loginWithGoogle when Google button clicked', () => {
            component.loginWithGoogle();

            expect(authServiceMock.loginWithGoogle).toHaveBeenCalledTimes(1);
        });
    });

    describe('template rendering', () => {
        beforeEach(() => {
            fixture.detectChanges();
        });

        it('should render login card', () => {
            const loginCard = fixture.nativeElement.querySelector('.login-card');
            expect(loginCard).toBeTruthy();
        });

        it('should render both OAuth buttons', () => {
            const githubBtn = fixture.nativeElement.querySelector('.oauth-btn.github');
            const googleBtn = fixture.nativeElement.querySelector('.oauth-btn.google');

            expect(githubBtn).toBeTruthy();
            expect(googleBtn).toBeTruthy();
            expect(githubBtn.textContent).toContain('Continue with GitHub');
            expect(googleBtn.textContent).toContain('Continue with Google');
        });

        it('should render error banner when error exists', fakeAsync(() => {
            queryParamsSubject.next({ error: 'session_expired' });
            fixture.detectChanges();
            tick();

            const errorBanner = fixture.nativeElement.querySelector('.error-banner');
            expect(errorBanner).toBeTruthy();
            expect(errorBanner.textContent).toContain('Your session has expired');
        }));

        it('should not render error banner when no error', () => {
            const errorBanner = fixture.nativeElement.querySelector('.error-banner');
            expect(errorBanner).toBeFalsy();
        });

        it('should disable buttons when loading', async () => {
            // Need to recreate component with loading=true since signals are bound at creation
            const loadingAuthServiceMock: jest.Mocked<Partial<AuthService>> = {
                isLoading: jest.fn().mockReturnValue(true),
                loginWithGitHub: jest.fn(),
                loginWithGoogle: jest.fn(),
                sendMagicLink: jest.fn().mockResolvedValue(true),
                resetMagicLinkState: jest.fn(),
                magicLinkSent: jest.fn().mockReturnValue(false),
                magicLinkEmail: jest.fn().mockReturnValue(null),
            };

            TestBed.resetTestingModule();
            await TestBed.configureTestingModule({
                imports: [LoginComponent],
                providers: [
                    { provide: AuthService, useValue: loadingAuthServiceMock },
                    {
                        provide: ActivatedRoute,
                        useValue: { queryParams: queryParamsSubject.asObservable() },
                    },
                ],
            }).compileComponents();

            const loadingFixture = TestBed.createComponent(LoginComponent);
            loadingFixture.detectChanges();

            const githubBtn = loadingFixture.nativeElement.querySelector('.oauth-btn.github');
            const googleBtn = loadingFixture.nativeElement.querySelector('.oauth-btn.google');

            expect(githubBtn.disabled).toBe(true);
            expect(googleBtn.disabled).toBe(true);
        });

        it('should render magic link email form', () => {
            const emailInput = fixture.nativeElement.querySelector('.input-group input[type="email"]');
            const magicLinkBtn = fixture.nativeElement.querySelector('.magic-link-btn');

            expect(emailInput).toBeTruthy();
            expect(magicLinkBtn).toBeTruthy();
            expect(magicLinkBtn.textContent).toContain('Continue with Email');
        });

        it('should render divider between OAuth and magic link', () => {
            const divider = fixture.nativeElement.querySelector('.divider');
            expect(divider).toBeTruthy();
            expect(divider.textContent).toContain('or');
        });
    });

    describe('magic link actions', () => {
        beforeEach(() => {
            fixture.detectChanges();
        });

        it('should call sendMagicLink when form is submitted', () => {
            component.email.set('test@example.com');
            component.sendMagicLink();

            expect(authServiceMock.sendMagicLink).toHaveBeenCalledWith('test@example.com');
        });

        it('should not call sendMagicLink when email is empty', () => {
            component.email.set('');
            component.sendMagicLink();

            expect(authServiceMock.sendMagicLink).not.toHaveBeenCalled();
        });

        it('should call resetMagicLinkState when resetting', () => {
            component.email.set('test@example.com');
            component.resetMagicLink();

            expect(authServiceMock.resetMagicLinkState).toHaveBeenCalled();
            expect(component.email()).toBe('');
        });
    });

    describe('magic link success state', () => {
        it('should show success message when magic link is sent', async () => {
            const sentAuthServiceMock: jest.Mocked<Partial<AuthService>> = {
                isLoading: jest.fn().mockReturnValue(false),
                loginWithGitHub: jest.fn(),
                loginWithGoogle: jest.fn(),
                sendMagicLink: jest.fn().mockResolvedValue(true),
                resetMagicLinkState: jest.fn(),
                magicLinkSent: jest.fn().mockReturnValue(true),
                magicLinkEmail: jest.fn().mockReturnValue('sent@example.com'),
            };

            TestBed.resetTestingModule();
            await TestBed.configureTestingModule({
                imports: [LoginComponent],
                providers: [
                    { provide: AuthService, useValue: sentAuthServiceMock },
                    {
                        provide: ActivatedRoute,
                        useValue: { queryParams: queryParamsSubject.asObservable() },
                    },
                ],
            }).compileComponents();

            const sentFixture = TestBed.createComponent(LoginComponent);
            sentFixture.detectChanges();

            const successSection = sentFixture.nativeElement.querySelector('.magic-link-success');
            expect(successSection).toBeTruthy();
            expect(successSection.textContent).toContain('Check your email');
            expect(successSection.textContent).toContain('sent@example.com');
        });
    });
});
