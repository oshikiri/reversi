const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");
const webpack = require("webpack");

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "./dist/"),
    filename: "bootstrap.js",
  },
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        { from: "public/index.html", to: "." },
        { from: "public/favicon.ico", to: "." },
        { from: "public/style.css", to: "." },
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
