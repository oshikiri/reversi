const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");
const webpack = require("webpack");

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "../docs/"),
    filename: "bootstrap.js",
  },
  devtool: "source-map",
  resolve: {
    extensions: ["", ".webpack.js", ".web.js", ".ts", ".tsx", ".js"],
  },
  module: {
    rules: [
      { test: /\.tsx?$/, loader: "ts-loader" },
      { test: /\.js$/, loader: "source-map-loader" },
    ],
  },
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        { from: "index.html", to: "." },
        { from: "favicon.ico", to: "." },
        { from: "style.css", to: "." },
      ],
    }),
    new webpack.EnvironmentPlugin({
      REVERSI_VERSION: "",
    }),
  ],
  experiments: {
    asyncWebAssembly: true,
  },
};
