## Tailwind generator

Generates code and css from a tailwind config file. Currently supports Elm and ReasonML!

### Commands:

To get help:

`$ tailwind-generator --help`

Options:

- `-c`, `--config` Provide tailwind.config.js path
- `-l`, `--lang` Language used in generated code (`elm|reasonml`)
- `-o`, `--output` Provide directory for generated code (default: `./src`)
- `--cssOutput` Provide full path (including file name) for generated css stylesheet (default: `./tailwind.css`)
- `--cssInput` Provide path of your css stylesheet which uses the @tailwind directive to inject Tailwind's preflight and utilities styles into your CSS - (default: `$TAILWIND_GENERATOR/assets/input.css`)
- `-h`, `--help` Display the help message
