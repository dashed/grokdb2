// TODO: dependencies
const _ = require('lodash');

const APPLY_REDUCER = '__REDUX_TREE__APPLY_REDUCER';
const EMPTY_OBJ = {};

const treeReducer = function(state = EMPTY_OBJ, action) {

    let patch = EMPTY_OBJ;

    switch(action.type) {

    case APPLY_REDUCER:

        const {path, reducer} = action.payload;

        // TODO: path is array
        // TODO: check reducer is function

        const specificState = _.get(state, path);

        // TODO: _.has(state, path);

        const result = reducer(specificState, action.payload.action);
        patch = _.set({}, path, result);

        // NOTE: the following will not work
        // return {...state, ...patch};

        return _.merge({}, state, patch);

    default:
        return state;
    }

    // unreachable!();

}

const applyReducer = function(path, reducer, action) {
    return {
        type: APPLY_REDUCER,
        payload: {
            path: path, // array
            reducer: reducer, // redux compatible reducer
            action: action // action to be applied to given reducer
        }
    }
}

module.exports = {

    // exported action
    APPLY_REDUCER: APPLY_REDUCER,
    action: APPLY_REDUCER,

    // export reducer
    treeReducer,
    reducer: treeReducer,

    applyReducer
};
