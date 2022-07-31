#!/bin/sh

# This script applies some simple changes to the generated client, to make it
# use stronger typing, e.g. making endpoints accept HexString instead of string.

sed -i.bak 's@type \{ Address \} from "../models/Address"@type \{ HexString \} from "../../hex_string"@g' src/generated/services/*.ts
sed -i.bak 's@address: Address@address: HexString@g' src/generated/services/*.ts
