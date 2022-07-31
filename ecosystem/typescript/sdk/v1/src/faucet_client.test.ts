import { AptosClient } from "./aptos_client";
import { FaucetClient } from "./faucet_client";
import { AptosAccount } from "./aptos_account";
import * as lodash from "lodash";
import { HexString } from "./hex_string";
import * as Gen from "./generated/index";

import { NODE_URL, FAUCET_URL } from "./util.test";
import { moveStructTagToParam } from "./util";

const aptosCoin = {
  address: "0x1",
  module: "coin",
  name: "CoinStore",
  generic_type_params: ["0x1::aptos_coin::AptosCoin"],
};

test(
  "full tutorial faucet flow",
  async () => {
    const client = new AptosClient(NODE_URL);
    const faucetClient = new FaucetClient(NODE_URL, FAUCET_URL);

    const account1 = new AptosAccount();
    const txns = await faucetClient.fundAccount(account1.address(), 5000);
    const tx1 = await client.transactions.getTransactionByHash(txns[1]);
    expect(tx1.type).toBe("user_transaction");
    let resources = await client.accounts.getAccountResources(account1.address().toString());
    let accountResource = resources.find((r) => lodash.isEqual(r.type, aptosCoin));
    expect((accountResource!.data as { coin: { value: string } }).coin.value).toBe("5000");

    const account2 = new AptosAccount();
    await faucetClient.fundAccount(account2.address(), 0);
    resources = await client.accounts.getAccountResources(account2.address().toString());
    accountResource = resources.find((r) => lodash.isEqual(r.type, aptosCoin));
    expect((accountResource!.data as { coin: { value: string } }).coin.value).toBe("0");

    const payload: Gen.TransactionPayload_ScriptFunctionPayload = {
      type: "script_function_payload",
      function: {
        module: {
          address: "0x1",
          name: "coin",
        },
        name: "transfer",
      },
      type_arguments: ["0x1::aptos_coin::AptosCoin"],
      arguments: [account2.address().hex(), "717"],
    };

    const txnRequest = await client.helpers.generateTransaction(account1.address(), payload);
    const signedTxn = await client.helpers.signTransaction(account1, txnRequest);
    const transactionRes = await client.transactions.submitTransaction(signedTxn);
    await client.helpers.waitForTransaction(transactionRes.hash);

    resources = await client.accounts.getAccountResources(account2.address().toString());
    accountResource = resources.find((r) => lodash.isEqual(r.type, aptosCoin));
    expect((accountResource!.data as { coin: { value: string } }).coin.value).toBe("717");

    const res = await client.transactions.getAccountTransactions(account1.address().toString(), "0");
    const tx = res.find((e) => e.type === "user_transaction") as Gen.UserTransaction;
    expect(new HexString(tx.sender).toShortString()).toBe(account1.address().toShortString());

    const events = await client.events.getEventsByEventHandle(
      tx.sender,
      moveStructTagToParam(aptosCoin),
      "withdraw_events",
    );
    expect(events[0].type).toBe("0x1::coin::WithdrawEvent");

    const eventSubset = await client.events.getEventsByEventHandle(
      tx.sender,
      moveStructTagToParam(aptosCoin),
      "withdraw_events",
      "0",
      1,
    );
    expect(eventSubset[0].type).toBe("0x1::coin::WithdrawEvent");

    const events2 = await client.events.getEventsByEventKey(events[0].key);
    expect(events2[0].type).toBe("0x1::coin::WithdrawEvent");
  },
  30 * 1000,
);
