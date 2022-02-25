import { CasperContractClient, helpers, utils } from "casper-js-client-helper";
import { createRecipientAddress } from "casper-js-client-helper/dist/helpers/lib";
import {
  CasperClient,
  CLPublicKey,
  CLValueBuilder,
  Contracts,
  Keys,
  RuntimeArgs,
} from "casper-js-sdk";

import { DEFAULT_TTL } from "../common/constants";
import { createRpcClient } from "../common/rpc-client";

export type NamedKeys = {
  balances: any;
  owner: any;
  stakes: any;
  total_supply: any;
  whitelist: any;
};

export class ReputationContractJSClient extends CasperContractClient {
  protected rpcClient: ReturnType<typeof createRpcClient>;
  protected contractClient: Contracts.Contract;
  protected namedKeys?: Partial<NamedKeys>;

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
    const key = Buffer.from(account.toAccountHash()).toString("base64");
    console.log(key);
    const result = await this.contractClient.queryContractDictionary(
      "balances",
      key
    );
    const maybeValue = result.value().unwrap();
    return maybeValue.value().toString();
  }

  /**
   * Returns whitelist status of the specified account
   */
  public async getWhitelistOf(account: CLPublicKey) {
    const result = await this.contractClient.queryContractDictionary(
      "whitelist",
      account.toAccountHashStr().slice(13)
    );
    const maybeValue = result.value().unwrap();
    return maybeValue.value().toString();
  }

  /**
   * Returns stake of the specified account
   */
  public async getStakeOf(account: CLPublicKey) {
    const result = await this.contractClient.queryContractDictionary(
      "stakes",
      account.toAccountHashStr().slice(13)
    );
    const maybeValue = result.value().unwrap();
    return maybeValue.value().toString();
  }

  /**
   * Transfer an amount of tokens from address to address
   *
   * @param keys AsymmetricKey that will be used to sign the transaction.
   * @param owner Owner address.
   * @param recipient Recipient address.
   * @param transferAmount Amount of tokens that will be transfered.
   * @param paymentAmount Amount that will be used to pay the transaction.
   * @param ttl Time to live in miliseconds after which transaction will be expired (defaults to 30m).
   *
   * @returns Deploy hash.
   */
  public async transferFrom(
    keys: Keys.AsymmetricKey,
    owner: CLPublicKey,
    recipient: CLPublicKey,
    transferAmount: string,
    paymentAmount: string,
    ttl = DEFAULT_TTL
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      owner: createRecipientAddress(owner),
      recipient: createRecipientAddress(recipient),
      amount: CLValueBuilder.u256(transferAmount),
    });

    const deployHash = await this.contractClient
      .callEntrypoint(
        "transfer_from",
        runtimeArgs,
        keys.publicKey,
        this.chainName,
        paymentAmount,
        [keys]
      )
      .send(this.nodeAddress);

    return deployHash;
  }

  /**
   * Mint an amount of tokens
   *
   * @param keys AsymmetricKey that will be used to sign the transaction.
   * @param recipient Recipient address.
   * @param amount Amount of tokens that will be minted.
   * @param paymentAmount Amount that will be used to pay the transaction.
   * @param ttl Time to live in miliseconds after which transaction will be expired (defaults to 30m).
   *
   * @returns Deploy hash.
   */
  public async mint(
    keys: Keys.AsymmetricKey,
    recipient: CLPublicKey,
    amount: string,
    paymentAmount: string,
    ttl = DEFAULT_TTL
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      recipient: createRecipientAddress(recipient),
      amount: CLValueBuilder.u256(amount),
    });

    const deployHash = await this.contractClient
      .callEntrypoint(
        "mint",
        runtimeArgs,
        keys.publicKey,
        this.chainName,
        paymentAmount,
        [keys]
      )
      .send(this.nodeAddress);

    return deployHash;
  }

  /**
   * Burn an amount of tokens
   *
   * @param keys AsymmetricKey that will be used to sign the transaction.
   * @param owner Owner address.
   * @param amount Amount of tokens that will be burned.
   * @param paymentAmount Amount that will be used to pay the transaction.
   * @param ttl Time to live in miliseconds after which transaction will be expired (defaults to 30m).
   *
   * @returns Deploy hash.
   */
  public async burn(
    keys: Keys.AsymmetricKey,
    owner: CLPublicKey,
    amount: string,
    paymentAmount: string,
    ttl = DEFAULT_TTL
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      owner: createRecipientAddress(owner),
      amount: CLValueBuilder.u256(amount),
    });

    const deployHash = await this.contractClient
      .callEntrypoint(
        "burn",
        runtimeArgs,
        keys.publicKey,
        this.chainName,
        paymentAmount,
        [keys]
      )
      .send(this.nodeAddress);

    return deployHash;
  }

  /**
   * Change contract owner
   *
   * @param keys AsymmetricKey that will be used to sign the transaction.
   * @param owner New owner address.
   * @param paymentAmount Amount that will be used to pay the transaction.
   * @param ttl Time to live in miliseconds after which transaction will be expired (defaults to 30m).
   *
   * @returns Deploy hash.
   */
  public async changeOwnership(
    keys: Keys.AsymmetricKey,
    owner: CLPublicKey,
    paymentAmount: string,
    ttl = DEFAULT_TTL
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      owner: createRecipientAddress(owner),
    });

    const deployHash = await this.contractClient
      .callEntrypoint(
        "change_ownership",
        runtimeArgs,
        keys.publicKey,
        this.chainName,
        paymentAmount,
        [keys]
      )
      .send(this.nodeAddress);

    return deployHash;
  }

  /**
   * Add to whitelist
   *
   * @param keys AsymmetricKey that will be used to sign the transaction.
   * @param address Recipient address.
   * @param paymentAmount Amount that will be used to pay the transaction.
   * @param ttl Time to live in miliseconds after which transaction will be expired (defaults to 30m).
   *
   * @returns Deploy hash.
   */
  public async addToWhitelist(
    keys: Keys.AsymmetricKey,
    address: CLPublicKey,
    paymentAmount: string,
    ttl = DEFAULT_TTL
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      address: createRecipientAddress(address),
    });

    const deployHash = await this.contractClient
      .callEntrypoint(
        "add_to_whitelist",
        runtimeArgs,
        keys.publicKey,
        this.chainName,
        paymentAmount,
        [keys]
      )
      .send(this.nodeAddress);

    return deployHash;
  }

  /**
   * Remove from whitelist
   *
   * @param keys AsymmetricKey that will be used to sign the transaction.
   * @param address Recipient address.
   * @param paymentAmount Amount that will be used to pay the transaction.
   * @param ttl Time to live in miliseconds after which transaction will be expired (defaults to 30m).
   *
   * @returns Deploy hash.
   */
  public async removeFromWhitelist(
    keys: Keys.AsymmetricKey,
    address: CLPublicKey,
    paymentAmount: string,
    ttl = DEFAULT_TTL
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      address: createRecipientAddress(address),
    });

    const deployHash = await this.contractClient
      .callEntrypoint(
        "remove_from_whitelist",
        runtimeArgs,
        keys.publicKey,
        this.chainName,
        paymentAmount,
        [keys]
      )
      .send(this.nodeAddress);

    return deployHash;
  }

  /**
   * Stake an amount of tokens
   *
   * @param keys AsymmetricKey that will be used to sign the transaction.
   * @param address Recipient address.
   * @param amount Amount of tokens that will be staked.
   * @param paymentAmount Amount that will be used to pay the transaction.
   * @param ttl Time to live in miliseconds after which transaction will be expired (defaults to 30m).
   *
   * @returns Deploy hash.
   */
  public async stake(
    keys: Keys.AsymmetricKey,
    address: CLPublicKey,
    amount: string,
    paymentAmount: string,
    ttl = DEFAULT_TTL
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      address: createRecipientAddress(address),
      amount: CLValueBuilder.u256(amount),
    });

    const deployHash = await this.contractClient
      .callEntrypoint(
        "stake",
        runtimeArgs,
        keys.publicKey,
        this.chainName,
        paymentAmount,
        [keys]
      )
      .send(this.nodeAddress);

    return deployHash;
  }

  /**
   * Unstake an amount of tokens
   *
   * @param keys AsymmetricKey that will be used to sign the transaction.
   * @param address Recipient address.
   * @param amount Amount of tokens that will be unstaked.
   * @param paymentAmount Amount that will be used to pay the transaction.
   * @param ttl Time to live in miliseconds after which transaction will be expired (defaults to 30m).
   *
   * @returns Deploy hash.
   */
  public async unstake(
    keys: Keys.AsymmetricKey,
    address: CLPublicKey,
    amount: string,
    paymentAmount: string,
    ttl = DEFAULT_TTL
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      address: createRecipientAddress(address),
      amount: CLValueBuilder.u256(amount),
    });

    const deployHash = await this.contractClient
      .callEntrypoint(
        "unstake",
        runtimeArgs,
        keys.publicKey,
        this.chainName,
        paymentAmount,
        [keys]
      )
      .send(this.nodeAddress);

    return deployHash;
  }

  // EOF
}
