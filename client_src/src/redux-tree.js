// TODO: doc all the things... and cleanly.


// TODO: specific lodash dependencies
const _ = require('lodash');
const isFunction = _.isFunction;
// TODO: npm install npm.im/warning

// sentinel values
const APPLY_REDUCER = ['__REDUX_TREE__APPLY_REDUCER'];
const NOT_SET = {};


const __getIn = (rootData, path) => {
    // TODO: check path is array
    return _.get(rootData, path)
};

const __setIn = (rootData, path, newValue) => {
    // TODO: check path is array
    const patch = _.set({}, path, newValue);
    // NOTE: the following will not work: {...state, ...patch};
    return _.merge({}, rootData, patch);
};

const treeReducer = (state = NOT_SET, action) => {

    if(state === NOT_SET) {
        throw Error('No initial state given.');
    }

    if(!action.type) {
        return state;
    }

    switch(action.type) {

    case APPLY_REDUCER:

        const {path, reducer, getIn, setIn} = action.payload;

        if (process.env.NODE_ENV !== 'production') {
            // TODO: check reducer is function
            if(!isFunction(reducer)) {
                throw Error(`Given reducer is not a function: ${reducer}`);
            }
        }

        const oldValue = getIn(state, path);
        const newValue = reducer(oldValue, action.payload.action);
        const newRoot = setIn(state, path, newValue);

        return newRoot;

    default:

        return state;
    }

    // unreachable!();

}

// Higher-order reducer that wraps a given reducer.
// The reducer function is the fallback reducer when actions not created with
// applyReducer/applyReducerWith are dispatched to the redux store.
const wrapReducer = (reducer) => {

    // cache
    const reducerIsFunction = _.isFunction(reducer);

    return (state, action) => {

        if(action.type && action.type === APPLY_REDUCER) {
            return treeReducer(state, action);
        }

        if(reducerIsFunction) {
            return reducer(state, action);
        }

        return state;
    }
};

// Action creator that works on global states that are vanilla JS objects.
//
// Applies reducer function at path with given action.
//
// NOTES: The reducer function should know how to consume given action.
const applyReducer = (reducer, path, action) => {
    return {
        type: APPLY_REDUCER,
        payload: {
            path: path, // array
            reducer: reducer, // redux compatible reducer
            action: action, // action to be applied to given reducer
            getIn: __getIn,
            setIn: __setIn
        }
    }
}

// Action creator that works on global states of any type.
//
// customize getIn and setIn.
//
// type getIn = (rootData, path) => valueAtPath
// type setIn = (rootData, path, value) => rootData
//
// Useful for Immutable.js objects
const applyReducerWith = (getIn, setIn) => {
    return (path, reducer, action) => {
        return {
            type: APPLY_REDUCER,
            payload: {
                path: path, // array
                reducer: reducer, // redux compatible reducer
                action: action, // action to be applied to given reducer
                getIn: getIn,
                setIn: setIn
            }
        }
    }
}

module.exports = {

    // NOTE: action type is not exported. Consumers should use applyReducer or
    // applyReducerWith

    // export reducer
    treeReducer,
    reducer: treeReducer,
    wrapReducer,

    // reducer application
    applyReducer,
    applyReducerWith
};
