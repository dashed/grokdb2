require('babel-polyfill');

global.Promise = require('bluebird');

const React = require('react');

const assign = require('lodash/assign');

const {createStore, applyMiddleware} = require('redux');
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
    DECK_NAME,

    POST_TO
} = require('global/constants');

const {reduceIn, makeReducer} = require('lib/redux-tree');

/* react components */

const MarkdownRender = require('components/dumb/markdown_render');
const MarkdownSource = require('components/dumb/markdown_source');
const MathJaxLine = require('components/dumb/mathjax_line');

// TODO: remove
const old__NewDeckContainer = function(props) {
    const {
        fields: { name, description},
        handleSubmit,
        // resetForm, // not used
        submitting,
        error
    } = props;

    const postURL = '/api/deck';

    return (
        <div>
            <div className='columns'>
                <div className='column col-12'>
                    {
                        (() => {

                            if(!name.touched || !name.error) {
                                return null;
                            }

                            return (
                                <div className='toast toast-danger'>
                                    {name.error}
                                </div>
                            );
                        })()
                    }
                    <div className='form-group'>
                        <label className='form-label' htmlFor='input-deck-name'>
                            {'Name'}
                        </label>
                        <input
                            className={classnames('form-input', {
                                'is-danger': name.touched && name.error
                            })}
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
            {
                (() => {

                    if(!error) {
                        return null;
                    }

                    return (<div className='columns'>
                        <div className='column col-12'>
                            <div className='toast toast-danger'>
                                {error.message}
                            </div>
                        </div>
                    </div>);
                })()
            }
            <div className='columns'>
                <div className='column'>
                    <a
                        href='#add-new-deck'
                        className={classnames('btn btn-success', {
                            'loading': submitting
                        })}
                        onClick={handleSubmit(addNewDeck.bind(null, postURL))}
                        disabled={submitting}
                    >
                        {'Add new deck'}
                    </a>
                </div>
            </div>
        </div>
    );
}


// TODO: remove
// const old_NewDeckContainer = reduxForm(
//     {
//         form: 'new_deck',
//         fields: ['name', 'description'],
//         validate: (values) => {
//             const errors = {};

//             if (!values.name) {
//                 errors.name = 'Deck name is required';
//             } else if(values.name.trim().length <= 0) {
//                 errors.name = 'Deck name cannot be entirely spaces (or whitespace).';
//             }

//             return errors;
//         },
//         initialValues: {
//             name: '',
//             description: ''
//         }
//     },
//     // mapStateToProps
//     // (state) => {

//     //     return {
//     //         [POST_TO]: state[POST_TO]
//     //     };
//     // }
//     // mapDispatchToProps
//     // (dispatch) => {
//     //     return {
//     //         addNewDeck
//     //     };
//     // }
// )(__NewDeckContainer);

////////////////////////////////////////////////////////////////////////////////

const RenderSourceNameComponent = connect(
    // mapStateToProps
    (state) => {
        return {
            [MARKDOWN_VIEW]: state[DECK_NAME][MARKDOWN_VIEW]
        };
    },
    // mapDispatchToProps
    (dispatch) => {
        return {
            // markdownView := MARKDOWN_VIEW_RENDER | MARKDOWN_VIEW_SOURCE
            switchTab: (markdownView) => {
                return switchMarkdownView(dispatch, [DECK_NAME, MARKDOWN_VIEW], markdownView);
            }
        };
    }
)(require('components/dumb/render_source'));

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
                <div style={sourceStyle}>
                    <MarkdownSource
                        id='input-deck-description'
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

            // from redux-form
            [MARKDOWN_CONTENTS]: ownProps.assignProps.value

        };
    }

)(__DeckDescriptionComponent);

const __NewDeckContainer = function(props) {

    const {
        mathjaxifyDeckName,
        fields: { name, description},
        submitting,
        handleSubmit,
        postURL
    } = props;

    // const __name = assign({}, name);
    const __description = assign({}, description);

    return (
        <div>
            <div className='columns' style={{marginBottom: 0}}>
                <div className='column'>
                    <label className='label' htmlFor='input-deck-name'>{'Name'}</label>
                        <MathJaxLine
                            content={name.value}
                            mathjaxify={mathjaxifyDeckName}
                            notice={'No deck name rendered.  Click on "Source" tab and enter a name.'}
                        >
                            <p className='control'>
                                <input
                                    id='input-deck-name'
                                    className='input'
                                    type='text'
                                    placeholder='Name for new deck'
                                    autoFocus
                                    {...assign({}, name)}
                                />
                            </p>
                        </MathJaxLine>
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <RenderSourceNameComponent
                        extraClasses='is-small'
                        reverse
                    />
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <hr className='is-marginless'/>
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <div className='columns' style={{marginBottom: 0}}>
                        <div className='column'>
                            <label className='label' htmlFor='input-deck-description'>{'Description'}</label>
                            <RenderSourceDescriptionComponent
                                reverse
                            />
                        </div>
                    </div>
                    <div className='columns'>
                        <div className='column'>
                            <DeckDescriptionComponent assignProps={__description} />
                        </div>
                    </div>
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <hr className='is-marginless'/>
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <a
                    className={classnames('button is-success', {
                        'is-disabled': submitting || String(name.value).trim().length <= 0,
                        'is-loading': submitting
                    })}
                    onClick={handleSubmit(addNewDeck.bind(null, postURL))}>
                        {'Add Deck'}
                    </a>
                </div>
            </div>
        </div>
    );
};

const NewDeckContainer = reduxForm(

    // config
    {
        form: 'new_deck',
        fields: ['name', 'description'],
        initialValues: {
            name: '',
            description: ''
        }
    },

    // mapStateToProps
    (state) => {
        return {
            mathjaxifyDeckName: state[DECK_NAME][MARKDOWN_VIEW] === MARKDOWN_VIEW_RENDER,
            postURL: state[POST_TO]
        };
    }

)(__NewDeckContainer);

if(process.env.NODE_ENV !== 'production') {
    __NewDeckContainer.propTypes = {
        fields: React.PropTypes.object.isRequired,
        handleSubmit: React.PropTypes.func.isRequired,
        submitting: React.PropTypes.bool.isRequired,
        mathjaxifyDeckName: React.PropTypes.bool.isRequired,
        postURL: React.PropTypes.string.isRequired,
        // TODO: fix
        // addNewDeck: React.PropTypes.func.isRequired,
    };
}

/* redux action dispatchers */
// NOTE: FSA compliant

// TODO: refactor
const addNewDeck = function(postURL, formData) {

    return new Promise((resolve, reject) => {

        fetch(postURL, {
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                name: formData.name,
                description: formData.description
            })
        })
        .then(function(response) {

            return Promise.all([response.status, response.json()]);
        }, function(err) {

            // TODO: handle on network failure, etc

            console.log('err:', err);

            reject({
                _error: {
                    message: 'Unable to create new deck.'
                }
            });
        })
        .then(function([statusCode, jsonResponse]) {

            console.log(jsonResponse);

            switch(statusCode) {
            case 400: // Bad Request
            case 500: // Internal Server Error

                // response.userMessage

                // TODO: description
                //
                // http://redux-form.com/5.2.5/#/api/props
                // how to detect errors
                reject({
                    _error: {
                        message: jsonResponse.userMessage
                    }
                });
                return;
                break;

            case 200: // Ok

                window.location.href = jsonResponse.profile_url;
                break;

            default: // Unexpected http status code
                reject({
                    _error: {
                        message: 'Unable to create new deck.'
                    }
                });
            }

        }, function(err) {
            // TODO: handle on json parsing fail

            console.log('err:', err);

            reject({
                _error: {
                    message: 'Unable to create new deck.'
                }
            });
        })
        .catch(function(err) {
            // TODO: handle
            console.log('err:', err);

            reject({
                _error: {
                    message: 'Unable to create new deck.'
                }
            });
        });

    });
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
};

/* default state */

const initialState = {

    [POST_TO]: '',

    [DECK_NAME]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_SOURCE,
        // NOTE: contents is stored and handled by redux-form
    },

    [DECK_DESCRIPTION]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_SOURCE,
        // NOTE: contents is stored and handled by redux-form
    },

    // redux-form. generate initial state.
    form: reduxformReducer()

};

/* exports */

const merge = require('lodash/merge');

const formReducer = (state, action) => {

    // NOTE: We're not using combineReducers from redux as redux-form expects.
    //       Defer any un-captured action to redux-form.

    const newForm = reduxformReducer(state.form, action);
    const newState = merge({}, state);
    newState.form = newForm;

    return newState;
};

const rehydrateFactory = require('helpers/hydrate');

// TODO: refactor to module
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
