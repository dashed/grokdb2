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
    DECK_NAME,

    POST_TO
} = require('global/constants');

const {reduceIn} = require('lib/redux-tree');

/* react components */

const MarkdownRender = require('components/dumb/markdown_render');
const MarkdownSource = require('components/dumb/markdown_source');
const MathJaxLine = require('components/dumb/mathjax_line');

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

if(process.env.NODE_ENV !== 'production') {
    __NewDeckContainer.propTypes = {
        fields: React.PropTypes.object.isRequired,
        handleSubmit: React.PropTypes.func.isRequired,
        submitting: React.PropTypes.bool.isRequired,
        mathjaxifyDeckName: React.PropTypes.bool.isRequired,
        postURL: React.PropTypes.string.isRequired
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

                // TODO: error fix
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
