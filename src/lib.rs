//! Reusable automation tactics for the spider-rs ecosystem.
//!
//! This crate provides self-contained skill definitions for solving common web
//! challenges: CAPTCHAs, anti-bot systems, interactive puzzles, access barriers,
//! and data extraction patterns. Skills are prompt fragments with trigger
//! conditions that get injected into the LLM context when the page state matches.
//!
//! # Architecture
//!
//! `spider_skills` is the **canonical source** for skill types and content.
//! Any spider-rs project can depend on this crate directly.
//!
//! ```text
//! ┌───────────────────────────┐
//! │     spider_skills         │  ← This crate: types + skill content
//! │  ┌──────────────────────┐ │
//! │  │ Skill, SkillTrigger, │ │  ← Core types (defined here)
//! │  │ SkillRegistry        │ │
//! │  ├──────────────────────┤ │
//! │  │ web_challenges       │ │  ← 69 built-in challenge skills
//! │  │ fetch                │ │  ← Optional: fetch skills from URLs
//! │  │ s3                   │ │  ← Optional: load skills from S3
//! │  └──────────────────────┘ │
//! └────────────┬──────────────┘
//!              │ used by
//!    ┌─────────┼─────────┐
//!    ▼         ▼         ▼
//! spider    spider     your
//! _agent   _worker    project
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

use std::collections::HashMap;

#[cfg(feature = "web_challenges")]
pub mod web_challenges;

#[cfg(feature = "fetch")]
pub mod fetch;

#[cfg(feature = "s3")]
pub mod s3;

// ─── Core Types ──────────────────────────────────────────────────────────

/// Trigger conditions for skill activation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SkillTrigger {
    /// Match if page title contains this string (case-insensitive).
    TitleContains(String),
    /// Match if URL contains this string (case-insensitive).
    UrlContains(String),
    /// Match if HTML contains this string (case-insensitive).
    HtmlContains(String),
    /// Always matches (for manually-activated skills).
    Always,
}

impl SkillTrigger {
    /// Create a title-contains trigger.
    pub fn title_contains(s: impl Into<String>) -> Self {
        Self::TitleContains(s.into())
    }

    /// Create a URL-contains trigger.
    pub fn url_contains(s: impl Into<String>) -> Self {
        Self::UrlContains(s.into())
    }

    /// Create an HTML-contains trigger.
    pub fn html_contains(s: impl Into<String>) -> Self {
        Self::HtmlContains(s.into())
    }

    /// Check if this trigger matches the given page state.
    pub fn matches(&self, url: &str, title: &str, html: &str) -> bool {
        match self {
            Self::TitleContains(s) => title.to_lowercase().contains(&s.to_lowercase()),
            Self::UrlContains(s) => url.to_lowercase().contains(&s.to_lowercase()),
            Self::HtmlContains(s) => html.to_lowercase().contains(&s.to_lowercase()),
            Self::Always => true,
        }
    }
}

/// A skill provides specialized context for solving specific challenge types.
///
/// Skills follow the [Agent Skills](https://github.com/anthropics/skills) pattern:
/// self-contained instruction sets with metadata for matching and loading.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Skill {
    /// Unique skill identifier (lowercase, hyphens for spaces).
    pub name: String,
    /// Description of what this skill handles and when to use it.
    pub description: String,
    /// Trigger conditions — if ANY match, the skill is activated.
    #[serde(default)]
    pub triggers: Vec<SkillTrigger>,
    /// The prompt content to inject when this skill is active.
    pub content: String,
    /// Optional JavaScript to execute via `page.evaluate()` BEFORE the LLM
    /// sees the page. The JS should write results into `document.title` so the
    /// model can read them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_evaluate: Option<String>,
    /// Optional JavaScript code snippets the LLM can use.
    /// Keys are descriptive names, values are JS code strings.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub code_snippets: HashMap<String, String>,
    /// Priority: higher priority skills are injected first. Default 0.
    #[serde(default)]
    pub priority: i32,
}

impl Skill {
    /// Create a new skill with name and description.
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            triggers: Vec::new(),
            content: String::new(),
            pre_evaluate: None,
            code_snippets: HashMap::new(),
            priority: 0,
        }
    }

    /// Add a trigger condition.
    pub fn with_trigger(mut self, trigger: SkillTrigger) -> Self {
        self.triggers.push(trigger);
        self
    }

    /// Set the prompt content.
    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into();
        self
    }

    /// Set JavaScript to execute before the LLM sees the page.
    pub fn with_pre_evaluate(mut self, js: impl Into<String>) -> Self {
        self.pre_evaluate = Some(js.into());
        self
    }

    /// Add a code snippet.
    pub fn with_snippet(mut self, name: impl Into<String>, code: impl Into<String>) -> Self {
        self.code_snippets.insert(name.into(), code.into());
        self
    }

    /// Set priority.
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    /// Check if this skill matches the given page state.
    /// Returns true if ANY trigger matches.
    pub fn matches(&self, url: &str, title: &str, html: &str) -> bool {
        if self.triggers.is_empty() {
            return false;
        }
        self.triggers.iter().any(|t| t.matches(url, title, html))
    }

    /// Parse a skill from Markdown with YAML frontmatter.
    ///
    /// Format:
    /// ```markdown
    /// ---
    /// name: skill-name
    /// description: What this skill does
    /// triggers:
    ///   - title_contains: "some text"
    ///   - html_contains: "some-class"
    /// priority: 0
    /// ---
    ///
    /// # Skill content here
    /// Instructions for the LLM...
    /// ```
    pub fn from_markdown(markdown: &str) -> Option<Self> {
        let trimmed = markdown.trim();
        if !trimmed.starts_with("---") {
            return None;
        }

        let rest = &trimmed[3..];
        let end = rest.find("---")?;
        let frontmatter = &rest[..end].trim();
        let content = rest[end + 3..].trim();

        let mut name = String::new();
        let mut description = String::new();
        let mut triggers = Vec::new();
        let mut priority = 0i32;

        for line in frontmatter.lines() {
            let line = line.trim();
            if line.starts_with("name:") {
                name = line[5..].trim().trim_matches('"').to_string();
            } else if line.starts_with("description:") {
                description = line[12..].trim().trim_matches('"').to_string();
            } else if line.starts_with("priority:") {
                priority = line[9..].trim().parse().unwrap_or(0);
            } else if line.starts_with("- title_contains:") {
                let val = line[17..].trim().trim_matches('"').to_string();
                triggers.push(SkillTrigger::TitleContains(val));
            } else if line.starts_with("- url_contains:") {
                let val = line[15..].trim().trim_matches('"').to_string();
                triggers.push(SkillTrigger::UrlContains(val));
            } else if line.starts_with("- html_contains:") {
                let val = line[16..].trim().trim_matches('"').to_string();
                triggers.push(SkillTrigger::HtmlContains(val));
            }
        }

        if name.is_empty() {
            return None;
        }

        Some(Self {
            name,
            description,
            triggers,
            content: content.to_string(),
            pre_evaluate: None,
            code_snippets: HashMap::new(),
            priority,
        })
    }
}

// ─── SkillRegistry ───────────────────────────────────────────────────────

/// Registry for managing and matching skills.
///
/// Skills are matched against page state each round. When a skill matches,
/// its content is injected into `system_prompt_extra` for that round.
#[derive(Debug, Clone, Default)]
pub struct SkillRegistry {
    skills: Vec<Skill>,
}

impl SkillRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a skill to the registry.
    pub fn add(&mut self, skill: Skill) {
        self.skills.push(skill);
    }

    /// Number of registered skills.
    pub fn len(&self) -> usize {
        self.skills.len()
    }

    /// Check if registry is empty.
    pub fn is_empty(&self) -> bool {
        self.skills.is_empty()
    }

    /// Iterate over all skill names in the registry.
    pub fn skill_names(&self) -> impl Iterator<Item = &str> {
        self.skills.iter().map(|s| s.name.as_str())
    }

    /// Find all skills matching the current page state.
    pub fn find_matching(&self, url: &str, title: &str, html: &str) -> Vec<&Skill> {
        let mut matched: Vec<&Skill> = self
            .skills
            .iter()
            .filter(|s| s.matches(url, title, html))
            .collect();
        matched.sort_by(|a, b| b.priority.cmp(&a.priority));
        matched
    }

    /// Get the combined prompt context for all matching skills.
    ///
    /// Returns a string suitable for injection into `system_prompt_extra`.
    /// Returns empty string if no skills match.
    ///
    /// Uses default limits: max 3 skills, max 4000 chars total.
    /// For custom limits, use [`match_context_limited`].
    pub fn match_context(&self, url: &str, title: &str, html: &str) -> String {
        self.match_context_limited(url, title, html, 3, 4000)
    }

    /// Get combined prompt context with explicit limits.
    ///
    /// - `max_skills`: maximum number of skills to inject (highest priority first)
    /// - `max_chars`: maximum total characters for the combined skill context
    pub fn match_context_limited(
        &self,
        url: &str,
        title: &str,
        html: &str,
        max_skills: usize,
        max_chars: usize,
    ) -> String {
        let matched = self.find_matching(url, title, html);
        if matched.is_empty() {
            return String::new();
        }

        let mut ctx = String::with_capacity(
            max_chars.min(matched.iter().map(|s| s.content.len() + 50).sum()),
        );
        let mut count = 0;

        for skill in &matched {
            if count >= max_skills {
                break;
            }

            let entry = {
                let mut entry = String::new();
                if !ctx.is_empty() {
                    entry.push_str("\n\n");
                }
                entry.push_str("## Skill: ");
                entry.push_str(&skill.name);
                entry.push('\n');
                entry.push_str(&skill.content);

                if !skill.code_snippets.is_empty() {
                    entry.push_str("\n\n### Available Code Snippets\n");
                    for (name, code) in &skill.code_snippets {
                        entry.push_str("**");
                        entry.push_str(name);
                        entry.push_str("**: `");
                        entry.push_str(code);
                        entry.push_str("`\n");
                    }
                }
                entry
            };

            if ctx.len() + entry.len() > max_chars && !ctx.is_empty() {
                break;
            }

            ctx.push_str(&entry);
            count += 1;
        }

        ctx
    }

    /// Find all matching skills that have a `pre_evaluate` JS payload.
    /// Returns `(skill_name, js_code)` pairs sorted by priority.
    pub fn find_pre_evaluates(&self, url: &str, title: &str, html: &str) -> Vec<(&str, &str)> {
        self.find_matching(url, title, html)
            .into_iter()
            .filter_map(|s| s.pre_evaluate.as_deref().map(|js| (s.name.as_str(), js)))
            .collect()
    }

    /// Get a skill by name.
    pub fn get(&self, name: &str) -> Option<&Skill> {
        self.skills.iter().find(|s| s.name == name)
    }

    /// Remove a skill by name.
    pub fn remove(&mut self, name: &str) {
        self.skills.retain(|s| s.name != name);
    }

    /// Load a skill from a markdown string (with YAML frontmatter).
    pub fn load_markdown(&mut self, markdown: &str) -> Option<String> {
        let skill = Skill::from_markdown(markdown)?;
        let name = skill.name.clone();
        self.add(skill);
        Some(name)
    }
}

/// Create a new empty skill registry.
pub fn new_registry() -> SkillRegistry {
    SkillRegistry::new()
}
