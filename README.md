# spider_skills

[![Crates.io](https://img.shields.io/crates/v/spider_skills.svg)](https://crates.io/crates/spider_skills)
[![Documentation](https://docs.rs/spider_skills/badge.svg)](https://docs.rs/spider_skills)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Reusable automation tactics for the [spider-rs](https://github.com/spider-rs/spider) ecosystem.

Pre-built skill definitions for solving common web challenges: CAPTCHAs, anti-bot systems, interactive puzzles, access barriers, and data extraction patterns. Skills are prompt fragments with trigger conditions that get dynamically injected into the LLM context when the page state matches.

## Install

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

## Skill Categories

### CAPTCHAs (20 skills)
reCAPTCHA v2/v3, hCaptcha, Cloudflare Turnstile, GeeTest, Arkose/FunCaptcha, text CAPTCHAs, image rotation CAPTCHAs, audio CAPTCHAs, math CAPTCHAs, puzzle piece CAPTCHAs, and more.

### Anti-Bot / Security (6 skills)
Bot detection, rate limiting, JS challenge pages, proof-of-work, fingerprint challenges, device verification.

### Interactive Puzzles (19 skills)
Image grids, tic-tac-toe, word search, sliding tile puzzles, mazes, sudoku, crosswords, memory games, jigsaw puzzles, rotation puzzles, and more.

### Access Barriers (10 skills)
Cookie consent, login walls, age verification, paywalls, popups/modals, redirect chains, iframe interactions.

### Data Extraction (6 skills)
Tables, product listings, contact info, pricing, search results, charts.

### Form Automation (8 skills)
Multi-step forms, file uploads, OTP inputs, payment forms, address forms, form validation.

## Architecture

```
┌───────────────────────────┐
│     spider_skills         │  ← This crate: types + skill content
│  ┌──────────────────────┐ │
│  │ Skill, SkillTrigger, │ │  ← Core types (defined here)
│  │ SkillRegistry        │ │
│  ├──────────────────────┤ │
│  │ web_challenges       │ │  ← 69 built-in .md skill files
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
