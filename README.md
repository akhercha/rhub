# rhub

Rhub is a CLI tool to create a github repository from a local folder.


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
