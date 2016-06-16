webpackJsonp([0],{

/***/ 0:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	
	var _defineProperty2 = __webpack_require__(1);
	
	var _defineProperty3 = _interopRequireDefault(_defineProperty2);
	
	var _initialState;
	
	function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }
	
	// TODO: this is fucking huge
	// require('babel-polyfill');
	
	var React = __webpack_require__(20);
	var ReactDOM = __webpack_require__(57);
	
	var _require = __webpack_require__(187);
	
	var Provider = _require.Provider;
	var connect = _require.connect;
	
	var _require2 = __webpack_require__(194);
	
	var createStore = _require2.createStore;
	
	var classnames = __webpack_require__(209);
	// const _ = require('lodash');
	// const {combineReducers} = require('redux');
	
	// const combineReducers = require('./combineReducers');
	// NOTE: above won't work; below is used to be in parity with redux upstream
	// import combineReducers from './combineReducers';
	
	var _require3 = __webpack_require__(212);
	
	var /*serializeProps, deserializeProps,*/genKey = _require3.genKey;
	
	var _require4 = __webpack_require__(213);
	
	var applyReducer = _require4.applyReducer;
	var treeReducer = _require4.reducer;
	
	/* react components */
	
	// const styleA = {
	//     marginTop: 0,
	//     marginBottom: 0
	// };
	
	// TODO: move to its own file
	// const CardTitleReviewComponent = React.createClass({
	//     render: function() {
	//         return (
	//             <h5 style={styleA} className='text-break'>
	//                 <small className='label'>{'Reviewing'}</small>
	//                 {' '}
	//                 {'What does the fox say?'}
	//             </h5>
	//         );
	//     }
	// });
	
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
	                href: '#',
	                className: classnames('btn', { 'btn-primary': tabType === TAB_QUESTION }) },
	            'Question'
	        ),
	        React.createElement(
	            'a',
	            {
	                onClick: switchTab(dispatch, TAB_DESCRIPTION),
	                href: '#',
	                className: classnames('btn', { 'btn-primary': tabType === TAB_DESCRIPTION }) },
	            'Description'
	        )
	    );
	};
	
	// eslint-disable-next-line camelcase
	var __CardReviewTabsComponent__mapStateToProps = function __CardReviewTabsComponent__mapStateToProps(state) {
	    return (0, _defineProperty3.default)({}, TAB, state[TAB]);
	};
	
	var CardReviewTabsComponent = connect(__CardReviewTabsComponent__mapStateToProps)(__CardReviewTabsComponent);
	
	var __RenderSourceComponent = function __RenderSourceComponent(props) {
	    var dispatch = props.dispatch;
	    var switchTab = props.switchTab;
	
	    var currentTab = props[MARKDOWN_VIEW];
	
	    return React.createElement(
	        'ul',
	        { className: 'tab' },
	        React.createElement(
	            'li',
	            { className: classnames('tab-item', { 'active': currentTab === MARKDOWN_VIEW_RENDER }) },
	            React.createElement(
	                'a',
	                {
	                    href: '#',
	                    onClick: switchTab(dispatch, MARKDOWN_VIEW_RENDER) },
	                'Render'
	            )
	        ),
	        React.createElement(
	            'li',
	            { className: classnames('tab-item', { 'active': currentTab === MARKDOWN_VIEW_SOURCE }) },
	            React.createElement(
	                'a',
	                {
	                    href: '#',
	                    onClick: switchTab(dispatch, MARKDOWN_VIEW_SOURCE) },
	                'Source'
	            )
	        )
	    );
	};
	
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
	__RenderSourceComponent);
	
	var ReviewScoreCommitComponent = function ReviewScoreCommitComponent() /* props */{
	    return React.createElement(
	        'div',
	        { className: 'columns' },
	        React.createElement(
	            'div',
	            { className: 'column col-9' },
	            React.createElement(
	                'a',
	                { href: '#', className: 'btn btn-block' },
	                'Reveal Answer'
	            )
	        ),
	        React.createElement(
	            'div',
	            { className: 'column col-3' },
	            React.createElement(
	                'a',
	                { href: '#', className: 'btn btn-block' },
	                'Skip Card'
	            )
	        )
	    );
	};
	
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
	                React.createElement(ReviewScoreCommitComponent, null)
	            )
	        )
	    );
	};
	
	/* constants */
	
	/*
	enum Tab {
	    Question,
	    Answer,
	    Description
	}
	 */
	var TAB = genKey();
	var TAB_QUESTION = genKey();
	var TAB_ANSWER = genKey();
	var TAB_DESCRIPTION = genKey();
	
	/*
	enum MarkdownView {
	    Render,
	    Source
	}
	 */
	var MARKDOWN_VIEW = genKey();
	var MARKDOWN_VIEW_RENDER = genKey();
	var MARKDOWN_VIEW_SOURCE = genKey();
	
	/* redux action creators */
	// NOTE: FSA compliant
	
	var switchTab = function switchTab(dispatch, tabType) {
	    return function (event) {
	        event.preventDefault();
	        dispatch(applyReducer(
	        // path
	        [TAB],
	        // reducer
	        tabReducer,
	        // action
	        {
	            type: tabType
	        }));
	    };
	};
	
	var switchMarkdownView = function switchMarkdownView(dispatch, target, markdownView) {
	    // target: Tab
	    return function (event) {
	        event.preventDefault();
	        dispatch(applyReducer(
	        // path
	        [target, MARKDOWN_VIEW],
	        // reducer
	        markdownViewReducer,
	        // action
	        {
	            type: markdownView
	        }));
	    };
	};
	
	/* redux reducers */
	
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
	
	/* redux store */
	
	var initialState = (_initialState = {}, (0, _defineProperty3.default)(_initialState, TAB, TAB_QUESTION), (0, _defineProperty3.default)(_initialState, TAB_QUESTION, (0, _defineProperty3.default)({}, MARKDOWN_VIEW, MARKDOWN_VIEW_RENDER)), (0, _defineProperty3.default)(_initialState, TAB_ANSWER, (0, _defineProperty3.default)({}, MARKDOWN_VIEW, MARKDOWN_VIEW_RENDER)), (0, _defineProperty3.default)(_initialState, TAB_DESCRIPTION, (0, _defineProperty3.default)({}, MARKDOWN_VIEW, MARKDOWN_VIEW_RENDER)), _initialState);
	
	var store = createStore(treeReducer, initialState);
	
	/* start */
	
	ReactDOM.render(React.createElement(
	    Provider,
	    { store: store },
	    React.createElement(DeckReviewContainer, null)
	), document.getElementById('deck-review-container'));

/***/ },

/***/ 1:
/***/ function(module, exports, __webpack_require__) {

	"use strict";
	
	exports.__esModule = true;
	
	var _defineProperty = __webpack_require__(2);
	
	var _defineProperty2 = _interopRequireDefault(_defineProperty);
	
	function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }
	
	exports.default = function (obj, key, value) {
	  if (key in obj) {
	    (0, _defineProperty2.default)(obj, key, {
	      value: value,
	      enumerable: true,
	      configurable: true,
	      writable: true
	    });
	  } else {
	    obj[key] = value;
	  }
	
	  return obj;
	};

/***/ },

/***/ 2:
/***/ function(module, exports, __webpack_require__) {

	module.exports = { "default": __webpack_require__(3), __esModule: true };

/***/ },

/***/ 3:
/***/ function(module, exports, __webpack_require__) {

	__webpack_require__(4);
	var $Object = __webpack_require__(7).Object;
	module.exports = function defineProperty(it, key, desc){
	  return $Object.defineProperty(it, key, desc);
	};

/***/ },

/***/ 212:
/***/ function(module, exports) {

	'use strict';
	
	// alternative to Symbol()
	var _objKey = 1;
	var genKey = function genKey() {
	    return '__genKey_' + String(_objKey++);
	};
	
	module.exports = {
	    genKey: genKey
	};

/***/ },

/***/ 213:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	
	var _ = __webpack_require__(210);
	
	var APPLY_REDUCER = 'APPLY_REDUCER';
	var EMPTY_OBJ = {};
	
	var treeReducer = function treeReducer() {
	    var state = arguments.length <= 0 || arguments[0] === undefined ? EMPTY_OBJ : arguments[0];
	    var action = arguments[1];
	
	
	    var patch = EMPTY_OBJ;
	
	    switch (action.type) {
	
	        case APPLY_REDUCER:
	            var _action$payload = action.payload;
	            var path = _action$payload.path;
	            var reducer = _action$payload.reducer;
	
	            // TODO: path is array
	            // TODO: check reducer is function
	
	            var specificState = _.get(state, path);
	
	            // TODO: _.has(state, path);
	
	            var result = reducer(specificState, action.payload.action);
	            patch = _.set({}, path, result);
	            break;
	
	        default:
	            return state;
	    }
	
	    // console.log('patched');
	    return _.assign({}, state, patch);
	    // return {...state, ...patch};
	};
	
	var applyReducer = function applyReducer(path, reducer, action) {
	    return {
	        type: APPLY_REDUCER,
	        payload: {
	            path: path, // array
	            reducer: reducer, // redux compatible reducer
	            action: action // action to be applied to given reducer
	        }
	    };
	};
	
	module.exports = {
	    reducer: treeReducer,
	    applyReducer: applyReducer
	};

/***/ }

});
//# sourceMappingURL=deck_review.js.map