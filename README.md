# autocshell

autocshell is a command line tool that enables you to generate shell files/scripts that you can use
in order to provide auto  completion capabilities for your command line programs.

This program is able to generate scripts for various shells (currently only bash and zsh) and not for c-shell only (not to be confused due to the name being  auto-**cshell**).

The only thing that autocshell requires is a simple to create configuration file that will take as input.
In order to check the configuration file format please run autocshell with the **--config-help** flag.

By default autocshell prints the script in the standard output so you can redirect the output to whatever file you like but by providing the **--output** option the program can do that for you :)

# Configuration File:

The configuration file that you must provide as input (using -c or --config option)
has the following format:

```
shell:        <shell_type> (bash|zsh)
program_name: <program_name>
use_equals_sign: (true|false) [default: true] (available only for zsh) 
option*:
    short?: <short_name> _
                          |-> At least one should exist
    long?:  <long_name>  ‾
    takes_value?:         (true|false) [default: true]  (available only for zsh)
    accepts_files?:       (true|false) [default: false]
    accepts_multiple?:    (true|false) [default: false] (available only for zsh)
    description?:                                       (available only for zsh)
    fixed_values?:        [<fixed_value>, ...]          (available only for zsh)
```

Field/Values explanation:

Field: shell\
Value: It's the shell you want to generate the script for.\
Mandatory: yes

Field: program_name\
Value: The name of you program to generate the autocompletions for\
Mandatory: yes

Field: use_equals_sign\
Value: Denotes whether we want to add an equals sign (=) after option completion. This is valid only for zsh.\
Default: true\
Mandatory: no

Field: option\
Value: None. The option field gets no value. It starts a new option definition\
Mandatory: no

Field: short\
Value: The short option description (- must be included)\
Mandatory: no*

Field: long\
Value: The long option description (-- must be included)\
Mandatory: no*

Field: takes_value\
Value: Denotes whether this option takes an option or not (it's a flag). This is valid only for zsh.\
Default: true\
Mandatory: no

Field: accepts_files\
Value: Denotes whether that option takes files/directories as value(s). Must be true or false\
Default: false\
Mandatory: no

Field: accepts_multiple\
Value: This value denotes whether the option can appear multiple times in the cli . This is valid only for zsh.\
Default: false\
Mandatory: no

Field: description\
Value: This value contains the description that will appear when auto completing this option. This is valid only for zsh.\
Mandatory: no

Field: fixed_values\
Value: This value is a bracketed comma separated list of fixed values that will be auto completed for that option. This is valid only for zsh.\
Mandatory: no

\* short and long fields are not mandatory, however if you define an option at least one of them must be present.