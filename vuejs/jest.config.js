module.exports = {
  preset: '@vue/cli-plugin-unit-jest',
  collectCoverage: true,
  collectCoverageFrom: [
    "**/src/**/*.{js,vue}",
    "!**/pkg/**",
    "!**/node_modules/**",
    "!**/*.spec.js",
  ],
  coverageProvider: "v8",
  coverageReporters: [ "text", "html" ], // Refer to https://github.com/istanbuljs/istanbuljs/tree/master/packages/istanbul-reports/lib for options
  testMatch: [
    "**/src/**/**.spec.js",
  ]
}
