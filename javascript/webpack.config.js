const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");
const webpack = require("webpack");

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "../docs/"),
    filename: "bootstrap.js",
  },
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        { from: "index.html", to: "." },
        { from: "favicon.ico", to: "." },
      ],
    }),
    new webpack.EnvironmentPlugin({
      REVERSI_VERSION: "",
    }),
  ],
};
