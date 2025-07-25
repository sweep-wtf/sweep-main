const anchor = require("@coral-xyz/anchor");

module.exports = async function (provider) {
  anchor.setProvider(provider);
  const program = anchor.workspace.Sweep;
  console.log("Deploying Sweep program...");
};
