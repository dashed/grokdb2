const React = require('react');
const {Provider, connect} = require('react-redux');
const {createStore, applyMiddleware} = require('redux');
const classnames = require('classnames');
const {reduxForm, reducer: reduxformReducer} = require('redux-form');

const {

    MARKDOWN_VIEW,
    MARKDOWN_VIEW_RENDER,
    MARKDOWN_VIEW_SOURCE,

    MARKDOWN_CONTENTS,
    DECK_DESCRIPTION,

    POST_TO,

} = require('global/constants');

const {reduceIn, makeReducer} = require('lib/redux-tree');

/* react components */

const MarkdownRender = require('components/dumb/markdown_render');
const MarkdownSource = require('components/dumb/markdown_source');

const RenderSourceComponent = connect(

    // mapStateToProps
    (state) => {
        return {
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
                <MarkdownRender contents={contents} />
            </div>
            <div>
                <div className='form-group' style={sourceStyle}>
                    <MarkdownSource
                        contents={contents}
                        placeholder={'Description for new deck'}
                        assignProps={props.assignProps}
                        editable
                    />
                </div>
            </div>
        </div>
    );

}

if(process.env.NODE_ENV !== 'production') {
    __DeckDescriptionComponent.propTypes = {
        assignProps: React.PropTypes.object,
    };
}

const DeckDescriptionComponent = connect(

    // mapStateToProps
    (state, ownProps) => {

        return {

            [MARKDOWN_VIEW]: state[DECK_DESCRIPTION][MARKDOWN_VIEW],

            // [MARKDOWN_CONTENTS]: state[DECK_DESCRIPTION][MARKDOWN_CONTENTS]
            // from redux-form
            [MARKDOWN_CONTENTS]: ownProps.assignProps.value

        };
    }
)(__DeckDescriptionComponent);

const __NewDeckContainer = function(props) {
    const {
        fields: { name, description},
        handleSubmit,
        // resetForm, // not used
        submitting,
        // addNewDeck
    } = props;

    return (
        <div>
            <div className='columns'>
                <div className='column col-12'>
                    <div className='form-group'>
                        <label className='form-label' htmlFor='input-deck-name'>
                            {'Name'}
                        </label>
                        <input
                            className='form-input'
                            type='text'
                            // id='input-deck-name'
                            placeholder='Name for new deck'
                            {...name}
                        />
                    </div>
                </div>
            </div>
            <div className='columns'>
                <div className='column col-12'>
                    <RenderSourceComponent />
                </div>
            </div>
            <div className='columns'>
                <div className='column col-12'>
                    <DeckDescriptionComponent assignProps={description} />
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <a
                        href='#add-new-deck'
                        className={classnames('btn btn-success', {
                            'loading': submitting
                        })}
                        onClick={handleSubmit(addNewDeck)}
                        disabled={submitting}
                    >
                        {'Add new deck'}
                    </a>
                </div>
            </div>
        </div>
    );
}

if(process.env.NODE_ENV !== 'production') {
    __NewDeckContainer.propTypes = {
        fields: React.PropTypes.object.isRequired,
        handleSubmit: React.PropTypes.func.isRequired,
        submitting: React.PropTypes.bool.isRequired,
        // addNewDeck: React.PropTypes.func.isRequired,
    };
}

const NewDeckContainer = reduxForm(
    {
        form: 'new_deck',
        fields: ['name', 'description'],
        initialValues: {
            name: '',
            description: ''
        }
    },
    // mapStateToProps
    // void 0,
    // mapDispatchToProps
    // (dispatch) => {
    //     return {
    //         addNewDeck
    //     };
    // }
)(__NewDeckContainer);


/* redux action dispatchers */
// NOTE: FSA compliant

const addNewDeck = function(formData) {
    return new Promise((resolve, reject) => {

        window.location.href = '/deck/1/decks';
    });
};

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

    [POST_TO]: '',

    [DECK_DESCRIPTION]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_SOURCE,
        // [MARKDOWN_CONTENTS]: '' // not used; value is stored and handled by redux-form
    },

    // redux-form
    form: reduxformReducer()

};

/* exports */

const merge = require('lodash/merge');

const formReducer = (state, action) => {

    // NOTE: we're not using combineReducers from redux

    const newForm = reduxformReducer(state.form, action);
    const newState = merge({}, state);
    newState.form = newForm;

    return newState;
};

const rehydrateFactory = require('helpers/hydrate');

const middleware = () => {

    if(process.env.NODE_ENV !== 'production') {

        const createLogger = require('redux-logger');
        const logger = createLogger();

        return applyMiddleware(logger);
    }

    return applyMiddleware();
};

module.exports = function(preRenderState) {

    if(preRenderState) {
        preRenderState = merge({}, initialState, preRenderState);
    } else {
        preRenderState = initialState;
    }

    const store = createStore(makeReducer({
        reducer: rehydrateFactory(formReducer)
    }), preRenderState, middleware());

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
