var
path = require('path'),
webpack = require('webpack');

var is_production = process.env.NODE_ENV === 'production';

var appRoot = __dirname;

module.exports = {
    colors: true,
    watch: true,
    entry: {
        deck_review: "./src/deck_review.js",
        new_deck: "./src/new_deck.js",
        vendor: [

            // babel
            'babel-polyfill',
            // 'babel-runtime/regenerator',

            // TODO: fix
            // 'core-js',
            // 'babel-runtime',


            // 'babel-runtime/core-js',

            // utils
            'classnames',
            // 'bluebird',
            // 'little-loader',
            'lodash',

            // react
            'react',
            'react-dom',



            // 3rd-party react components
            'react-textarea-autosize',



            // redux
            'redux',
            'react-redux',
            'redux-form',

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
