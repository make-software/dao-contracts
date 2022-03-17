import { CLMap, CLValueBuilder, CLValueParsers } from "casper-js-sdk";

export enum ReputationContractEvents {
  Transfer = "transfer",
  Mint = "mint",
  Burn = "burn",
  OwnerChanged = "owner_changed",
  AddedToWhitelist = "added_to_whitelist",
  RemovedFromWhitelist = "removed_from_whitelist",
  TokensStaked = "tokens_staked",
  TokensUnstaked = "tokens_unstaked",
}

export const ReputationContractEventParser = (
  {
    contractPackageHash,
    eventNames,
  }: { contractPackageHash: string; eventNames: ReputationContractEvents[] },
  value: any
) => {
  if (value.body.DeployProcessed.execution_result.Success) {
    const { transforms } =
      value.body.DeployProcessed.execution_result.Success.effect;

    const eventsData = transforms.reduce((acc: any, val: any) => {
      if (
        val.transform.hasOwnProperty("WriteCLValue") &&
        typeof val.transform.WriteCLValue.parsed === "object" &&
        val.transform.WriteCLValue.parsed !== null
      ) {
        const maybeCLValue = CLValueParsers.fromJSON(
          val.transform.WriteCLValue
        );
        const clValue = maybeCLValue.unwrap();
        if (clValue && clValue instanceof CLMap) {
          const hash = clValue.get(
            CLValueBuilder.string("contract_package_hash")
          );
          const event = clValue.get(CLValueBuilder.string("event_type"));
          if (
            hash &&
            // NOTE: Calling toLowerCase() because current JS-SDK doesn't support checksumed hashes and returns all lower case value
            // Remove it after updating SDK
            hash.value() === contractPackageHash.slice(5).toLowerCase() &&
            event &&
            eventNames.includes(event.value())
          ) {
            acc = [...acc, { name: event.value(), clValue }];
          }
        }
      }
      return acc;
    }, []);

    return { error: null, success: !!eventsData.length, data: eventsData };
  }

  return null;
};
