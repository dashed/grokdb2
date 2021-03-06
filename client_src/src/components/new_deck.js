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
    DECK_NAME,

    POST_TO
} = require('global/constants');

const {reduceIn} = require('lib/redux-tree');

/* react components */

const MarkdownRender = require('components/dumb/markdown_render');
const MarkdownSource = require('components/dumb/markdown_source');
const MathJaxLine = require('components/dumb/mathjax_line');
const ErrorComponent = require('components/dumb/error');

const renderSourceComponentFactory = function(key) {

    return connect(
        // mapStateToProps
        (state) => {
            return {
                [MARKDOWN_VIEW]: state[key][MARKDOWN_VIEW]
            };
        },
        // mapDispatchToProps
        (dispatch) => {
            return {
                // markdownView := MARKDOWN_VIEW_RENDER | MARKDOWN_VIEW_SOURCE
                switchTab: (markdownView) => {
                    return switchMarkdownView(dispatch, [key, MARKDOWN_VIEW], markdownView);
                }
            };
        }
    )(require('components/dumb/render_source'));
};

const RenderSourceNameComponent = renderSourceComponentFactory(DECK_NAME);
const RenderSourceDescriptionComponent = renderSourceComponentFactory(DECK_DESCRIPTION);

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
                        placeholder={'Deck Description'}
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
        postURL,
        error
    } = props;

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
                                    placeholder='Deck Name'
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
            <ErrorComponent error={error && error.message || ''} />
            <div className='columns'>
                <div className='column'>
                    <a
                    className={classnames('button is-success is-bold', {
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

if(process.env.NODE_ENV !== 'production') {
    __NewDeckContainer.propTypes = {
        fields: React.PropTypes.object.isRequired,
        handleSubmit: React.PropTypes.func.isRequired,
        submitting: React.PropTypes.bool.isRequired,
        mathjaxifyDeckName: React.PropTypes.bool.isRequired,
        postURL: React.PropTypes.string.isRequired,
        // TODO: better prop type
        error: React.PropTypes.object
    };
}

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

/* redux action dispatchers */
// NOTE: FSA compliant

const defaultRESTError = 'Unable to create new deck. Please try again.';
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

                window.location.href = jsonResponse.payload.profile_url;
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

const formReducer = require('helpers/form_reducer');
const componentCreator = require('helpers/component_factory');

module.exports = componentCreator(initialState, function(store) {

    const component = (
        <Provider store={store}>
            <NewDeckContainer />
        </Provider>
    );

    return component;

}, formReducer);

module.exports.initialState = initialState;
