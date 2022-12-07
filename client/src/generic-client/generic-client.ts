import { CasperContractClient } from "casper-js-client-helper";
import {
  CasperClient,
  CLBool,
  CLKey,
  CLPublicKey,
  CLU128,
  CLU512,
  CLU32,
  CLU512,
  CLU64,
  CLU8,
  Contracts,
  Keys,
  RuntimeArgs,
} from "casper-js-sdk";
import { Deploy } from "casper-js-sdk/dist/lib/DeployUtil";
import fs from "fs";
import { Err, Ok, Result } from "ts-results";
import YAML from "yaml";

import { createDictionaryGetter } from "../../e2e/utils";
import { DEFAULT_TTL } from "../common/constants";
import { createRpcClient } from "../common/rpc-client";

/** TYPES */

export type GenericContractSchema = {
  entry_points: Record<string, { arguments: GenericContractSchemaArguments }>;
  named_keys: Record<
    string,
    {
      named_key: string;
      cl_type: GenericContractSchemaClType;
    }
  >;
};

export type GenericContractSchemaArguments = {
  name: string;
  cl_type: string;
}[];

export type GenericContractSchemaClType =
  | string
  | Array<
      | { name: string; inner: string }
      | { name: string; key: string; value: string }
    >;

/** CONSTANTS */
const CLTypeDict = {
  Address: CLKey,
  U8: CLU8,
  U32: CLU32,
  U64: CLU64,
  U128: CLU128,
  U512: CLU512,: CLU512,
  Bool: CLBool,
};

/** MAIN */

export class GenericContractJSClient<
  T extends GenericContractSchema
> extends CasperContractClient {
  protected rpcClient: ReturnType<typeof createRpcClient>;
  protected contractClient: Contracts.Contract;

  constructor(
    nodeAddress: string,
    chainName: string,
    eventStreamAddress: string,
    contractHash: string,
    contractPackageHash: string,
    schemaPath: string
  ) {
    super(nodeAddress, chainName, eventStreamAddress);
    this.contractClient = new Contracts.Contract(new CasperClient(nodeAddress));
    this.contractClient.setContractHash(contractHash, contractPackageHash);
    this.rpcClient = createRpcClient(nodeAddress);

    const file = fs.readFileSync(schemaPath, "utf8");
    const parsed: {
      entry_points: Array<{
        name: string;
        arguments: Array<{ name: string; cl_type: string }>;
      }>;
      named_keys: Array<{
        name: string;
        named_key: string;
        cl_type: GenericContractSchemaClType;
      }>;
    } = YAML.parse(file);

    const schema: GenericContractSchema = {
      entry_points: parsed.entry_points.reduce((result, val) => {
        result[val.name] = { arguments: val.arguments };
        return result;
      }, {} as GenericContractSchema['entry_points']),
      named_keys: parsed.named_keys.reduce((result, val) => {
        result[val.name] = { named_key: val.named_key, cl_type: val.cl_type };
        return result;
      }, {} as GenericContractSchema['named_keys']),
    };

    const { entry_points, named_keys } = schema;

    // create entry_points deploy creator methods and attach to the class instance
    for (const key in entry_points) {
      const argsSchema = entry_points[key].arguments;
      this[`${key}`] = this.createDeployCreator(argsSchema);
    }

    // create named_keys getters methods and attach to the class instance
    for (const key in named_keys) {
      const { named_key: name, cl_type } = named_keys[key];
      this[`${key}`] = this.createNamedKeyGetterMethod(name, cl_type);
    }
  }

  public createDeploy(
    name: string,
    senderPublicKey: CLPublicKey,
    paymentAmount: string,
    ...args
  ): Result<Deploy, string> {
    return this[name](name, senderPublicKey, paymentAmount, ...args);
  }

  private createDeployCreator(
    argsSchema: {
      name: string;
      cl_type: string;
    }[]
  ) {
    return async (
      name: string,
      senderPublicKey: CLPublicKey,
      paymentAmount: string,
      ...args
    ) => {
      const runtimeArgs = RuntimeArgs.fromMap({});
      // prepare runtime args
      for (let index = 0; index < argsSchema.length; index++) {
        const argDefinition = argsSchema[index];
        // argument cl type validation
        const arg = args[index];
        if (!(arg instanceof CLTypeDict[argDefinition.cl_type])) {
          return Err(
            `Invalid type for ${argDefinition.name} argument, expected ${argDefinition.cl_type}`
          );
        }
        runtimeArgs.insert(argDefinition.name, arg);
      }

      const deploy = this.contractClient.callEntrypoint(
        name,
        runtimeArgs,
        senderPublicKey,
        this.chainName,
        paymentAmount,
        []
      );

      return Ok<Deploy>(deploy); // new is optional here
    };
  } 

  public async getNamedKey(name: string, ...args: any[]): Promise<unknown> {
    return await this[name](...args);
  }

  private createNamedKeyGetterMethod(
    name: string,
    schemaClType: GenericContractSchemaClType
  ) {
    return async (...args: any[]) => {
      if (typeof schemaClType === "string") {
        const CLTypeClass = CLTypeDict[schemaClType];
        if (CLTypeClass) {
          // this method returns parsed cltype value from sdk
          const value = await this.contractClient.queryContractData([name]);
          // console.log(` - sdk: ${JSON.stringify(value, null, 2)}`);
          return value;
        }
      } else if (Array.isArray(schemaClType) && schemaClType[0]) {
        const clTypeObj = schemaClType[0];
        if ("inner" in clTypeObj) {
          // this method returns parsed cltype value from sdk
          const value = await this.contractClient.queryContractData([name]);
          // console.log(` - sdk: ${JSON.stringify(value, null, 2)}`);
          return value;
        } else if ("key" in clTypeObj && "value" in clTypeObj) {
          if (clTypeObj.name === "Mapping") {
            const ArgumentClass = CLTypeDict[clTypeObj.key];
            const value = await createDictionaryGetter(
              this.contractClient,
              name,
              args[0]
              // new ArgumentClass(args[0])
            );
            // console.log(` sdk value: ${JSON.stringify(value, null, 2)}`);
            return value;
          }
        }
      }

      throw Error("Unknown CLType, possibly the library is outdated.");
    };
  }
}
