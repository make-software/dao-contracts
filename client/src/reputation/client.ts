import { CasperContractClient } from "casper-js-client-helper";
import { createRecipientAddress } from "casper-js-client-helper/dist/helpers/lib";
import {
  CasperClient,
  CLPublicKey,
  CLValueBuilder,
  Contracts,
  Keys,
  RuntimeArgs,
} from "casper-js-sdk";

import { createDictionaryGetter } from "../../e2e/utils";
import { DEFAULT_TTL } from "../common/constants";
import { createRpcClient } from "../common/rpc-client";

export class ReputationContractJSClient extends CasperContractClient {
  protected rpcClient: ReturnType<typeof createRpcClient>;
  protected contractClient: Contracts.Contract;

  constructor(
    nodeAddress: string,
    chainName: string,
    contractHash: string,
    contractPackageHash: string,
    eventStreamAddress?: string
  ) {
    super(nodeAddress, chainName, eventStreamAddress);
    this.contractClient = new Contracts.Contract(new CasperClient(nodeAddress));
    this.contractClient.setContractHash(contractHash, contractPackageHash);
    this.rpcClient = createRpcClient(nodeAddress);
  }

  /**
   * Returns owner
   */
  public async getOwner() {
    return this.contractClient.queryContractData(["owner"]);
  }

  /**
   * Returns total supply
   */
  public async getTotalSupply() {
    return this.contractClient.queryContractData(["total_supply"]);
  }

  /**
   * Returns balance of the specified account
   */
  public async getBalanceOf(account: CLPublicKey) {
    return await createDictionaryGetter(
      this.contractClient,
      "balances",
      account
    );
  }

  /**
   * Returns whitelist status of the specified account
   */
  public async getWhitelistOf(account: CLPublicKey) {
    return await createDictionaryGetter(
      this.contractClient,
      "whitelist",
      account
    );
  }

  /**
   * Returns stake of the specified account
   */
  public async getStakeOf(account: CLPublicKey) {
    return await createDictionaryGetter(this.contractClient, "stakes", account);
  }

  /**
   * Transfer an amount of tokens from address to address
   *
   * @param owner Owner address.
   * @param recipient Recipient address.
   * @param transferAmount Amount of tokens that will be transfered.
   * @param paymentAmount Amount that will be used to pay the transaction.
   * @param keys AsymmetricKey that will be used to sign the transaction.
   *
   * @returns Deploy hash.
   */
  public createDeployTransferFrom(
    owner: CLPublicKey,
    recipient: CLPublicKey,
    transferAmount: string,
    paymentAmount: string,
    keys: Keys.AsymmetricKey = undefined
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      owner: createRecipientAddress(owner),
      recipient: createRecipientAddress(recipient),
      amount: CLValueBuilder.u256(transferAmount),
    });

    const deployHash = this.contractClient.callEntrypoint(
      "transfer_from",
      runtimeArgs,
      keys.publicKey,
      this.chainName,
      paymentAmount,
      keys && [keys]
    );

    return deployHash;
  }

  /**
   * Mint an amount of tokens
   *
   * @param recipient Recipient address.
   * @param amount Amount of tokens that will be minted.
   * @param paymentAmount Amount that will be used to pay the transaction.
   * @param keys AsymmetricKey that will be used to sign the transaction.
   *
   * @returns Deploy hash.
   */
  public createDeployMint(
    recipient: CLPublicKey,
    amount: string,
    paymentAmount: string,
    keys: Keys.AsymmetricKey = undefined
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      recipient: createRecipientAddress(recipient),
      amount: CLValueBuilder.u256(amount),
    });

    const deployHash = this.contractClient.callEntrypoint(
      "mint",
      runtimeArgs,
      keys.publicKey,
      this.chainName,
      paymentAmount,
      [keys]
    );

    return deployHash;
  }

  /**
   * Burn an amount of tokens
   *
   * @param owner Owner address.
   * @param amount Amount of tokens that will be burned.
   * @param paymentAmount Amount that will be used to pay the transaction.
   * @param keys AsymmetricKey that will be used to sign the transaction.
   *
   * @returns Deploy hash.
   */
  public createDeployBurn(
    owner: CLPublicKey,
    amount: string,
    paymentAmount: string,
    keys: Keys.AsymmetricKey = undefined
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      owner: createRecipientAddress(owner),
      amount: CLValueBuilder.u256(amount),
    });

    const deployHash = this.contractClient.callEntrypoint(
      "burn",
      runtimeArgs,
      keys.publicKey,
      this.chainName,
      paymentAmount,
      keys && [keys]
    );

    return deployHash;
  }

  /**
   * Change contract owner
   *
   * @param owner New owner address.
   * @param paymentAmount Amount that will be used to pay the transaction.
   * @param keys AsymmetricKey that will be used to sign the transaction.
   *
   * @returns Deploy hash.
   */
  public createDeployChangeOwnership(
    owner: CLPublicKey,
    paymentAmount: string,
    keys: Keys.AsymmetricKey = undefined
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      owner: createRecipientAddress(owner),
    });

    const deployHash = this.contractClient.callEntrypoint(
      "change_ownership",
      runtimeArgs,
      keys.publicKey,
      this.chainName,
      paymentAmount,
      keys && [keys]
    );

    return deployHash;
  }

  /**
   * Add to whitelist
   *
   * @param address Recipient address.
   * @param paymentAmount Amount that will be used to pay the transaction.
   * @param keys AsymmetricKey that will be used to sign the transaction.
   *
   * @returns Deploy hash.
   */
  public createDeployAddToWhitelist(
    address: CLPublicKey,
    paymentAmount: string,
    keys: Keys.AsymmetricKey = undefined
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      address: createRecipientAddress(address),
    });

    const deployHash = this.contractClient.callEntrypoint(
      "add_to_whitelist",
      runtimeArgs,
      keys.publicKey,
      this.chainName,
      paymentAmount,
      keys && [keys]
    );

    return deployHash;
  }

  /**
   * Remove from whitelist
   *
   * @param address Recipient address.
   * @param paymentAmount Amount that will be used to pay the transaction.
   * @param keys AsymmetricKey that will be used to sign the transaction.
   *
   * @returns Deploy hash.
   */
  public createDeployRemoveFromWhitelist(
    address: CLPublicKey,
    paymentAmount: string,
    keys: Keys.AsymmetricKey = undefined
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      address: createRecipientAddress(address),
    });

    const deployHash = this.contractClient.callEntrypoint(
      "remove_from_whitelist",
      runtimeArgs,
      keys.publicKey,
      this.chainName,
      paymentAmount,
      keys && [keys]
    );

    return deployHash;
  }

  /**
   * Stake an amount of tokens
   *
   * @param address Recipient address.
   * @param amount Amount of tokens that will be staked.
   * @param paymentAmount Amount that will be used to pay the transaction.
   * @param keys AsymmetricKey that will be used to sign the transaction.
   *
   * @returns Deploy hash.
   */
  public createDeployStake(
    address: CLPublicKey,
    amount: string,
    paymentAmount: string,
    keys: Keys.AsymmetricKey = undefined
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      address: createRecipientAddress(address),
      amount: CLValueBuilder.u256(amount),
    });

    const deployHash = this.contractClient.callEntrypoint(
      "stake",
      runtimeArgs,
      keys.publicKey,
      this.chainName,
      paymentAmount,
      keys && [keys]
    );

    return deployHash;
  }

  /**
   * Unstake an amount of tokens
   *
   * @param address Recipient address.
   * @param amount Amount of tokens that will be unstaked.
   * @param paymentAmount Amount that will be used to pay the transaction.
   * @param keys AsymmetricKey that will be used to sign the transaction.
   *
   * @returns Deploy hash.
   */
  public createDeployUnstake(
    address: CLPublicKey,
    amount: string,
    paymentAmount: string,
    keys: Keys.AsymmetricKey = undefined
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      address: createRecipientAddress(address),
      amount: CLValueBuilder.u256(amount),
    });

    const deploy = this.contractClient.callEntrypoint(
      "unstake",
      runtimeArgs,
      keys.publicKey,
      this.chainName,
      paymentAmount,
      keys && [keys]
    );

    return deploy;
  }

  // EOF
}
