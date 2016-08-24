var
path = require('path'),
webpack = require('webpack');

var is_production = process.env.NODE_ENV === 'production';

var appRoot = __dirname;

// TODO: delete this
var vendor = [

    // babel
    // TODO: deferred to cdn
    // 'babel-polyfill',

    // TODO: generators are not used
    // 'babel-runtime/regenerator',

    // TODO: fix
    // 'core-js',
    // 'babel-runtime',


    // 'babel-runtime/core-js',

    // utils
    'classnames',
    // 'bluebird',
    // 'little-loader',


    // TODO: too big; import individual functions instead
    // 'lodash',
    'lodash/isFunction',
    'lodash/get',
    'lodash/set',
    'lodash/merge',

    // TODO: deferred to cdn via webpack externals
    // react
    'react',
    'react-dom',



    // 3rd-party react components
    'react-textarea-autosize',



    // redux
    'redux',
    'react-redux',
    'redux-form',

];

module.exports = {
    colors: true,
    watch: true,
    entry: {
        deck_review: "./src/deck_review.js",
        new_deck: "./src/new_deck.js",
        deck_description: "./src/deck_description.js",
        // TODO: remove
        // vendor: vendor
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

        // TODO: avoid chunk loading to decrease loading cost
        // new webpack.optimize.CommonsChunkPlugin(/* chunkName= */"vendor", /* filename= */"vendor.js")
    ],


    externals: {
        // TODO: cdn only for saas
        // served via cdnjs.com
        // react: 'React',
        // 'react-dom': 'ReactDOM'
    }
};
