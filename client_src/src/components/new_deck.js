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
            switchTab: (dispatch, markdownView) => switchMarkdownView(dispatch, markdownView)
        };
    }
)(require('components/dumb/source_render'));

const newDeckStyle = {
    marginTop: 0,
    marginBottom: 0
};
const NewDeckContainer = function(/* props */) {
    return (
        <div>
            <div className='columns'>
                <div className='column'>
                    <h5 style={newDeckStyle}>
                        {'New Deck'}
                    </h5>
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <div className='form-group'>
                        <label className='form-label' htmlFor='input-deck-name'>
                            {'Name'}
                        </label>
                        <input
                            className='form-input'
                            type='text'
                            id='input-deck-name'
                            placeholder='Name for new deck'
                        />
                    </div>
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <RenderSourceComponent />
                </div>
            </div>
        </div>
    );
}


/* redux action dispatchers */
// NOTE: FSA compliant

const switchMarkdownView = function(dispatch, markdownView) {
    return function(event) {
        event.preventDefault();
        dispatch(
            reduceIn(
                // reducer
                markdownViewReducer,
                // path
                [MARKDOWN_VIEW],
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
