webpackJsonp([0],{

/***/ 0:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	
	var client = __webpack_require__(1);
	
	var maker = __webpack_require__(5);
	
	// TODO: debug
	var lodashMerge = __webpack_require__(103);
	var initialState = lodashMerge({}, maker.initialState);
	initialState.TAB_QUESTION.CARD_CONTENTS = 'question';
	window.__INITIAL_STATE__ = initialState;
	
	client(maker, window.__INITIAL_STATE__, document.getElementById('deck-review-container'));

/***/ },

/***/ 5:
/***/ function(module, exports, __webpack_require__) {

	/* WEBPACK VAR INJECTION */(function(process) {'use strict';
	
	var _defineProperty2 = __webpack_require__(6);
	
	var _defineProperty3 = _interopRequireDefault(_defineProperty2);
	
	var _TAB_QUESTION, _TAB_ANSWER, _TAB_DESCRIPTION, _initialState;
	
	function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }
	
	var React = __webpack_require__(25);
	
	var _require = __webpack_require__(26);
	
	var Provider = _require.Provider;
	var connect = _require.connect;
	
	var _require2 = __webpack_require__(33);
	
	var createStore = _require2.createStore;
	
	var classnames = __webpack_require__(48);
	var TextareaAutosize = __webpack_require__(49).default;
	
	var _require3 = __webpack_require__(51);
	
	var TAB = _require3.TAB;
	var TAB_QUESTION = _require3.TAB_QUESTION;
	var TAB_ANSWER = _require3.TAB_ANSWER;
	var TAB_DESCRIPTION = _require3.TAB_DESCRIPTION;
	var MARKDOWN_VIEW = _require3.MARKDOWN_VIEW;
	var MARKDOWN_VIEW_RENDER = _require3.MARKDOWN_VIEW_RENDER;
	var MARKDOWN_VIEW_SOURCE = _require3.MARKDOWN_VIEW_SOURCE;
	var CARD_CONTENTS = _require3.CARD_CONTENTS;
	var CARD_PERF_CONTROL_VIEW = _require3.CARD_PERF_CONTROL_VIEW;
	var CARD_PERF_CONTROL__INITIAL = _require3.CARD_PERF_CONTROL__INITIAL;
	var CARD_PERF_CONTROL__DEFAULT_CHOICES = _require3.CARD_PERF_CONTROL__DEFAULT_CHOICES;
	var SKIPCARD_VIEW = _require3.SKIPCARD_VIEW;
	var SKIPCARD_INITIAL = _require3.SKIPCARD_INITIAL;
	var SKIPCARD_CONFIRM = _require3.SKIPCARD_CONFIRM;
	
	var _require4 = __webpack_require__(52);
	
	var reduceIn = _require4.reduceIn;
	var makeReducer = _require4.makeReducer;
	
	/* react components */
	
	// TODO: move to its own file for re-use
	
	var __CardReviewTabsComponent = function __CardReviewTabsComponent(props) {
	
	    var tabType = props[TAB];
	    var dispatch = props.dispatch;
	
	
	    return React.createElement(
	        'div',
	        { className: 'btn-group btn-group-block' },
	        React.createElement(
	            'a',
	            {
	                onClick: switchTab(dispatch, TAB_QUESTION),
	                href: '#question',
	                className: classnames('btn', { 'btn-primary': tabType === TAB_QUESTION }) },
	            'Question'
	        ),
	        React.createElement(
	            'a',
	            {
	                onClick: switchTab(dispatch, TAB_DESCRIPTION),
	                href: '#description',
	                className: classnames('btn', { 'btn-primary': tabType === TAB_DESCRIPTION }) },
	            'Description'
	        )
	    );
	};
	if (process.env.NODE_ENV !== 'production') {
	    __CardReviewTabsComponent.propTypes = (0, _defineProperty3.default)({
	        dispatch: React.PropTypes.func
	    }, TAB, React.PropTypes.oneOf([TAB_QUESTION, TAB_DESCRIPTION, TAB_ANSWER]));
	}
	
	var CardReviewTabsComponent = connect(
	// mapStateToProps
	function (state) {
	    return (0, _defineProperty3.default)({}, TAB, state[TAB]);
	})(__CardReviewTabsComponent);
	
	var RenderSourceComponent = connect(
	
	// mapStateToProps
	function (state) {
	    var _ref2;
	
	    var currentCardTab = state[TAB];
	    return _ref2 = {}, (0, _defineProperty3.default)(_ref2, MARKDOWN_VIEW, state[currentCardTab][MARKDOWN_VIEW]), (0, _defineProperty3.default)(_ref2, 'switchTab', function switchTab(dispatch, markdownView) {
	        return switchMarkdownView(dispatch, currentCardTab, markdownView);
	    }), _ref2;
	})(
	
	// mapDispatchToProps
	// (dispatch) => {
	//     return {
	//         switchTab:
	//     };
	// }
	__webpack_require__(176));
	
	var __RevealCommitButtonComponent = function __RevealCommitButtonComponent(props) {
	    var dispatch = props.dispatch;
	
	
	    var perfControlView = props[CARD_PERF_CONTROL_VIEW];
	
	    switch (perfControlView) {
	        case CARD_PERF_CONTROL__INITIAL:
	
	            return React.createElement(
	                'a',
	                {
	                    href: '#confirm-skip',
	                    className: 'btn btn-block',
	                    onClick: switchCardPerfControl(dispatch, CARD_PERF_CONTROL__DEFAULT_CHOICES) },
	                'Reveal Answer'
	            );
	
	        case CARD_PERF_CONTROL__DEFAULT_CHOICES:
	        default:
	
	            return React.createElement(
	                'a',
	                {
	                    href: '#confirm-skip',
	                    className: 'btn btn-block',
	                    onClick: function onClick() {
	                        return void 0;
	                    } },
	                'Commit Score & Next Card'
	            );
	
	    }
	};
	if (process.env.NODE_ENV !== 'production') {
	    __RevealCommitButtonComponent.propTypes = (0, _defineProperty3.default)({
	        dispatch: React.PropTypes.func
	    }, CARD_PERF_CONTROL_VIEW, React.PropTypes.oneOf([CARD_PERF_CONTROL__INITIAL, CARD_PERF_CONTROL__DEFAULT_CHOICES]));
	}
	
	var RevealCommitButtonComponent = connect(
	// mapStateToProps
	function (state) {
	    return (0, _defineProperty3.default)({}, CARD_PERF_CONTROL_VIEW, state[CARD_PERF_CONTROL_VIEW]);
	})(__RevealCommitButtonComponent);
	
	var __CardPerformanceControlComponent = function __CardPerformanceControlComponent(props) {
	    var shouldNotShow = props.shouldNotShow;
	
	
	    if (shouldNotShow) {
	        return null;
	    }
	
	    // switch(perfControlView) {
	    // case CARD_PERF_CONTROL__INITIAL:
	    //     return;
	    // }
	
	    return React.createElement(
	        'div',
	        { className: 'columns' },
	        React.createElement(
	            'div',
	            { className: 'column col-4' },
	            React.createElement(
	                'a',
	                {
	                    href: '#confirm-skip',
	                    className: 'btn btn-block',
	                    onClick: function onClick() {
	                        return void 0;
	                    } },
	                'Fail'
	            )
	        ),
	        React.createElement(
	            'div',
	            { className: 'column col-4' },
	            React.createElement(
	                'a',
	                {
	                    href: '#confirm-skip',
	                    className: 'btn btn-block',
	                    onClick: function onClick() {
	                        return void 0;
	                    } },
	                'Success'
	            )
	        ),
	        React.createElement(
	            'div',
	            { className: 'column col-4' },
	            React.createElement(
	                'a',
	                {
	                    href: '#confirm-skip',
	                    className: 'btn btn-block',
	                    onClick: function onClick() {
	                        return void 0;
	                    } },
	                'Reset Score'
	            )
	        )
	    );
	};
	if (process.env.NODE_ENV !== 'production') {
	    __CardPerformanceControlComponent.propTypes = {
	        shouldNotShow: React.PropTypes.bool
	    };
	}
	
	var CardPerformanceControlComponent = connect(
	// mapStateToProps
	function (state) {
	    var shouldNotShow = state[SKIPCARD_VIEW] === SKIPCARD_CONFIRM || state[CARD_PERF_CONTROL_VIEW] === CARD_PERF_CONTROL__INITIAL;
	
	    return {
	        // [CARD_PERF_CONTROL_VIEW]: state[CARD_PERF_CONTROL_VIEW],
	        // shouldShow: !shouldNotShow
	        shouldNotShow: shouldNotShow
	    };
	})(__CardPerformanceControlComponent);
	
	var CustomScoreButtonComponent = function CustomScoreButtonComponent() /*props*/{
	    return React.createElement(
	        'div',
	        { className: 'columns' },
	        React.createElement(
	            'div',
	            { className: 'column' },
	            React.createElement(
	                'a',
	                {
	                    href: '#custom-score',
	                    className: 'btn btn-block',
	                    onClick: function onClick() {
	                        return void 0;
	                    } },
	                'Set/Append Custom Score'
	            )
	        )
	    );
	};
	
	var __ReviewScoreCommitComponent = function __ReviewScoreCommitComponent(props) {
	    var dispatch = props.dispatch;
	    var showConfirmSkipCard = props.showConfirmSkipCard;
	
	
	    if (showConfirmSkipCard) {
	
	        return React.createElement(
	            'div',
	            { key: '0' },
	            React.createElement(
	                'div',
	                { className: 'columns' },
	                React.createElement(
	                    'div',
	                    { className: 'column col-6' },
	                    React.createElement(
	                        'a',
	                        {
	                            href: '#confirm-skip',
	                            className: 'btn btn-block',
	                            onClick: function onClick() {
	                                return void 0;
	                            } },
	                        'Yes, skip'
	                    )
	                ),
	                React.createElement(
	                    'div',
	                    { className: 'column col-6' },
	                    React.createElement(
	                        'a',
	                        {
	                            href: '#cancel-skip',
	                            className: 'btn btn-block btn-primary',
	                            onClick: skipCard(dispatch, SKIPCARD_INITIAL) },
	                        'No, do not skip'
	                    )
	                )
	            )
	        );
	    }
	
	    return React.createElement(
	        'div',
	        { key: '1' },
	        React.createElement(CustomScoreButtonComponent, null),
	        React.createElement(CardPerformanceControlComponent, null),
	        React.createElement(
	            'div',
	            { className: 'columns' },
	            React.createElement(
	                'div',
	                { className: 'column col-9' },
	                React.createElement(RevealCommitButtonComponent, null)
	            ),
	            React.createElement(
	                'div',
	                { className: 'column col-3' },
	                React.createElement(
	                    'a',
	                    {
	                        href: '#skip',
	                        className: 'btn btn-block',
	                        onClick: skipCard(dispatch, SKIPCARD_CONFIRM) },
	                    'Skip Card'
	                )
	            )
	        )
	    );
	};
	if (process.env.NODE_ENV !== 'production') {
	    __ReviewScoreCommitComponent.propTypes = {
	        dispatch: React.PropTypes.func,
	        showConfirmSkipCard: React.PropTypes.bool
	    };
	}
	
	var ReviewScoreCommitComponent = connect(
	
	// mapStateToProps
	function (state) {
	    return {
	        showConfirmSkipCard: state[SKIPCARD_VIEW] === SKIPCARD_CONFIRM
	    };
	})(__ReviewScoreCommitComponent);
	
	var CardSource = function CardSource(props) {
	    return React.createElement(TextareaAutosize, { style: props.style,
	        key: 'textarea',
	        useCacheForDOMMeasurements: true,
	        minRows: 6,
	        maxRows: 10,
	        className: 'form-input'
	        // id="deck_source"
	        // placeholder={placeholder}
	        // onChange={this.onSourceChange}
	        , value: props.contents,
	        readOnly: true
	    });
	};
	if (process.env.NODE_ENV !== 'production') {
	    CardSource.propTypes = {
	        style: React.PropTypes.object,
	        contents: React.PropTypes.string
	    };
	}
	
	var __DISPLAY_NONE = { display: 'none' };
	var __CardContentsComponent = function __CardContentsComponent(props) {
	    var _sourceStyles;
	
	    var currentTab = props[TAB];
	    var contents = props[currentTab];
	    var markdownView = props[MARKDOWN_VIEW];
	
	    var sourceStyles = (_sourceStyles = {}, (0, _defineProperty3.default)(_sourceStyles, TAB_QUESTION, __DISPLAY_NONE), (0, _defineProperty3.default)(_sourceStyles, TAB_ANSWER, __DISPLAY_NONE), (0, _defineProperty3.default)(_sourceStyles, TAB_DESCRIPTION, __DISPLAY_NONE), _sourceStyles);
	    var renderStyle = {};
	
	    switch (markdownView) {
	        case MARKDOWN_VIEW_RENDER:
	            // no-op
	            break;
	
	        case MARKDOWN_VIEW_SOURCE:
	        default:
	
	            sourceStyles[currentTab] = {};
	            renderStyle.display = 'none';
	    }
	
	    return React.createElement(
	        'div',
	        null,
	        React.createElement(
	            'div',
	            { style: renderStyle },
	            contents
	        ),
	        React.createElement(
	            'div',
	            null,
	            React.createElement(CardSource, { contents: props[TAB_QUESTION], style: sourceStyles[TAB_QUESTION] }),
	            React.createElement(CardSource, { contents: props[TAB_ANSWER], style: sourceStyles[TAB_ANSWER] }),
	            React.createElement(CardSource, { contents: props[TAB_DESCRIPTION], style: sourceStyles[TAB_DESCRIPTION] })
	        )
	    );
	};
	
	var CardContentsComponent = connect(
	
	// mapStateToProps
	function (state) {
	    var _ref4;
	
	    var currentCardTab = state[TAB];
	    return _ref4 = {}, (0, _defineProperty3.default)(_ref4, TAB, state[TAB]), (0, _defineProperty3.default)(_ref4, MARKDOWN_VIEW, state[currentCardTab][MARKDOWN_VIEW]), (0, _defineProperty3.default)(_ref4, TAB_QUESTION, state[TAB_QUESTION][CARD_CONTENTS]), (0, _defineProperty3.default)(_ref4, TAB_ANSWER, state[TAB_ANSWER][CARD_CONTENTS]), (0, _defineProperty3.default)(_ref4, TAB_DESCRIPTION, state[TAB_DESCRIPTION][CARD_CONTENTS]), _ref4;
	})(__CardContentsComponent);
	
	var DeckReviewContainer = function DeckReviewContainer() /* props */{
	    return React.createElement(
	        'div',
	        null,
	        React.createElement(
	            'div',
	            { className: 'columns' },
	            React.createElement(
	                'div',
	                { className: 'column' },
	                React.createElement(CardReviewTabsComponent, null)
	            )
	        ),
	        React.createElement(
	            'div',
	            { className: 'columns' },
	            React.createElement(
	                'div',
	                { className: 'column' },
	                React.createElement(RenderSourceComponent, null)
	            )
	        ),
	        React.createElement(
	            'div',
	            { className: 'columns' },
	            React.createElement(
	                'div',
	                { className: 'column' },
	                React.createElement(CardContentsComponent, null)
	            )
	        ),
	        React.createElement(
	            'div',
	            { className: 'columns' },
	            React.createElement(
	                'div',
	                { className: 'column' },
	                React.createElement(ReviewScoreCommitComponent, null)
	            )
	        )
	    );
	};
	
	/* redux action dispatchers */
	// NOTE: FSA compliant
	
	var switchCardPerfControl = function switchCardPerfControl(dispatch, nextCardPerfView) {
	    return function (event) {
	        event.preventDefault();
	        dispatch(reduceIn(
	        // reducer
	        cardPerfReducer,
	        // path
	        [CARD_PERF_CONTROL_VIEW],
	        // action
	        {
	            type: nextCardPerfView
	        }));
	    };
	};
	
	var skipCard = function skipCard(dispatch, nextSkipCardView) {
	    return function (event) {
	        event.preventDefault();
	        dispatch(reduceIn(
	        // reducer
	        skipCardReducer,
	        // path
	        [SKIPCARD_VIEW],
	        // action
	        {
	            type: nextSkipCardView
	        }));
	    };
	};
	
	var switchTab = function switchTab(dispatch, tabType) {
	    return function (event) {
	        event.preventDefault();
	        dispatch(reduceIn(
	        // reducer
	        tabReducer,
	        // path
	        [TAB],
	        // action
	        {
	            type: tabType
	        }));
	    };
	};
	
	var switchMarkdownView = function switchMarkdownView(dispatch, target, markdownView) {
	    // target: Tab such as question, question, or description
	    return function (event) {
	        event.preventDefault();
	        dispatch(reduceIn(
	        // reducer
	        markdownViewReducer,
	        // path
	        [target, MARKDOWN_VIEW],
	        // action
	        {
	            type: markdownView
	        }));
	    };
	};
	
	/* redux reducers */
	
	var cardPerfReducer = function cardPerfReducer() {
	    var state = arguments.length <= 0 || arguments[0] === undefined ? CARD_PERF_CONTROL__INITIAL : arguments[0];
	    var action = arguments[1];
	
	
	    switch (action.type) {
	        case CARD_PERF_CONTROL__INITIAL:
	        case CARD_PERF_CONTROL__DEFAULT_CHOICES:
	            state = action.type;
	            break;
	
	        default:
	            state = CARD_PERF_CONTROL__INITIAL;
	    }
	
	    return state;
	};
	
	var skipCardReducer = function skipCardReducer() {
	    var state = arguments.length <= 0 || arguments[0] === undefined ? SKIPCARD_INITIAL : arguments[0];
	    var action = arguments[1];
	
	
	    switch (action.type) {
	        case SKIPCARD_INITIAL:
	        case SKIPCARD_CONFIRM:
	            state = action.type;
	            break;
	
	        default:
	            state = SKIPCARD_INITIAL;
	    }
	
	    return state;
	};
	
	var tabReducer = function tabReducer() {
	    var state = arguments.length <= 0 || arguments[0] === undefined ? TAB_QUESTION : arguments[0];
	    var action = arguments[1];
	
	
	    switch (action.type) {
	        case TAB_QUESTION:
	        case TAB_DESCRIPTION:
	            state = action.type;
	            break;
	
	        default:
	            state = TAB_QUESTION;
	    }
	
	    return state;
	};
	
	var markdownViewReducer = function markdownViewReducer() {
	    var state = arguments.length <= 0 || arguments[0] === undefined ? MARKDOWN_VIEW_RENDER : arguments[0];
	    var action = arguments[1];
	
	
	    switch (action.type) {
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
	
	var initialState = (_initialState = {}, (0, _defineProperty3.default)(_initialState, TAB, TAB_QUESTION), (0, _defineProperty3.default)(_initialState, TAB_QUESTION, (_TAB_QUESTION = {}, (0, _defineProperty3.default)(_TAB_QUESTION, MARKDOWN_VIEW, MARKDOWN_VIEW_RENDER), (0, _defineProperty3.default)(_TAB_QUESTION, CARD_CONTENTS, ''), _TAB_QUESTION)), (0, _defineProperty3.default)(_initialState, TAB_ANSWER, (_TAB_ANSWER = {}, (0, _defineProperty3.default)(_TAB_ANSWER, MARKDOWN_VIEW, MARKDOWN_VIEW_RENDER), (0, _defineProperty3.default)(_TAB_ANSWER, CARD_CONTENTS, ''), _TAB_ANSWER)), (0, _defineProperty3.default)(_initialState, TAB_DESCRIPTION, (_TAB_DESCRIPTION = {}, (0, _defineProperty3.default)(_TAB_DESCRIPTION, MARKDOWN_VIEW, MARKDOWN_VIEW_RENDER), (0, _defineProperty3.default)(_TAB_DESCRIPTION, CARD_CONTENTS, ''), _TAB_DESCRIPTION)), (0, _defineProperty3.default)(_initialState, SKIPCARD_VIEW, SKIPCARD_INITIAL), (0, _defineProperty3.default)(_initialState, CARD_PERF_CONTROL_VIEW, CARD_PERF_CONTROL__INITIAL), _initialState);
	
	/* exports */
	
	var rehydrate = __webpack_require__(3);
	
	module.exports = function () {
	
	    var store = createStore(makeReducer({
	        reducer: rehydrate
	    }), initialState);
	
	    var component = React.createElement(
	        Provider,
	        { store: store },
	        React.createElement(DeckReviewContainer, null)
	    );
	
	    return {
	        store: store,
	        component: component
	    };
	};
	
	module.exports.initialState = initialState;
	/* WEBPACK VAR INJECTION */}.call(exports, __webpack_require__(4)))

/***/ }

});
//# sourceMappingURL=deck_review.js.map