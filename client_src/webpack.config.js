// NOTE:
// - https://github.com/webpack/docs/wiki/optimization#multi-page-app

var
path = require('path'),
webpack = require('webpack');
var CommonsChunkPlugin = require("webpack/lib/optimize/CommonsChunkPlugin");
var StatsPlugin = require('stats-webpack-plugin');

// TODO: fix
var is_production = process.env.NODE_ENV === 'production';

var appRoot = __dirname;

module.exports = {
    colors: true,
    watch: true,
    entry: {
        mathjax_inline: "./src/mathjax_inline.js",
        new_deck: "./src/new_deck.js",
        new_card: "./src/new_card.js",
        deck_description: "./src/deck_description.js",
        deck_settings_main: "./src/deck_settings_main.js",
        deck_card_profile: "./src/deck_card_profile.js",
        deck_card_settings: "./src/deck_card_settings.js",
        deck_review: "./src/deck_review.js",
        card_review: "./src/card_review.js",
        card_move_settings: "./src/card_move_settings.js",
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
                // needed for markdown-it
                test: /\.json$/,
                loader: 'json-loader'
            },
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
        new webpack.DefinePlugin({
          'process.env':{
            'NODE_ENV': JSON.stringify(is_production ? 'production' : 'development')
          }
        }),

        new CommonsChunkPlugin("commons.js"),
        new webpack.optimize.DedupePlugin(),
        new StatsPlugin('stats.json', {
          chunkModules: true,
          // exclude: [/node_modules[\\\/]react/]
        })
    ],


    externals: {
        // TODO: cdn only for saas
        // served via cdnjs.com
        // react: 'React',
        // 'react-dom': 'ReactDOM'
    }
};
