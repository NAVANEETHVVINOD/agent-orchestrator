# Zero-cost remote MCP hosting assessment

Updated 9 October 2026 from provider documentation. No account was connected,
deployment provisioned, or external endpoint published. The user has no hosting
budget; ongoing cost must remain $0 unless they revise that constraint.

## Options reviewed

| Provider | Verified free constraint | Fit for this Rust MCP | Status |
| --- | --- | --- | --- |
| Cloudflare Workers Free | 100,000 requests/day and 10 ms CPU per invocation; Rust support is labelled Beta | Best candidate for a stateless bounded wasm handler if the actual Rust parser/kernel compiles and benchmarks below 10 ms at the imposed task/record limits. The current Tokio/rmcp stdio server is not proven portable there; remote HTTP may require a separately bounded adapter. | Candidate only; no build, benchmark or provider account verified |
| Render Free web service | Sleeps after 15 minutes without traffic, then takes about a minute to wake; local filesystem is ephemeral | Easiest full native Rust container path, but first MCP calls can be delayed and there is no persistent local state. | Fallback candidate if cold-start behavior is acceptable |
| Vercel Hobby + Rust runtime | Rust runtime exists; Hobby usage is limited to personal, non-commercial use and usage limits can pause service | Not a safe hosting assumption for a public product that may be monetized. This project has not settled its business model. | Not selected |

Sources: [Cloudflare Rust beta and supported crates](https://developers.cloudflare.com/workers/languages/rust/),
[Cloudflare Workers pricing](https://developers.cloudflare.com/workers/platform/pricing/),
[Cloudflare runtime limits](https://developers.cloudflare.com/workers/platform/limits/),
[Render free service limits](https://render.com/docs/free),
[Vercel Rust runtime](https://vercel.com/docs/functions/runtimes/rust), and
[Vercel Hobby terms](https://vercel.com/docs/plans/hobby).

## Recommendation and open decision

First finish and verify the local stdio facade. Then evaluate a Cloudflare Rust/Wasm
prototype only for a small, stateless JSON-RPC/MCP handler, with strict request/task
limits, exact-source worst-case CPU benchmarks, and no user plan storage. Do not migrate
the existing Tokio server until the actual target compiler and protocol client prove
it works. If it misses the free CPU budget, try Render only if the documented cold
start is acceptable for public ChatGPT availability. Neither free tier is a service
reliability promise.

The remaining data choice is whether the remote service may process request bodies
transiently without retaining them. The safer default for this zero-cost prototype
is no accounts, no private code/secrets, no plan persistence, minimal redacted logs,
bounded public routes and an explicit notice that request data is processed by the
hosting provider. A shared unauthenticated public endpoint still needs abuse controls
and must not return sensitive user-submitted content to other callers. Authenticated
private saved plans require identity, storage, deletion, retention and cross-tenant
tests, and are outside the current zero-cost verified scope.

The user has not yet selected this data boundary or provider. No deployment work
depends on an invented account, domain, private-data promise or commercial-use assumption.
