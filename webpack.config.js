const { VueLoaderPlugin } = require("vue-loader");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const path = require("path");
const { DefinePlugin, ProvidePlugin } = require("webpack");
const { GenerateSW } = require("workbox-webpack-plugin");

module.exports = (env, argv) => {
	let mode = "production";
	if (argv.mode) {
		mode = "development";
	}

	const config = {
		entry: {
			results: {
				import: "./public/bookshelf.js",
			},
		},
		mode: mode,
		output: {
			filename: "[name].[contenthash].js",
			path: path.resolve(__dirname, "./dist"),
			clean: true,
			publicPath: "/",
		},
		module: {
			rules: [
				{
					test: /\.json/,
					type: "asset/resource",
					generator: {
						filename: "[name][ext]",
					},
				},
				{
					test: /\.(jpg|png)$/,
					type: "asset/resource",
					generator: {
						filename: "images/[name][ext]",
					},
				},
				{
					test: (value) => value.includes("icon-"),
					type: "asset/resource",
					generator: {
						filename: "images/icons/[name][ext]",
					},
				},
				{
					test: /\.vue$/,
					loader: "vue-loader",
				},
				{
					test: /\.css$/,
					use: ["vue-style-loader", "css-loader"],
				},
				{
					test: /\.s[ac]ss$/i,
					use: [MiniCssExtractPlugin.loader, "css-loader", "sass-loader"],
				},
			],
		},
		plugins: [
			new ProvidePlugin({
				JSZip: "jszip",
			}),
			new GenerateSW({
				skipWaiting: true,
				clientsClaim: true,
				runtimeCaching: [
					{
						urlPattern: new RegExp("/api/.*"),
						method: "POST",
						handler: "NetworkOnly",
						options: {
							backgroundSync: {
								name: "api-retry",
								options: {
									maxRetentionTime: 24 * 60,
								},
							},
							// plugins: [statusPlugin]
						},
					},
					{
						urlPattern: new RegExp("/api/.*"),
						method: "GET",
						handler: "NetworkFirst",
					},
				],
			}),
			new DefinePlugin({
				__VUE_OPTIONS_API__: true,
				__VUE_PROD_DEVTOOLS__: false,
			}),
			new VueLoaderPlugin(),
			new HtmlWebpackPlugin({
				title: "bookshelf",
				filename: "index.html",
				template: "public/index.ejs",
				// favicon: 'public/assets/images/favicon.ico',
				meta: {
					viewport: "initial-scale=1, maximum-scale=1",
				},
				templateParameters: {
					mode: mode,
				},
			}),
			new MiniCssExtractPlugin({
				filename: "[name].[contenthash].css",
			}),
		],
	};

	return config;
};
