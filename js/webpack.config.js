const path = require("path");
const webpack = require("webpack");

module.exports = {
  entry: "./src/index.ts",
  // devtool: "inline-source-map",
  module: {
    rules: [{
      test: /\.ts$/,
      use: "ts-loader",
      exclude: /node_modules/
    }]
  },
  resolve: {
    modules: [process.env.NODE_PATH || "node_modules"],
    extensions: [".ts", ".js"]
  },
  output: {
    filename: "loadavg.js",
    path: path.resolve(__dirname, "dist")
  },
  plugins: [
    new webpack.BannerPlugin({
      banner: "#!/usr/bin/env node",
      raw: true,
    })
  ],
  target: "node"
};
