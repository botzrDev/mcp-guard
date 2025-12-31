#!/usr/bin/env python3
import json, base64, hmac, hashlib, time

def b64url(data):
    return base64.urlsafe_b64encode(data).rstrip(b'=').decode()

header = b64url(json.dumps({'alg': 'HS256', 'typ': 'JWT'}).encode())
payload = b64url(json.dumps({
    'sub': 'jwt-user-123',
    'scope': 'read:files',
    'iss': 'https://test.mcp-guard.io',
    'aud': 'mcp-guard',
    'exp': int(time.time()) + 3600
}).encode())

secret = 'mcp-guard-test-secret-key-32-chars!!'
signature = b64url(hmac.new(secret.encode(), f'{header}.{payload}'.encode(), hashlib.sha256).digest())

print(f'{header}.{payload}.{signature}')
