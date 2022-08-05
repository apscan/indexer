# Copyright (c) Aptos
# SPDX-License-Identifier: Apache-2.0

import time
from typing import Any, Dict, List, Optional

import httpx

from .account import Account
from .account_address import AccountAddress



TESTNET_URL = "https://fullnode.devnet.aptoslabs.com"
FAUCET_URL = "https://faucet.devnet.aptoslabs.com"

U64_MAX = 18446744073709551615


class FaucetClient:
    """Faucet creates and funds accounts. This is a thin wrapper around that."""

    base_url: str
    rest_client: RestClient

    def __init__(self, base_url: str, rest_client: RestClient):
        self.base_url = base_url
        self.rest_client = rest_client

    def close(self):
        self.rest_client.close()

    def fund_account(self, address: str, amount: int):
        """This creates an account if it does not exist and mints the specified amount of
        coins into that account."""
        txns = self.rest_client.client.post(
            f"{self.base_url}/mint?amount={amount}&address={address}"
        )
        assert txns.status_code == 200, txns.text
        for txn_hash in txns.json():
            self.rest_client.wait_for_transaction(txn_hash)
