webpackJsonp([1],{

/***/ 0:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	
	var client = __webpack_require__(1);
	
	var maker = __webpack_require__(177);
	
	// TODO: debug
	// const initialState = require('lodash').merge({}, maker.initialState);
	// initialState.TAB_QUESTION.CARD_CONTENTS = 'question';
	// window.__INITIAL_STATE__ = initialState;
	
	client(maker, window.__INITIAL_STATE__, document.getElementById('new-deck-container'));

/***/ },

/***/ 177:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	
	var _defineProperty2 = __webpack_require__(6);
	
	var _defineProperty3 = _interopRequireDefault(_defineProperty2);
	
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
	
	var MARKDOWN_VIEW = _require3.MARKDOWN_VIEW;
	var MARKDOWN_VIEW_RENDER = _require3.MARKDOWN_VIEW_RENDER;
	var MARKDOWN_VIEW_SOURCE = _require3.MARKDOWN_VIEW_SOURCE;
	
	var _require4 = __webpack_require__(52);
	
	var reduceIn = _require4.reduceIn;
	var makeReducer = _require4.makeReducer;
	
	/* react components */
	
	var RenderSourceComponent = connect(
	
	// mapStateToProps
	function (state) {
	    var _ref;
	
	    return _ref = {}, (0, _defineProperty3.default)(_ref, MARKDOWN_VIEW, state[MARKDOWN_VIEW]), (0, _defineProperty3.default)(_ref, 'switchTab', function switchTab(dispatch, markdownView) {
	        return switchMarkdownView(dispatch, markdownView);
	    }), _ref;
	})(
	
	// mapDispatchToProps
	// (dispatch) => {
	//     return {
	//         switchTab:
	//     };
	// }
	__webpack_require__(176));
	
	var newDeckStyle = {
	    marginTop: 0,
	    marginBottom: 0
	};
	var NewDeckContainer = function NewDeckContainer() /* props */{
	    return React.createElement(
	        'div',
	        null,
	        React.createElement(
	            'div',
	            { className: 'columns' },
	            React.createElement(
	                'div',
	                { className: 'column' },
	                React.createElement(
	                    'h5',
	                    { style: newDeckStyle },
	                    'New Deck'
	                )
	            )
	        ),
	        React.createElement(
	            'div',
	            { className: 'columns' },
	            React.createElement(
	                'div',
	                { className: 'column' },
	                React.createElement(
	                    'div',
	                    { className: 'form-group' },
	                    React.createElement(
	                        'label',
	                        { className: 'form-label', htmlFor: 'input-deck-name' },
	                        'Name'
	                    ),
	                    React.createElement('input', {
	                        className: 'form-input',
	                        type: 'text',
	                        id: 'input-deck-name',
	                        placeholder: 'Name for new deck'
	                    })
	                )
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
	        )
	    );
	};
	
	/* redux action dispatchers */
	// NOTE: FSA compliant
	
	var switchMarkdownView = function switchMarkdownView(dispatch, markdownView) {
	    return function (event) {
	        event.preventDefault();
	        dispatch(reduceIn(
	        // reducer
	        markdownViewReducer,
	        // path
	        [MARKDOWN_VIEW],
	        // action
	        {
	            type: markdownView
	        }));
	    };
	};
	
	/* redux reducers */
	
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
	
	var initialState = (0, _defineProperty3.default)({}, MARKDOWN_VIEW, MARKDOWN_VIEW_SOURCE);
	
	/* exports */
	
	var rehydrate = __webpack_require__(3);
	
	module.exports = function () {
	
	    var store = createStore(makeReducer({
	        reducer: rehydrate
	    }), initialState);
	
	    var component = React.createElement(
	        Provider,
	        { store: store },
	        React.createElement(NewDeckContainer, null)
	    );
	
	    return {
	        store: store,
	        component: component
	    };
	};
	
	module.exports.initialState = initialState;

/***/ }

});
//# sourceMappingURL=new_deck.js.map