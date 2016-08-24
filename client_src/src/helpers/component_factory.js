const merge = require('lodash/merge');

const {makeReducer} = require('lib/redux-tree');

const IDENTITY = function(x) {
    return x;
};

const componentCreator = function(initialState, getComponent, fallbackReducer = IDENTITY) {

    return function(preRenderState) {

        if(preRenderState) {
            preRenderState = merge({}, initialState, preRenderState);
        } else {
            preRenderState = initialState;
        }

        const rehydrateFactory = require('helpers/hydrate');
        const {createStore, applyMiddleware} = require('redux');

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
        }), preRenderState, middleware());

        const component = getComponent(store);

        return {
            store,
            component
        };

    };
};

module.exports = componentCreator;
