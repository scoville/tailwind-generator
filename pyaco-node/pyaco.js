#!/usr/bin/env node

const yargs = require("yargs/yargs");
const { hideBin } = require("yargs/helpers");

const pyaco = require("./index.node");

yargs(hideBin(process.argv))
  .command(
    "generate",
    "generate code",
    (yargs) => {
      return yargs
        .positional("output-filename", {
          alias: "f",
          describe: "Filename (without extension) used for the generated code",
        })
        .positional("input", {
          alias: "i",
          describe: "CSS file path and/or URL to parse and generate code from",
        })
        .positional("lang", {
          alias: "l",
          describe:
            "Language used in generated code (elm|purescript|rescript|typescript|typescript-type-1|typescript-type-2)",
        })
        .option("watch", {
          alias: "w",
          type: "boolean",
          describe:
            "Watch for changes in the provided css file and regenarate the code (doesn't work with URL)",
          default: false,
        })
        .option("output-directory", {
          alias: "o",
          describe: "Directory for generated code",
          default: "./",
        });
    },
    (argv) =>
      pyaco.generate({
        input: argv.input,
        lang: argv.lang,
        outputDirectory: argv["output-directory"],
        watch: argv.watch,
        outputFilename: argv["output-filename"],
      })
  )
  .command(
    "validate",
    "validate code",
    (yargs) => {
      return yargs
        .positional("css-input", {
          alias: "c",
          describe: "CSS file path or URL used for code verification",
        })
        .positional("input-glob", {
          alias: "i",
          describe: "Glob pointing to the files to validate",
        })
        .option("capture-regex", {
          describe:
            "lasses matcher regex, must include a capture containing all the classes",
          default: 'class="([^"]+)"',
        })
        .option("max-opened-files", {
          type: "number",
          describe:
            "How many files can be read concurrently at most, setting this value to a big number might break depending on your system",
          default: 128,
        })
        .option("split-regex", {
          describe:
            "Classes splitter regex, will split the string captured with the `capture_regex` argument and split it into classes",
          default: "\\s+",
        });
    },
    (argv) =>
      pyaco.validate(
        {
          cssInput: argv["css-input"],
          inputGlob: argv["input-glob"],
          captureRegex: argv["capture-regex"],
          maxOpenedFiles: argv["max-opened-files"],
          splitRegex: argv["split-regex"],
        },
        () => {}
      )
  ).argv;
