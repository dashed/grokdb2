require('babel-polyfill');
global.Promise = require('bluebird');

const React = require('react');

const assign = require('lodash/assign');

const {Provider, connect} = require('react-redux');
const {reduxForm, reducer: reduxformReducer} = require('redux-form');
const classnames = require('classnames');

const fetch = require('fetch-ponyfill')({
    Promise: require('bluebird')
});


const {

    MARKDOWN_VIEW,
    MARKDOWN_VIEW_RENDER,
    MARKDOWN_VIEW_SOURCE,
    MARKDOWN_CONTENTS,

    DECK_DESCRIPTION,

    POST_TO,

    IS_EDITING
} = require('global/constants');

const {reduceIn} = require('lib/redux-tree');

/* react components */

const MarkdownRender = require('components/dumb/markdown_render');
const MarkdownSource = require('components/dumb/markdown_source');
const MathJaxLine = require('components/dumb/mathjax_line');

const __ToolBar = function(props) {

    const {isEditing, dispatch} = props;

    if(isEditing) {
        return (
            // TODO: disable unless there are changes
            <div className='level'>
                <div className='level-left'>
                    <div className='level-item'>
                        <a
                            className={classnames('button is-success')}
                            onClick={switchEditMode(dispatch, false)}>
                            {'Save'}
                        </a>
                    </div>
                </div>
                <div className='level-right'>
                    <div className='level-item'>
                        <a
                            className={classnames('button is-danger')}
                            onClick={switchEditMode(dispatch, false)}>
                            {'Cancel & Discard'}
                        </a>
                    </div>
                </div>
            </div>
        );
    }

    return (
        <div className='level'>
            <div className='level-left'>
                <div className='level-item'>
                    <a
                        className={classnames('button is-success')}
                        onClick={switchEditMode(dispatch, true)}>
                        {'Edit'}
                    </a>
                </div>
            </div>
        </div>
    );
};

if(process.env.NODE_ENV !== 'production') {
    __ToolBar.propTypes = {
        isEditing: React.PropTypes.bool.isRequired,
        dispatch: React.PropTypes.func.isRequired,
    };
}

const ToolBar = connect(
    // mapStateToProps
    (state) => {
        return {
            isEditing: state[DECK_DESCRIPTION][IS_EDITING]
        };
    }

)(__ToolBar);

const DeckDescriptionEditing = function() {
    return (
        <div>
            editing
        </div>
    );
};

const __DeckDescriptionMarkdown = function(props) {
    return (<MarkdownRender
        contents={props.contents}
        noContentMessage={'No description set for this deck. Click "Edit" button to add a description.'}
        />
    );
};

if(process.env.NODE_ENV !== 'production') {
    __DeckDescriptionMarkdown.propTypes = {
        contents: React.PropTypes.string.isRequired
    };
}

const DeckDescriptionMarkdown = connect(
    // mapStateToProps
    (state) => {
        return {
            contents: state[DECK_DESCRIPTION][MARKDOWN_CONTENTS]
        };
    }

)(__DeckDescriptionMarkdown);

const __DeckDescription = function(props) {

    const {isEditing} = props;

    if(isEditing) {
        return (<DeckDescriptionEditing />);
    }

    return (<DeckDescriptionMarkdown />);
};

if(process.env.NODE_ENV !== 'production') {
    __DeckDescription.propTypes = {
        isEditing: React.PropTypes.bool.isRequired
    };
}

const DeckDescription = connect(
    // mapStateToProps
    (state) => {
        return {
            isEditing: state[DECK_DESCRIPTION][IS_EDITING]
        };
    }

)(__DeckDescription);

const __DeckDescriptionContainer = function(props) {

    const {
        mathjaxifyDeckName,
        fields: { description },
        submitting,
        handleSubmit,
        resetForm,
        postURL
    } = props;

    const __description = assign({}, description);

    return (
        <div>
            <div className='columns' style={{marginBottom: 0}}>
                <div className='column'>
                    <ToolBar />
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <hr className='is-marginless'/>
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <DeckDescription
                        editedContents={description.value}
                        resetForm={resetForm}
                    />
                </div>
            </div>
        </div>
    );

};

const deckDescriptionContainerFactory = function(preRenderState) {

    return reduxForm(

        // config
        {
            form: 'deck_description',
            fields: ['description'],
            initialValues: {
                description: preRenderState[DECK_DESCRIPTION][MARKDOWN_CONTENTS]
            }
        },

        // mapStateToProps
        (state) => {
            return {
                postURL: state[POST_TO]
            };
        }

    )(__DeckDescriptionContainer);

};

/* redux action dispatchers */
// NOTE: FSA compliant

const switchEditMode = function(dispatch, isEditing) {
    return function(event) {
        event.preventDefault();
        dispatch(
            reduceIn(
                // reducer
                editingReducer,
                // path
                [DECK_DESCRIPTION, IS_EDITING],
                // action
                {
                    type: isEditing
                }
            )
        );
    }
}

const switchMarkdownView = function(dispatch, path, markdownView) {
    return function(event) {
        event.preventDefault();
        dispatch(
            reduceIn(
                // reducer
                markdownViewReducer,
                // path
                path,
                // action
                {
                    type: markdownView
                }
            )
        );
    }
};

/* redux reducers */

const markdownViewReducer = require('reducers/markdown_view');

const editingReducer = function(state = false, action) {

    switch(action.type) {
    case true:
    case false:
        state = action.type;
        break;

    default:
        state = false;
    }

    return state;
};

/* default state */

const initialState = {

    // NOTE: populated by window.__PRE_RENDER_STATE__
    [POST_TO]: '',

    [DECK_DESCRIPTION]: {
        [IS_EDITING]: false,
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_SOURCE,
        [MARKDOWN_CONTENTS]: ''
        // NOTE: contents is stored and handled by redux-form
    },

    // redux-form. generate initial state.
    form: reduxformReducer()

};

/* exports */

const formReducer = require('helpers/form_reducer');
const componentCreator = require('helpers/component_factory');

module.exports = componentCreator(initialState, function(store) {

    const __Component = deckDescriptionContainerFactory(store.getState());

    const component = (
        <Provider store={store}>
            <__Component />
        </Provider>
    );

    return component;

}, formReducer);

module.exports.initialState = initialState;
