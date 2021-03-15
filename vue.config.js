module.exports = {
  lintOnSave: false,
  chainWebpack: config => {
    config.entry('app').clear().add('./src/async-shim.ts').end();
    return config;
  }
};
