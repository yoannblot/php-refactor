use bumpalo::Bump;
use mago_database::file::FileId;
use mago_syntax::ast::{Modifier, Sequence, Statement};
use mago_syntax::parser::parse_file_content;
use std::fs;

/// File-aware entry point: takes a path and returns stats.
pub fn apply(path: &str) -> crate::rules::RuleResult {
    let files = match crate::resolver::collect_php_files(path) {
        Ok(f) => f,
        Err(_) => {
            return crate::rules::RuleResult {
                files_changed: 0,
                files_analyzed: 0,
            };
        }
    };

    let mut files_changed = 0;
    let total_analyzed = files.len();

    for file_path in files {
        let Ok(original) = fs::read_to_string(&file_path) else {
            continue;
        };

        if let Some(modified) = apply_to_source(&original) {
            files_changed += usize::from(fs::write(&file_path, &modified).is_ok());
        }
    }

    crate::rules::RuleResult {
        files_changed,
        files_analyzed: total_analyzed,
    }
}

/// Pure source transformation: used by tests.
pub fn apply_to_source(source: &str) -> Option<String> {
    let arena = Bump::new();
    let file_id = FileId::zero();
    let program = parse_file_content(&arena, file_id, source);

    let mut insertions: Vec<usize> = Vec::new();
    collect_insertions(&program.statements, &mut insertions);

    if insertions.is_empty() {
        return None;
    }

    // Apply insertions back-to-front to preserve offsets
    let mut result = source.to_string();
    insertions.sort_unstable_by(|a, b| b.cmp(a));
    for offset in insertions {
        result.insert_str(offset, "final ");
    }

    Some(result)
}

fn collect_insertions<'a>(
    statements: &'a Sequence<'a, Statement<'a>>,
    insertions: &mut Vec<usize>,
) {
    for stmt in statements {
        match stmt {
            Statement::Class(class) => {
                let mods = &class.modifiers;
                if mods.contains_final() || mods.contains_abstract() {
                    continue;
                }
                // Insert before `readonly` if present, else before `class` keyword
                let offset = if mods.contains_readonly() {
                    match mods.get_readonly() {
                        Some(Modifier::Readonly(kw)) => kw.span.start.offset as usize,
                        _ => unreachable!(),
                    }
                } else {
                    class.class.span.start.offset as usize
                };
                insertions.push(offset);
            }
            Statement::Interface(_) => {} // skip
            Statement::Namespace(ns) => {
                collect_insertions(ns.statements(), insertions);
            }
            _ => {}
        }
    }
}
