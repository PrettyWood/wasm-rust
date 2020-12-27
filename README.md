### 🛠️ Build

```
# Create the `pkg` directory
# It will contain the `.wasm` compiled from the Rust code (in `src`) and the JS glue to make it work
wasm-pack build

# In `site` directory
# Install JS dependencies
npm install
```

### 🚀 Run

```
# in `site` directory
npm run serve
```