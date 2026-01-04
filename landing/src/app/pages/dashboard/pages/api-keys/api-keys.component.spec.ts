import { ComponentFixture, TestBed, fakeAsync, tick } from '@angular/core/testing';
import { ApiKeysComponent } from './api-keys.component';

describe('ApiKeysComponent', () => {
    let component: ApiKeysComponent;
    let fixture: ComponentFixture<ApiKeysComponent>;

    beforeEach(async () => {
        await TestBed.configureTestingModule({
            imports: [ApiKeysComponent],
        }).compileComponents();

        fixture = TestBed.createComponent(ApiKeysComponent);
        component = fixture.componentInstance;
        fixture.detectChanges();
    });

    afterEach(() => {
        jest.clearAllMocks();
    });

    it('should create', () => {
        expect(component).toBeTruthy();
    });

    describe('initial state', () => {
        it('should have default API keys', () => {
            expect(component.apiKeys().length).toBe(2);
            expect(component.apiKeys()[0].name).toBe('Production Server');
        });

        it('should not show modals initially', () => {
            expect(component.showCreateModal()).toBe(false);
            expect(component.showRevokeModal()).toBe(false);
        });
    });

    describe('create key modal', () => {
        it('should open create modal', () => {
            component.openCreateModal();

            expect(component.showCreateModal()).toBe(true);
            expect(component.newKeyCreated()).toBe(false);
        });

        it('should close create modal', () => {
            component.openCreateModal();
            component.closeCreateModal();

            expect(component.showCreateModal()).toBe(false);
        });

        it('should reset form when opening modal', () => {
            component.newKeyName = 'Previous Name';
            component.newKeyRateLimit = 100;
            component.newKeyCreated.set(true);

            component.openCreateModal();

            expect(component.newKeyName).toBe('');
            expect(component.newKeyRateLimit).toBeNull();
            expect(component.newKeyCreated()).toBe(false);
        });

        it('should create a new key', () => {
            const initialCount = component.apiKeys().length;
            component.newKeyName = 'Test Key';
            component.newKeyRateLimit = 50;

            const event = new Event('submit');
            event.preventDefault = jest.fn();
            component.createKey(event);

            expect(event.preventDefault).toHaveBeenCalled();
            expect(component.newKeyCreated()).toBe(true);
            expect(component.createdKeyValue()).toMatch(/^mcp_/);
            expect(component.apiKeys().length).toBe(initialCount + 1);

            const newKey = component.apiKeys()[component.apiKeys().length - 1];
            expect(newKey.name).toBe('Test Key');
            expect(newKey.rate_limit).toBe(50);
        });

        it('should create key without rate limit', () => {
            component.newKeyName = 'No Rate Limit Key';
            component.newKeyRateLimit = null;

            const event = new Event('submit');
            event.preventDefault = jest.fn();
            component.createKey(event);

            const newKey = component.apiKeys()[component.apiKeys().length - 1];
            expect(newKey.rate_limit).toBeUndefined();
        });
    });

    describe('copy key', () => {
        it('should copy new key to clipboard', fakeAsync(() => {
            component.newKeyName = 'Test';
            const event = new Event('submit');
            event.preventDefault = jest.fn();
            component.createKey(event);

            component.copyNewKey();

            expect(navigator.clipboard.writeText).toHaveBeenCalledWith(
                component.createdKeyValue()
            );
            expect(component.keyCopied()).toBe(true);

            tick(2000);
            expect(component.keyCopied()).toBe(false);
        }));
    });

    describe('revoke key modal', () => {
        it('should open revoke modal with key', () => {
            const keyToRevoke = component.apiKeys()[0];

            component.confirmRevoke(keyToRevoke);

            expect(component.showRevokeModal()).toBe(true);
            expect(component.keyToRevoke()).toBe(keyToRevoke);
        });

        it('should close revoke modal', () => {
            const keyToRevoke = component.apiKeys()[0];
            component.confirmRevoke(keyToRevoke);

            component.closeRevokeModal();

            expect(component.showRevokeModal()).toBe(false);
            expect(component.keyToRevoke()).toBeNull();
        });

        it('should revoke key', () => {
            const initialCount = component.apiKeys().length;
            const keyToRevoke = component.apiKeys()[0];
            component.confirmRevoke(keyToRevoke);

            component.revokeKey();

            expect(component.apiKeys().length).toBe(initialCount - 1);
            expect(component.apiKeys().find((k) => k.id === keyToRevoke.id)).toBeUndefined();
            expect(component.showRevokeModal()).toBe(false);
        });

        it('should not revoke if no key selected', () => {
            const initialCount = component.apiKeys().length;
            component.keyToRevoke.set(null);

            component.revokeKey();

            expect(component.apiKeys().length).toBe(initialCount);
        });
    });

    describe('formatDate', () => {
        it('should format date correctly', () => {
            const result = component.formatDate('2025-12-01T10:00:00Z');

            expect(result).toMatch(/Dec/);
            expect(result).toMatch(/1/);
            expect(result).toMatch(/2025/);
        });
    });

    describe('template rendering', () => {
        it('should render API keys table when keys exist', () => {
            const table = fixture.nativeElement.querySelector('.keys-table');
            expect(table).toBeTruthy();
        });

        it('should render empty state when no keys', () => {
            component.apiKeys.set([]);
            fixture.detectChanges();

            const emptyState = fixture.nativeElement.querySelector('.empty-state');
            const table = fixture.nativeElement.querySelector('.keys-table');

            expect(emptyState).toBeTruthy();
            expect(table).toBeFalsy();
        });

        it('should render create button in header', () => {
            const createBtn = fixture.nativeElement.querySelector(
                '.page-header .create-btn'
            );
            expect(createBtn).toBeTruthy();
            expect(createBtn.textContent).toContain('Create Key');
        });

        it('should render correct number of table rows', () => {
            const rows = fixture.nativeElement.querySelectorAll('.table-row');
            expect(rows.length).toBe(component.apiKeys().length);
        });
    });
});
