const React = require('react');
const {Provider, connect} = require('react-redux');
const {createStore} = require('redux');
const classnames = require('classnames');

const {

    MARKDOWN_VIEW,
    MARKDOWN_VIEW_RENDER,
    MARKDOWN_VIEW_SOURCE,

    MARKDOWN_CONTENTS,
    DECK_DESCRIPTION,

} = require('global/constants');

const {reduceIn, makeReducer} = require('lib/redux-tree');

/* react components */

const MarkdownSource = require('components/dumb/markdown_source');

const RenderSourceComponent = connect(

    // mapStateToProps
    (state) => {
        return{
            [MARKDOWN_VIEW]: state[DECK_DESCRIPTION][MARKDOWN_VIEW],
            switchTab: (dispatch, markdownView) => switchMarkdownView(dispatch, markdownView)
        };
    }
)(require('components/dumb/source_render'));

const __DeckDescriptionComponent = function(props) {

    const markdownView = props[MARKDOWN_VIEW];
    const contents = props[MARKDOWN_CONTENTS];

    let sourceStyle = {};
    let renderStyle = {};

    switch(markdownView) {
    case MARKDOWN_VIEW_RENDER:
        sourceStyle.display = 'none';
        break;

    case MARKDOWN_VIEW_SOURCE:
    default:
        renderStyle.display = 'none';
    }

    return (
        <div>
            <div style={renderStyle}>
                {contents}
            </div>
            <div>
                <MarkdownSource
                    contents={contents}
                    style={sourceStyle}
                    placeholder={'Description for new deck'}
                    editable
                />
            </div>
        </div>
    );

}

const DeckDescriptionComponent = connect(

    // mapStateToProps
    (state) => {

        return {

            [MARKDOWN_VIEW]: state[DECK_DESCRIPTION][MARKDOWN_VIEW],
            [MARKDOWN_CONTENTS]: state[DECK_DESCRIPTION][MARKDOWN_CONTENTS]

        };
    }
)(__DeckDescriptionComponent);

const NewDeckContainer = function(/* props */) {
    return (
        <div>
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
            <div className='columns'>
                <div className='column'>
                    <DeckDescriptionComponent />
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <a
                        href='#add-new-deck'
                        className='btn btn-success'>
                        {'Add new deck'}
                    </a>
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
                [DECK_DESCRIPTION, MARKDOWN_VIEW],
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

    [DECK_DESCRIPTION]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_SOURCE,
        [MARKDOWN_CONTENTS]: 'descrip'
    },

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
