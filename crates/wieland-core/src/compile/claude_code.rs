use super::{
    Artifact, CompileError, detect_line, in_pack, meta_line, ordered, pack_language, tier0,
};
use crate::corpus::Corpus;
use crate::rule::{Language, Rule};

struct SkillText {
    name: &'static str,
    description: &'static str,
    heading: &'static str,
    intro: &'static str,
}

fn skill_text(language: Language) -> SkillText {
    match language {
        Language::Rust => SkillText {
            name: "wieland-rust",
            description: "Rust coding rules — ownership, errors, unsafe, async, API design. Load a rule file from rules/ by id for detail.",
            heading: "# Wieland — Rust rules",
            intro: "Canonical Rust coding rules.",
        },
        Language::Typescript => SkillText {
            name: "wieland-typescript",
            description: "TypeScript 7 coding rules — promises, unions, any and casts, untrusted input, strict tsconfig, erasable syntax. Load a rule file from rules/ by id for detail.",
            heading: "# Wieland — TypeScript rules",
            intro: "Canonical TypeScript 7 coding rules.",
        },
        Language::Go => SkillText {
            name: "wieland-go",
            description: "Go coding rules — errors, context, goroutines and channels, resources, interfaces. Load a rule file from rules/ by id for detail.",
            heading: "# Wieland — Go rules",
            intro: "Canonical Go coding rules.",
        },
    }
}

pub fn render(corpus: &Corpus, sha: &str) -> Result<Vec<Artifact>, CompileError> {
    let language = pack_language(corpus);
    let mut artifacts = vec![Artifact {
        rel_path: in_pack(language, "claude-code/SKILL.md"),
        contents: skill_md(corpus, sha),
    }];

    for rule in ordered(corpus) {
        artifacts.push(Artifact {
            rel_path: in_pack(language, &format!("claude-code/rules/{}.md", rule.id)),
            contents: rule_file(rule, sha),
        });
    }

    Ok(artifacts)
}

fn skill_md(corpus: &Corpus, sha: &str) -> String {
    let text = skill_text(pack_language(corpus));
    let mut out = String::new();
    out.push_str("---\n");
    out.push_str(&format!("name: {}\n", text.name));
    out.push_str(&format!("description: {}\n", text.description));
    out.push_str("---\n\n");
    out.push_str(&format!("<!-- wieland-corpus: {sha} -->\n\n"));
    out.push_str(&format!("{}\n\n", text.heading));
    out.push_str(&format!(
        "{} The always-on core (L0) is inlined below; every rule has a loadable file at `rules/<id>.md`. Tiers: 0 always-on · 1 on-demand · 2 machine-checked · 3 hook/CI-enforced.\n\n",
        text.intro
    ));

    out.push_str("## Always-on core (L0)\n");
    for rule in tier0(corpus) {
        out.push('\n');
        out.push_str(&format!("### {} — {}\n", rule.id, rule.title));
        out.push_str(&format!("{}\n\n", meta_line(rule)));
        out.push_str(rule.body.trim_end());
        out.push('\n');
    }

    out.push_str("\n## Index\n\n");
    let mut current = String::new();
    for rule in ordered(corpus) {
        if rule.category != current {
            current = rule.category.clone();
            out.push_str(&format!("\n### {current}\n"));
        }
        out.push_str(&format!(
            "- [`{}`](rules/{}.md) — {} (tier {} · {})\n",
            rule.id,
            rule.id,
            rule.title,
            rule.tier.level(),
            enforceability_word(rule),
        ));
    }

    out
}

fn rule_file(rule: &Rule, sha: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("<!-- wieland-corpus: {sha} -->\n"));
    out.push_str(&format!("# {}\n\n", rule.id));
    out.push_str(&format!("{}\n", rule.title));
    out.push_str(&format!("{}\n", meta_line(rule)));
    out.push_str(&format!("{}\n\n", detect_line(rule)));
    out.push_str(rule.body.trim_end());
    out.push('\n');
    out
}

fn enforceability_word(rule: &Rule) -> &'static str {
    use crate::rule::Enforceability::*;
    match rule.enforceability {
        Compiler => "compiler",
        Gate => "gate",
        Check => "check",
        Judge => "judge",
    }
}
