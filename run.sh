# build wasm
./build.sh

#Deploy with near-dev
near dev-deploy --wasmFile ./res/unit_testing.wasm

source neardev/dev-account.env
echo $CONTRACT_NAME

username=''
echo $username
 
#### Initialize contract
near call $CONTRACT_NAME new '{"owner_id":"'$username'", "placeholder": 0}' --accountId $username

#### Call say_hello
near call $CONTRACT_NAME say_hello --accountId $username