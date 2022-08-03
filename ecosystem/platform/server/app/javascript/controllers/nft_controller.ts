import { Controller } from "./controller";
import type { FrameElement } from "@hotwired/turbo/dist/types/elements";
import type {AptosAccount, AptosClient, FaucetClient, Types} from "aptos";

const ADDRESS = "0xb6944e1bcbededcc0be23e33ff9d6d083d2a40bf96f18ac073bf40d9c1750877";
const MODULE = "NFT3";

async function signAndSubmit(client: AptosClient, payload: Types.TransactionPayload, account: AptosAccount): Promise<Types.OnChainTransaction> {
  const txnRequest = await client.generateTransaction(account.address(), payload);
  const signedTxn = await client.signTransaction(account, txnRequest);
  const pendingTransaction = await client.submitTransaction(signedTxn);
  await client.waitForTransaction(pendingTransaction.hash);
  const txn = await client.getTransaction(pendingTransaction.hash);
  if ('version' in txn) return txn;
  throw new Error(`transaction ${txn.hash} is not on chain`);
}
// Connects to data-controller="nft"
export default class extends Controller<FrameElement> {
  static targets = ['form', 'explorerUrl'];

  declare readonly hasFormTarget: boolean;
  declare readonly formTarget: HTMLFormElement;
  declare readonly explorerUrlTarget: HTMLInputElement;

  connect() {
    if (this.hasFormTarget) {
      this.mint();
    }
  }

  async mint() {
    const aptos = await import("aptos");

    const NODE_URL = "https://fullnode.nft-nyc.aptoslabs.com";
    const FAUCET_URL = "https://faucet.nft-nyc.aptoslabs.com";

    const client = new aptos.AptosClient(NODE_URL);
    const faucet = new aptos.FaucetClient(NODE_URL, FAUCET_URL);

    const account = new aptos.AptosAccount();
    await this.fundAccount(account, faucet);

    const txn = await this.createSubmitMint(client, account);
    const explorerUrl = `https://explorer.devnet.aptos.dev/txn/${txn.version}?network=nft-nyc`

    // Update the explorer_url.
    this.explorerUrlTarget.value = explorerUrl;
    this.formTarget.requestSubmit();
  }

  async fundAccount(account: AptosAccount, faucet: FaucetClient) {
    await faucet.fundAccount(account.address(), 10_000);
    return account;
  }

  async createSubmitMint(client: AptosClient, account: AptosAccount) {
    const payload = { //{ function: string; arguments: string[]; type: string; type_arguments: any[] }
      type: "script_function_payload",
      function: `${ADDRESS}::${MODULE}::mint`,
      type_arguments: [],
      arguments: []
    };
    return await signAndSubmit(client, payload, account);
  }
}
