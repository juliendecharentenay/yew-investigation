const path = require("path");
const { defineConfig } = require('@vue/cli-service')
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = defineConfig({
  chainWebpack: (config) => {
    // rust wasm bindgen https://github.com/rustwasm/wasm-bindgen
    config
      .plugin("wasm-pack")
      .use(WasmPackPlugin)
      .init(
        (Plugin) =>
          new Plugin({
            crateDirectory: path.resolve(__dirname, "../vuejs_wasm_rust"),
            outDir: path.resolve(__dirname, "./src/pkg"),
            // forceMode: "development",
            forceMode: "production",
          })
      )
      .end()
  },
  publicPath: 'yew_investigation/vuejs_wasm',
  devServer: {
    port: 8082,
  },
  configureWebpack: {
    experiments: {
      asyncWebAssembly: true,
    },
  },
  transpileDependencies: true,
  productionSourceMap: false
})
