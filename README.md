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
