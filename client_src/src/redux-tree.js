// TODO: doc all the things... and cleanly.


// TODO: specific lodash dependencies
const _ = require('lodash');
const isFunction = _.isFunction;
// TODO: npm install npm.im/warning

// sentinel value
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

const treeReducer = (state, action) => {

    if (process.env.NODE_ENV !== 'production') {
        if(!action.meta) {
            // TODO: improve error
            throw Error('no meta in action');
        }
    }

    const {reducer, path, getIn, setIn} = action.meta;

    if (process.env.NODE_ENV !== 'production') {

        if(!reducer) {
            // TODO: improve error
            throw Error('no reducer');
        }

        if(!path) {
            // TODO: improve error
            throw Error('no path');
        }
    }

    const oldValue = getIn(state, path);
    const newValue = reducer(oldValue, action);
    const newRoot = setIn(state, path, newValue);

    return newRoot;
}

// reducer factory
const makeReducer = ({reducer: fallbackReducer = NOT_SET} = {}) => {

    let afterInit = false;

    // const fallbackReducer = reducer;
    const fallbackReducerIsFunction = isFunction(fallbackReducer);

    return (state = NOT_SET, action) => {

        if (process.env.NODE_ENV !== 'production') {
            if(state === NOT_SET) {
                // TODO: improve error
                throw Error('no initial state given');
            }
        }

        if(!afterInit) {
            afterInit = true;
            return state;
        }

        if(action.meta) {
            const {reducer = NOT_SET} = action.meta;

            if(reducer === NOT_SET && fallbackReducerIsFunction) {
                return fallbackReducer(state, action);
            }
        }

        return treeReducer(state, action);

    };

};


// redux-tree action creator/transformer that adds the following to the given action
//
// action = {
//     // ...
//     meta: {
//         __redux_tree: {
//             path: path,
//             reducer: reducer,
//             getIn: getIn,
//             setIn: setIn
//         }
//     }
// };
//
// NOTE: given action must be flux standard action (FSA) compliant.
//
// The returned action is flux standard action (FSA) compliant.
//
const reduceIn = (reducer, path, action, getIn = __getIn, setIn = __setIn, shouldPollute = false) => {

    const patch = {
        meta: {
            path: path, // array
            reducer: reducer, // redux compatible reducer
            getIn: getIn,
            setIn: setIn
        }
    };

    if (shouldPollute) {
        return _.merge(action, patch);
    } else {
        return _.merge({}, action, patch);
    }

};

// reduceIn factory to cache given getIn, setIn, shouldPollute = false
//
// Be able to customize getIn and setIn.
//
// type getIn = (rootData, path) => valueAtPath
// type setIn = (rootData, path, value) => rootData
//
// Useful for Immutable.js objects
const reduceInWith = (getIn = __getIn, setIn = __setIn, shouldPollute = false) => {
    return (reducer, path, action) => {
        return reduceIn(reducer, path, action, getIn, setIn, shouldPollute);
    };
}

module.exports = {

    // export reducer
    makeReducer,

    // reducer application
    reduceIn,
    reduceInWith
};
