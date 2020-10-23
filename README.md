## Tailwind generator

Generates code and css from a tailwind config file. Currently supports Elm, ReasonML, TypeScript, and PureScript!

### Commands:

To get help:

`$ tailwind-generator --help`

Options:

- `-c`, `--config` Provide tailwind.config.js path
- `-l`, `--lang` Language used in generated code (`purescript|elm|reasonml|typescript|typescript-type-level|typescript-type-level-2`)
- `-o`, `--output` Provide directory for generated code (default: `./src`)
- `--cssOutput` Provide full path (including file name) for generated css stylesheet (default: `./tailwind.css`)
- `--cssInput` Provide path of your css stylesheet which uses the @tailwind directive to inject Tailwind's preflight and utilities styles into your CSS - (default: `$TAILWIND_GENERATOR/assets/input.css`)
- `-h`, `--help` Display the help message

### Generators

#### TypeScript (typescript)

The simplest generator for TypeScript, it exports an [opaque type](https://en.wikipedia.org/wiki/Opaque_data_type) `Tailwind`, `tailwind` function build, and a set of `Tailwind` objects:

```ts
import {
  tailwind,
  textBlue100,
  rounded,
  border,
  borderBlue300,
} from "./tailwind.ts";

// ...

<div className={tailwind([textBlue100, rounded, border, borderBlue300])}>
  Hello
</div>;
```

Pros:

- Easy to use
- Very flexible
- Compatible with most TypeScript versions
- Safe, you can't pass any string to the `tailwind` function
- Autocomplete

Cons:

- Cost at runtime: each Tailwind object is a JavaScript object to ensure type opacity
- Cost at runtime: the array has to be joined into a string
- Imports can be verbose
- Not the "standard" class names, `h-full` becomes `hFull`, etc...

#### TypeScript type level (typescript-type-level)

This generator doesn't generate any runtime code apart from the `tailwind` constructor.

```ts
import { tailwind } from "./tailwind.ts";

// ...

<div
  className={tailwind("text-blue-100", "rounded", "border", "border-blue-300")}
>
  Hello
</div>;
```

Pros:

- Easy to use
- Very flexible
- Compatible with most TypeScript versions
- Safe, you can't pass any string to the `tailwind` function
- "Standard" class names
- Light import (you only new `tailwind` most of the time)
- Autocomplete

Cons:

- Cost at runtime: the array has to be joined into a string
- You need to escape the class names

#### TypeScript type level 2 (typescript-type-level-2)

This generator doesn't generate any runtime code apart from the `tailwind` constructor.

```ts
import { tailwind } from "./tailwind.ts";

// ...

<div className={tailwind("text-blue-100 rounded border border-blue-300")}>
  Hello
</div>;
```

Pros:

- Super easy to use
- Safe, you can't pass any string to the `tailwind` function
- "Standard" class names
- Light import (you only new `tailwind` most of the time)
- No runtime cost at all
- Partial support for autocompletion

Cons:

- Not as flexible as the 2 other generators
- Compatible with TypeScript > 4.1 only
- Type error can be hard to debug
- Doesn't accept multiple spaces
- You need to escape the class names
