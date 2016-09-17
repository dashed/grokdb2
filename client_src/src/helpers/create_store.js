const {makeReducer} = require('lib/redux-tree');

const {createStore, applyMiddleware} = require('redux');

const rehydrateFactory = require('helpers/hydrate');

const IDENTITY = function(x) {
    return x;
};

module.exports = function(initialState = void 0, fallbackReducer = IDENTITY) {

    if(process.env.NODE_ENV !== 'production') {
        if(initialState === void 0 ) {
            throw Error('expected initialState');
        }
    }

    // TODO: refactor to module
    const middleware = () => {

        if(process.env.NODE_ENV !== 'production') {

            const createLogger = require('redux-logger');
            const logger = createLogger();

            return applyMiddleware(logger);
        }

        return applyMiddleware();
    };

    const store = createStore(makeReducer({
        reducer: rehydrateFactory(fallbackReducer)
    }), initialState, middleware());

    return store;
};
