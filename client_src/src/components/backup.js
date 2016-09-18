require('global/normalize');

const React = require('react');
const ReactDOM = require('react-dom');

const {Provider, connect} = require('react-redux');
const classnames = require('classnames');
const get = require('lodash/get');

const {reduceIn} = require('lib/redux-tree');

const fetch = require('fetch-ponyfill')({
    Promise: require('bluebird')
});

const jsonDecode = require('helpers/json_decode');

const {
    ERROR,
    ERROR_MESSAGE,

} = require('global/constants');

// TODO: move
const RESULT = 'RESULT';
const SUBMITTING = 'SUBMITTING';

/* react components */

const ErrorComponent = require('components/dumb/error');

const Message = function(props) {

    const {result} = props;

    let msg = String(result).trim();

    if(msg.length <= 0) {
        return null;
    }

    return (
        <p>
            {msg}
        </p>
    );
}

if(process.env.NODE_ENV !== 'production') {
    Message.propTypes = {
        result: React.PropTypes.string.isRequired,
    };
}

const __BackupButton = function(props) {

    const {dispatch, submitting, error, result} = props;

    const onClick = (event) => {
        event.preventDefault();
        backupDatabase(dispatch, submitting);
    };

    return (
        <div>
            <a
                className={classnames('button is-primary is-bold is-outlined', {
                    'is-disabled': submitting,
                    'is-loading': submitting
                })}
                onClick={onClick}
            >
                {'Backup database'}
            </a>
            <br/>
            <Message result={result} />
            <br/>
            <ErrorComponent error={error && error.message || ''} onConfirm={confirmError(dispatch)} />
        </div>
    );
};

if(process.env.NODE_ENV !== 'production') {
    __BackupButton.propTypes = {
        submitting: React.PropTypes.bool.isRequired,
        dispatch: React.PropTypes.func.isRequired,
        error: React.PropTypes.object.isRequired,
        result: React.PropTypes.string.isRequired,
    };
}

const BackupButton = connect(
    // mapStateToProps
    (state) => {
        return {
            submitting: state[SUBMITTING],
            error: state[ERROR],
            result: state[RESULT],
        };
    }
)(__BackupButton);

/* redux action dispatchers */
// NOTE: FSA compliant

const confirmError = function(dispatch) {
    return function(event) {
        event.preventDefault();
        dispatch(
            reduceIn(
                // reducer
                errorReducer,
                // path
                [ERROR],
                // action
                {
                    type: ERROR_MESSAGE,
                    payload: ''
                }
            )
        );
    };
};

const defaultRESTError = 'Unable to backup database. Please try again.';
const backupDatabase = function(dispatch, submitting = true) {

    if(submitting) {
        return;
    }

    dispatch(
        reduceIn(
            // reducer
            boolReducer,
            // path
            [SUBMITTING],
            // action
            {
                type: true
            }
        )
    );

    fetch('/api/backup', {
        method: 'POST'
    })
    .then(function(response) {
        return Promise.all([response.status, jsonDecode(response)]);
    })
    .then(function([statusCode, jsonResponse]) {

        dispatch(
            reduceIn(
                // reducer
                boolReducer,
                // path
                [SUBMITTING],
                // action
                {
                    type: false
                }
            )
        );

        switch(statusCode) {
        case 500: // Internal Server Error
        case 400: // Bad Request

            dispatch(
                reduceIn(
                    // reducer
                    errorReducer,
                    // path
                    [ERROR],
                    // action
                    {
                        type: ERROR_MESSAGE,
                        payload: get(jsonResponse, ['error'], defaultRESTError)
                    }
                )
            );

            return;
            break;
        case 200: // Ok

            dispatch(
                reduceIn(
                    // reducer
                    resultReducer,
                    // path
                    [RESULT],
                    // action
                    {
                        type: SET,
                        payload: jsonResponse.payload
                    }
                )
            );

            return;
            break;

        default: // Unexpected http status code
            dispatch(
                reduceIn(
                    // reducer
                    errorReducer,
                    // path
                    [ERROR],
                    // action
                    {
                        type: ERROR_MESSAGE,
                        payload: get(jsonResponse, ['error'], defaultRESTError)
                    }
                )
            );
        }

    })
    .catch(function(/*err*/) {

        // any other errors
        // console.log('err:', err);

        dispatch(
            reduceIn(
                // reducer
                boolReducer,
                // path
                [SUBMITTING],
                // action
                {
                    type: false
                }
            )
        );

        dispatch(
            reduceIn(
                // reducer
                errorReducer,
                // path
                [ERROR],
                // action
                {
                    type: ERROR_MESSAGE,
                    payload: defaultRESTError
                }
            )
        );

    });


};

/* redux reducers */

const boolReducer = require('reducers/bool');
const errorReducer = require('reducers/error_message');

const SET = 'SET';
const resultReducer = function(state = '', action) {

    switch(action.type) {
    case SET:

        state = `Database backed up to: ${action.payload.dest_path}`;

        break;
    default:
        state = '';
    }

    return String(state);
};

/* default state */

const initialState = {

    [RESULT]: '',

    [SUBMITTING]: false,

    [ERROR]: errorReducer(),
};

/* exports */

const createStore = require('helpers/create_store');

module.exports = function(elem) {

    const cloneDeep = require('lodash/cloneDeep');

    const __initialState = cloneDeep(initialState);

    const store = createStore(__initialState);

    ReactDOM.render(
        <Provider store={store}>
            <BackupButton />
        </Provider>,
        elem,
    );
};
