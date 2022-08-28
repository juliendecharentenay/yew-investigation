const { defineConfig } = require('@vue/cli-service')
module.exports = defineConfig({
  publicPath: '/yew_investigation/vuejs',
  devServer: {
    port: 8081,
  },
  transpileDependencies: true,
  productionSourceMap: false
})
