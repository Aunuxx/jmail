export interface Env {
    // Environment variables and bindings
    MY_SECRET: string;
}

export default {
    async fetch(
        request: Request,
        env: Env,
        ctx: ExecutionContext
    ): Promise<Response> {

        return new Response("Hello from Cloudflare Workers!");

    }
};