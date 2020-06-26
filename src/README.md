# autocshell

autocshell is a command line tool that enables you to generate shell files/scripts that you can use
in order to provide auto  completion capabilities for your command line programs.

This program is able to generate scripts for various shells (currently only bash) and not for c-shell only (not to be confused due to the name being  auto-**cshell**).

The only thing that autocshell requires is a simple to create configuration file that will take as input.
In order to check the configuration file format please run autocshell with the **--config-help** flag.

By default autocshell prints the script in the standard output so you can redirect the output to whatever file you like but by providing the **--output** option the program can do that for you :)
