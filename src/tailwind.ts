#!/usr/bin/env node

import cac from "cac";
import fs from "fs";
import path from "path";

import langs from "./langs";
import { Adapter, Lang } from "./types";
import * as utils from "./utils";

// tslint:disable-next-line: no-var-requires
const build = require("tailwindcss/lib/cli/commands/build");

interface Options {
  config?: string;
  cssInput: string;
  cssOutput: string;
  lang: string;
  output: string;
}

const loadAdapters: () => Record<Lang, Promise<Adapter>> = () =>
  langs.reduce(
    (agg, lang) => ({
      ...agg,
      [lang]: import(`./adapters/${lang}`),
    }),
    // tslint:disable-next-line: no-object-literal-type-assertion
    {} as Record<Lang, Promise<Adapter>>,
  );

const main = async ({ config, cssInput, cssOutput, lang, output }: Options) => {
  const adapters = await loadAdapters();

  if (!utils.isValidLang(lang)) {
    // tslint:disable-next-line: no-console
    console.error(
      `lang should be one of ${Object.keys(adapters).join(", ")}, got ${lang}`,
    );

    return process.exit(1);
  }

  const adapter = await adapters[lang];

  await utils.shutDownLog(async () => {
    await build.run([cssInput], {
      config: config ? [config] : [],
      output: [cssOutput],
    });

    try {
      fs.mkdirSync(output, { recursive: true });
    } catch (error) {
      // tslint:disable-next-line: no-console
      console.error(`Couldn't create directory ${output}`);
      return process.exit(1);
    }

    adapter.save(output, utils.readClasses(cssOutput));
  });

  // tslint:disable-next-line: no-console
  console.log("Successfully generated files!");
};

const cli = cac();

// CLI options

cli.command("", "Generates code and css from a tailwind config file");

cli.option("-c, --config <config>", "Provide tailwind.config.js path");

cli.option(
  "-l, --lang <lang>",
  "Language used in generated code (elm|reasonml|typescript|purescript)",
);

cli.option("-o, --output <dir>", "Provide directory for generated code", {
  default: "./src",
});

cli.option(
  "--cssOutput <stylesheet>",
  "Provide full path (including file name) for generated css stylesheet",
  {
    default: "./tailwind.css",
  },
);

cli.option(
  "--cssInput <stylesheet>",
  "Provide path of your css stylesheet which uses the @tailwind directive to inject Tailwind's preflight and utilities styles into your CSS",
  { default: path.join(__dirname, "..", "assets", "input.css") },
);

cli.help();

// Run the command with options
main(cli.parse().options as Options);
