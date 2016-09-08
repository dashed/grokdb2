require('global/normalize');

const React = require('react');

const assign = require('lodash/assign');
const get = require('lodash/get');

const {Provider, connect} = require('react-redux');
const {reduxForm, reducer: reduxformReducer} = require('redux-form');
const classnames = require('classnames');

const fetch = require('fetch-ponyfill')({
    Promise: require('bluebird')
});

const jsonDecode = require('helpers/json_decode');

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
const ErrorComponent = require('components/dumb/error');

const __ToolBar = function(props) {

    const {isEditing, dispatch, submitting} = props;

    if(isEditing) {

        const {handleSubmit, initialContent, newContent, postURL, resetForm} = props;

        const shouldNotSave = String(newContent).trim() == String(initialContent).trim();

        const cancel = function() {
            resetForm();
        };

        return (
            <div className='level'>
                <div className='level-left'>
                    <div className='level-item'>
                        <a
                            className={classnames('button is-success', {
                                'is-disabled': submitting || shouldNotSave,
                                'is-loading': submitting
                            })}
                            onClick={handleSubmit(saveDescription.bind(null, dispatch, postURL))}>
                            {'Save'}
                        </a>
                    </div>
                </div>
                <div className='level-right'>
                    <div className='level-item'>
                        <a
                            className={classnames('button is-danger')}
                            onClick={switchEditMode(dispatch, false, cancel)}>
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
        submitting: React.PropTypes.bool.isRequired,
        initialContent: React.PropTypes.string.isRequired,
        newContent: React.PropTypes.string.isRequired,
        postURL: React.PropTypes.string.isRequired,
        handleSubmit: React.PropTypes.func.isRequired,
        resetForm: React.PropTypes.func.isRequired,
    };
}

const ToolBar = connect(
    // mapStateToProps
    (state) => {
        return {
            isEditing: state[DECK_DESCRIPTION][IS_EDITING],
            initialContent: state[DECK_DESCRIPTION][MARKDOWN_CONTENTS],
            postURL: state[POST_TO]
        };
    }

)(__ToolBar);

const RenderSourceDescriptionComponent = connect(
    // mapStateToProps
    (state) => {
        return {
            [MARKDOWN_VIEW]: state[DECK_DESCRIPTION][MARKDOWN_VIEW]
        };
    },
    // mapDispatchToProps
    (dispatch) => {
        return {
            // markdownView := MARKDOWN_VIEW_RENDER | MARKDOWN_VIEW_SOURCE
            switchTab: (markdownView) => {
                return switchMarkdownView(dispatch, [DECK_DESCRIPTION, MARKDOWN_VIEW], markdownView);
            }
        };
    }
)(require('components/dumb/render_source'));

const __DeckDescription = function(props) {

    const description = props.description;
    const isEditing = props.isEditing;
    const markdownView = props[MARKDOWN_VIEW];

    const initialContents = props.initialContents;
    const editedContents = description.value;

    const renderContents = isEditing ? editedContents : initialContents;
    const sourceContents = isEditing ? editedContents : '';

    let sourceStyle = {};
    let renderStyle = {};

    switch(markdownView) {
    case MARKDOWN_VIEW_RENDER:
        sourceStyle.display = 'none';

        renderStyle.marginTop = '20px';
        renderStyle.marginBottom = '20px';

        break;

    case MARKDOWN_VIEW_SOURCE:
    default:
        renderStyle.display = 'none';
    }

    const noContentMessage = isEditing ?
        'No description set for this deck. Click "Source" button to add a description.' :
        'No description set for this deck. Click "Edit" button to add a description.';

    return (
        <div>
            <div key='render_source' className='columns'>
                <div className='column'>
                    <RenderSourceDescriptionComponent />
                </div>
            </div>
            <div key='content' className='columns'>
                <div className='column'>

                    <div key='render' style={renderStyle}>
                        <MarkdownRender
                            contents={renderContents}
                            noContentMessage={noContentMessage}
                        />
                    </div>
                    <div key='source' style={sourceStyle}>
                        <MarkdownSource
                            id='input-deck-description'
                            contents={sourceContents}
                            placeholder={'Deck Description'}
                            assignProps={description}
                            editable={isEditing}
                        />
                    </div>

                </div>
            </div>
        </div>
    );


};

if(process.env.NODE_ENV !== 'production') {
    __DeckDescription.propTypes = {
        description: React.PropTypes.object.isRequired,
        isEditing: React.PropTypes.bool.isRequired,
        initialContents: React.PropTypes.string.isRequired,
        showNoContentMessage: React.PropTypes.bool.isRequired,
        [MARKDOWN_VIEW]: React.PropTypes.oneOf([MARKDOWN_VIEW_RENDER, MARKDOWN_VIEW_SOURCE])
    };
}

const DeckDescription = connect(
    // mapStateToProps
    (state) => {
        return {
            [MARKDOWN_VIEW]: state[DECK_DESCRIPTION][MARKDOWN_VIEW],
            initialContents: state[DECK_DESCRIPTION][MARKDOWN_CONTENTS],
            isEditing: state[DECK_DESCRIPTION][IS_EDITING],
            showNoContentMessage: state[DECK_DESCRIPTION].showNoContentMessage
        };
    }

)(__DeckDescription);

const __DeckDescriptionContainer = function(props) {

    const {
        fields: { description },
        submitting,
        handleSubmit,
        resetForm,
        error
    } = props;

    const __description = assign({}, description);

    if(!props.showNoContentMessage) {

        return (
            <div>
                <div className='columns' style={{marginBottom: 0}}>
                    <div className='column'>
                        <ToolBar
                            handleSubmit={handleSubmit}
                            resetForm={resetForm}
                            submitting={submitting}
                            newContent={description.value}
                        />
                    </div>
                </div>
                <ErrorComponent error={error && error.message || ''} />
                <div className='columns'>
                    <div className='column'>
                        <hr className='is-marginless'/>
                    </div>
                </div>
            </div>
        );

    }

    return (
        <div>
            <div className='columns' style={{marginBottom: 0}}>
                <div className='column'>
                    <ToolBar
                        handleSubmit={handleSubmit}
                        resetForm={resetForm}
                        submitting={submitting}
                        newContent={description.value}
                    />
                </div>
            </div>
            <ErrorComponent error={error && error.message || ''} />
            <div className='columns'>
                <div className='column'>
                    <hr className='is-marginless'/>
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <DeckDescription
                        description={__description}
                    />
                </div>
            </div>
        </div>
    );

};

if(process.env.NODE_ENV !== 'production') {
    __DeckDescriptionContainer.propTypes = {
        fields: React.PropTypes.object.isRequired,
        handleSubmit: React.PropTypes.func.isRequired,
        submitting: React.PropTypes.bool.isRequired,
        postURL: React.PropTypes.string.isRequired,
        resetForm: React.PropTypes.func.isRequired,
        showNoContentMessage: React.PropTypes.bool.isRequired,
        // TODO: better prop type
        error: React.PropTypes.object
    };
}

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
                postURL: state[POST_TO],
                initialValues: {
                    // NOTE: this commits initial value after save
                    description: state[DECK_DESCRIPTION][MARKDOWN_CONTENTS],
                },
                showNoContentMessage: state[DECK_DESCRIPTION].showNoContentMessage
            };
        }

    )(__DeckDescriptionContainer);
};

/* redux action dispatchers */
// NOTE: FSA compliant

const defaultRESTError = 'Unable to update deck description. Please try again.';
const saveDescription = function(dispatch, postURL, formData) {

    return new Promise((resolve, reject) => {

        const finalDescription = String(formData.description).trim();

        fetch(postURL, {
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                description: finalDescription
            })
        })
        .then(function(response) {
            return Promise.all([response.status, jsonDecode(response)]);
        })
        .then(function([statusCode, jsonResponse]) {

            switch(statusCode) {
            case 400: // Bad Request
            case 500: // Internal Server Error

                reject({
                    _error: {
                        message: get(jsonResponse, ['error'], defaultRESTError)
                    }
                });

                return;
                break;

            case 200: // Ok

                // update deck description
                dispatch(
                    reduceIn(
                        // reducer
                        markdownContentsReducer,
                        // path
                        [DECK_DESCRIPTION, MARKDOWN_CONTENTS],
                        // action
                        {
                            type: MARKDOWN_CONTENTS,
                            payload: finalDescription
                        }
                    )
                );

                // switch out of edit mode
                dispatch(
                    reduceIn(
                        // reducer
                        editingReducer,
                        // path
                        [DECK_DESCRIPTION, IS_EDITING],
                        // action
                        {
                            type: false
                        }
                    )
                );

                // reset form to render mode
                dispatch(
                    reduceIn(
                        // reducer
                        markdownViewReducer,
                        // path
                        [DECK_DESCRIPTION, MARKDOWN_VIEW],
                        // action
                        {
                            type: MARKDOWN_VIEW_RENDER
                        }
                    )
                );

                resolve();

                break;

            default: // Unexpected http status code
                reject({
                    _error: {
                        message: defaultRESTError
                    }
                });
            }

        })
        .catch(function(/*err*/) {

            // any other errors
            // console.log('err:', err);

            reject({
                _error: {
                    message: defaultRESTError
                }
            });
        });

    });

};

const NOTHING = function() {};
const switchEditMode = function(dispatch, isEditing, after = NOTHING) {
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
        after();

        dispatch(
            reduceIn(
                // reducer
                markdownViewReducer,
                // path
                [DECK_DESCRIPTION, MARKDOWN_VIEW],
                // action
                {
                    type: isEditing ? MARKDOWN_VIEW_SOURCE : MARKDOWN_VIEW_RENDER
                }
            )
        );
    }
};

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
const showNoContentMessageReducer = require('reducers/bool');
const markdownContentsReducer = require('reducers/markdown_contents');
const editingReducer = require('reducers/bool');

/* default state */

const initialState = {

    // NOTE: populated by window.__PRE_RENDER_STATE__
    [POST_TO]: '',

    [DECK_DESCRIPTION]: {
        [IS_EDITING]: false,
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_RENDER,
        [MARKDOWN_CONTENTS]: '',
        showNoContentMessage: false
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

module.exports.preRender = function() {
    window.document.getElementById('deck_description_container_stub').style.display = 'none';
};

module.exports.afterRender = function(store) {
    // NOTE: By default, the markdown content is empty and MarkdownRender will NOT display
    // a notice of empty content. This reverts this behaviour.
    store.dispatch(
        reduceIn(
            // reducer
            showNoContentMessageReducer,
            // path
            [DECK_DESCRIPTION, 'showNoContentMessage'],
            // action
            {
                type: true
            }
        )
    );
};
