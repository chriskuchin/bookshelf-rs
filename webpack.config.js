const { VueLoaderPlugin } = require('vue-loader');
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const HtmlWebpackPlugin = require('html-webpack-plugin');
const path = require('path');
const { DefinePlugin } = require('webpack');

module.exports = (env, argv) => {
  var mode = "production"
  if (argv.mode) {
    mode = "development"
  }

  var config = {
    entry: {
      results: {
        import: './public/bookshelf.js',
      },
    },
    mode: mode,
    output: {
      filename: '[name].[contenthash].js',
      path: path.resolve(__dirname, './dist'),
      clean: true,
    },
    module: {
      rules: [
        {
          test: /\.(jpg|png)$/,
          use: {
            loader: 'url-loader',
          },
        },
        {
          test: /\.vue$/,
          loader: 'vue-loader'
        },
        {
          test: /\.css$/,
          use: [
            'vue-style-loader',
            'css-loader'
          ]
        },
        {
          test: /\.s[ac]ss$/i,
          use: [
            MiniCssExtractPlugin.loader,
            "css-loader",
            "sass-loader",
          ],
        },
      ]
    },
    plugins: [
      new DefinePlugin({
        __VUE_OPTIONS_API__: true,
        __VUE_PROD_DEVTOOLS__: false,
      }),
      new VueLoaderPlugin(),
      new HtmlWebpackPlugin({
        title: "bookshelf",
        filename: "index.html",
        template: 'public/index.ejs',
        // favicon: 'public/assets/images/favicon.ico',
        meta: {
          viewport: "initial-scale=1, maximum-scale=1",
        },
        templateParameters: {
          mode: mode,
        }
      }),
      new MiniCssExtractPlugin({
        filename: "[name].[contenthash].css"
      }),
    ],
  }

  return config
};