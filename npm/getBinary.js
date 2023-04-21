const { Binary } = require("@cloudflare/binary-install");
const package = require("../package.json");

const os = require("os");

const baseUrl = `https://github.com/scoville/tailwind-generator/releases/download`;

function getPlatform() {
  const type = os.type();
  const arch = os.arch();

  if (type === "Windows_NT" && arch === "x64") return "win64";
  if (type === "Linux" && arch === "x64") return "linux";
  if (type === "Darwin" && arch === "x64") return "macos";
  // if (type === "Darwin" && arch === "arm64") return "macos-arm64";

  console.warn(
    `Unknown platform: ${type} ${arch}.
Defaulting to the node native module which might be slower.
`,
  );

  return null;
}

function getNativeNodeUrl() {
  return `${baseUrl}/v${package.version}/pyaco-index.node`;
}

function getBinary(platform) {
  return new Binary(`${baseUrl}/v${package.version}/pyaco-${platform}.tar.gz`, {
    name: "pyaco",
  });
}

exports.getBinary = getBinary;

exports.getNativeNodeUrl = getNativeNodeUrl;

exports.getPlatform = getPlatform;
