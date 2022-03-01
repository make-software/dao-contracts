import { CasperClient, Contracts, Keys, RuntimeArgs } from "casper-js-sdk";
import * as fs from "fs";

const getBinary = (pathToBinary: string) => {
  return new Uint8Array(fs.readFileSync(pathToBinary, null).buffer);
};

/**
 * Installs the contract.
 *
 * @param chainName Name of the chain
 * @param nodeAddress Url of node
 * @param paymentAmount The payment amount that will be used to install the contract.
 * @param wasmPath Path to the WASM file that will be installed.
 * @param keys AsymmetricKey that will be used to install the contract.
 *
 * @returns Installation deploy hash.
 */
export function createInstallReputationContractDeploy(
  chainName: string,
  nodeAddress: string,
  paymentAmount: string,
  wasmPath: string,
  keys?: Keys.AsymmetricKey
) {
  const contractClient = new Contracts.Contract(new CasperClient(nodeAddress));
  const runtimeArgs = RuntimeArgs.fromNamedArgs([]);

  const deploy = contractClient.install(
    getBinary(wasmPath),
    runtimeArgs,
    paymentAmount,
    keys.publicKey,
    chainName,
    keys && [keys]
  );

  return deploy;
}
