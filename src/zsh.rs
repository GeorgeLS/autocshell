use crate::config::*;
use std::cell::RefCell;

enum OptionKind {
    Short,
    Long,
}

#[inline]
fn file_options(option: &ProgramOption) -> String {
    if option.accepts_files {
        ":file:_files".to_string()
    } else if option.accepts_value {
        if option.fixed_values.len() > 0 {
            std::format!(": :({})", option.fixed_values.join(" "))
        } else {
            ": :".to_string()
        }
    } else {
        String::new()
    }
}

#[inline]
fn is_help_option(option: &ProgramOption) -> bool {
    if !option.short.is_empty() {
        option.short == "-h"
    } else if !option.long.is_empty() {
        option.long == "--help"
    } else {
        false
    }
}

#[inline]
fn equals_sign_or_empty(cfg: &Config, option: &ProgramOption) -> &'static str {
    if !is_help_option(option) && cfg.use_equals_sign {
        "="
    } else {
        ""
    }
}

fn format_option_with_multiple_args(cfg: &Config, option: &ProgramOption) -> String {
    let format_spec = |o: &ProgramOption, kind: OptionKind| {
        std::format!("\t\t'*{short_or_long}{equals_sign}[{desc}]{file_opt}'",
            short_or_long = match kind {
                OptionKind::Short => &o.short,
                OptionKind::Long => &o.long,
            },
            equals_sign = equals_sign_or_empty(cfg, option),
            desc = o.description,
            file_opt = file_options(o)
        )
    };

    let short_spec = if !option.short.is_empty() {
        format_spec(option, OptionKind::Short)
    } else {
        String::new()
    };

    let long_spec = if !option.long.is_empty() {
        format_spec(option, OptionKind::Long)
    } else {
        String::new()
    };

    std::format!("{}\n\t{}", short_spec, long_spec)
        .trim()
        .to_string()
}

fn format_groups(cfg: &Config) -> Vec<String> {
    let group_counter = RefCell::new(0);
    let group_fmt = |o: &ProgramOption| {
        *group_counter.borrow_mut() += 1;
        let comma = if o.short.is_empty() { "" } else { "," };

        std::format!("\t\t+ '(group_{counter})' \\\n\t\t{{{short}{comma}{long}}}'{equals}[{desc}]{file_opt}'",
            counter = *group_counter.borrow(),
            short = o.short,
            comma = comma,
            long = o.long,
            equals = equals_sign_or_empty(cfg, o),
            desc = o.description,
            file_opt = file_options(o)
        )
    };

    cfg.prog_options
        .iter()
        .filter(|o| !o.accepts_multiple && !o.short.is_empty() && !o.long.is_empty())
        .map(|o| group_fmt(o))
        .collect()
}

fn format_options_with_multiple_args(cfg: &Config) -> Vec<String> {
    cfg.prog_options
        .iter()
        .filter(|o| o.accepts_multiple)
        .map(|o| format_option_with_multiple_args(cfg, o))
        .collect()
}

fn format_options_with_one_representation(cfg: &Config) -> Vec<String> {
    cfg.prog_options
        .iter()
        .filter(|o| !o.accepts_multiple && (o.short.is_empty() || o.long.is_empty()))
        .map(|o| {
            let short_or_long = if o.short.is_empty() {
                o.long.to_string()
            } else {
                o.short.to_string()
            };

            std::format!("\t\t'{short_or_long}{equals_sign}[{desc}]{file_opt}'",
                short_or_long = short_or_long,
                equals_sign = equals_sign_or_empty(cfg, o),
                desc = o.description,
                file_opt = file_options(o)
            )
        })
        .collect()
}

pub fn generate_zsh(cfg: &Config) -> String {
    let opts_with_multiple_args = format_options_with_multiple_args(cfg);
    let groups = format_groups(cfg);
    let singles = format_options_with_one_representation(cfg);
    let arguments = singles.iter()
        .chain(opts_with_multiple_args.iter())
        .chain(groups.iter())
        .map(|a| a.as_str())
        .collect::<Vec<_>>()
        .join(" \\\n");

    std::format!(
        "compdef _{prog_name} {prog_name}

function _{prog_name}() {{
\t_arguments \\\n\t\t{arguments}
}}",
        prog_name = cfg.prog_name,
        arguments = arguments
    )
}
