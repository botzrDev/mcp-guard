import { Component, signal, ChangeDetectionStrategy } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { ApiKey } from '../../../../core/auth';

@Component({
    selector: 'app-api-keys',
    standalone: true,
    imports: [CommonModule, FormsModule],
    changeDetection: ChangeDetectionStrategy.OnPush,
    template: `
        <div class="api-keys-page">
            <div class="page-header">
                <div class="header-content">
                    <h1>API Keys</h1>
                    <p class="page-subtitle">Create and manage API keys for your MCP servers.</p>
                </div>
                <button class="create-btn" (click)="openCreateModal()">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <line x1="12" y1="5" x2="12" y2="19"/>
                        <line x1="5" y1="12" x2="19" y2="12"/>
                    </svg>
                    Create Key
                </button>
            </div>

            @if (apiKeys().length === 0) {
                <div class="empty-state">
                    <div class="empty-icon">
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
                            <path d="M7 11V7a5 5 0 0110 0v4"/>
                        </svg>
                    </div>
                    <h2>No API keys yet</h2>
                    <p>Create your first API key to start securing your MCP servers.</p>
                    <button class="create-btn" (click)="openCreateModal()">
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <line x1="12" y1="5" x2="12" y2="19"/>
                            <line x1="5" y1="12" x2="19" y2="12"/>
                        </svg>
                        Create API Key
                    </button>
                </div>
            } @else {
                <div class="keys-table">
                    <div class="table-header">
                        <span class="col-name">Name</span>
                        <span class="col-key">Key</span>
                        <span class="col-created">Created</span>
                        <span class="col-used">Last Used</span>
                        <span class="col-actions">Actions</span>
                    </div>
                    @for (key of apiKeys(); track key.id) {
                        <div class="table-row">
                            <span class="col-name">{{ key.name }}</span>
                            <span class="col-key">
                                <code>{{ key.key_preview }}</code>
                            </span>
                            <span class="col-created">{{ formatDate(key.created_at) }}</span>
                            <span class="col-used">{{ key.last_used_at ? formatDate(key.last_used_at) : 'Never' }}</span>
                            <span class="col-actions">
                                <button class="action-btn danger" (click)="confirmRevoke(key)" title="Revoke">
                                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                        <polyline points="3 6 5 6 21 6"/>
                                        <path d="M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2"/>
                                    </svg>
                                </button>
                            </span>
                        </div>
                    }
                </div>
            }
        </div>

        @if (showCreateModal()) {
            <div class="modal-backdrop" (click)="closeCreateModal()">
                <div class="modal" (click)="$event.stopPropagation()">
                    <div class="modal-header">
                        <h2>Create API Key</h2>
                        <button class="close-btn" (click)="closeCreateModal()">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <line x1="18" y1="6" x2="6" y2="18"/>
                                <line x1="6" y1="6" x2="18" y2="18"/>
                            </svg>
                        </button>
                    </div>

                    @if (newKeyCreated()) {
                        <div class="modal-body">
                            <div class="success-message">
                                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <path d="M22 11.08V12a10 10 0 11-5.93-9.14"/>
                                    <polyline points="22 4 12 14.01 9 11.01"/>
                                </svg>
                                <span>API key created successfully!</span>
                            </div>
                            <div class="new-key-display">
                                <label>Your new API key (copy it now, it won't be shown again):</label>
                                <div class="key-box">
                                    <code>{{ createdKeyValue() }}</code>
                                    <button class="copy-btn" (click)="copyNewKey()">
                                        @if (keyCopied()) {
                                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                <polyline points="20 6 9 17 4 12"/>
                                            </svg>
                                            Copied!
                                        } @else {
                                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                                <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
                                                <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/>
                                            </svg>
                                            Copy
                                        }
                                    </button>
                                </div>
                            </div>
                        </div>
                        <div class="modal-footer">
                            <button class="btn btn-primary" (click)="closeCreateModal()">Done</button>
                        </div>
                    } @else {
                        <form (submit)="createKey($event)">
                            <div class="modal-body">
                                <div class="form-group">
                                    <label for="keyName">Key Name</label>
                                    <input
                                        type="text"
                                        id="keyName"
                                        [(ngModel)]="newKeyName"
                                        name="keyName"
                                        placeholder="e.g., Production Server"
                                        required
                                    />
                                    <span class="form-hint">A descriptive name to identify this key</span>
                                </div>

                                <div class="form-group">
                                    <label for="rateLimit">Rate Limit (optional)</label>
                                    <input
                                        type="number"
                                        id="rateLimit"
                                        [(ngModel)]="newKeyRateLimit"
                                        name="rateLimit"
                                        placeholder="50"
                                        min="1"
                                        max="1000"
                                    />
                                    <span class="form-hint">Requests per second (leave empty for default)</span>
                                </div>
                            </div>
                            <div class="modal-footer">
                                <button type="button" class="btn btn-secondary" (click)="closeCreateModal()">Cancel</button>
                                <button type="submit" class="btn btn-primary" [disabled]="!newKeyName">Create Key</button>
                            </div>
                        </form>
                    }
                </div>
            </div>
        }

        @if (showRevokeModal()) {
            <div class="modal-backdrop" (click)="closeRevokeModal()">
                <div class="modal modal-sm" (click)="$event.stopPropagation()">
                    <div class="modal-header">
                        <h2>Revoke API Key</h2>
                        <button class="close-btn" (click)="closeRevokeModal()">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <line x1="18" y1="6" x2="6" y2="18"/>
                                <line x1="6" y1="6" x2="18" y2="18"/>
                            </svg>
                        </button>
                    </div>
                    <div class="modal-body">
                        <p class="warning-text">
                            Are you sure you want to revoke <strong>{{ keyToRevoke()?.name }}</strong>?
                            This action cannot be undone.
                        </p>
                    </div>
                    <div class="modal-footer">
                        <button class="btn btn-secondary" (click)="closeRevokeModal()">Cancel</button>
                        <button class="btn btn-danger" (click)="revokeKey()">Revoke Key</button>
                    </div>
                </div>
            </div>
        }
    `,
    styles: [`
        .api-keys-page {
            max-width: 1000px;
            margin: 0 auto;
        }

        .page-header {
            display: flex;
            justify-content: space-between;
            align-items: flex-start;
            margin-bottom: var(--space-8);

            @media (max-width: 640px) {
                flex-direction: column;
                gap: var(--space-4);
            }

            h1 {
                font-family: var(--font-display);
                font-size: var(--text-3xl);
                font-weight: var(--weight-bold);
                margin-bottom: var(--space-2);
            }
        }

        .page-subtitle {
            color: var(--text-muted);
            font-size: var(--text-base);
        }

        .create-btn {
            display: flex;
            align-items: center;
            gap: var(--space-2);
            padding: var(--space-2-5) var(--space-4);
            background: var(--text-primary);
            color: var(--bg-primary);
            border: none;
            border-radius: var(--radius-lg);
            font-size: var(--text-sm);
            font-weight: var(--weight-semibold);
            cursor: pointer;
            transition: all var(--duration-fast) var(--ease-out);

            svg {
                width: var(--icon-sm);
                height: var(--icon-sm);
            }

            &:hover {
                transform: translateY(-2px);
                box-shadow: var(--shadow-lg);
            }
        }

        .empty-state {
            text-align: center;
            padding: var(--space-16) var(--space-8);
            background: var(--bg-secondary);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-2xl);
        }

        .empty-icon {
            width: var(--space-16);
            height: var(--space-16);
            background: var(--bg-elevated);
            border-radius: var(--radius-full);
            display: flex;
            align-items: center;
            justify-content: center;
            margin: 0 auto var(--space-6);

            svg {
                width: var(--space-8);
                height: var(--space-8);
                color: var(--text-muted);
            }
        }

        .empty-state h2 {
            font-size: var(--text-xl);
            font-weight: var(--weight-semibold);
            margin-bottom: var(--space-2);
        }

        .empty-state p {
            color: var(--text-muted);
            margin-bottom: var(--space-6);
        }

        .keys-table {
            background: var(--bg-secondary);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-xl);
            overflow: hidden;
        }

        .table-header,
        .table-row {
            display: grid;
            grid-template-columns: 1.5fr 2fr 1fr 1fr auto;
            align-items: center;
            padding: var(--space-4);
            gap: var(--space-4);

            @media (max-width: 768px) {
                grid-template-columns: 1fr;
                gap: var(--space-2);
            }
        }

        .table-header {
            background: var(--bg-elevated);
            border-bottom: 1px solid var(--border-subtle);
            font-size: var(--text-xs);
            font-weight: var(--weight-semibold);
            text-transform: uppercase;
            letter-spacing: var(--tracking-wider);
            color: var(--text-muted);

            @media (max-width: 768px) {
                display: none;
            }
        }

        .table-row {
            border-bottom: 1px solid var(--border-subtle);

            &:last-child {
                border-bottom: none;
            }

            @media (max-width: 768px) {
                padding: var(--space-5);
            }
        }

        .col-name {
            font-weight: var(--weight-medium);
        }

        .col-key code {
            font-family: var(--font-mono);
            font-size: var(--text-sm);
            color: var(--accent-cyan);
            background: var(--bg-primary);
            padding: var(--space-1) var(--space-2);
            border-radius: var(--radius-sm);
        }

        .col-created,
        .col-used {
            font-size: var(--text-sm);
            color: var(--text-muted);
        }

        .col-actions {
            display: flex;
            justify-content: flex-end;
        }

        .action-btn {
            width: var(--space-8);
            height: var(--space-8);
            display: flex;
            align-items: center;
            justify-content: center;
            background: transparent;
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-md);
            color: var(--text-muted);
            cursor: pointer;
            transition: all var(--duration-fast) var(--ease-out);

            svg {
                width: var(--icon-sm);
                height: var(--icon-sm);
            }

            &:hover {
                background: var(--bg-elevated);
                color: var(--text-primary);
            }

            &.danger:hover {
                background: rgba(239, 68, 68, 0.1);
                border-color: rgba(239, 68, 68, 0.3);
                color: var(--accent-red);
            }
        }

        .modal-backdrop {
            position: fixed;
            inset: 0;
            background: rgba(0, 0, 0, 0.7);
            backdrop-filter: blur(4px);
            display: flex;
            align-items: center;
            justify-content: center;
            z-index: var(--z-modal);
            padding: var(--space-4);
        }

        .modal {
            width: 100%;
            max-width: 480px;
            background: var(--bg-secondary);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-2xl);
            overflow: hidden;

            &.modal-sm {
                max-width: 400px;
            }
        }

        .modal-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: var(--space-5);
            border-bottom: 1px solid var(--border-subtle);

            h2 {
                font-size: var(--text-lg);
                font-weight: var(--weight-semibold);
            }
        }

        .close-btn {
            width: var(--space-8);
            height: var(--space-8);
            display: flex;
            align-items: center;
            justify-content: center;
            background: transparent;
            border: none;
            color: var(--text-muted);
            cursor: pointer;
            border-radius: var(--radius-md);
            transition: all var(--duration-fast) var(--ease-out);

            svg {
                width: var(--icon-sm);
                height: var(--icon-sm);
            }

            &:hover {
                background: var(--bg-elevated);
                color: var(--text-primary);
            }
        }

        .modal-body {
            padding: var(--space-5);
        }

        .modal-footer {
            display: flex;
            justify-content: flex-end;
            gap: var(--space-3);
            padding: var(--space-5);
            border-top: 1px solid var(--border-subtle);
            background: var(--bg-elevated);
        }

        .form-group {
            margin-bottom: var(--space-5);

            &:last-child {
                margin-bottom: 0;
            }

            label {
                display: block;
                font-size: var(--text-sm);
                font-weight: var(--weight-medium);
                margin-bottom: var(--space-2);
            }

            input {
                width: 100%;
                padding: var(--space-3);
                background: var(--bg-primary);
                border: 1px solid var(--border-subtle);
                border-radius: var(--radius-lg);
                font-size: var(--text-base);
                color: var(--text-primary);
                transition: all var(--duration-fast) var(--ease-out);

                &:focus {
                    outline: none;
                    border-color: var(--accent-cyan);
                    box-shadow: 0 0 0 3px rgba(78, 205, 196, 0.1);
                }

                &::placeholder {
                    color: var(--text-dim);
                }
            }
        }

        .form-hint {
            display: block;
            margin-top: var(--space-1-5);
            font-size: var(--text-xs);
            color: var(--text-muted);
        }

        .btn {
            padding: var(--space-2-5) var(--space-4);
            border-radius: var(--radius-lg);
            font-size: var(--text-sm);
            font-weight: var(--weight-semibold);
            cursor: pointer;
            transition: all var(--duration-fast) var(--ease-out);
            border: 1px solid transparent;
        }

        .btn-primary {
            background: var(--text-primary);
            color: var(--bg-primary);

            &:hover:not(:disabled) {
                transform: translateY(-1px);
                box-shadow: var(--shadow-md);
            }

            &:disabled {
                opacity: 0.5;
                cursor: not-allowed;
            }
        }

        .btn-secondary {
            background: var(--bg-elevated);
            border-color: var(--border-subtle);
            color: var(--text-primary);

            &:hover {
                background: var(--bg-hover);
            }
        }

        .btn-danger {
            background: var(--accent-red);
            color: white;

            &:hover {
                background: #dc2626;
            }
        }

        .success-message {
            display: flex;
            align-items: center;
            gap: var(--space-3);
            padding: var(--space-4);
            background: rgba(34, 197, 94, 0.1);
            border: 1px solid rgba(34, 197, 94, 0.2);
            border-radius: var(--radius-lg);
            color: var(--accent-green);
            margin-bottom: var(--space-5);

            svg {
                width: var(--icon-md);
                height: var(--icon-md);
            }
        }

        .new-key-display {
            label {
                display: block;
                font-size: var(--text-sm);
                color: var(--text-muted);
                margin-bottom: var(--space-2);
            }
        }

        .key-box {
            display: flex;
            align-items: center;
            gap: var(--space-3);
            padding: var(--space-3) var(--space-4);
            background: var(--bg-primary);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-lg);

            code {
                flex: 1;
                font-family: var(--font-mono);
                font-size: var(--text-sm);
                color: var(--accent-cyan);
                word-break: break-all;
            }
        }

        .copy-btn {
            display: flex;
            align-items: center;
            gap: var(--space-1-5);
            padding: var(--space-2) var(--space-3);
            background: var(--bg-elevated);
            border: 1px solid var(--border-subtle);
            border-radius: var(--radius-md);
            font-size: var(--text-xs);
            font-weight: var(--weight-medium);
            color: var(--text-secondary);
            cursor: pointer;
            transition: all var(--duration-fast) var(--ease-out);
            white-space: nowrap;

            svg {
                width: var(--icon-xs);
                height: var(--icon-xs);
            }

            &:hover {
                background: var(--bg-hover);
                color: var(--text-primary);
            }
        }

        .warning-text {
            font-size: var(--text-base);
            color: var(--text-secondary);

            strong {
                color: var(--text-primary);
            }
        }
    `]
})
export class ApiKeysComponent {
    apiKeys = signal<ApiKey[]>([
        {
            id: '1',
            name: 'Production Server',
            key_preview: 'mcp_abc...xyz',
            created_at: '2025-12-01T10:00:00Z',
            last_used_at: '2026-01-03T15:30:00Z'
        },
        {
            id: '2',
            name: 'Development',
            key_preview: 'mcp_dev...123',
            created_at: '2025-12-15T14:00:00Z',
            last_used_at: undefined
        }
    ]);

    showCreateModal = signal(false);
    showRevokeModal = signal(false);
    newKeyCreated = signal(false);
    createdKeyValue = signal('');
    keyCopied = signal(false);
    keyToRevoke = signal<ApiKey | null>(null);

    newKeyName = '';
    newKeyRateLimit: number | null = null;

    formatDate(dateString: string): string {
        const date = new Date(dateString);
        return date.toLocaleDateString('en-US', {
            month: 'short',
            day: 'numeric',
            year: 'numeric'
        });
    }

    openCreateModal(): void {
        this.showCreateModal.set(true);
        this.newKeyCreated.set(false);
        this.newKeyName = '';
        this.newKeyRateLimit = null;
    }

    closeCreateModal(): void {
        this.showCreateModal.set(false);
        this.newKeyCreated.set(false);
    }

    createKey(event: Event): void {
        event.preventDefault();
        // In production, this would call the API
        const newKey = `mcp_${this.generateRandomString(32)}`;
        this.createdKeyValue.set(newKey);
        this.newKeyCreated.set(true);

        // Add to list
        const newApiKey: ApiKey = {
            id: Date.now().toString(),
            name: this.newKeyName,
            key_preview: newKey.substring(0, 7) + '...' + newKey.substring(newKey.length - 3),
            created_at: new Date().toISOString(),
            rate_limit: this.newKeyRateLimit || undefined
        };
        this.apiKeys.update(keys => [...keys, newApiKey]);
    }

    copyNewKey(): void {
        navigator.clipboard.writeText(this.createdKeyValue());
        this.keyCopied.set(true);
        setTimeout(() => this.keyCopied.set(false), 2000);
    }

    confirmRevoke(key: ApiKey): void {
        this.keyToRevoke.set(key);
        this.showRevokeModal.set(true);
    }

    closeRevokeModal(): void {
        this.showRevokeModal.set(false);
        this.keyToRevoke.set(null);
    }

    revokeKey(): void {
        const key = this.keyToRevoke();
        if (key) {
            this.apiKeys.update(keys => keys.filter(k => k.id !== key.id));
        }
        this.closeRevokeModal();
    }

    private generateRandomString(length: number): string {
        const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
        let result = '';
        for (let i = 0; i < length; i++) {
            result += chars.charAt(Math.floor(Math.random() * chars.length));
        }
        return result;
    }
}
