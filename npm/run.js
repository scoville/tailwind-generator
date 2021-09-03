#!/usr/bin/env node

const { getBinary, getPlatform } = require("./getBinary");

const platform = getPlatform();

if (platform) {
  // Run the Rust binary on supported platforms
  const binary = getBinary(platform);

  binary.run();
} else {
  // And the Node facade otherwise
  require("./cli");
}
