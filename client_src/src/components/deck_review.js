require('global/normalize');

const React = require('react');

const assign = require('lodash/assign');

const {Provider, connect} = require('react-redux');
const classnames = require('classnames');

const fetch = require('fetch-ponyfill')({
    Promise: require('bluebird')
});

const {

    MARKDOWN_VIEW,
    MARKDOWN_VIEW_RENDER,
    MARKDOWN_VIEW_SOURCE,

    MARKDOWN_CONTENTS,

    CARD_TITLE,
    CARD_DESCRIPTION,
    CARD_QUESTION,
    CARD_ANSWER,
    CARD_IS_ACTIVE,

    POST_TO,
    VALUE,

    IS_CONFIRM_SKIP,
    CURRENT_TAB
} = require('global/constants');

const {reduceIn} = require('lib/redux-tree');

/* react components */

const __PerformanceControls = function(props) {

    const isConfirmSkip = props[IS_CONFIRM_SKIP];

    if(isConfirmSkip) {
        return null;
    }

    const showMainControls = props[SHOW_MAIN_CONTROLS];

    if(!showMainControls) {
        return null;
    }

    return (
        <div className='columns'>
            <div className='column is-one-third'>
                <a className='button is-success is-fullwidth is-bold is-outlined'>
                    {'Right'}
                </a>
            </div>
            <div className='column is-one-third'>
                <a className='button is-danger is-fullwidth is-bold is-outlined'>
                    {'Wrong'}
                </a>
            </div>
            <div className='column is-one-third'>
                <a
                    className='button is-warning is-fullwidth is-bold is-outlined'
                    style={{color: '#978b52'}}
                >
                    {'Forgot'}
                </a>
            </div>
        </div>
    );
};

if(process.env.NODE_ENV !== 'production') {
    __PerformanceControls.propTypes = {
        [IS_CONFIRM_SKIP]: React.PropTypes.bool.isRequired,
        [SHOW_MAIN_CONTROLS]: React.PropTypes.bool.isRequired
    };
}

const PerformanceControls = connect(
    // mapStateToProps
    (state) => {
        return {
            [IS_CONFIRM_SKIP]: state[IS_CONFIRM_SKIP],
            [SHOW_MAIN_CONTROLS]: state[SHOW_MAIN_CONTROLS]
        };
    }

)(__PerformanceControls);

const __CommitButton = function(props) {

    const showMainControls = props[SHOW_MAIN_CONTROLS];

    if(showMainControls) {

        return (
            <a
                href='#next_card'
                className='button is-success is-fullwidth is-bold'
            >
                {'Next Card'}
            </a>
        );

    }

    const {dispatch} = props;

    return (
        <a
            href='#reveal_answer'
            className='button is-success is-fullwidth is-bold'
            onClick={shouldRevealAnswer(dispatch, true)}
        >
            {'Reveal Answer'}
        </a>
    );
};

if(process.env.NODE_ENV !== 'production') {
    __CommitButton.propTypes = {
        [SHOW_MAIN_CONTROLS]: React.PropTypes.bool.isRequired
    };
}

const CommitButton = connect(
    // mapStateToProps
    (state) => {
        return {
            [SHOW_MAIN_CONTROLS]: state[SHOW_MAIN_CONTROLS]
        };
    }

)(__CommitButton);

const __MainControls = function(props) {

    const {dispatch} = props;
    const isConfirmSkip = props[IS_CONFIRM_SKIP];

    if(isConfirmSkip) {
        return (

                <div className='columns'>
                    <div className='column is-half'>
                        <a
                            href='#confirm_skip'
                            className='button is-success is-fullwidth is-bold'
                        >
                            {'Yes, skip'}
                        </a>
                    </div>
                    <div className='column is-half'>
                        <a
                            href='#cancel_confirm_skip'
                            className='button is-danger is-fullwidth is-bold'
                            onClick={shouldConfirmSkip(dispatch, false)}
                        >
                            {'No, do not skip'}
                        </a>
                    </div>
                </div>
        );
    }

    return (

            <div className='columns'>
                <div className='column is-two-thirds'>
                    <CommitButton />
                </div>
                <div className='column is-one-third'>
                    <a
                        href='#confirm_skip'
                        className='button is-danger is-fullwidth is-bold'
                        onClick={shouldConfirmSkip(dispatch, true)}
                    >
                        {'Skip Card'}
                    </a>
                </div>
            </div>
    );
};

if(process.env.NODE_ENV !== 'production') {
    __MainControls.propTypes = {
        [IS_CONFIRM_SKIP]: React.PropTypes.bool.isRequired
    };
}

const MainControls = connect(
    // mapStateToProps
    (state) => {
        return {
            [IS_CONFIRM_SKIP]: state[IS_CONFIRM_SKIP]
        };
    }

)(__MainControls);

const ReviewControls = function(props) {

    return (
        <div>
            <MainControls />
            <PerformanceControls />
        </div>
    );

};

const DeckReview = function(props) {

    return (
        <div>
            <ReviewControls />
        </div>
    );
};

/* redux action dispatchers */
// NOTE: FSA compliant

const shouldConfirmSkip = function(dispatch, isConfirming) {
    return function(event) {
        event.preventDefault();
        dispatch(
            reduceIn(
                // reducer
                boolReducer,
                // path
                [IS_CONFIRM_SKIP],
                // action
                {
                    type: isConfirming
                }
            )
        );
    }
};

const shouldRevealAnswer = function(dispatch, shouldReveal) {
    return function(event) {
        event.preventDefault();
        dispatch(
            reduceIn(
                // reducer
                boolReducer,
                // path
                [SHOW_MAIN_CONTROLS],
                // action
                {
                    type: shouldReveal
                }
            )
        );
    }
};

/* redux reducers */

const boolReducer = require('reducers/bool');

/* default state */

const SHOW_MAIN_CONTROLS = 'SHOW_MAIN_CONTROLS';

const initialState = {

    [POST_TO]: '',

    [CARD_TITLE]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_RENDER,
        [MARKDOWN_CONTENTS]: ''
    },

    // CURRENT_TAB = CARD_QUESTION | CARD_ANSWER | CARD_DESCRIPTION
    [CURRENT_TAB]: CARD_QUESTION,

    [CARD_DESCRIPTION]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_RENDER,
        [MARKDOWN_CONTENTS]: ''
    },

    [CARD_QUESTION]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_RENDER,
        [MARKDOWN_CONTENTS]: ''
    },

    [CARD_ANSWER]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_RENDER,
        [MARKDOWN_CONTENTS]: ''
    },

    [CARD_IS_ACTIVE]: {
        [VALUE]: true
    },

    [IS_CONFIRM_SKIP]: false,
    [SHOW_MAIN_CONTROLS]: false

};

/* exports */

const componentCreator = require('helpers/component_factory');

module.exports = componentCreator(initialState, function(store) {

    const component = (
        <Provider store={store}>
            <DeckReview />
        </Provider>
    );

    return component;

});

module.exports.initialState = initialState;
