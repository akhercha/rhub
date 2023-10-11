# rhub

Rhub is a CLI tool that automatically creates github repositories from local folders.

```bash
Rhub is a CLI for pushing local repositories to Github.

Usage: rhub [OPTIONS] [directory]

Arguments:
  [directory]  The directory to send to Github [default: .]

Options:
  -d, --description <DESCRIPTION>  The description of the repository [default: ]
  -n, --name <NAME>                The name that the repository will take on GitHub [default: ]
  -p, --private                    If present, makes the repository private
  -h, --help                       Print help
  -V, --version                    Print version
```

## Credentials

You must set the GITHUB_TOKEN environment variable to your github token:

```bash
export GITHUB_TOKEN=your_token
```

TODO: Add a way to set the token in a config file / CLI parameter.

## Todo

- [x] Create the CLI args
- [x] Create the config file
- [x] See how to call git command from rust
- [x] Validate the CLI args
- [x] Check if the repo exists on github
- [x] Create a new repo on github using arguments
- [x] Set the remote origin to github
- [x] Push the local files to the repo
- [x] Remove the .toml file requirement & change the way the token is provided
- [ ] Refacto the git command flow (yikes)
- [ ] Build the command & try to use it as a binary in linux
- [ ] See if the configuration is good for this kind of binary,
