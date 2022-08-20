
source neardev/dev-account.env
echo $CONTRACT_NAME

near view $CONTRACT_NAME pprint_u128 '{}'
near view $CONTRACT_NAME pprint_str '{}'

near view $CONTRACT_NAME default_u128 '{}'
near view $CONTRACT_NAME default_str '{}'
