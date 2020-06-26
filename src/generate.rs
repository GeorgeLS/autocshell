use crate::config::*;

fn format_cases_for_bash(program_option: &ProgramOption) -> String {
    let compreply = "
            COMPREPLY=( $(compgen -f \"${current}\") )
            return 0
            ;;
    ";

    let short_case = if !program_option.short.is_empty() {
        std::format!("{})\n\t{}", program_option.short, compreply)
    } else {
        String::new()
    };

    let long_case = if !program_option.long.is_empty() {
        std::format!("{})\n\t{}", program_option.long, compreply)
    } else {
        String::new()
    };

    std::format!("{}\n\t{}", short_case, long_case)
}

fn generate_bash(cfg: &Config) -> String {
    let opts = cfg
        .prog_options
        .iter()
        .map(|o| std::format!("{} {}", o.short, o.long).trim().to_string())
        .collect::<Vec<_>>()
        .join(" ");

    let cases = cfg
        .prog_options
        .iter()
        .filter(|o| o.accepts_files)
        .map(|o| format_cases_for_bash(o))
        .collect::<Vec<_>>()
        .join("");

    std::format!(
        "#!/usr/bin/bash
_{prog_name}_completions() {{
    COMPREPLY=()
    local current=${{COMP_WORDS[COMP_CWORD]}}
    local previous=${{COMP_WORDS[COMP_CWORD-1]}}
    local opts=\"{opts}\"
    
    if [[ ${{current}} == -* || ${{COMP_CWORD}} -eq 1 ]]; then
        COMPREPLY=( $(compgen -W \"${{opts}}\" -- \"${{current}}\") )
        return 0
    fi

    case \"${{previous}}\" in
        {cases}
        *)
            COMPREPLY=()
            ;;
    esac
    COMPREPLY=( $(compgen -W \"${{opts}}\" -- \"${{current}}\") )
    return 0
}}

complete -F _{prog_name}_completions -o bashdefault -o default {prog_name}",
        prog_name = cfg.prog_name,
        opts = opts,
        cases = cases
    )
}

pub fn generate_script(cfg: &Config) -> String {
    if cfg.shell_type == "bash" {
        generate_bash(cfg)
    } else {
        String::new()
    }
}
