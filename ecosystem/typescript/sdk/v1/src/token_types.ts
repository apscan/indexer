export interface TokenData {
  /** Unique name within this creator's account for this Token's collection */
  collection: string;

  /** Description of Token */
  description: string;

  /** Name of Token */
  name: string;

  /** Optional maximum number of this Token */
  maximum?: number;

  /** Total number of this type of Token */
  supply: number;

  /** URL for additional information / media */
  uri: string;
}

export interface TokenId {
  /** Token creator address */
  creator: string;

  /** Unique name within this creator's account for this Token's collection */
  collection: string;

  /** Name of Token */
  name: string;
}

export interface Token {
  id: TokenId;
  value: number;
}
