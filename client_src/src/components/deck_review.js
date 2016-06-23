const React = require('react');
const {Provider, connect} = require('react-redux');
const {createStore} = require('redux');
const classnames = require('classnames');

const {

    TAB,
    TAB_QUESTION,
    TAB_ANSWER,
    TAB_DESCRIPTION,

    MARKDOWN_VIEW,
    MARKDOWN_VIEW_RENDER,
    MARKDOWN_VIEW_SOURCE,

    CARD_CONTENTS,

    CARD_PERF_CONTROL_VIEW,
    CARD_PERF_CONTROL__INITIAL,
    CARD_PERF_CONTROL__DEFAULT_CHOICES,

    SKIPCARD_VIEW,
    SKIPCARD_INITIAL,
    SKIPCARD_CONFIRM,

} = require('global/constants');

const {reduceIn, makeReducer} = require('lib/redux-tree');

/* react components */

const MarkdownSource = require('components/dumb/markdown_source');

// TODO: move to its own file for re-use
const __CardReviewTabsComponent = function(props) {

    const tabType = props[TAB];
    const {dispatch} = props;

    return (
        <div className='btn-group btn-group-block'>
            <a
                onClick={switchTab(dispatch, TAB_QUESTION)}
                href='#question'
                className = {classnames('btn', {'btn-primary': tabType === TAB_QUESTION})}>
                {'Question'}
            </a>
            <a
                onClick={switchTab(dispatch, TAB_DESCRIPTION)}
                href='#description'
                className = {classnames('btn', {'btn-primary': tabType === TAB_DESCRIPTION})}>
                {'Description'}
            </a>
        </div>
    );
}
if(process.env.NODE_ENV !== 'production') {
    __CardReviewTabsComponent.propTypes = {
        dispatch: React.PropTypes.func,
        [TAB]: React.PropTypes.oneOf([TAB_QUESTION, TAB_DESCRIPTION, TAB_ANSWER])
    };
}

const CardReviewTabsComponent = connect(
    // mapStateToProps
    (state) => {
        return{
            [TAB]: state[TAB]
        };
    }
)(__CardReviewTabsComponent);

const RenderSourceComponent = connect(

    // mapStateToProps
    (state) => {
        const currentCardTab = state[TAB];
        return{
            [MARKDOWN_VIEW]: state[currentCardTab][MARKDOWN_VIEW],
            switchTab: (dispatch, markdownView) => switchMarkdownView(dispatch, currentCardTab, markdownView)
        };
    },

    // mapDispatchToProps
    // (dispatch) => {
    //     return {
    //         switchTab:
    //     };
    // }
)(require('components/dumb/render_source'));

const __RevealCommitButtonComponent = function(props) {

    const {dispatch} = props;

    const perfControlView = props[CARD_PERF_CONTROL_VIEW];

    switch(perfControlView) {
    case CARD_PERF_CONTROL__INITIAL:


        return (
            <a
                href='#confirm-skip'
                className='btn btn-block'
                onClick={switchCardPerfControl(dispatch, CARD_PERF_CONTROL__DEFAULT_CHOICES)}>
                {'Reveal Answer'}
            </a>
        );

    case CARD_PERF_CONTROL__DEFAULT_CHOICES:
    default:

        return (
            <a
                href='#confirm-skip'
                className='btn btn-block'
                onClick={() => void 0}>
                {'Commit Score & Next Card'}
            </a>
        );

    }

}
if(process.env.NODE_ENV !== 'production') {
    __RevealCommitButtonComponent.propTypes = {
        dispatch: React.PropTypes.func,
        [CARD_PERF_CONTROL_VIEW]: React.PropTypes.oneOf([
            CARD_PERF_CONTROL__INITIAL,
            CARD_PERF_CONTROL__DEFAULT_CHOICES
        ])
    };
}

const RevealCommitButtonComponent = connect(
    // mapStateToProps
    (state) => {
        return{
            [CARD_PERF_CONTROL_VIEW]: state[CARD_PERF_CONTROL_VIEW]
        };
    },

)(__RevealCommitButtonComponent);

const __CardPerformanceControlComponent = function(props) {

    const {shouldNotShow} = props;

    if(shouldNotShow) {
        return null;
    }

    // switch(perfControlView) {
    // case CARD_PERF_CONTROL__INITIAL:
    //     return;
    // }

    return (
        <div className='columns'>
            <div className='column col-4'>
                <a
                    href='#confirm-skip'
                    className='btn btn-block'
                    onClick={() => void 0}>
                    {'Fail'}
                </a>
            </div>
            <div className='column col-4'>
                <a
                    href='#confirm-skip'
                    className='btn btn-block'
                    onClick={() => void 0}>
                    {'Success'}
                </a>
            </div>
            <div className='column col-4'>
                <a
                    href='#confirm-skip'
                    className='btn btn-block'
                    onClick={() => void 0}>
                    {'Reset Score'}
                </a>
            </div>
        </div>
    );
}
if(process.env.NODE_ENV !== 'production') {
    __CardPerformanceControlComponent.propTypes = {
        shouldNotShow: React.PropTypes.bool,
    };
}

const CardPerformanceControlComponent = connect(
    // mapStateToProps
    (state) => {
        const shouldNotShow = (state[SKIPCARD_VIEW] === SKIPCARD_CONFIRM) ||
            state[CARD_PERF_CONTROL_VIEW] === CARD_PERF_CONTROL__INITIAL;

        return{
            // [CARD_PERF_CONTROL_VIEW]: state[CARD_PERF_CONTROL_VIEW],
            // shouldShow: !shouldNotShow
            shouldNotShow
        };
    },

)(__CardPerformanceControlComponent);

const CustomScoreButtonComponent = function(/*props*/) {
    return (
        <div className='columns'>
            <div className='column'>
                <a
                    href='#custom-score'
                    className='btn btn-block'
                    onClick={() => void 0}>
                    {'Set/Append Custom Score'}
                </a>
            </div>
        </div>
    );
}

const __ReviewScoreCommitComponent = function(props) {

    const {dispatch, showConfirmSkipCard} = props;

    if(showConfirmSkipCard) {

        return (
            <div key='0'>
                <div className='columns'>
                    <div className='column col-6'>
                        <a
                            href='#confirm-skip'
                            className='btn btn-block'
                            onClick={() => void 0}>
                            {'Yes, skip'}
                        </a>
                    </div>
                    <div className='column col-6'>
                        <a
                            href='#cancel-skip'
                            className='btn btn-block btn-primary'
                            onClick={skipCard(dispatch, SKIPCARD_INITIAL)}>
                            {'No, do not skip'}
                        </a>
                    </div>
                </div>
            </div>
        );

    }

    return (
        <div key='1'>
            <CustomScoreButtonComponent />
            <CardPerformanceControlComponent />
            <div className='columns'>
                <div className='column col-9'>
                    <RevealCommitButtonComponent />
                </div>
                <div className='column col-3'>
                    <a
                        href='#skip'
                        className='btn btn-block'
                        onClick={skipCard(dispatch, SKIPCARD_CONFIRM)}>
                        {'Skip Card'}
                    </a>
                </div>
            </div>
        </div>
    );
}
if(process.env.NODE_ENV !== 'production') {
    __ReviewScoreCommitComponent.propTypes = {
        dispatch: React.PropTypes.func,
        showConfirmSkipCard: React.PropTypes.bool,
    };
}

const ReviewScoreCommitComponent = connect(

    // mapStateToProps
    (state) => {
        return{
            showConfirmSkipCard: state[SKIPCARD_VIEW] === SKIPCARD_CONFIRM
        };
    },


)(__ReviewScoreCommitComponent);

const __DISPLAY_NONE = {display: 'none'};
const __CardContentsComponent = function(props) {

    const currentTab = props[TAB];
    const contents = props[currentTab]
    const markdownView = props[MARKDOWN_VIEW];


    let sourceStyles = {
        [TAB_QUESTION]: __DISPLAY_NONE,
        [TAB_ANSWER]: __DISPLAY_NONE,
        [TAB_DESCRIPTION]: __DISPLAY_NONE
    };
    let renderStyle = {};

    switch(markdownView) {
    case MARKDOWN_VIEW_RENDER:
        // no-op
        break;

    case MARKDOWN_VIEW_SOURCE:
    default:

        sourceStyles[currentTab] = {};
        renderStyle.display = 'none';
    }

    return (
        <div>
            <div style={renderStyle}>
                {contents}
            </div>
            <div>
                <MarkdownSource contents={props[TAB_QUESTION]} style={sourceStyles[TAB_QUESTION]} />
                <MarkdownSource contents={props[TAB_ANSWER]} style={sourceStyles[TAB_ANSWER]} />
                <MarkdownSource contents={props[TAB_DESCRIPTION]} style={sourceStyles[TAB_DESCRIPTION]} />
            </div>
        </div>
    );

}

const CardContentsComponent = connect(

    // mapStateToProps
    (state) => {
        const currentCardTab = state[TAB];
        return {

            // current tab
            [TAB]: state[TAB],

            // markdown view of current tab
            [MARKDOWN_VIEW]: state[currentCardTab][MARKDOWN_VIEW],

            // card contents
            [TAB_QUESTION]: state[TAB_QUESTION][CARD_CONTENTS],
            [TAB_ANSWER]: state[TAB_ANSWER][CARD_CONTENTS],
            [TAB_DESCRIPTION]: state[TAB_DESCRIPTION][CARD_CONTENTS],

        };
    }
)(__CardContentsComponent);

const DeckReviewContainer = function(/* props */) {
    return (
        <div>
            <div className='columns'>
                <div className='column'>
                    <CardReviewTabsComponent />
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <RenderSourceComponent />
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <CardContentsComponent />
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <ReviewScoreCommitComponent />
                </div>
            </div>
        </div>
    );
}


/* redux action dispatchers */
// NOTE: FSA compliant

const switchCardPerfControl = function(dispatch, nextCardPerfView) {
    return function(event) {
        event.preventDefault();
        dispatch(
            reduceIn(
                // reducer
                cardPerfReducer,
                // path
                [CARD_PERF_CONTROL_VIEW],
                // action
                {
                    type: nextCardPerfView
                }
            )
        );
    }
}

const skipCard = function(dispatch, nextSkipCardView) {
    return function(event) {
        event.preventDefault();
        dispatch(
            reduceIn(
                // reducer
                skipCardReducer,
                // path
                [SKIPCARD_VIEW],
                // action
                {
                    type: nextSkipCardView
                }
            )
        );
    }
}

const switchTab = function(dispatch, tabType) {
    return function(event) {
        event.preventDefault();
        dispatch(
            reduceIn(
                // reducer
                tabReducer,
                // path
                [TAB],
                // action
                {
                    type: tabType
                }
            )
        );
    }
}

const switchMarkdownView = function(dispatch, target, markdownView) {
    // target: Tab such as question, question, or description
    return function(event) {
        event.preventDefault();
        dispatch(
            reduceIn(
                // reducer
                markdownViewReducer,
                // path
                [target, MARKDOWN_VIEW],
                // action
                {
                    type: markdownView
                }
            )
        );
    }
}


/* redux reducers */

const cardPerfReducer = function(state = CARD_PERF_CONTROL__INITIAL, action) {

    switch(action.type) {
    case CARD_PERF_CONTROL__INITIAL:
    case CARD_PERF_CONTROL__DEFAULT_CHOICES:
        state = action.type;
        break;

    default:
        state = CARD_PERF_CONTROL__INITIAL;
    }

    return state;
}

const skipCardReducer = function(state = SKIPCARD_INITIAL, action) {

    switch(action.type) {
    case SKIPCARD_INITIAL:
    case SKIPCARD_CONFIRM:
        state = action.type;
        break;

    default:
        state = SKIPCARD_INITIAL;
    }

    return state;
}

const tabReducer = function(state = TAB_QUESTION, action) {

    switch(action.type) {
    case TAB_QUESTION:
    case TAB_DESCRIPTION:
        state = action.type;
        break;

    default:
        state = TAB_QUESTION;
    }

    return state;
}

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

    // TODO: check window.location.hash.substr(1)
    [TAB]: TAB_QUESTION,

    [TAB_QUESTION]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_RENDER,
        [CARD_CONTENTS]: ''
    },

    [TAB_ANSWER]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_RENDER,
        [CARD_CONTENTS]: ''
    },

    [TAB_DESCRIPTION]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_RENDER,
        [CARD_CONTENTS]: ''
    },

    [SKIPCARD_VIEW]: SKIPCARD_INITIAL,

    [CARD_PERF_CONTROL_VIEW]: CARD_PERF_CONTROL__INITIAL
};

/* exports */

const rehydrateFactory = require('helpers/hydrate');

module.exports = function() {

    const store = createStore(makeReducer({
        reducer: rehydrateFactory()
    }), initialState);

    const component = (
        <Provider store={store}>
            <DeckReviewContainer />
        </Provider>
    );

    return {
        store,
        component
    };

};

module.exports.initialState = initialState;
