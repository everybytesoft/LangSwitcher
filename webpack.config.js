const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");

const getPath = (dir) => {
  return path.resolve(__dirname, dir || ".");
};

module.exports = {
  entry: getPath("src/index.jsx"),

  output: {
    path: getPath("dist"),
    filename: "index.js",
    assetModuleFilename: "assets/[hash][ext]",
    clean: true,
  },

  module: {
    rules: [
      {
        test: /\.(jsx)$/,
        exclude: /node_modules/,
        use: {
          loader: "babel-loader",
        },
      },
      {
        test: /\.(scss)$/,
        use: [MiniCssExtractPlugin.loader, "css-loader", "sass-loader"],
      },
      {
        test: /\.(svg)$/,
        type: "asset/resource",
      },
    ],
  },

  plugins: [
    new MiniCssExtractPlugin({
      filename: "index.css",
    }),
    new CopyWebpackPlugin({
      patterns: [{ from: "public", to: "." }],
    }),
  ],

  devServer: {
    watchFiles: getPath("src"),
    port: 3000,
  },
};
