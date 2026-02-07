---
name: spider-api-unblocker
description: Bypass anti-bot protections using the spider.cloud unblocker API
triggers:
  - url_contains: "api.spider.cloud/unblocker"
priority: 3
---

# Spider API — Unblocker

**POST** `https://api.spider.cloud/unblocker`

Bypass anti-bot protections on challenging websites. Costs 10-40 additional credits per successful unblock.

For prompt-based AI routes, use `/ai/*` endpoints (`/ai/crawl`, `/ai/scrape`, `/ai/search`, `/ai/browser`, `/ai/links`) with an AI subscription plan: <https://spider.cloud/ai/pricing>

## Authentication

```
Authorization: Bearer $SPIDER_API_KEY
Content-Type: application/json
```

## Required Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `url` | string | Target URL to unblock |

## Parameters

Same as `/scrape` endpoint — supports all page-loading, proxy, browser control, and content processing parameters.

Key parameters for unblocking:

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `return_format` | string | | Output format |
| `stealth` | boolean | true | Headless stealth mode |
| `fingerprint` | boolean | true | Fingerprint evasion |
| `proxy_enabled` | boolean | false | Premium proxy rotation |
| `proxy` | string | | Pool: `"residential"`, `"mobile"`, `"isp"` |

## Example

```json
{
  "url": "https://protected-site.com",
  "return_format": "markdown",
  "stealth": true,
  "fingerprint": true,
  "proxy_enabled": true
}
```
