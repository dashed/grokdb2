var
path = require('path'),
webpack = require('webpack');

var appRoot = __dirname;

module.exports = {
    colors: true,
    watch: true,
    entry: {
        deck_review: "./src/deck_review.js",
        vendor: [
            // 'babel-polyfill',
            // 'babel-runtime/regenerator',

            'babel-runtime/core-js',
            'react',
            'react-dom',
            'classnames',

            // 'little-loader',

            'redux',
            'react-redux',
            'lodash',
            'react-textarea-autosize'
        ]
    },

    output: {
        // path: path.join(appRoot, "./dist/"),
        path: path.join(appRoot, "../assets/"),
        filename: "[name].js"
    },
    devtool: "#source-map",

    resolve: {
        root: path.join(appRoot, "/src/"),
        modulesDirectories: ["node_modules"]
    },

    module: {
        loaders: [
            {
                test: /\.jsx?$/,
                exclude: /node_modules/,
                loader: 'babel-loader',
                query: {
                    cacheDirectory: true,
                    presets: ['es2015', 'react', 'stage-2'],
                    plugins: ['transform-runtime']
                }
            },
            {
                test: /\.jsx?$/,
                loader: "eslint-loader",
                exclude: /node_modules/
            }
        ]
    },

    plugins: [
        new webpack.optimize.CommonsChunkPlugin(/* chunkName= */"vendor", /* filename= */"vendor.js")
    ]
};
