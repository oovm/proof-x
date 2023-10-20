use super::*;

pub(super) fn parse_cst(input: &str, rule: Rule) -> OutputResult<Rule> {
    state(input, |state| match rule {
        Rule::IgnoreText => unreachable!(),
        Rule::IgnoreRegex => unreachable!(),
    })
}

/// All rules ignored in ast mode, inline is not recommended
fn builtin_ignore(state: Input) -> Output {
    state.repeat(0..u32::MAX, |s| {
        
    })

}

fn builtin_any(state: Input) -> Output {
    state.rule(Rule::IgnoreText, |s| s.match_char_if(|_| true))
}

fn builtin_text<'i>(state: Input<'i>, text: &'static str, case: bool) -> Output<'i> {
    state.rule(Rule::IgnoreText, |s| s.match_string(text, case))
}

fn builtin_regex<'i, 'r>(state: Input<'i>, regex: &'r Regex) -> Output<'i> {
    state.rule(Rule::IgnoreRegex, |s| s.match_regex(regex))
}