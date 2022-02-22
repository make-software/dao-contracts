import {
  Keys,
  RuntimeArgs,
} from "casper-js-sdk";
import {
  CasperContractClient,
  helpers,
} from "casper-js-client-helper";

import { DAOReputationEvents } from "./constants";

const {
  installContract,
  setClient,
  contractSimpleGetter,
} = helpers;

class Client extends CasperContractClient {
  protected namedKeys?: {
    allowances: string;
    balances: string;
  };

  /**
   * Installs the contract.
   *
   * @param keys AsymmetricKey that will be used to install the contract.
   * @param paymentAmount The payment amount that will be used to install the contract.
   * @param wasmPath Path to the WASM file that will be installed.
   *
   * @returns Installation deploy hash.
   */
  public async install(
    keys: Keys.AsymmetricKey,
    paymentAmount: string,
    wasmPath: string
  ) {
    const runtimeArgs = RuntimeArgs.fromNamedArgs([]);

    try {
      return await installContract(
        this.chainName,
        this.nodeAddress,
        keys,
        runtimeArgs,
        paymentAmount,
        wasmPath
      );
    } catch (error) {
      console.error(error);
      return Promise.reject(error);
    }    
  }

  /**
   * Set contract hash so its possible to communicate with it.
   *
   * @param hash Contract hash (raw hex string as well as `hash-` prefixed format is supported).
   */
  public async setContractHash(hash: string) {
    const properHash = hash.startsWith("hash-") ? hash.slice(5) : hash;
    const { contractPackageHash, namedKeys } = await setClient(
      this.nodeAddress,
      properHash,
      ["todo"]
    );
    this.contractHash = hash;
    this.contractPackageHash = contractPackageHash;
    /* @ts-ignore */
    this.namedKeys = namedKeys;
  }

  /**
   * Returns the todo
   */
  public async todo() {
    return await contractSimpleGetter(this.nodeAddress, this.contractHash!, [
      "todo",
    ]);
  }
}

export default Client;
