# build wasm
./build.sh

#Deploy with near-dev
near dev-deploy --wasmFile ./res/callback_results.wasm

source neardev/dev-account.env
echo $CONTRACT_NAME

source .env
echo $username

near call $CONTRACT_NAME call_all2 '{ "fail_b" : true, "c_value": 1 }' --accountId $username --gas 300000000000000