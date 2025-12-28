/**
 * Cloudflare Workers script for serving static assets
 * Handles SPA routing by falling back to index.html for unknown routes
 */

import { getAssetFromKV, NotFoundError, MethodNotAllowedError } from '@cloudflare/kv-asset-handler';

export interface Env {
    __STATIC_CONTENT: KVNamespace;
    __STATIC_CONTENT_MANIFEST: string;
}

export default {
    async fetch(request: Request, env: Env, ctx: ExecutionContext): Promise<Response> {
        const url = new URL(request.url);

        try {
            // Try to serve the requested static asset
            return await getAssetFromKV(
                {
                    request,
                    waitUntil: ctx.waitUntil.bind(ctx),
                },
                {
                    ASSET_NAMESPACE: env.__STATIC_CONTENT,
                    ASSET_MANIFEST: JSON.parse(env.__STATIC_CONTENT_MANIFEST),
                }
            );
        } catch (e) {
            if (e instanceof NotFoundError) {
                // SPA fallback: serve index.html for client-side routing
                // This allows Angular router to handle the route
                const indexRequest = new Request(
                    new URL('/index.html', url.origin).toString(),
                    request
                );

                try {
                    return await getAssetFromKV(
                        {
                            request: indexRequest,
                            waitUntil: ctx.waitUntil.bind(ctx),
                        },
                        {
                            ASSET_NAMESPACE: env.__STATIC_CONTENT,
                            ASSET_MANIFEST: JSON.parse(env.__STATIC_CONTENT_MANIFEST),
                        }
                    );
                } catch {
                    return new Response('Not Found', { status: 404 });
                }
            } else if (e instanceof MethodNotAllowedError) {
                return new Response('Method Not Allowed', { status: 405 });
            }

            return new Response('Internal Error', { status: 500 });
        }
    },
};
