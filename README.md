# spider_skills

[![Crates.io](https://img.shields.io/crates/v/spider_skills.svg)](https://crates.io/crates/spider_skills)
[![Documentation](https://docs.rs/spider_skills/badge.svg)](https://docs.rs/spider_skills)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Skills and automation tactics for [spider](https://github.com/spider-rs/spider) rust projects.

Pre-built skill definitions for solving common web challenges and interacting with the [spider.cloud](https://spider.cloud) API. Skills are markdown prompt fragments with trigger conditions that get dynamically injected into the LLM context when the page state matches.

> **Note:** The Rust crate is optional — it provides a typed integration layer for the [spider.rs](https://github.com/spider-rs/spider) ecosystem. The skill definitions in `skills/` are standalone markdown files usable with any LLM-based automation system.

## Skill Folders

```
skills/
  automation/   69 web challenge skills (CAPTCHAs, puzzles, forms, security, data extraction)
  api/           8 spider.cloud API reference skills (crawl, scrape, search, screenshot, etc.)
  core/          agent-agnostic skill specs
  codex/         Codex adapter skills (SKILL.md format)
  claude/        Claude adapter skills
```

## Cross-Agent Skills

This repository includes a generic core skill spec plus platform adapters.

Current Spider CLI extraction skill:

- Core: `skills/core/spider-cli-extraction.md`
- Codex adapter: `skills/codex/spider-cli-extraction/`
- Claude adapter: `skills/claude/spider-cli-extraction.md`

To use with Codex, copy `skills/codex/spider-cli-extraction/` to `$CODEX_HOME/skills/spider-cli-extraction`.

### Automation Skills (`skills/automation/`)

Pre-built tactics for common web challenges encountered during crawling and browser automation. Each `.md` file contains YAML frontmatter (trigger conditions, priority) and prompt content for LLM-driven solving.

**Categories:**

| Category | Count | Examples |
|----------|-------|---------|
| CAPTCHAs | 20 | reCAPTCHA v2/v3, hCaptcha, Turnstile, GeeTest, FunCaptcha, audio, math, puzzle piece |
| Interactive Puzzles | 19 | Image grids, tic-tac-toe, word search, sliding tiles, mazes, sudoku, crosswords, memory games |
| Access Barriers | 10 | Cookie consent, login walls, age verification, paywalls, popups, redirect chains, iframes |
| Form Automation | 8 | Multi-step forms, file uploads, OTP inputs, payment forms, address forms |
| Anti-Bot / Security | 6 | Bot detection, rate limiting, JS challenges, proof-of-work, fingerprinting, device verification |
| Data Extraction | 6 | Tables, product listings, contact info, pricing, search results, charts |

### API Skills (`skills/api/`)

Reference skills for the [spider.cloud API](https://spider.cloud/docs/api) — endpoint documentation, parameters, and usage examples.

| Skill | Endpoint | Description |
|-------|----------|-------------|
| `crawl` | POST `/crawl` | Multi-page website crawling |
| `scrape` | POST `/scrape` | Single-page data extraction |
| `search` | POST `/search` | SERP queries with optional content fetch |
| `links` | POST `/links` | Link discovery and extraction |
| `screenshot` | POST `/screenshot` | Visual page capture |
| `transform` | POST `/transform` | HTML-to-markdown/text conversion |
| `unblocker` | POST `/unblocker` | Anti-bot bypass (10-40 extra credits) |
| `ai` | POST `/ai/crawl`, `/ai/scrape`, `/ai/search`, `/ai/browser`, `/ai/links` | AI-powered Spider routes ([subscription required](https://spider.cloud/ai/pricing)) |

## Install (Rust)

```toml
[dependencies]
spider_skills = "0.1"
```

### Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `web_challenges` | Yes | 69 built-in web challenge skills |
| `fetch` | No | Load skills from remote URLs at runtime |
| `s3` | No | Load skills from AWS S3 buckets |

```toml
# All features
spider_skills = { version = "0.1", features = ["web_challenges", "fetch", "s3"] }

# Minimal (just core types, no built-in skills)
spider_skills = { version = "0.1", default-features = false }
```

## Usage

```rust
use spider_skills::web_challenges;

// Get a registry with all 69 built-in skills
let registry = web_challenges::registry();

// Or pick specific skill categories
let mut registry = spider_skills::new_registry();
web_challenges::add_image_grid(&mut registry);
web_challenges::add_text_captcha(&mut registry);
web_challenges::add_tic_tac_toe(&mut registry);
```

### Matching Skills Against Page State

```rust
let registry = spider_skills::web_challenges::registry();

// Returns combined prompt context for matching skills
let context = registry.match_context(
    "https://example.com/login",  // url
    "Sign In",                     // title
    "<div class='g-recaptcha'>",   // html
);

// context now contains the login-wall and recaptcha-v2 skill prompts
```

### Custom Skills from Markdown

```rust
let mut registry = spider_skills::new_registry();
registry.load_markdown(r#"---
name: my-skill
description: Custom challenge solver
triggers:
  - title_contains: "my challenge"
  - html_contains: "challenge-widget"
---

Strategy for solving my custom challenge...
"#);
```

### Loading Skills from URLs

```rust,no_run
# async fn example() {
let mut registry = spider_skills::new_registry();
spider_skills::fetch::fetch_skill(&mut registry, "https://example.com/skills/my-skill.md").await.unwrap();
# }
```

### Loading Skills from S3

```rust,no_run
# async fn example() -> Result<(), Box<dyn std::error::Error>> {
use spider_skills::s3::S3SkillSource;

let source = S3SkillSource::new("my-skills-bucket").await;
let mut registry = spider_skills::new_registry();
source.load_into(&mut registry, "skills/").await?;
# Ok(())
# }
```

## Architecture

```
┌───────────────────────────┐
│     spider_skills         │  ← This crate: types + skill content
│  ┌──────────────────────┐ │
│  │ Skill, SkillTrigger, │ │  ← Core types (defined here)
│  │ SkillRegistry        │ │
│  ├──────────────────────┤ │
│  │ web_challenges       │ │  ← 69 built-in automation skills
│  │ fetch                │ │  ← Optional: fetch from URLs
│  │ s3                   │ │  ← Optional: load from S3
│  └──────────────────────┘ │
└────────────┬──────────────┘
             │ used by
   ┌─────────┼─────────┐
   ▼         ▼         ▼
spider    spider     your
_agent   _worker    project
```

## Authoring Skills

Each skill is a markdown file with YAML frontmatter:

```markdown
---
name: my-challenge-solver
description: Solves a specific type of web challenge
triggers:
  - title_contains: "challenge keyword"
  - html_contains: "challenge-css-class"
  - url_contains: "/challenge/"
priority: 5
---

# Strategy

Step-by-step instructions for the LLM to follow when this
challenge type is detected...
```

**Trigger types:**
- `title_contains` — case-insensitive match on page title
- `url_contains` — case-insensitive match on page URL
- `html_contains` — case-insensitive match on page HTML

**Priority:** Higher values are injected first. Use 1-3 for low priority, 4-5 for medium, 6+ for high.

## License

MIT
