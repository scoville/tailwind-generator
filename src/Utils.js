const { mkdirSync } = require("fs");
const { run } = require("tailwindcss/lib/cli/commands/build");

exports._tailwindBuild = function (config, cssInput, cssOutput) {
  // It seems there is no quiet/silent option when building tailwind assets, so had to improvise...
  const log = console.log;
  const warn = console.warn;

  console.log = () => {};
  console.warn = () => {};

  return run([cssInput], {
    config: config ? [config] : [],
    output: [cssOutput],
  }).finally(() => {
    console.log = log;
    console.warn = warn;
  });
};

exports._mkdirp = function (path) {
  mkdirSync(path, { recursive: true });
};
