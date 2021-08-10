function getBinary(name) {
  try {
    const getBinary = require("./getBinary");

    return getBinary(name);
  } catch {}
}

const generate = getBinary("generate");

if (generate) {
  generate.uninstall();
}

const validate = getBinary("validate");

const validate = getBinary("generate");

if (validate) {
  validate.uninstall();
}
