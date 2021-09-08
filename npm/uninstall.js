try {
  const { getBinary, getPlatform } = require("./getBinary");

  const platform = getPlatform();

  if (platform) {
    // Supported platform, use an actual binary
    const binary = getBinary(platform);

    binary.uninstall();
  }
} catch {
  // Not already installed, does nothing
}
