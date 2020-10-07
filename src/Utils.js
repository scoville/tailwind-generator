const { mkdirSync } = require("fs");
const { run } = require("tailwindcss/lib/cli/commands/build");
const css = require("css");

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

exports._getClassNames = function (cssContent, source) {
  try {
    return Array.from(
      new Set(
        css
          .parse(cssContent, { source })
          .stylesheet.rules.flatMap((rule) =>
            rule.type === "rule" ? rule.selectors : []
          )
          .reduce((acc, selector) => {
            try {
              const className = selector.match(/\.((\w|\\\:|\\\/|\-)+)/)[1];

              if (!className) {
                return acc;
              }

              return [...acc, className.replace(/\\/, "\\\\")];
            } catch (_) {
              return acc;
            }
          }, [])
      )
    );
  } catch (_) {
    return [];
  }
};
