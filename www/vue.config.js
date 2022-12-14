const { defineConfig } = require('@vue/cli-service')
module.exports = defineConfig({
  publicPath: '/yew_investigation',
  devServer: {
    proxy: {
      '^/vuejs/':      { target: 'http://locahost:8081/' },
      '^/vuejs_wasm/': { target: 'http://locahost:8082/' },
      '^/yew/':        { target: 'http://locahost:8083/' },
    }
  },
  transpileDependencies: true,
  productionSourceMap: false
})
