const axios = require("axios");
const { createWriteStream } = require("fs");

const { getBinary, getNativeNodeUrl, getPlatform } = require("./getBinary");

const platform = getPlatform();

if (platform) {
  // Supported platform, use an actual binary
  const binary = getBinary(platform);

  binary.install();
}

// Always install the native `.node` file
axios({
  method: "get",
  url: getNativeNodeUrl(),
  responseType: "stream",
}).then((response) =>
  response.data.pipe(createWriteStream("./npm/index.node"))
);
