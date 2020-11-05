use crate::config::*;
use std::cell::RefCell;

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

fn get_option_attributes(cfg: &Config, option: &ProgramOption) -> String {
    let maybe_equals_sign = if !option.is_help() && cfg.use_equals_sign {
        "="
    } else {
        ""
    };

    format!(
        "{equals_sign}[{desc}]{file_options}",
        equals_sign = maybe_equals_sign,
        desc = option.description,
        file_options = file_options(option)
    )
}

fn format_option_with_multiple_args(cfg: &Config, option: &ProgramOption) -> String {
    let option_attributes = get_option_attributes(cfg, option);
    let maybe_backslash = if option.has_one_represenation() {
        ""
    } else {
        "\\"
    };

    let short = if !option.short.is_empty() {
        format!(
            "\t\t'*{opt}{attributes}'{backslash}",
            opt = option.short,
            attributes = option_attributes,
            backslash = maybe_backslash
        )
    } else {
        String::new()
    };

    let long = if !option.long.is_empty() {
        format!(
            "\t\t'*{opt}{attributes}{backslash}'",
            opt = option.long,
            attributes = option_attributes,
            backslash = maybe_backslash
        )
    } else {
        String::new()
    };

    format!(
        "{}{newline}{}",
        short,
        long,
        newline = if short.is_empty() { "" } else { "\n" }
    )
}

fn format_option_group(cfg: &Config, option: &ProgramOption, group_num: u32) -> String {
    let option_attributes = get_option_attributes(cfg, option);
    let maybe_comma = if option.has_one_represenation() {
        ""
    } else {
        ","
    };

    format!(
        "\t\t+ '(group_{num})' \\\n\
        \t\t{{{short}{comma}{long}}}'{attributes}'",
        num = group_num,
        short = option.short,
        comma = maybe_comma,
        long = option.long,
        attributes = option_attributes
    )
}

fn format_option_with_one_representation(cfg: &Config, option: &ProgramOption) -> String {
    let opt = if option.short.is_empty() {
        &option.long
    } else {
        &option.short
    };

    let option_attributes = get_option_attributes(cfg, option);

    format!(
        "\t\t'{opt}{attributes}'",
        opt = opt,
        attributes = option_attributes
    )
}

pub fn generate_zsh(cfg: &Config) -> String {
    let group_counter = RefCell::new(0);
    let arguments = cfg
        .program_options
        .iter()
        .map(|option| {
            if option.has_one_represenation() {
                format_option_with_one_representation(cfg, option)
            } else if option.accepts_multiple {
                format_option_with_multiple_args(cfg, option)
            } else {
                *group_counter.borrow_mut() += 1;
                format_option_group(cfg, option, *group_counter.borrow())
            }
        })
        .collect::<Vec<_>>()
        .join(" \\\n");

    format!(
        "\
    compdef _{prog_name} {prog_name}\n\n\
    function _{prog_name}() {{\n\
        \t_arguments \\\n\
        {arguments}\n\
    }}",
        prog_name = cfg.program_name,
        arguments = arguments
    )
}
