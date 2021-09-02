const { Binary } = require("@cloudflare/binary-install");
const package = require("../package.json");

const os = require("os");

function getPlatform() {
  const type = os.type();
  const arch = os.arch();

  if (type === "Windows_NT" && arch === "x64") return "win64";
  if (type === "Linux" && arch === "x64") return "linux";
  if (type === "Darwin" && arch === "x64") return "macos";

  throw new Error(`Unsupported platform: ${type} ${arch}`);
}

function getBinary() {
  const platform = getPlatform();
  const version = package.version;
  const url = `https://github.com/scoville/tailwind-generator/releases/download/v${version}/pyaco-${platform}.tar.gz`;

  return new Binary(url, { name: "pyaco" });
}

module.exports = getBinary;
