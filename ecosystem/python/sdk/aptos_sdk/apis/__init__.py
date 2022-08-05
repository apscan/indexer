# coding: utf-8

# flake8: noqa

# Import all APIs into this package.
# If you have many APIs here with many many models used in each API this may
# raise a `RecursionError`.
# In order to avoid this, import only the API that you directly need like:
#
#   from aptos_sdk.api.accounts_api import AccountsApi
#
# or import this package, but before doing it, use:
#
#   import sys
#   sys.setrecursionlimit(n)

# Import APIs into API package:
from aptos_sdk.api.accounts_api import AccountsApi
from aptos_sdk.api.events_api import EventsApi
from aptos_sdk.api.general_api import GeneralApi
from aptos_sdk.api.tables_api import TablesApi
from aptos_sdk.api.transactions_api import TransactionsApi
