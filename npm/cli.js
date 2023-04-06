// An alternative CLI that uses the node native module
// and acts as a polyfill for unsupported platforms.

const yargs = require("yargs/yargs");
const { hideBin } = require("yargs/helpers");

const { generate, validate } = require("../index");

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
            "Language used in generated code (elm|purescript|rescript|typescript|typescript-type1|typescript-type2)",
        })
        .option("watch", {
          alias: "w",
          type: "boolean",
          describe:
            "Watch for changes in the provided css file and regenarate the code (doesn't work with URL)",
          default: false,
        })
        .option("watch-debounce-duration", {
          type: "number",
          describe:
            "Watch debounce duration (in ms), if files are validated twice after saving the css file, you should try to increase this value",
          default: 10,
        })
        .option("output-directory", {
          alias: "o",
          describe: "Directory for generated code",
          default: "./",
        });
    },
    (argv) =>
      generate({
        input: argv.input,
        lang: argv.lang,
        output_directory: argv["output-directory"],
        watch: argv.watch,
        watch_debounce_duration: argv["watch-debounce-duration"],
        output_filename: argv["output-filename"],
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
        })
        .option("watch", {
          alias: "w",
          type: "boolean",
          describe:
            "Watch for changes in the provided css file and files and regenarate the code (doesn't work with URL)",
          default: false,
        })
        .option("watch-debounce-duration", {
          type: "number",
          describe:
            "Watch debounce duration (in ms), if files are validated twice after saving a file, you should try to increase this value",
          default: 10,
        });
    },
    (argv) => {
      validate({
        css_input: argv["css-input"],
        input_glob: argv["input-glob"],
        capture_regex: argv["capture-regex"],
        max_opened_files: argv["max-opened-files"],
        split_regex: argv["split-regex"],
        watch: argv["watch"],
        watch_debounce_duration: argv["watch-debounce-duration"],
      });
    }
  ).argv;
