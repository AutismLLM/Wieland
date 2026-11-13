use super::{
    Artifact, CompileError, L0_TOKEN_BUDGET, approx_tokens, detect_line, in_pack, meta_line,
    ordered, pack_language, pack_title, tier0,
};
use crate::corpus::Corpus;
use crate::rule::Rule;

pub fn render(corpus: &Corpus, sha: &str) -> Result<Vec<Artifact>, CompileError> {
    let language = pack_language(corpus);
    let title = pack_title(language);
    let all = ordered(corpus);
    let core = tier0(corpus);

    let all_txt = section(
        sha,
        &format!("{title} — full rule corpus ({} rules)", all.len()),
        &all,
    );
    let core_txt = section(
        sha,
        &format!("{title} — L0 core (always-on) ({} rules)", core.len()),
        &core,
    );

    let core_tokens = approx_tokens(&core_txt);
    if core_tokens > L0_TOKEN_BUDGET {
        return Err(CompileError::Budget {
            artifact: in_pack(language, "core.txt").display().to_string(),
            tokens: core_tokens,
            budget: L0_TOKEN_BUDGET,
        });
    }

    Ok(vec![
        Artifact {
            rel_path: in_pack(language, "all.txt"),
            contents: all_txt,
        },
        Artifact {
            rel_path: in_pack(language, "core.txt"),
            contents: core_txt,
        },
    ])
}

fn section(sha: &str, heading: &str, rules: &[&Rule]) -> String {
    let mut out = String::new();
    out.push_str(&format!("# wieland-corpus: {sha}\n"));
    out.push_str(&format!("# {heading}\n"));
    for rule in rules {
        out.push('\n');
        out.push_str(&"=".repeat(64));
        out.push('\n');
        out.push_str(&format!("{} — {}\n", rule.id, rule.title));
        out.push_str(&meta_line(rule));
        out.push('\n');
        out.push_str(&detect_line(rule));
        out.push('\n');
        out.push_str(&"-".repeat(64));
        out.push('\n');
        out.push_str(rule.body.trim_end());
        out.push('\n');
    }
    out
}
