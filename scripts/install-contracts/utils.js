const fs = require('fs');

const {
  CLValueBuilder,
  RuntimeArgs,
  encodeBase16,
  decodeBase16,
} = require('casper-js-sdk');

async function installContract(contractClient, rpcAPI, wasmPath, rawArgs, paymentAmount, chainName, pk) {
  const wasm = new Uint8Array(fs.readFileSync(wasmPath, null).buffer);
  const args = RuntimeArgs.fromMap(rawArgs);

  const signedDeploy = await contractClient.install(wasm, args, paymentAmount, pk.publicKey, chainName, [pk]);

  await rpcAPI.deploy(signedDeploy);

  const processedDeploy = await waitForDeploy(rpcAPI, signedDeploy.hash);

  return processedDeploy;
};

async function waitForDeploy(rpcAPI, deployHash, timeout = 80000) {
  const sleep = (ms) => {
    return new Promise(resolve => setTimeout(resolve, ms));
  };

  const timer = setTimeout(() => {
    throw new Error('Timeout');
  }, timeout);
  while (true) {
    try {
      const deploy = await rpcAPI.getDeployInfo(encodeBase16(deployHash));
      if (deploy.execution_results.length > 0) {
        clearTimeout(timer);
        return deploy;
      } else {
        await sleep(1000);
      }
    } catch (err) {
      console.log({ err });
      await sleep(1000);
    }
  }
}

function parseExecutionResult(deploy) {
  if (deploy.execution_results.length < 0) {
    throw new Error('empty deploy execution result');
  }

  const executionResult = deploy.execution_results[0].result.Success;
  if (!executionResult) {
    throw new Error('failed deploy');
  }

  return executionResult;
}

function parseWriteContract(deploy) {
  const executionResult = parseExecutionResult(deploy);

  const writeContractTransform = executionResult.effect.transforms.find(t => t.transform === 'WriteContract');
  if (!writeContractTransform) {
    throw new Error('no write contract transform');
  }

  const writeContractPackageTransform = executionResult.effect.transforms.find(t => t.transform === 'WriteContractPackage');
  if (!writeContractPackageTransform) {
    throw new Error('no write contract package transform');
  }

  return {
    actualCost: executionResult.cost,
    contractHash: writeContractTransform.key.replace('hash-', ''),
    contractPackageHash: writeContractPackageTransform.key.replace('hash-', ''),
  };
}

function stringToCLKey(param) {
  return CLValueBuilder.key(CLValueBuilder.byteArray(decodeBase16(param)))
}

module.exports = {
  installContract,
  waitForDeploy,
  parseExecutionResult,
  parseWriteContract,
  stringToCLKey,
};
