//! Built-in web challenge solving skills.
//!
//! Each function adds a specific skill to a registry. Use [`registry()`] to
//! get a pre-loaded registry with all built-in skills, or add individual
//! skills for a smaller footprint.

use crate::{Skill, SkillRegistry, SkillTrigger};

/// Create a registry pre-loaded with all built-in web challenge skills.
pub fn registry() -> SkillRegistry {
    let mut r = SkillRegistry::new();
    add_image_grid(&mut r);
    add_text_captcha(&mut r);
    add_rotation_puzzle(&mut r);
    add_tic_tac_toe(&mut r);
    add_word_search(&mut r);
    add_slider_drag(&mut r);
    r
}

/// Add image grid selection skill (e.g., "select all stop signs").
pub fn add_image_grid(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new(
            "image-grid-selection",
            "Select matching images from a grid challenge",
        )
        .with_trigger(SkillTrigger::html_contains("grid-item"))
        .with_trigger(SkillTrigger::html_contains("challenge-grid"))
        .with_trigger(SkillTrigger::title_contains("select all"))
        .with_priority(5)
        .with_content(include_str!("skills/image_grid.md")),
    );
}

/// Add text CAPTCHA / distorted text skill.
pub fn add_text_captcha(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new(
            "text-captcha",
            "Solve text-based CAPTCHAs, distorted text, and math challenges",
        )
        .with_trigger(SkillTrigger::html_contains("captcha"))
        .with_trigger(SkillTrigger::title_contains("captcha"))
        .with_trigger(SkillTrigger::title_contains("wiggles"))
        .with_trigger(SkillTrigger::title_contains("verify"))
        .with_priority(3)
        .with_content(include_str!("skills/text_captcha.md")),
    );
}

/// Add rotation puzzle skill.
pub fn add_rotation_puzzle(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new(
            "rotation-puzzle",
            "Rotate an image or element to the correct orientation",
        )
        .with_trigger(SkillTrigger::title_contains("rotat"))
        .with_trigger(SkillTrigger::html_contains("rotate"))
        .with_trigger(SkillTrigger::html_contains("slider"))
        .with_priority(5)
        .with_content(include_str!("skills/rotation.md")),
    );
}

/// Add tic-tac-toe / XOXO skill.
pub fn add_tic_tac_toe(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new(
            "tic-tac-toe",
            "Play tic-tac-toe (noughts and crosses) game",
        )
        .with_trigger(SkillTrigger::title_contains("xoxo"))
        .with_trigger(SkillTrigger::title_contains("tic-tac"))
        .with_trigger(SkillTrigger::title_contains("tic tac"))
        .with_trigger(SkillTrigger::html_contains("tic-tac"))
        .with_priority(5)
        .with_content(include_str!("skills/tic_tac_toe.md")),
    );
}

/// Add word search skill.
pub fn add_word_search(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new(
            "word-search",
            "Find and select words in a letter grid",
        )
        .with_trigger(SkillTrigger::title_contains("word search"))
        .with_trigger(SkillTrigger::title_contains("wordsearch"))
        .with_priority(5)
        .with_content(include_str!("skills/word_search.md")),
    );
}

/// Add slider / drag challenge skill.
pub fn add_slider_drag(registry: &mut SkillRegistry) {
    registry.add(
        Skill::new(
            "slider-drag",
            "Solve slider and drag-to-position challenges",
        )
        .with_trigger(SkillTrigger::html_contains("slider-track"))
        .with_trigger(SkillTrigger::html_contains("slider-handle"))
        .with_trigger(SkillTrigger::html_contains("range-slider"))
        .with_priority(4)
        .with_content(include_str!("skills/slider_drag.md")),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_loads_all_skills() {
        let r = registry();
        assert!(r.len() >= 6);
    }

    #[test]
    fn test_image_grid_matches() {
        let r = registry();
        let ctx = r.match_context("", "", "<div class='grid-item'>img</div>");
        assert!(ctx.contains("image-grid-selection"));
    }

    #[test]
    fn test_tic_tac_toe_matches() {
        let r = registry();
        let ctx = r.match_context("", "XOXO Game", "");
        assert!(ctx.contains("tic-tac-toe"));
    }

    #[test]
    fn test_word_search_matches() {
        let r = registry();
        let ctx = r.match_context("", "Word Search Puzzle", "");
        assert!(ctx.contains("word-search"));
    }

    #[test]
    fn test_text_captcha_matches_wiggles() {
        let r = registry();
        let ctx = r.match_context("", "Wiggles Level", "");
        assert!(ctx.contains("text-captcha"));
    }

    #[test]
    fn test_no_match_on_unrelated_page() {
        let r = registry();
        let ctx = r.match_context("https://example.com", "Home", "<div>hello</div>");
        assert!(ctx.is_empty());
    }
}
