const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

const path = require("path");

module.exports = {
    entry: "./src/index.tsx",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "bundle.[hash].js",
    },
    devServer: {
        compress: true,
        port: 8080,
        hot: true,
        static: "./dist",
        historyApiFallback: true,
        open: true,
    },
    module: {
        rules: [
            {
                test: /\.(js|tsx)$/,
                exclude: /node_modules/,
                use: {
                    loader: "babel-loader",
                },
            },
            {
                test: /\.s[ca]ss$/i,
                use: [MiniCssExtractPlugin.loader, {
                    loader: "css-loader",
                    options: {
                        import: false,
                    },
                }, "sass-loader"],
            },
        ],
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: __dirname + "/public/index.html",
            filename: "index.html",
        }),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, ".")
        }),
        new MiniCssExtractPlugin({
            filename: "[name].css",
        }),
    ],
    mode: "development",
    devtool: "inline-source-map",
    experiments: {
        asyncWebAssembly: true
    }
};