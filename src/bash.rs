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

pub fn generate_bash(cfg: &Config) -> String {
    let opts = cfg
        .prog_options
        .iter()
        .map(|o| std::format!("{} {}", o.short, o.long).trim().to_owned())
        .collect::<Vec<_>>()
        .join(" ");

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

    let complete_current = format!(r#"COMPREPLY=( $(compgen -W "${{opts}}" -- "${{current}}") )"#,);

    std::format!(
        "#!/usr/bin/bash
_{prog_name}_completions() {{
    COMPREPLY=()
    local current=${{COMP_WORDS[COMP_CWORD]}}
    local previous=${{COMP_WORDS[COMP_CWORD-1]}}
    local opts=\"{opts}\"
    {fixed_value_vars}

    if [[ ${{current}} == -* || ${{COMP_CWORD}} -eq 1 ]]; then
        {complete_current}
        return 0
    fi

    case \"${{previous}}\" in
    {cases}

        *)
            COMPREPLY=()
            ;;
    esac

    {complete_current}
    return 0
}}

complete -F _{prog_name}_completions -o bashdefault -o default {prog_name}",
        prog_name = cfg.prog_name,
        opts = opts,
        cases = cases,
        complete_current = complete_current,
        fixed_value_vars = fixed_value_vars,
    )
}
