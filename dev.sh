# Compile Rust code to WebAssembly
cd insilico-social-core

export RUSTFLAGS='--cfg getrandom_backend="wasm_js"'

wasm-pack build --target bundler

# Check if the build was successful
if [ $? -ne 0 ]; then
    echo "wasm-pack build failed"
    exit 1
fi

# Go back to the insilico-social-web directory
cd ../insilico-social-web

# Install dependencies
npm install

npm install --save ../insilico-social-core/pkg

# Run the project
npm run dev