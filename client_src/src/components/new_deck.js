const React = require('react');
const {Provider, connect} = require('react-redux');
const {createStore} = require('redux');
const classnames = require('classnames');
const TextareaAutosize = require('react-textarea-autosize').default;

const {

    MARKDOWN_VIEW,
    MARKDOWN_VIEW_RENDER,
    MARKDOWN_VIEW_SOURCE,

} = require('global/constants');

const {reduceIn, makeReducer} = require('lib/redux-tree');

/* react components */

const RenderSourceComponent = connect(

    // mapStateToProps
    (state) => {
        return{
            [MARKDOWN_VIEW]: state[MARKDOWN_VIEW],
            switchTab: (dispatch, markdownView) => switchMarkdownView(dispatch, currentCardTab, markdownView)
        };
    },

    // mapDispatchToProps
    // (dispatch) => {
    //     return {
    //         switchTab:
    //     };
    // }
)(require('components/dumb/render_source'));

const NewDeckContainer = function(/* props */) {
    return (
        <div>
            {'lol'}
        </div>
    );
}


/* redux action dispatchers */
// NOTE: FSA compliant

const switchMarkdownView = function(dispatch, target, markdownView) {
    // target: Tab such as question, question, or description
    return function(event) {
        event.preventDefault();
        dispatch(
            reduceIn(
                // reducer
                markdownViewReducer,
                // path
                [target, MARKDOWN_VIEW],
                // action
                {
                    type: markdownView
                }
            )
        );
    }
}


/* redux reducers */

const markdownViewReducer = function(state = MARKDOWN_VIEW_RENDER, action) {

    switch(action.type) {
    case MARKDOWN_VIEW_RENDER:
    case MARKDOWN_VIEW_SOURCE:
        state = action.type;
        break;

    default:
        state = MARKDOWN_VIEW_RENDER;
    }

    return state;
}

/* default state */

const initialState = {

    // for deck description
    [MARKDOWN_VIEW]: MARKDOWN_VIEW_SOURCE

};

/* exports */

const rehydrate = require('helpers/hydrate');

module.exports = function() {

    const store = createStore(makeReducer({
        reducer: rehydrate
    }), initialState);

    const component = (
        <Provider store={store}>
            <NewDeckContainer />
        </Provider>
    );

    return {
        store,
        component
    };

};

module.exports.initialState = initialState;
