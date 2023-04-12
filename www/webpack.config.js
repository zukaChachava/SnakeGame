const path = require("path");
const CopyWebpackPlugin = require('copy-webpack-plugin')

module.exports = {
    entry: "./index.ts",
    output: {
        path: path.resolve(__dirname, "public"),
        filename: 'index.js'
    },
    mode: 'development',
    module: {
        rules: [
          {
            test: /\.tsx?$/,
            use: 'ts-loader',
            exclude: /node_modules/,
          },
        ],
    },
    resolve: {
    extensions: ['.tsx', '.ts', '.js'],
    },
    plugins: [
        new CopyWebpackPlugin({
            patterns: [
                {from: './index.html', to: './'}
            ]
        })
    ]
}