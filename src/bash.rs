use crate::config::*;

pub fn get_fixed_values_var_name_for_option(program_option: &ProgramOption) -> String {
    let option = if program_option.short.is_empty() {
        &program_option.long
    } else {
        &program_option.short
    };

    format!(
        "{prefix}_fixed_values",
        prefix = option.trim_start_matches("-")
    )
}

fn format_option_cases(program_option: &ProgramOption) -> String {
    let compreply = format!(
        r#"
            COMPREPLY=( $(compgen {} "${{current}}") )
            return 0
            ;;
        "#,
        if program_option.accepts_files {
            "-f".to_owned()
        } else if program_option.fixed_values.len() != 0 {
            format!(
                r#"-W "${{{}}}" --"#,
                get_fixed_values_var_name_for_option(program_option)
            )
        } else {
            String::new()
        }
    );

    let short_case = if !program_option.short.is_empty() {
        format!("\t{})\n{}", program_option.short, compreply)
    } else {
        String::new()
    };

    let long_case = if !program_option.long.is_empty() {
        format!("\t{})\n{}", program_option.long, compreply)
    } else {
        String::new()
    };

    format!("{}\n\t{}", short_case, long_case)
}

pub fn format_option(max_opt_len: usize, program_option: &ProgramOption) -> String {
    if !program_option.description.is_empty() {
        let short_opt = if !program_option.short.is_empty() {
            format!(
                "{:width$} -- {}",
                program_option.short,
                program_option.description,
                width = max_opt_len
            )
        } else {
            String::new()
        };

        let long_opt = if !program_option.long.is_empty() {
            format!(
                "{:width$} -- {}",
                program_option.long,
                program_option.description,
                width = max_opt_len
            )
        } else {
            String::new()
        };

        format!("{}\n{}", short_opt, long_opt).trim().to_owned()
    } else {
        format!("{} {}", program_option.short, program_option.long)
            .trim()
            .to_owned()
    }
}

pub fn generate_bash(cfg: &Config) -> String {
    let any_with_description = cfg.prog_options.iter().any(|o| !o.description.is_empty());
    let max_option_len = if any_with_description {
        cfg.prog_options.iter().fold(0, |curr_max: usize, o| {
            let opt = if o.long.is_empty() { &o.short } else { &o.long };
            std::cmp::max(curr_max, opt.len())
        })
    } else {
        0
    };

    let opts = cfg
        .prog_options
        .iter()
        .map(|o| format_option(max_option_len, o))
        .collect::<Vec<_>>()
        .join(if any_with_description { "\n" } else { " " });

    let opts = if any_with_description {
        format!("\n{}", opts)
    } else {
        opts
    };

    let cases = cfg
        .prog_options
        .iter()
        .filter(|o| o.accepts_files || o.fixed_values.len() != 0)
        .map(|o| format_option_cases(o))
        .collect::<Vec<_>>()
        .join("");

    let fixed_value_vars = cfg
        .prog_options
        .iter()
        .filter(|o| o.fixed_values.len() > 0)
        .map(|o| {
            format!(
                r#"local {fixed_values_var}="{fixed_values}""#,
                fixed_values_var = get_fixed_values_var_name_for_option(o),
                fixed_values = o.fixed_values.join(" ")
            )
        })
        .collect::<Vec<_>>()
        .join("\n\t");

    let complete_current = format!(r#"COMPREPLY=( $(compgen -W "${{opts}}" -- "${{current}}") )"#);

    let ifs_change = if any_with_description {
        r#"
        local OLDIFS="$IFS"
        local IFS=$'\n'"#
    } else {
        ""
    };

    let ifs_restore = if any_with_description {
        r#"IFS="$OLDIFS""#
    } else {
        ""
    };

    let truncate_description = if any_with_description {
        r#"
        if [[ ${#COMPREPLY[*]} -eq 1 ]]; then
            COMPREPLY=(${COMPREPLY[0]%% -- *})
        fi"#
    } else {
        ""
    };

    std::format!(
        r##"#!/usr/bin/bash
_{prog_name}_completions() {{
    COMPREPLY=()
    local current=${{COMP_WORDS[COMP_CWORD]}}
    local previous=${{COMP_WORDS[COMP_CWORD-1]}}
    local opts="{opts}"
    {fixed_value_vars}

    if [[ ${{current}} == -* || ${{COMP_CWORD}} -eq 1 ]]; then
        {ifs_change}
        {complete_current}
        {ifs_restore}
        {truncate_description}
        return 0
    fi

    case "${{previous}}" in
    {cases}

        *)
            COMPREPLY=()
            ;;
    esac

    {ifs_change}
    {complete_current}
    {ifs_restore}
    {truncate_description}
    return 0
}}

complete -F _{prog_name}_completions -o bashdefault -o default {prog_name}"##,
        prog_name = cfg.prog_name,
        opts = opts,
        cases = cases,
        complete_current = complete_current,
        fixed_value_vars = fixed_value_vars,
        ifs_change = ifs_change,
        ifs_restore = ifs_restore,
        truncate_description = truncate_description
    )
}
