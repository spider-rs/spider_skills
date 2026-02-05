# spider_skills

Web challenge solving skills for [spider's](https://github.com/spider-rs/spider) automation agent.

Pre-built skill definitions for solving common web challenges: CAPTCHAs, anti-bot systems, interactive puzzles, access barriers, and data extraction patterns. Skills are prompt fragments with trigger conditions that get dynamically injected into the LLM context when the page state matches.

## Install

```toml
[dependencies]
spider_skills = "0.1"
```

## Usage

```rust
use spider_skills::web_challenges;

// Get a registry with all 69 built-in skills
let registry = web_challenges::registry();

// Or pick specific skills
let mut registry = spider_skills::new_registry();
web_challenges::add_image_grid(&mut registry);
web_challenges::add_text_captcha(&mut registry);
web_challenges::add_tic_tac_toe(&mut registry);
```

Skills are matched per-round against the current page state (URL, title, HTML). When a trigger fires, the skill content is injected into the system prompt.

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

## Custom Skills

Load skills from markdown with YAML frontmatter:

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

## Architecture

```
spider_skills          ← Skill content + registry builders
  ├── web_challenges   ← 69 built-in .md skill files (include_str!)
  └── fetch            ← Optional: fetch skills from remote URLs
        ↓
spider_agent           ← Core types: Skill, SkillTrigger, SkillRegistry
  (skills feature)
        ↓
LLM System Prompt      ← "ACTIVATED SKILLS: ..."
```

## License

MIT
