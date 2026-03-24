pub mod quality;

pub type RuleFn = fn(&str) -> Option<String>;

pub fn all_rules() -> Vec<(&'static str, RuleFn)> {
    vec![(
        "quality/add_final_keyword",
        quality::add_final_keyword::apply,
    )]
}
