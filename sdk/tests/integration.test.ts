import assert from 'node:assert/strict';
import test from 'node:test';

import { Contract, QuantumVMClient, QuantumVMSDK } from '../src/sdk';
import { Transaction } from '../src/tx';

type RpcRequest = {
  url: string;
  payload: {
    jsonrpc: string;
    method: string;
    params: unknown[];
    id: number;
  };
};

type MockResponse = {
  result: unknown;
};

function installFetchMock(responses: MockResponse[]) {
  const captured: RpcRequest[] = [];
  const originalFetch = globalThis.fetch;
  let index = 0;

  (globalThis as Record<string, unknown>).fetch = async (
    url: string,
    init: RequestInit
  ) => {
    const body = JSON.parse(String(init.body));
    captured.push({
      url,
      payload: body
    });

    const response = responses[index];
    index += 1;
    return {
      json: async () => ({ result: response?.result ?? null })
    };
  };

  return {
    captured,
    restore: () => {
      (globalThis as Record<string, unknown>).fetch = originalFetch;
    }
  };
}

test('Contract deploy/call emit expected JSON-RPC envelopes', async () => {
  const mock = installFetchMock([{ result: '0xcontract' }, { result: { ok: true } }]);
  try {
    const client = new QuantumVMClient('http://localhost:8545');
    const contract = new Contract(client, [{ name: 'run' }], '0xdeadbeef');

    const deployResult = await contract.deploy('0xabc', 500_000);
    const callResult = await contract.call('run', ['arg1']);

    assert.equal(deployResult, '0xcontract');
    assert.deepEqual(callResult, { ok: true });
    assert.equal(mock.captured.length, 2);

    assert.equal(mock.captured[0].url, 'http://localhost:8545');
    assert.equal(mock.captured[0].payload.method, 'contract_deploy');
    assert.deepEqual(mock.captured[0].payload.params, ['0xabc', '0xdeadbeef', 500_000]);

    assert.equal(mock.captured[1].payload.method, 'contract_call');
    assert.deepEqual(mock.captured[1].payload.params, ['run', ['arg1']]);
  } finally {
    mock.restore();
  }
});

test('QuantumVMSDK tx/balance/block RPC calls are end-to-end well-formed', async () => {
  const mock = installFetchMock([
    { result: '0xtxhash' },
    { result: '42' },
    { result: 777 }
  ]);

  try {
    const sdk = new QuantumVMSDK('http://localhost:8545');
    const tx = new Transaction(
      '0xfrom',
      '0xto',
      1000n,
      5,
      21000,
      new Uint8Array([1, 2, 3]),
      1700000000
    );
    tx.setSignature(new Uint8Array([9, 8, 7]));

    const txHash = await sdk.sendTransaction(tx);
    const balance = await sdk.getBalance('0xfrom');
    const block = await sdk.getBlockNumber();

    assert.equal(txHash, '0xtxhash');
    assert.equal(balance, '42');
    assert.equal(block, 777);
    assert.equal(mock.captured.length, 3);

    const sent = mock.captured[0].payload;
    assert.equal(sent.method, 'tx_sendRaw');
    assert.equal(Array.isArray(sent.params), true);
    assert.equal(Array.isArray(sent.params[0] as unknown[]), true);
    assert.deepEqual(sent.params[1], [9, 8, 7]);

    assert.equal(mock.captured[1].payload.method, 'get_balance');
    assert.deepEqual(mock.captured[1].payload.params, ['0xfrom']);

    assert.equal(mock.captured[2].payload.method, 'get_blockNumber');
    assert.deepEqual(mock.captured[2].payload.params, []);
  } finally {
    mock.restore();
  }
});
