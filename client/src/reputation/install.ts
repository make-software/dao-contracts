import { helpers } from "casper-js-client-helper";
import { CasperClient, Contracts, Keys, RuntimeArgs } from "casper-js-sdk";
import * as fs from "fs";

const getBinary = (pathToBinary: string) => {
  return new Uint8Array(fs.readFileSync(pathToBinary, null).buffer);
};

const { installContract } = helpers;
/**
 * Installs the contract.
 *
 * @param keys AsymmetricKey that will be used to install the contract.
 * @param paymentAmount The payment amount that will be used to install the contract.
 * @param wasmPath Path to the WASM file that will be installed.
 *
 * @returns Installation deploy hash.
 */
export async function installReputationContract(
  chainName: string,
  nodeAddress: string,
  keys: Keys.AsymmetricKey,
  paymentAmount: string,
  wasmPath: string
) {
  const contractClient = new Contracts.Contract(new CasperClient(nodeAddress));
  const runtimeArgs = RuntimeArgs.fromNamedArgs([]);

  const deploy = contractClient.install(
    getBinary(wasmPath),
    runtimeArgs,
    paymentAmount,
    keys.publicKey,
    chainName,
    [keys]
  );

  const deployHash = await deploy.send(nodeAddress);
  return deployHash;
}
