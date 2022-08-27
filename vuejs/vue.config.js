const { defineConfig } = require('@vue/cli-service')
module.exports = defineConfig({
  publicPath: '/vuejs',
  devServer: {
    port: 8081,
  },
  transpileDependencies: true
})
