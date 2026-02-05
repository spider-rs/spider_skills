//! Web challenge solving skills for spider's automation agent.
//!
//! This crate provides pre-built skill definitions for solving common web
//! challenges: image grids, CAPTCHAs, puzzles, games, and more. Skills are
//! prompt fragments with trigger conditions that get injected into the LLM
//! context when the page state matches.
//!
//! # Architecture
//!
//! Skills are matched per-round against the current page state (URL, title,
//! HTML). When a skill's trigger fires, its content is injected into the
//! system prompt, giving the LLM specialized strategies for that challenge.
//!
//! ```text
//! ┌─────────────────────┐
//! │   spider_skills      │  ← This crate: skill content + registry builders
//! │   ┌───────────────┐  │
//! │   │ web_challenges │  │  ← Built-in challenge skills
//! │   │ fetch          │  │  ← Optional: fetch skills from URLs
//! │   └───────────────┘  │
//! └──────────┬──────────┘
//!            │ provides SkillRegistry
//!            ▼
//! ┌─────────────────────┐
//! │   spider_agent       │  ← Core types: Skill, SkillTrigger, SkillRegistry
//! │   (skills feature)   │
//! └──────────┬──────────┘
//!            │ injected per-round
//!            ▼
//! ┌─────────────────────┐
//! │   LLM System Prompt  │  ← "ACTIVATED SKILLS: ..."
//! └─────────────────────┘
//! ```
//!
//! # Usage
//!
//! ```rust
//! use spider_skills::web_challenges;
//!
//! // Get a registry with all built-in web challenge skills
//! let registry = web_challenges::registry();
//!
//! // Or pick specific skill categories
//! let mut registry = spider_skills::new_registry();
//! web_challenges::add_image_grid(&mut registry);
//! web_challenges::add_tic_tac_toe(&mut registry);
//! ```
//!
//! # Adding Custom Skills
//!
//! Skills can be loaded from markdown with YAML frontmatter:
//!
//! ```rust
//! let mut registry = spider_skills::new_registry();
//! registry.load_markdown(r#"---
//! name: my-skill
//! description: Custom challenge solver
//! triggers:
//!   - title_contains: "my challenge"
//! ---
//!
//! Strategy for solving my custom challenge...
//! "#);
//! ```

// Re-export core types from spider_agent
pub use spider_agent::automation::skills::{Skill, SkillRegistry, SkillTrigger};

#[cfg(feature = "web_challenges")]
pub mod web_challenges;

#[cfg(feature = "fetch")]
pub mod fetch;

/// Create a new empty skill registry.
pub fn new_registry() -> SkillRegistry {
    SkillRegistry::new()
}
