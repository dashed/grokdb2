const merge = require('lodash/merge');

const {reducer: reduxformReducer} = require('redux-form');

const formReducer = (state, action) => {

    // NOTE: We're not using combineReducers from redux as redux-form expects.
    //       Defer any un-captured action to redux-form.

    const newForm = reduxformReducer(state.form, action);
    const newState = merge({}, state);
    newState.form = newForm;

    return newState;
};

module.exports = formReducer;
