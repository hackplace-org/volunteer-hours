# Volunteer Hours

This tool uses the .dev_hours file in hack.place() repositories to generate a descriptive report of volunteer hours.

> **Note**
> If work on the repository began before hour tracking was implemented, that history will be consolidated into the initial entry instead.

```bash
Usage: volunteer-hours [OPTIONS] --url <URL> --name <NAME>

Options:
  -u, --url <URL>
          URL of the repository to analyze

  -n, --name <NAME>
          Your full name

  -d, --dir <DIR>
          Directory to clone the repository to

          [default: ./repo]

  -h, --help
          Print help (see a summary with '-h')
```
