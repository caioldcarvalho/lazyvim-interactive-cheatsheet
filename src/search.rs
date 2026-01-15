use crate::commands::Command;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

pub struct SearchEngine {
    matcher: SkimMatcherV2,
}

impl Default for SearchEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchEngine {
    pub fn new() -> Self {
        Self {
            matcher: SkimMatcherV2::default(),
        }
    }

    /// Search commands by query, returns matches sorted by score (best first)
    pub fn search<'a>(&self, commands: &'a [Command], query: &str) -> Vec<(&'a Command, i64)> {
        if query.is_empty() {
            // Return all commands with score 0 when query is empty
            return commands.iter().map(|cmd| (cmd, 0i64)).collect();
        }

        let query_lower = query.to_lowercase();
        let mut results: Vec<(&Command, i64)> = Vec::new();

        for cmd in commands {
            let mut best_score: Option<i64> = None;

            // Search in description (highest weight)
            if let Some(score) = self.matcher.fuzzy_match(&cmd.description.to_lowercase(), &query_lower) {
                let weighted = score * 3;
                best_score = Some(best_score.map_or(weighted, |s| s.max(weighted)));
            }

            // Search in keys
            if let Some(score) = self.matcher.fuzzy_match(&cmd.keys.to_lowercase(), &query_lower) {
                let weighted = score * 2;
                best_score = Some(best_score.map_or(weighted, |s| s.max(weighted)));
            }

            // Search in category
            if let Some(score) = self.matcher.fuzzy_match(&cmd.category.as_str().to_lowercase(), &query_lower) {
                best_score = Some(best_score.map_or(score, |s| s.max(score)));
            }

            if let Some(score) = best_score {
                results.push((cmd, score));
            }
        }

        // Sort by score descending
        results.sort_by(|a, b| b.1.cmp(&a.1));
        results
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::{Category, Mode};

    fn sample_commands() -> Vec<Command> {
        vec![
            Command {
                keys: "<leader>ff".to_string(),
                description: "Find files".to_string(),
                category: Category::Search,
                mode: Mode::Normal,
            },
            Command {
                keys: "<leader>fg".to_string(),
                description: "Live grep".to_string(),
                category: Category::Search,
                mode: Mode::Normal,
            },
            Command {
                keys: "gd".to_string(),
                description: "Go to definition".to_string(),
                category: Category::Lsp,
                mode: Mode::Normal,
            },
            Command {
                keys: "<leader>gg".to_string(),
                description: "Open LazyGit".to_string(),
                category: Category::Git,
                mode: Mode::Normal,
            },
        ]
    }

    #[test]
    fn test_search_by_description() {
        let engine = SearchEngine::new();
        let commands = sample_commands();

        let results = engine.search(&commands, "find");
        assert!(!results.is_empty());
        assert_eq!(results[0].0.keys, "<leader>ff");
    }

    #[test]
    fn test_search_by_keys() {
        let engine = SearchEngine::new();
        let commands = sample_commands();

        let results = engine.search(&commands, "ff");
        assert!(!results.is_empty());
        // Should find <leader>ff
        assert!(results.iter().any(|(cmd, _)| cmd.keys.contains("ff")));
    }

    #[test]
    fn test_search_by_category() {
        let engine = SearchEngine::new();
        let commands = sample_commands();

        let results = engine.search(&commands, "git");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_empty_query() {
        let engine = SearchEngine::new();
        let commands = sample_commands();

        let results = engine.search(&commands, "");
        assert_eq!(results.len(), commands.len());
    }

}
