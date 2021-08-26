# runtime::put_key vs set_key

## TestNet

```
casper-client put-deploy --chain-name casper-test --node-address http://16.162.124.124:7777 \
--secret-key /home/jiuhong/keys/test29/secret_key.pem \
--session-path /home/jiuhong/rust/erc20/my-project/contract/target/wasm32-unknown-unknown/release/contract.wasm \
--payment-amount 4000000000
```

## NCTL

```
casper-client put-deploy \
--chain-name casper-net-1 \
--node-address http://0.0.0.0:11101 \
--secret-key /home/jiuhong/nctl/casper-node/utils/nctl/assets/net-1/nodes/node-1/keys/secret_key.pem \
--session-path /home/jiuhong/rust/erc20/my-project/contract/target/wasm32-unknown-unknown/release/contract.wasm \
--payment-amount 4000000000
```
