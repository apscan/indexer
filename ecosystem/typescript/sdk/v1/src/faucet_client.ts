/** Faucet creates and funds accounts. This is a thin wrapper around that. */
import axios from "axios";
import { AptosClient } from "./aptos_client";
import { OpenAPIConfig } from "./generated";
import { HexString, MaybeHexString } from "./hex_string";

/**
 * Class for requsting tokens from faucet
 */
export class FaucetClient extends AptosClient {
  faucetUrl: string;

  /**
   * Establishes a connection to Aptos node
   * @param nodeUrl A url of the Aptos Node API endpoint
   * @param faucetUrl A faucet url
   * @param config An optional config for inner axios instance
   * Detailed config description: {@link https://github.com/axios/axios#request-config}
   */
  constructor(nodeUrl: string, faucetUrl: string, config?: OpenAPIConfig) {
    super(nodeUrl, config);
    this.faucetUrl = faucetUrl;
  }

  /**
   * This creates an account if it does not exist and mints the specified amount of
   * coins into that account
   * @param address Hex-encoded 16 bytes Aptos account address wich mints tokens
   * @param amount Amount of tokens to mint
   * @returns Hashes of submitted transactions
   */
  async fundAccount(address: MaybeHexString, amount: number): Promise<string[]> {
    const url = `${this.faucetUrl}/mint?amount=${amount}&address=${HexString.ensure(address).noPrefix()}`;
    const response = await axios.post<Array<string>>(url, {}, { validateStatus: () => true });

    const tnxHashes = response.data;
    const promises: Promise<void>[] = [];
    for (let i = 0; i < tnxHashes.length; i += 1) {
      const tnxHash = tnxHashes[i];
      promises.push(this.helpers.waitForTransaction(tnxHash));
    }
    await Promise.all(promises);
    return tnxHashes;
  }
}
