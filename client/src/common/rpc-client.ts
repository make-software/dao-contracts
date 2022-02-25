import { RequestManager, HTTPTransport, Client } from "@open-rpc/client-js";
import { StoredValue } from "casper-js-sdk/dist/lib/StoredValue";

export const createRpcClient = (nodeAddress: string) => {
  const transport = new HTTPTransport(nodeAddress);
  const requestManager = new RequestManager([transport]);
  const rpcClient = new Client(requestManager);

  const fetchStateGetItem = async (
    stateRootHash: string,
    key: string,
    path: string[] = []
  ) => {
    return await rpcClient
      .request({
        method: "state_get_item",
        params: {
          state_root_hash: stateRootHash,
          key: key,
          path,
        },
      })
      .then((res) =>
        res.error != null ? res.error : (res.stored_value as StoredValue)
      ).catch(err => {
        console.error(err);
      });
  };

  return {
    fetchStateGetItem,
  };
};
