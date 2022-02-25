import {
  CasperClient,
  CasperServiceByJsonRPC,
  CLPublicKey,
} from "casper-js-sdk";

export const sleep = (ms: number) => {
  return new Promise((resolve) => setTimeout(resolve, ms));
};

export const getDeploy = async (NODE_ADDRESS: string, deployHash: string) => {
  const client = new CasperClient(NODE_ADDRESS);
  let i = 300;
  while (i != 0) {
    const [deploy, raw] = await client.getDeploy(deployHash);

    if (raw.execution_results.length !== 0) {
      // @ts-ignore
      if (raw.execution_results[0].result.Success) {
        return deploy;
      } else {
        // @ts-ignore
        throw Error(
          "Deploy execution: " +
            // @ts-ignore
            raw.execution_results[0].result.Failure.error_message
        );
      }
    } else {
      i--;
      await sleep(3000);
      continue;
    }
  }
  throw Error("Timeout after " + i + "s. Something's wrong");
};

export const waitForDeploy = async (
  NODE_ADDRESS: string,
  deployHash: string
) => {
  console.log(
    `... Contract deploy is pending, waiting for next block finalisation (deployHash: ${deployHash}) ...`
  );

  const deploy = await getDeploy(NODE_ADDRESS, deployHash);
  return deploy;
};

export const getAccountInfo: any = async (
  nodeAddress: string,
  publicKey: CLPublicKey
) => {
  const client = new CasperServiceByJsonRPC(nodeAddress);
  const stateRootHash = await client.getStateRootHash();
  const accountHash = publicKey.toAccountHashStr();
  const blockState = await client.getBlockState(stateRootHash, accountHash, []);
  return blockState.Account;
};

/**
 * Returns a value under an on-chain account's storage.
 * @param accountInfo - On-chain account's info.
 * @param namedKey - A named key associated with an on-chain account.
 */
export const getAccountNamedKeyValue = (accountInfo: any, namedKey: string) => {
  const found = accountInfo.namedKeys.find((i: any) => i.name === namedKey);
  if (found) {
    return found.key;
  }
  return undefined;
};
