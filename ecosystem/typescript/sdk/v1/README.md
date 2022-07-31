# Aptos SDK for v1 API
[![Discord][discord-image]][discord-url]
[![NPM Package Version][npm-image-version]][npm-url]
[![NPM Package Downloads][npm-image-downloads]][npm-url]

## API Docs
Docs can be found [here][api-doc]

## Usage
For Javascript or Typescript usage, check out the [`./examples`][examples] folder with ready-made `package.json` files to get you going quickly!

If you are using the types in a `commonjs` module, like in a Node app, you just have to enable `esModuleInterop`
and `allowSyntheticDefaultImports` in your `tsconfig` for types compatibility:

```json
{
  ...
  "compilerOptions": {
    "allowSyntheticDefaultImports": true,
    "esModuleInterop": true
    ...
  }
}
```

### Requirements
- [Node.js](https://nodejs.org)
- [Yarn](https://yarnpkg.com/)

```bash
yarn install
```

### Generating API client
This SDK is composed of two parts, a core client generated from the OpenAPI spec of the API, and a set of wrappers that make it nicer to use, enable certain content types, etc.

To generate the core client from the spec, run:
```bash
yarn generate-client
```

### Testing against devnet
```bash
yarn test
```

### Testing against local node

Run a local node (run from the root of the repo):
```
cargo run -p aptos -- node run-local-testnet --with-faucet --faucet-port 8081 --force-restart --assume-yes
```

Setup an env to configure the URLs:
```
rm .env
echo 'APTOS_NODE_URL="http://127.0.0.1:8080/v1"' >> .env
echo 'APTOS_FAUCET_URL="http://127.0.01:8081"' >> .env
```

Run the tests:
```
yarn test
```

## Semantic versioning
This project follows [semver](https://semver.org/) as closely as possible.

## References
[examples]: https://github.com/aptos-labs/aptos-core/blob/main/ecosystem/typescript/sdk/examples/
[repo]: https://github.com/aptos-labs/aptos-core
[npm-image-version]: https://img.shields.io/npm/v/aptos.svg
[npm-image-downloads]: https://img.shields.io/npm/dm/aptos.svg
[npm-url]: https://npmjs.org/package/aptos
[discord-image]: https://img.shields.io/discord/945856774056083548?label=Discord&logo=discord&style=flat~~~~
[discord-url]: https://discord.gg/aptoslabs
[api-doc]: https://aptos-labs.github.io/ts-sdk-doc/

## TODO:
TODO: Convert these into issues
- Update the examples to use AptosCoin.
- Confirm that the tests for `yarn test` are being run against a node built from this commit in CI.
