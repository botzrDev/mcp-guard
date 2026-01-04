export type UserRole = 'user' | 'admin';
export type AuthProvider = 'github' | 'google';
export type LicenseTier = 'free' | 'pro' | 'enterprise';
export type LicenseStatus = 'active' | 'expired' | 'trial';

export interface User {
    id: string;
    email: string;
    name: string;
    avatar_url?: string;
    role: UserRole;
    provider: AuthProvider;
    created_at: string;
    last_login_at?: string;
}

export interface JwtPayload {
    sub: string;
    email: string;
    name: string;
    avatar_url?: string;
    role: UserRole;
    provider: AuthProvider;
    exp: number;
    iat: number;
}

export interface License {
    tier: LicenseTier;
    status: LicenseStatus;
    key_preview?: string;
    expires_at?: string;
    features: string[];
    limits: {
        api_calls_per_month: number;
        api_keys: number;
    };
}

export interface ApiKey {
    id: string;
    name: string;
    key_preview: string;
    created_at: string;
    last_used_at?: string;
    allowed_tools?: string[];
    rate_limit?: number;
}

export interface ApiKeyCreate {
    name: string;
    allowed_tools?: string[];
    rate_limit?: number;
}

export interface ApiKeyCreated extends ApiKey {
    key: string; // Full key, only shown once
}

export interface Usage {
    period: 'current_month' | 'last_30_days';
    api_calls: number;
    api_calls_limit: number;
    unique_tools: number;
    success_rate: number;
    avg_latency_ms: number;
}

export interface UsageHistory {
    date: string;
    api_calls: number;
    errors: number;
}

export interface AuditEvent {
    id: string;
    timestamp: string;
    event_type: string;
    identity_id?: string;
    tool_name?: string;
    success: boolean;
    details?: Record<string, unknown>;
}

export interface SystemHealth {
    status: 'healthy' | 'degraded' | 'unhealthy';
    version: string;
    uptime_secs: number;
    components: {
        name: string;
        status: 'ok' | 'warning' | 'error';
        message?: string;
    }[];
}
