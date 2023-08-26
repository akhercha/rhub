# rhub

Rhub is a CLI tool to create a github repository from a local folder.

```bash
Rhub is a CLI for pushing local repositories to Github.

Usage: rhub [OPTIONS] [directory]

Arguments:
  [directory]  The directory to send to Github [default: .]

Options:
  -c, --config <CONFIG>            The path to the configuration file containing the
                                   Github API key [default: config.toml] 
  -d, --description <DESCRIPTION>  The description of the repository [default: ]
  -n, --name <NAME>                The name of the repository [default: " "]
  -p, --private                    If present, makes the repository private
  -h, --help                       Print help
  -V, --version                    Print version
  ```

## Requirements

Copy the `config_example.toml` file to `config.toml` and fill the fields with your github credentials.


## Todo

- [x] Create the CLI args
- [x] Create the config file
- [x] See how to call git command from rust
- [ ] Validate the CLI args
- [ ] ... 

## Command flow

Main command: `rhub` => sends the current folder to github

```bash
($FOLDER default is ".")
`rhub $FOLDER` => sends the $FOLDER to github:
    -> if no .git folder
        -> create one
    -> if the repo has the same name in github
        -> stop
    -> create a new repo in github using POST
    -> set the remote origin to github 
    -> push the local repo to github
```


Optionnal commands:

```bash
`-n --name`
    name that the repository will take on github (default is the current folder name)
`-d --description`
    description of the repository (default is empty)
`-p --private`
    set the repository to private (default is public)
`-c --config`
    set the path to the config file (default is config.toml)
```
