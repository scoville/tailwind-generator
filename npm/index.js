const { generate, validate } = require("./index.node");

exports.generate = generate;

exports.validate = (options) =>
  new Promise((resolve) =>
    validate(options, () => {
      resolve();
    })
  );
