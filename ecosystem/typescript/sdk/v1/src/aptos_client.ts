import * as Gen from "./generated/index";
import { Helpers } from "./helpers";

/**
 * Provides methods for retrieving data from Aptos node.
 * For more detailed API specification see {@link https://fullnode.devnet.aptoslabs.com/v1/spec}
 *
 * This client also exposes helper methods under `helpers`. These help with
 * common operations that aren't captured by a single API call, such as
 * waiting for a transaction to be processed, generating a transaction, etc.
 */
export class AptosClient {
  // The client generated from the OpenAPI spec.
  private client: Gen.AptosGeneratedClient;

  public helpers: Helpers;

  // Re-expose the services within the client at this top level.
  public get accounts() {
    return this.client.accounts;
  }

  public get events() {
    return this.client.events;
  }

  public get general() {
    return this.client.general;
  }

  public get tables() {
    return this.client.tables;
  }

  public get transactions() {
    return this.client.transactions;
  }

  /**
   * Build a client configured to conntext to an Aptos node at the given endpoint.
   * @param nodeUrl A url of the Aptos Node API endpoint
   * @param config Additional configuration options for the generated client.
   */
  constructor(nodeUrl: string, config?: Partial<Gen.OpenAPIConfig>) {
    if (config === undefined || config === null) {
      config = {};
    }
    config.BASE = nodeUrl;
    this.client = new Gen.AptosGeneratedClient(config);
    this.helpers = new Helpers(this.client);
  }
}
