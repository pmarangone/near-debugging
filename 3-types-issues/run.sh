
#!/bin/bash
set -e

if [ -d "res" ]; then
  echo ""
else
  mkdir res
fi

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release

cp target/wasm32-unknown-unknown/release/types_issues.wasm res/

# #Deploy with near-dev
near dev-deploy --wasmFile res/types_issues.wasm
