webpackJsonp([3],{

/***/ 0:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	
	var client = __webpack_require__(737);
	
	if (true) {
	    var invariant = __webpack_require__(680);
	    invariant(window.__PRE_RENDER_STATE__, 'we expect to consume window.__PRE_RENDER_STATE__');
	    invariant(!window.__POST_RENDER_STATE__, 'we do not expect to consume window.__POST_RENDER_STATE__');
	}
	
	/* delete component */
	
	var cardSettingsDeleteMaker = __webpack_require__(913);
	
	var preRenderStateDelete = window.__PRE_RENDER_STATE__;
	var postRenderStateDelete = cardSettingsDeleteMaker.initialState;
	
	client(cardSettingsDeleteMaker, preRenderStateDelete, postRenderStateDelete, document.getElementById('card_settings_main_delete_container'));

/***/ },

/***/ 412:
/***/ function(module, exports, __webpack_require__) {

	"use strict";
	
	exports.__esModule = true;
	
	var _defineProperty = __webpack_require__(413);
	
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

/***/ 413:
/***/ function(module, exports, __webpack_require__) {

	module.exports = { "default": __webpack_require__(414), __esModule: true };

/***/ },

/***/ 414:
/***/ function(module, exports, __webpack_require__) {

	__webpack_require__(415);
	var $Object = __webpack_require__(418).Object;
	module.exports = function defineProperty(it, key, desc){
	  return $Object.defineProperty(it, key, desc);
	};

/***/ },

/***/ 415:
[955, 416, 426, 422],

/***/ 416:
/***/ function(module, exports, __webpack_require__) {

	var global    = __webpack_require__(417)
	  , core      = __webpack_require__(418)
	  , ctx       = __webpack_require__(419)
	  , hide      = __webpack_require__(421)
	  , PROTOTYPE = 'prototype';
	
	var $export = function(type, name, source){
	  var IS_FORCED = type & $export.F
	    , IS_GLOBAL = type & $export.G
	    , IS_STATIC = type & $export.S
	    , IS_PROTO  = type & $export.P
	    , IS_BIND   = type & $export.B
	    , IS_WRAP   = type & $export.W
	    , exports   = IS_GLOBAL ? core : core[name] || (core[name] = {})
	    , expProto  = exports[PROTOTYPE]
	    , target    = IS_GLOBAL ? global : IS_STATIC ? global[name] : (global[name] || {})[PROTOTYPE]
	    , key, own, out;
	  if(IS_GLOBAL)source = name;
	  for(key in source){
	    // contains in native
	    own = !IS_FORCED && target && target[key] !== undefined;
	    if(own && key in exports)continue;
	    // export native or passed
	    out = own ? target[key] : source[key];
	    // prevent global pollution for namespaces
	    exports[key] = IS_GLOBAL && typeof target[key] != 'function' ? source[key]
	    // bind timers to global for call from export context
	    : IS_BIND && own ? ctx(out, global)
	    // wrap global constructors for prevent change them in library
	    : IS_WRAP && target[key] == out ? (function(C){
	      var F = function(a, b, c){
	        if(this instanceof C){
	          switch(arguments.length){
	            case 0: return new C;
	            case 1: return new C(a);
	            case 2: return new C(a, b);
	          } return new C(a, b, c);
	        } return C.apply(this, arguments);
	      };
	      F[PROTOTYPE] = C[PROTOTYPE];
	      return F;
	    // make static versions for prototype methods
	    })(out) : IS_PROTO && typeof out == 'function' ? ctx(Function.call, out) : out;
	    // export proto methods to core.%CONSTRUCTOR%.methods.%NAME%
	    if(IS_PROTO){
	      (exports.virtual || (exports.virtual = {}))[key] = out;
	      // export proto methods to core.%CONSTRUCTOR%.prototype.%NAME%
	      if(type & $export.R && expProto && !expProto[key])hide(expProto, key, out);
	    }
	  }
	};
	// type bitmap
	$export.F = 1;   // forced
	$export.G = 2;   // global
	$export.S = 4;   // static
	$export.P = 8;   // proto
	$export.B = 16;  // bind
	$export.W = 32;  // wrap
	$export.U = 64;  // safe
	$export.R = 128; // real proto method for `library` 
	module.exports = $export;

/***/ },

/***/ 417:
5,

/***/ 418:
10,

/***/ 419:
[930, 420],

/***/ 420:
22,

/***/ 421:
[924, 422, 430, 426],

/***/ 422:
[925, 423, 425, 429, 426],

/***/ 423:
[926, 424],

/***/ 424:
14,

/***/ 425:
[927, 426, 427, 428],

/***/ 426:
[923, 427],

/***/ 427:
8,

/***/ 428:
[928, 424, 417],

/***/ 429:
[929, 424],

/***/ 430:
18,

/***/ 431:
/***/ function(module, exports, __webpack_require__) {

	"use strict";
	
	exports.__esModule = true;
	
	var _isIterable2 = __webpack_require__(432);
	
	var _isIterable3 = _interopRequireDefault(_isIterable2);
	
	var _getIterator2 = __webpack_require__(469);
	
	var _getIterator3 = _interopRequireDefault(_getIterator2);
	
	function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }
	
	exports.default = function () {
	  function sliceIterator(arr, i) {
	    var _arr = [];
	    var _n = true;
	    var _d = false;
	    var _e = undefined;
	
	    try {
	      for (var _i = (0, _getIterator3.default)(arr), _s; !(_n = (_s = _i.next()).done); _n = true) {
	        _arr.push(_s.value);
	
	        if (i && _arr.length === i) break;
	      }
	    } catch (err) {
	      _d = true;
	      _e = err;
	    } finally {
	      try {
	        if (!_n && _i["return"]) _i["return"]();
	      } finally {
	        if (_d) throw _e;
	      }
	    }
	
	    return _arr;
	  }
	
	  return function (arr, i) {
	    if (Array.isArray(arr)) {
	      return arr;
	    } else if ((0, _isIterable3.default)(Object(arr))) {
	      return sliceIterator(arr, i);
	    } else {
	      throw new TypeError("Invalid attempt to destructure non-iterable instance");
	    }
	  };
	}();

/***/ },

/***/ 432:
/***/ function(module, exports, __webpack_require__) {

	module.exports = { "default": __webpack_require__(433), __esModule: true };

/***/ },

/***/ 433:
/***/ function(module, exports, __webpack_require__) {

	__webpack_require__(434);
	__webpack_require__(465);
	module.exports = __webpack_require__(467);

/***/ },

/***/ 434:
/***/ function(module, exports, __webpack_require__) {

	__webpack_require__(435);
	var global        = __webpack_require__(417)
	  , hide          = __webpack_require__(421)
	  , Iterators     = __webpack_require__(438)
	  , TO_STRING_TAG = __webpack_require__(462)('toStringTag');
	
	for(var collections = ['NodeList', 'DOMTokenList', 'MediaList', 'StyleSheetList', 'CSSRuleList'], i = 0; i < 5; i++){
	  var NAME       = collections[i]
	    , Collection = global[NAME]
	    , proto      = Collection && Collection.prototype;
	  if(proto && !proto[TO_STRING_TAG])hide(proto, TO_STRING_TAG, NAME);
	  Iterators[NAME] = Iterators.Array;
	}

/***/ },

/***/ 435:
[973, 436, 437, 438, 439, 443],

/***/ 436:
/***/ function(module, exports) {

	module.exports = function(){ /* empty */ };

/***/ },

/***/ 437:
195,

/***/ 438:
130,

/***/ 439:
[940, 440, 442],

/***/ 440:
[941, 441],

/***/ 441:
35,

/***/ 442:
36,

/***/ 443:
[967, 444, 416, 445, 421, 446, 438, 447, 461, 463, 462],

/***/ 444:
/***/ function(module, exports) {

	module.exports = true;

/***/ },

/***/ 445:
/***/ function(module, exports, __webpack_require__) {

	module.exports = __webpack_require__(421);

/***/ },

/***/ 446:
6,

/***/ 447:
[968, 448, 430, 461, 421, 462],

/***/ 448:
[948, 423, 449, 459, 456, 428, 460],

/***/ 449:
[949, 422, 423, 450, 426],

/***/ 450:
[938, 451, 459],

/***/ 451:
[939, 446, 439, 452, 456],

/***/ 452:
[942, 439, 453, 455],

/***/ 453:
[943, 454],

/***/ 454:
39,

/***/ 455:
[944, 454],

/***/ 456:
[945, 457, 458],

/***/ 457:
[932, 417],

/***/ 458:
20,

/***/ 459:
42,

/***/ 460:
[950, 417],

/***/ 461:
[933, 422, 446, 462],

/***/ 462:
[934, 457, 458, 417],

/***/ 463:
[959, 446, 464, 456],

/***/ 464:
[958, 442],

/***/ 465:
[965, 466, 443],

/***/ 466:
[966, 454, 442],

/***/ 467:
/***/ function(module, exports, __webpack_require__) {

	var classof   = __webpack_require__(468)
	  , ITERATOR  = __webpack_require__(462)('iterator')
	  , Iterators = __webpack_require__(438);
	module.exports = __webpack_require__(418).isIterable = function(it){
	  var O = Object(it);
	  return O[ITERATOR] !== undefined
	    || '@@iterator' in O
	    || Iterators.hasOwnProperty(classof(O));
	};

/***/ },

/***/ 468:
[964, 441, 462],

/***/ 469:
/***/ function(module, exports, __webpack_require__) {

	module.exports = { "default": __webpack_require__(470), __esModule: true };

/***/ },

/***/ 470:
/***/ function(module, exports, __webpack_require__) {

	__webpack_require__(434);
	__webpack_require__(465);
	module.exports = __webpack_require__(471);

/***/ },

/***/ 471:
/***/ function(module, exports, __webpack_require__) {

	var anObject = __webpack_require__(423)
	  , get      = __webpack_require__(472);
	module.exports = __webpack_require__(418).getIterator = function(it){
	  var iterFn = get(it);
	  if(typeof iterFn != 'function')throw TypeError(it + ' is not iterable!');
	  return anObject(iterFn.call(it));
	};

/***/ },

/***/ 472:
[971, 468, 462, 438, 418],

/***/ 473:
/***/ function(module, exports, __webpack_require__) {

	module.exports = { "default": __webpack_require__(474), __esModule: true };

/***/ },

/***/ 474:
/***/ function(module, exports, __webpack_require__) {

	__webpack_require__(475);
	__webpack_require__(465);
	__webpack_require__(434);
	__webpack_require__(476);
	module.exports = __webpack_require__(418).Promise;

/***/ },

/***/ 475:
/***/ function(module, exports) {



/***/ },

/***/ 476:
[974, 444, 417, 419, 468, 416, 424, 420, 477, 478, 481, 482, 484, 462, 485, 461, 486, 418, 487],

/***/ 477:
206,

/***/ 478:
[975, 419, 479, 480, 423, 453, 472],

/***/ 479:
[969, 423],

/***/ 480:
[970, 438, 462],

/***/ 481:
[976, 423, 420, 462],

/***/ 482:
[977, 419, 483, 460, 428, 417, 441],

/***/ 483:
79,

/***/ 484:
[978, 417, 482, 441],

/***/ 485:
/***/ function(module, exports, __webpack_require__) {

	var hide = __webpack_require__(421);
	module.exports = function(target, src, safe){
	  for(var key in src){
	    if(safe && target[key])target[key] = src[key];
	    else hide(target, key, src[key]);
	  } return target;
	};

/***/ },

/***/ 486:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	var global      = __webpack_require__(417)
	  , core        = __webpack_require__(418)
	  , dP          = __webpack_require__(422)
	  , DESCRIPTORS = __webpack_require__(426)
	  , SPECIES     = __webpack_require__(462)('species');
	
	module.exports = function(KEY){
	  var C = typeof core[KEY] == 'function' ? core[KEY] : global[KEY];
	  if(DESCRIPTORS && C && !C[SPECIES])dP.f(C, SPECIES, {
	    configurable: true,
	    get: function(){ return this; }
	  });
	};

/***/ },

/***/ 487:
[972, 462],

/***/ 660:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	
	exports.__esModule = true;
	exports.connect = exports.Provider = undefined;
	
	var _Provider = __webpack_require__(661);
	
	var _Provider2 = _interopRequireDefault(_Provider);
	
	var _connect = __webpack_require__(664);
	
	var _connect2 = _interopRequireDefault(_connect);
	
	function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { "default": obj }; }
	
	exports.Provider = _Provider2["default"];
	exports.connect = _connect2["default"];

/***/ },

/***/ 661:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	
	exports.__esModule = true;
	exports["default"] = undefined;
	
	var _react = __webpack_require__(490);
	
	var _storeShape = __webpack_require__(662);
	
	var _storeShape2 = _interopRequireDefault(_storeShape);
	
	var _warning = __webpack_require__(663);
	
	var _warning2 = _interopRequireDefault(_warning);
	
	function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { "default": obj }; }
	
	function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }
	
	function _possibleConstructorReturn(self, call) { if (!self) { throw new ReferenceError("this hasn't been initialised - super() hasn't been called"); } return call && (typeof call === "object" || typeof call === "function") ? call : self; }
	
	function _inherits(subClass, superClass) { if (typeof superClass !== "function" && superClass !== null) { throw new TypeError("Super expression must either be null or a function, not " + typeof superClass); } subClass.prototype = Object.create(superClass && superClass.prototype, { constructor: { value: subClass, enumerable: false, writable: true, configurable: true } }); if (superClass) Object.setPrototypeOf ? Object.setPrototypeOf(subClass, superClass) : subClass.__proto__ = superClass; }
	
	var didWarnAboutReceivingStore = false;
	function warnAboutReceivingStore() {
	  if (didWarnAboutReceivingStore) {
	    return;
	  }
	  didWarnAboutReceivingStore = true;
	
	  (0, _warning2["default"])('<Provider> does not support changing `store` on the fly. ' + 'It is most likely that you see this error because you updated to ' + 'Redux 2.x and React Redux 2.x which no longer hot reload reducers ' + 'automatically. See https://github.com/reactjs/react-redux/releases/' + 'tag/v2.0.0 for the migration instructions.');
	}
	
	var Provider = function (_Component) {
	  _inherits(Provider, _Component);
	
	  Provider.prototype.getChildContext = function getChildContext() {
	    return { store: this.store };
	  };
	
	  function Provider(props, context) {
	    _classCallCheck(this, Provider);
	
	    var _this = _possibleConstructorReturn(this, _Component.call(this, props, context));
	
	    _this.store = props.store;
	    return _this;
	  }
	
	  Provider.prototype.render = function render() {
	    var children = this.props.children;
	
	    return _react.Children.only(children);
	  };
	
	  return Provider;
	}(_react.Component);
	
	exports["default"] = Provider;
	
	if (true) {
	  Provider.prototype.componentWillReceiveProps = function (nextProps) {
	    var store = this.store;
	    var nextStore = nextProps.store;
	
	    if (store !== nextStore) {
	      warnAboutReceivingStore();
	    }
	  };
	}
	
	Provider.propTypes = {
	  store: _storeShape2["default"].isRequired,
	  children: _react.PropTypes.element.isRequired
	};
	Provider.childContextTypes = {
	  store: _storeShape2["default"].isRequired
	};

/***/ },

/***/ 662:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	
	exports.__esModule = true;
	
	var _react = __webpack_require__(490);
	
	exports["default"] = _react.PropTypes.shape({
	  subscribe: _react.PropTypes.func.isRequired,
	  dispatch: _react.PropTypes.func.isRequired,
	  getState: _react.PropTypes.func.isRequired
	});

/***/ },

/***/ 663:
/***/ function(module, exports) {

	'use strict';
	
	exports.__esModule = true;
	exports["default"] = warning;
	/**
	 * Prints a warning in the console if it exists.
	 *
	 * @param {String} message The warning message.
	 * @returns {void}
	 */
	function warning(message) {
	  /* eslint-disable no-console */
	  if (typeof console !== 'undefined' && typeof console.error === 'function') {
	    console.error(message);
	  }
	  /* eslint-enable no-console */
	  try {
	    // This error was thrown as a convenience so that you can use this stack
	    // to find the callsite that caused this warning to fire.
	    throw new Error(message);
	    /* eslint-disable no-empty */
	  } catch (e) {}
	  /* eslint-enable no-empty */
	}

/***/ },

/***/ 664:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	
	var _extends = Object.assign || function (target) { for (var i = 1; i < arguments.length; i++) { var source = arguments[i]; for (var key in source) { if (Object.prototype.hasOwnProperty.call(source, key)) { target[key] = source[key]; } } } return target; };
	
	exports.__esModule = true;
	exports["default"] = connect;
	
	var _react = __webpack_require__(490);
	
	var _storeShape = __webpack_require__(662);
	
	var _storeShape2 = _interopRequireDefault(_storeShape);
	
	var _shallowEqual = __webpack_require__(665);
	
	var _shallowEqual2 = _interopRequireDefault(_shallowEqual);
	
	var _wrapActionCreators = __webpack_require__(666);
	
	var _wrapActionCreators2 = _interopRequireDefault(_wrapActionCreators);
	
	var _warning = __webpack_require__(663);
	
	var _warning2 = _interopRequireDefault(_warning);
	
	var _isPlainObject = __webpack_require__(669);
	
	var _isPlainObject2 = _interopRequireDefault(_isPlainObject);
	
	var _hoistNonReactStatics = __webpack_require__(679);
	
	var _hoistNonReactStatics2 = _interopRequireDefault(_hoistNonReactStatics);
	
	var _invariant = __webpack_require__(680);
	
	var _invariant2 = _interopRequireDefault(_invariant);
	
	function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { "default": obj }; }
	
	function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }
	
	function _possibleConstructorReturn(self, call) { if (!self) { throw new ReferenceError("this hasn't been initialised - super() hasn't been called"); } return call && (typeof call === "object" || typeof call === "function") ? call : self; }
	
	function _inherits(subClass, superClass) { if (typeof superClass !== "function" && superClass !== null) { throw new TypeError("Super expression must either be null or a function, not " + typeof superClass); } subClass.prototype = Object.create(superClass && superClass.prototype, { constructor: { value: subClass, enumerable: false, writable: true, configurable: true } }); if (superClass) Object.setPrototypeOf ? Object.setPrototypeOf(subClass, superClass) : subClass.__proto__ = superClass; }
	
	var defaultMapStateToProps = function defaultMapStateToProps(state) {
	  return {};
	}; // eslint-disable-line no-unused-vars
	var defaultMapDispatchToProps = function defaultMapDispatchToProps(dispatch) {
	  return { dispatch: dispatch };
	};
	var defaultMergeProps = function defaultMergeProps(stateProps, dispatchProps, parentProps) {
	  return _extends({}, parentProps, stateProps, dispatchProps);
	};
	
	function getDisplayName(WrappedComponent) {
	  return WrappedComponent.displayName || WrappedComponent.name || 'Component';
	}
	
	var errorObject = { value: null };
	function tryCatch(fn, ctx) {
	  try {
	    return fn.apply(ctx);
	  } catch (e) {
	    errorObject.value = e;
	    return errorObject;
	  }
	}
	
	// Helps track hot reloading.
	var nextVersion = 0;
	
	function connect(mapStateToProps, mapDispatchToProps, mergeProps) {
	  var options = arguments.length <= 3 || arguments[3] === undefined ? {} : arguments[3];
	
	  var shouldSubscribe = Boolean(mapStateToProps);
	  var mapState = mapStateToProps || defaultMapStateToProps;
	
	  var mapDispatch = undefined;
	  if (typeof mapDispatchToProps === 'function') {
	    mapDispatch = mapDispatchToProps;
	  } else if (!mapDispatchToProps) {
	    mapDispatch = defaultMapDispatchToProps;
	  } else {
	    mapDispatch = (0, _wrapActionCreators2["default"])(mapDispatchToProps);
	  }
	
	  var finalMergeProps = mergeProps || defaultMergeProps;
	  var _options$pure = options.pure;
	  var pure = _options$pure === undefined ? true : _options$pure;
	  var _options$withRef = options.withRef;
	  var withRef = _options$withRef === undefined ? false : _options$withRef;
	
	  var checkMergedEquals = pure && finalMergeProps !== defaultMergeProps;
	
	  // Helps track hot reloading.
	  var version = nextVersion++;
	
	  return function wrapWithConnect(WrappedComponent) {
	    var connectDisplayName = 'Connect(' + getDisplayName(WrappedComponent) + ')';
	
	    function checkStateShape(props, methodName) {
	      if (!(0, _isPlainObject2["default"])(props)) {
	        (0, _warning2["default"])(methodName + '() in ' + connectDisplayName + ' must return a plain object. ' + ('Instead received ' + props + '.'));
	      }
	    }
	
	    function computeMergedProps(stateProps, dispatchProps, parentProps) {
	      var mergedProps = finalMergeProps(stateProps, dispatchProps, parentProps);
	      if (true) {
	        checkStateShape(mergedProps, 'mergeProps');
	      }
	      return mergedProps;
	    }
	
	    var Connect = function (_Component) {
	      _inherits(Connect, _Component);
	
	      Connect.prototype.shouldComponentUpdate = function shouldComponentUpdate() {
	        return !pure || this.haveOwnPropsChanged || this.hasStoreStateChanged;
	      };
	
	      function Connect(props, context) {
	        _classCallCheck(this, Connect);
	
	        var _this = _possibleConstructorReturn(this, _Component.call(this, props, context));
	
	        _this.version = version;
	        _this.store = props.store || context.store;
	
	        (0, _invariant2["default"])(_this.store, 'Could not find "store" in either the context or ' + ('props of "' + connectDisplayName + '". ') + 'Either wrap the root component in a <Provider>, ' + ('or explicitly pass "store" as a prop to "' + connectDisplayName + '".'));
	
	        var storeState = _this.store.getState();
	        _this.state = { storeState: storeState };
	        _this.clearCache();
	        return _this;
	      }
	
	      Connect.prototype.computeStateProps = function computeStateProps(store, props) {
	        if (!this.finalMapStateToProps) {
	          return this.configureFinalMapState(store, props);
	        }
	
	        var state = store.getState();
	        var stateProps = this.doStatePropsDependOnOwnProps ? this.finalMapStateToProps(state, props) : this.finalMapStateToProps(state);
	
	        if (true) {
	          checkStateShape(stateProps, 'mapStateToProps');
	        }
	        return stateProps;
	      };
	
	      Connect.prototype.configureFinalMapState = function configureFinalMapState(store, props) {
	        var mappedState = mapState(store.getState(), props);
	        var isFactory = typeof mappedState === 'function';
	
	        this.finalMapStateToProps = isFactory ? mappedState : mapState;
	        this.doStatePropsDependOnOwnProps = this.finalMapStateToProps.length !== 1;
	
	        if (isFactory) {
	          return this.computeStateProps(store, props);
	        }
	
	        if (true) {
	          checkStateShape(mappedState, 'mapStateToProps');
	        }
	        return mappedState;
	      };
	
	      Connect.prototype.computeDispatchProps = function computeDispatchProps(store, props) {
	        if (!this.finalMapDispatchToProps) {
	          return this.configureFinalMapDispatch(store, props);
	        }
	
	        var dispatch = store.dispatch;
	
	        var dispatchProps = this.doDispatchPropsDependOnOwnProps ? this.finalMapDispatchToProps(dispatch, props) : this.finalMapDispatchToProps(dispatch);
	
	        if (true) {
	          checkStateShape(dispatchProps, 'mapDispatchToProps');
	        }
	        return dispatchProps;
	      };
	
	      Connect.prototype.configureFinalMapDispatch = function configureFinalMapDispatch(store, props) {
	        var mappedDispatch = mapDispatch(store.dispatch, props);
	        var isFactory = typeof mappedDispatch === 'function';
	
	        this.finalMapDispatchToProps = isFactory ? mappedDispatch : mapDispatch;
	        this.doDispatchPropsDependOnOwnProps = this.finalMapDispatchToProps.length !== 1;
	
	        if (isFactory) {
	          return this.computeDispatchProps(store, props);
	        }
	
	        if (true) {
	          checkStateShape(mappedDispatch, 'mapDispatchToProps');
	        }
	        return mappedDispatch;
	      };
	
	      Connect.prototype.updateStatePropsIfNeeded = function updateStatePropsIfNeeded() {
	        var nextStateProps = this.computeStateProps(this.store, this.props);
	        if (this.stateProps && (0, _shallowEqual2["default"])(nextStateProps, this.stateProps)) {
	          return false;
	        }
	
	        this.stateProps = nextStateProps;
	        return true;
	      };
	
	      Connect.prototype.updateDispatchPropsIfNeeded = function updateDispatchPropsIfNeeded() {
	        var nextDispatchProps = this.computeDispatchProps(this.store, this.props);
	        if (this.dispatchProps && (0, _shallowEqual2["default"])(nextDispatchProps, this.dispatchProps)) {
	          return false;
	        }
	
	        this.dispatchProps = nextDispatchProps;
	        return true;
	      };
	
	      Connect.prototype.updateMergedPropsIfNeeded = function updateMergedPropsIfNeeded() {
	        var nextMergedProps = computeMergedProps(this.stateProps, this.dispatchProps, this.props);
	        if (this.mergedProps && checkMergedEquals && (0, _shallowEqual2["default"])(nextMergedProps, this.mergedProps)) {
	          return false;
	        }
	
	        this.mergedProps = nextMergedProps;
	        return true;
	      };
	
	      Connect.prototype.isSubscribed = function isSubscribed() {
	        return typeof this.unsubscribe === 'function';
	      };
	
	      Connect.prototype.trySubscribe = function trySubscribe() {
	        if (shouldSubscribe && !this.unsubscribe) {
	          this.unsubscribe = this.store.subscribe(this.handleChange.bind(this));
	          this.handleChange();
	        }
	      };
	
	      Connect.prototype.tryUnsubscribe = function tryUnsubscribe() {
	        if (this.unsubscribe) {
	          this.unsubscribe();
	          this.unsubscribe = null;
	        }
	      };
	
	      Connect.prototype.componentDidMount = function componentDidMount() {
	        this.trySubscribe();
	      };
	
	      Connect.prototype.componentWillReceiveProps = function componentWillReceiveProps(nextProps) {
	        if (!pure || !(0, _shallowEqual2["default"])(nextProps, this.props)) {
	          this.haveOwnPropsChanged = true;
	        }
	      };
	
	      Connect.prototype.componentWillUnmount = function componentWillUnmount() {
	        this.tryUnsubscribe();
	        this.clearCache();
	      };
	
	      Connect.prototype.clearCache = function clearCache() {
	        this.dispatchProps = null;
	        this.stateProps = null;
	        this.mergedProps = null;
	        this.haveOwnPropsChanged = true;
	        this.hasStoreStateChanged = true;
	        this.haveStatePropsBeenPrecalculated = false;
	        this.statePropsPrecalculationError = null;
	        this.renderedElement = null;
	        this.finalMapDispatchToProps = null;
	        this.finalMapStateToProps = null;
	      };
	
	      Connect.prototype.handleChange = function handleChange() {
	        if (!this.unsubscribe) {
	          return;
	        }
	
	        var storeState = this.store.getState();
	        var prevStoreState = this.state.storeState;
	        if (pure && prevStoreState === storeState) {
	          return;
	        }
	
	        if (pure && !this.doStatePropsDependOnOwnProps) {
	          var haveStatePropsChanged = tryCatch(this.updateStatePropsIfNeeded, this);
	          if (!haveStatePropsChanged) {
	            return;
	          }
	          if (haveStatePropsChanged === errorObject) {
	            this.statePropsPrecalculationError = errorObject.value;
	          }
	          this.haveStatePropsBeenPrecalculated = true;
	        }
	
	        this.hasStoreStateChanged = true;
	        this.setState({ storeState: storeState });
	      };
	
	      Connect.prototype.getWrappedInstance = function getWrappedInstance() {
	        (0, _invariant2["default"])(withRef, 'To access the wrapped instance, you need to specify ' + '{ withRef: true } as the fourth argument of the connect() call.');
	
	        return this.refs.wrappedInstance;
	      };
	
	      Connect.prototype.render = function render() {
	        var haveOwnPropsChanged = this.haveOwnPropsChanged;
	        var hasStoreStateChanged = this.hasStoreStateChanged;
	        var haveStatePropsBeenPrecalculated = this.haveStatePropsBeenPrecalculated;
	        var statePropsPrecalculationError = this.statePropsPrecalculationError;
	        var renderedElement = this.renderedElement;
	
	        this.haveOwnPropsChanged = false;
	        this.hasStoreStateChanged = false;
	        this.haveStatePropsBeenPrecalculated = false;
	        this.statePropsPrecalculationError = null;
	
	        if (statePropsPrecalculationError) {
	          throw statePropsPrecalculationError;
	        }
	
	        var shouldUpdateStateProps = true;
	        var shouldUpdateDispatchProps = true;
	        if (pure && renderedElement) {
	          shouldUpdateStateProps = hasStoreStateChanged || haveOwnPropsChanged && this.doStatePropsDependOnOwnProps;
	          shouldUpdateDispatchProps = haveOwnPropsChanged && this.doDispatchPropsDependOnOwnProps;
	        }
	
	        var haveStatePropsChanged = false;
	        var haveDispatchPropsChanged = false;
	        if (haveStatePropsBeenPrecalculated) {
	          haveStatePropsChanged = true;
	        } else if (shouldUpdateStateProps) {
	          haveStatePropsChanged = this.updateStatePropsIfNeeded();
	        }
	        if (shouldUpdateDispatchProps) {
	          haveDispatchPropsChanged = this.updateDispatchPropsIfNeeded();
	        }
	
	        var haveMergedPropsChanged = true;
	        if (haveStatePropsChanged || haveDispatchPropsChanged || haveOwnPropsChanged) {
	          haveMergedPropsChanged = this.updateMergedPropsIfNeeded();
	        } else {
	          haveMergedPropsChanged = false;
	        }
	
	        if (!haveMergedPropsChanged && renderedElement) {
	          return renderedElement;
	        }
	
	        if (withRef) {
	          this.renderedElement = (0, _react.createElement)(WrappedComponent, _extends({}, this.mergedProps, {
	            ref: 'wrappedInstance'
	          }));
	        } else {
	          this.renderedElement = (0, _react.createElement)(WrappedComponent, this.mergedProps);
	        }
	
	        return this.renderedElement;
	      };
	
	      return Connect;
	    }(_react.Component);
	
	    Connect.displayName = connectDisplayName;
	    Connect.WrappedComponent = WrappedComponent;
	    Connect.contextTypes = {
	      store: _storeShape2["default"]
	    };
	    Connect.propTypes = {
	      store: _storeShape2["default"]
	    };
	
	    if (true) {
	      Connect.prototype.componentWillUpdate = function componentWillUpdate() {
	        if (this.version === version) {
	          return;
	        }
	
	        // We are hot reloading!
	        this.version = version;
	        this.trySubscribe();
	        this.clearCache();
	      };
	    }
	
	    return (0, _hoistNonReactStatics2["default"])(Connect, WrappedComponent);
	  };
	}

/***/ },

/***/ 665:
/***/ function(module, exports) {

	"use strict";
	
	exports.__esModule = true;
	exports["default"] = shallowEqual;
	function shallowEqual(objA, objB) {
	  if (objA === objB) {
	    return true;
	  }
	
	  var keysA = Object.keys(objA);
	  var keysB = Object.keys(objB);
	
	  if (keysA.length !== keysB.length) {
	    return false;
	  }
	
	  // Test for A's keys different from B.
	  var hasOwn = Object.prototype.hasOwnProperty;
	  for (var i = 0; i < keysA.length; i++) {
	    if (!hasOwn.call(objB, keysA[i]) || objA[keysA[i]] !== objB[keysA[i]]) {
	      return false;
	    }
	  }
	
	  return true;
	}

/***/ },

/***/ 666:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	
	exports.__esModule = true;
	exports["default"] = wrapActionCreators;
	
	var _redux = __webpack_require__(667);
	
	function wrapActionCreators(actionCreators) {
	  return function (dispatch) {
	    return (0, _redux.bindActionCreators)(actionCreators, dispatch);
	  };
	}

/***/ },

/***/ 667:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	
	exports.__esModule = true;
	exports.compose = exports.applyMiddleware = exports.bindActionCreators = exports.combineReducers = exports.createStore = undefined;
	
	var _createStore = __webpack_require__(668);
	
	var _createStore2 = _interopRequireDefault(_createStore);
	
	var _combineReducers = __webpack_require__(674);
	
	var _combineReducers2 = _interopRequireDefault(_combineReducers);
	
	var _bindActionCreators = __webpack_require__(676);
	
	var _bindActionCreators2 = _interopRequireDefault(_bindActionCreators);
	
	var _applyMiddleware = __webpack_require__(677);
	
	var _applyMiddleware2 = _interopRequireDefault(_applyMiddleware);
	
	var _compose = __webpack_require__(678);
	
	var _compose2 = _interopRequireDefault(_compose);
	
	var _warning = __webpack_require__(675);
	
	var _warning2 = _interopRequireDefault(_warning);
	
	function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { 'default': obj }; }
	
	/*
	* This is a dummy function to check if the function name has been altered by minification.
	* If the function has been minified and NODE_ENV !== 'production', warn the user.
	*/
	function isCrushed() {}
	
	if (("development") !== 'production' && typeof isCrushed.name === 'string' && isCrushed.name !== 'isCrushed') {
	  (0, _warning2['default'])('You are currently using minified code outside of NODE_ENV === \'production\'. ' + 'This means that you are running a slower development build of Redux. ' + 'You can use loose-envify (https://github.com/zertosh/loose-envify) for browserify ' + 'or DefinePlugin for webpack (http://stackoverflow.com/questions/30030031) ' + 'to ensure you have the correct code for your production build.');
	}
	
	exports.createStore = _createStore2['default'];
	exports.combineReducers = _combineReducers2['default'];
	exports.bindActionCreators = _bindActionCreators2['default'];
	exports.applyMiddleware = _applyMiddleware2['default'];
	exports.compose = _compose2['default'];

/***/ },

/***/ 668:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	
	exports.__esModule = true;
	exports.ActionTypes = undefined;
	exports['default'] = createStore;
	
	var _isPlainObject = __webpack_require__(669);
	
	var _isPlainObject2 = _interopRequireDefault(_isPlainObject);
	
	var _symbolObservable = __webpack_require__(671);
	
	var _symbolObservable2 = _interopRequireDefault(_symbolObservable);
	
	function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { 'default': obj }; }
	
	/**
	 * These are private action types reserved by Redux.
	 * For any unknown actions, you must return the current state.
	 * If the current state is undefined, you must return the initial state.
	 * Do not reference these action types directly in your code.
	 */
	var ActionTypes = exports.ActionTypes = {
	  INIT: '@@redux/INIT'
	};
	
	/**
	 * Creates a Redux store that holds the state tree.
	 * The only way to change the data in the store is to call `dispatch()` on it.
	 *
	 * There should only be a single store in your app. To specify how different
	 * parts of the state tree respond to actions, you may combine several reducers
	 * into a single reducer function by using `combineReducers`.
	 *
	 * @param {Function} reducer A function that returns the next state tree, given
	 * the current state tree and the action to handle.
	 *
	 * @param {any} [preloadedState] The initial state. You may optionally specify it
	 * to hydrate the state from the server in universal apps, or to restore a
	 * previously serialized user session.
	 * If you use `combineReducers` to produce the root reducer function, this must be
	 * an object with the same shape as `combineReducers` keys.
	 *
	 * @param {Function} enhancer The store enhancer. You may optionally specify it
	 * to enhance the store with third-party capabilities such as middleware,
	 * time travel, persistence, etc. The only store enhancer that ships with Redux
	 * is `applyMiddleware()`.
	 *
	 * @returns {Store} A Redux store that lets you read the state, dispatch actions
	 * and subscribe to changes.
	 */
	function createStore(reducer, preloadedState, enhancer) {
	  var _ref2;
	
	  if (typeof preloadedState === 'function' && typeof enhancer === 'undefined') {
	    enhancer = preloadedState;
	    preloadedState = undefined;
	  }
	
	  if (typeof enhancer !== 'undefined') {
	    if (typeof enhancer !== 'function') {
	      throw new Error('Expected the enhancer to be a function.');
	    }
	
	    return enhancer(createStore)(reducer, preloadedState);
	  }
	
	  if (typeof reducer !== 'function') {
	    throw new Error('Expected the reducer to be a function.');
	  }
	
	  var currentReducer = reducer;
	  var currentState = preloadedState;
	  var currentListeners = [];
	  var nextListeners = currentListeners;
	  var isDispatching = false;
	
	  function ensureCanMutateNextListeners() {
	    if (nextListeners === currentListeners) {
	      nextListeners = currentListeners.slice();
	    }
	  }
	
	  /**
	   * Reads the state tree managed by the store.
	   *
	   * @returns {any} The current state tree of your application.
	   */
	  function getState() {
	    return currentState;
	  }
	
	  /**
	   * Adds a change listener. It will be called any time an action is dispatched,
	   * and some part of the state tree may potentially have changed. You may then
	   * call `getState()` to read the current state tree inside the callback.
	   *
	   * You may call `dispatch()` from a change listener, with the following
	   * caveats:
	   *
	   * 1. The subscriptions are snapshotted just before every `dispatch()` call.
	   * If you subscribe or unsubscribe while the listeners are being invoked, this
	   * will not have any effect on the `dispatch()` that is currently in progress.
	   * However, the next `dispatch()` call, whether nested or not, will use a more
	   * recent snapshot of the subscription list.
	   *
	   * 2. The listener should not expect to see all state changes, as the state
	   * might have been updated multiple times during a nested `dispatch()` before
	   * the listener is called. It is, however, guaranteed that all subscribers
	   * registered before the `dispatch()` started will be called with the latest
	   * state by the time it exits.
	   *
	   * @param {Function} listener A callback to be invoked on every dispatch.
	   * @returns {Function} A function to remove this change listener.
	   */
	  function subscribe(listener) {
	    if (typeof listener !== 'function') {
	      throw new Error('Expected listener to be a function.');
	    }
	
	    var isSubscribed = true;
	
	    ensureCanMutateNextListeners();
	    nextListeners.push(listener);
	
	    return function unsubscribe() {
	      if (!isSubscribed) {
	        return;
	      }
	
	      isSubscribed = false;
	
	      ensureCanMutateNextListeners();
	      var index = nextListeners.indexOf(listener);
	      nextListeners.splice(index, 1);
	    };
	  }
	
	  /**
	   * Dispatches an action. It is the only way to trigger a state change.
	   *
	   * The `reducer` function, used to create the store, will be called with the
	   * current state tree and the given `action`. Its return value will
	   * be considered the **next** state of the tree, and the change listeners
	   * will be notified.
	   *
	   * The base implementation only supports plain object actions. If you want to
	   * dispatch a Promise, an Observable, a thunk, or something else, you need to
	   * wrap your store creating function into the corresponding middleware. For
	   * example, see the documentation for the `redux-thunk` package. Even the
	   * middleware will eventually dispatch plain object actions using this method.
	   *
	   * @param {Object} action A plain object representing “what changed”. It is
	   * a good idea to keep actions serializable so you can record and replay user
	   * sessions, or use the time travelling `redux-devtools`. An action must have
	   * a `type` property which may not be `undefined`. It is a good idea to use
	   * string constants for action types.
	   *
	   * @returns {Object} For convenience, the same action object you dispatched.
	   *
	   * Note that, if you use a custom middleware, it may wrap `dispatch()` to
	   * return something else (for example, a Promise you can await).
	   */
	  function dispatch(action) {
	    if (!(0, _isPlainObject2['default'])(action)) {
	      throw new Error('Actions must be plain objects. ' + 'Use custom middleware for async actions.');
	    }
	
	    if (typeof action.type === 'undefined') {
	      throw new Error('Actions may not have an undefined "type" property. ' + 'Have you misspelled a constant?');
	    }
	
	    if (isDispatching) {
	      throw new Error('Reducers may not dispatch actions.');
	    }
	
	    try {
	      isDispatching = true;
	      currentState = currentReducer(currentState, action);
	    } finally {
	      isDispatching = false;
	    }
	
	    var listeners = currentListeners = nextListeners;
	    for (var i = 0; i < listeners.length; i++) {
	      listeners[i]();
	    }
	
	    return action;
	  }
	
	  /**
	   * Replaces the reducer currently used by the store to calculate the state.
	   *
	   * You might need this if your app implements code splitting and you want to
	   * load some of the reducers dynamically. You might also need this if you
	   * implement a hot reloading mechanism for Redux.
	   *
	   * @param {Function} nextReducer The reducer for the store to use instead.
	   * @returns {void}
	   */
	  function replaceReducer(nextReducer) {
	    if (typeof nextReducer !== 'function') {
	      throw new Error('Expected the nextReducer to be a function.');
	    }
	
	    currentReducer = nextReducer;
	    dispatch({ type: ActionTypes.INIT });
	  }
	
	  /**
	   * Interoperability point for observable/reactive libraries.
	   * @returns {observable} A minimal observable of state changes.
	   * For more information, see the observable proposal:
	   * https://github.com/zenparsing/es-observable
	   */
	  function observable() {
	    var _ref;
	
	    var outerSubscribe = subscribe;
	    return _ref = {
	      /**
	       * The minimal observable subscription method.
	       * @param {Object} observer Any object that can be used as an observer.
	       * The observer object should have a `next` method.
	       * @returns {subscription} An object with an `unsubscribe` method that can
	       * be used to unsubscribe the observable from the store, and prevent further
	       * emission of values from the observable.
	       */
	      subscribe: function subscribe(observer) {
	        if (typeof observer !== 'object') {
	          throw new TypeError('Expected the observer to be an object.');
	        }
	
	        function observeState() {
	          if (observer.next) {
	            observer.next(getState());
	          }
	        }
	
	        observeState();
	        var unsubscribe = outerSubscribe(observeState);
	        return { unsubscribe: unsubscribe };
	      }
	    }, _ref[_symbolObservable2['default']] = function () {
	      return this;
	    }, _ref;
	  }
	
	  // When a store is created, an "INIT" action is dispatched so that every
	  // reducer returns their initial state. This effectively populates
	  // the initial state tree.
	  dispatch({ type: ActionTypes.INIT });
	
	  return _ref2 = {
	    dispatch: dispatch,
	    subscribe: subscribe,
	    getState: getState,
	    replaceReducer: replaceReducer
	  }, _ref2[_symbolObservable2['default']] = observable, _ref2;
	}

/***/ },

/***/ 669:
/***/ function(module, exports, __webpack_require__) {

	var getPrototype = __webpack_require__(670),
	    isHostObject = __webpack_require__(344),
	    isObjectLike = __webpack_require__(316);
	
	/** `Object#toString` result references. */
	var objectTag = '[object Object]';
	
	/** Used for built-in method references. */
	var funcProto = Function.prototype,
	    objectProto = Object.prototype;
	
	/** Used to resolve the decompiled source of functions. */
	var funcToString = funcProto.toString;
	
	/** Used to check objects for own properties. */
	var hasOwnProperty = objectProto.hasOwnProperty;
	
	/** Used to infer the `Object` constructor. */
	var objectCtorString = funcToString.call(Object);
	
	/**
	 * Used to resolve the
	 * [`toStringTag`](http://ecma-international.org/ecma-262/7.0/#sec-object.prototype.tostring)
	 * of values.
	 */
	var objectToString = objectProto.toString;
	
	/**
	 * Checks if `value` is a plain object, that is, an object created by the
	 * `Object` constructor or one with a `[[Prototype]]` of `null`.
	 *
	 * @static
	 * @memberOf _
	 * @since 0.8.0
	 * @category Lang
	 * @param {*} value The value to check.
	 * @returns {boolean} Returns `true` if `value` is a plain object, else `false`.
	 * @example
	 *
	 * function Foo() {
	 *   this.a = 1;
	 * }
	 *
	 * _.isPlainObject(new Foo);
	 * // => false
	 *
	 * _.isPlainObject([1, 2, 3]);
	 * // => false
	 *
	 * _.isPlainObject({ 'x': 0, 'y': 0 });
	 * // => true
	 *
	 * _.isPlainObject(Object.create(null));
	 * // => true
	 */
	function isPlainObject(value) {
	  if (!isObjectLike(value) ||
	      objectToString.call(value) != objectTag || isHostObject(value)) {
	    return false;
	  }
	  var proto = getPrototype(value);
	  if (proto === null) {
	    return true;
	  }
	  var Ctor = hasOwnProperty.call(proto, 'constructor') && proto.constructor;
	  return (typeof Ctor == 'function' &&
	    Ctor instanceof Ctor && funcToString.call(Ctor) == objectCtorString);
	}
	
	module.exports = isPlainObject;


/***/ },

/***/ 670:
/***/ function(module, exports, __webpack_require__) {

	var overArg = __webpack_require__(322);
	
	/** Built-in value references. */
	var getPrototype = overArg(Object.getPrototypeOf, Object);
	
	module.exports = getPrototype;


/***/ },

/***/ 671:
/***/ function(module, exports, __webpack_require__) {

	module.exports = __webpack_require__(672);


/***/ },

/***/ 672:
/***/ function(module, exports, __webpack_require__) {

	/* WEBPACK VAR INJECTION */(function(global) {'use strict';
	
	Object.defineProperty(exports, "__esModule", {
		value: true
	});
	
	var _ponyfill = __webpack_require__(673);
	
	var _ponyfill2 = _interopRequireDefault(_ponyfill);
	
	function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { 'default': obj }; }
	
	var root = undefined; /* global window */
	
	if (typeof global !== 'undefined') {
		root = global;
	} else if (typeof window !== 'undefined') {
		root = window;
	}
	
	var result = (0, _ponyfill2['default'])(root);
	exports['default'] = result;
	/* WEBPACK VAR INJECTION */}.call(exports, (function() { return this; }())))

/***/ },

/***/ 673:
/***/ function(module, exports) {

	'use strict';
	
	Object.defineProperty(exports, "__esModule", {
		value: true
	});
	exports['default'] = symbolObservablePonyfill;
	function symbolObservablePonyfill(root) {
		var result;
		var _Symbol = root.Symbol;
	
		if (typeof _Symbol === 'function') {
			if (_Symbol.observable) {
				result = _Symbol.observable;
			} else {
				result = _Symbol('observable');
				_Symbol.observable = result;
			}
		} else {
			result = '@@observable';
		}
	
		return result;
	};

/***/ },

/***/ 674:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	
	exports.__esModule = true;
	exports['default'] = combineReducers;
	
	var _createStore = __webpack_require__(668);
	
	var _isPlainObject = __webpack_require__(669);
	
	var _isPlainObject2 = _interopRequireDefault(_isPlainObject);
	
	var _warning = __webpack_require__(675);
	
	var _warning2 = _interopRequireDefault(_warning);
	
	function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { 'default': obj }; }
	
	function getUndefinedStateErrorMessage(key, action) {
	  var actionType = action && action.type;
	  var actionName = actionType && '"' + actionType.toString() + '"' || 'an action';
	
	  return 'Given action ' + actionName + ', reducer "' + key + '" returned undefined. ' + 'To ignore an action, you must explicitly return the previous state.';
	}
	
	function getUnexpectedStateShapeWarningMessage(inputState, reducers, action, unexpectedKeyCache) {
	  var reducerKeys = Object.keys(reducers);
	  var argumentName = action && action.type === _createStore.ActionTypes.INIT ? 'preloadedState argument passed to createStore' : 'previous state received by the reducer';
	
	  if (reducerKeys.length === 0) {
	    return 'Store does not have a valid reducer. Make sure the argument passed ' + 'to combineReducers is an object whose values are reducers.';
	  }
	
	  if (!(0, _isPlainObject2['default'])(inputState)) {
	    return 'The ' + argumentName + ' has unexpected type of "' + {}.toString.call(inputState).match(/\s([a-z|A-Z]+)/)[1] + '". Expected argument to be an object with the following ' + ('keys: "' + reducerKeys.join('", "') + '"');
	  }
	
	  var unexpectedKeys = Object.keys(inputState).filter(function (key) {
	    return !reducers.hasOwnProperty(key) && !unexpectedKeyCache[key];
	  });
	
	  unexpectedKeys.forEach(function (key) {
	    unexpectedKeyCache[key] = true;
	  });
	
	  if (unexpectedKeys.length > 0) {
	    return 'Unexpected ' + (unexpectedKeys.length > 1 ? 'keys' : 'key') + ' ' + ('"' + unexpectedKeys.join('", "') + '" found in ' + argumentName + '. ') + 'Expected to find one of the known reducer keys instead: ' + ('"' + reducerKeys.join('", "') + '". Unexpected keys will be ignored.');
	  }
	}
	
	function assertReducerSanity(reducers) {
	  Object.keys(reducers).forEach(function (key) {
	    var reducer = reducers[key];
	    var initialState = reducer(undefined, { type: _createStore.ActionTypes.INIT });
	
	    if (typeof initialState === 'undefined') {
	      throw new Error('Reducer "' + key + '" returned undefined during initialization. ' + 'If the state passed to the reducer is undefined, you must ' + 'explicitly return the initial state. The initial state may ' + 'not be undefined.');
	    }
	
	    var type = '@@redux/PROBE_UNKNOWN_ACTION_' + Math.random().toString(36).substring(7).split('').join('.');
	    if (typeof reducer(undefined, { type: type }) === 'undefined') {
	      throw new Error('Reducer "' + key + '" returned undefined when probed with a random type. ' + ('Don\'t try to handle ' + _createStore.ActionTypes.INIT + ' or other actions in "redux/*" ') + 'namespace. They are considered private. Instead, you must return the ' + 'current state for any unknown actions, unless it is undefined, ' + 'in which case you must return the initial state, regardless of the ' + 'action type. The initial state may not be undefined.');
	    }
	  });
	}
	
	/**
	 * Turns an object whose values are different reducer functions, into a single
	 * reducer function. It will call every child reducer, and gather their results
	 * into a single state object, whose keys correspond to the keys of the passed
	 * reducer functions.
	 *
	 * @param {Object} reducers An object whose values correspond to different
	 * reducer functions that need to be combined into one. One handy way to obtain
	 * it is to use ES6 `import * as reducers` syntax. The reducers may never return
	 * undefined for any action. Instead, they should return their initial state
	 * if the state passed to them was undefined, and the current state for any
	 * unrecognized action.
	 *
	 * @returns {Function} A reducer function that invokes every reducer inside the
	 * passed object, and builds a state object with the same shape.
	 */
	function combineReducers(reducers) {
	  var reducerKeys = Object.keys(reducers);
	  var finalReducers = {};
	  for (var i = 0; i < reducerKeys.length; i++) {
	    var key = reducerKeys[i];
	
	    if (true) {
	      if (typeof reducers[key] === 'undefined') {
	        (0, _warning2['default'])('No reducer provided for key "' + key + '"');
	      }
	    }
	
	    if (typeof reducers[key] === 'function') {
	      finalReducers[key] = reducers[key];
	    }
	  }
	  var finalReducerKeys = Object.keys(finalReducers);
	
	  if (true) {
	    var unexpectedKeyCache = {};
	  }
	
	  var sanityError;
	  try {
	    assertReducerSanity(finalReducers);
	  } catch (e) {
	    sanityError = e;
	  }
	
	  return function combination() {
	    var state = arguments.length <= 0 || arguments[0] === undefined ? {} : arguments[0];
	    var action = arguments[1];
	
	    if (sanityError) {
	      throw sanityError;
	    }
	
	    if (true) {
	      var warningMessage = getUnexpectedStateShapeWarningMessage(state, finalReducers, action, unexpectedKeyCache);
	      if (warningMessage) {
	        (0, _warning2['default'])(warningMessage);
	      }
	    }
	
	    var hasChanged = false;
	    var nextState = {};
	    for (var i = 0; i < finalReducerKeys.length; i++) {
	      var key = finalReducerKeys[i];
	      var reducer = finalReducers[key];
	      var previousStateForKey = state[key];
	      var nextStateForKey = reducer(previousStateForKey, action);
	      if (typeof nextStateForKey === 'undefined') {
	        var errorMessage = getUndefinedStateErrorMessage(key, action);
	        throw new Error(errorMessage);
	      }
	      nextState[key] = nextStateForKey;
	      hasChanged = hasChanged || nextStateForKey !== previousStateForKey;
	    }
	    return hasChanged ? nextState : state;
	  };
	}

/***/ },

/***/ 675:
/***/ function(module, exports) {

	'use strict';
	
	exports.__esModule = true;
	exports['default'] = warning;
	/**
	 * Prints a warning in the console if it exists.
	 *
	 * @param {String} message The warning message.
	 * @returns {void}
	 */
	function warning(message) {
	  /* eslint-disable no-console */
	  if (typeof console !== 'undefined' && typeof console.error === 'function') {
	    console.error(message);
	  }
	  /* eslint-enable no-console */
	  try {
	    // This error was thrown as a convenience so that if you enable
	    // "break on all exceptions" in your console,
	    // it would pause the execution at this line.
	    throw new Error(message);
	    /* eslint-disable no-empty */
	  } catch (e) {}
	  /* eslint-enable no-empty */
	}

/***/ },

/***/ 676:
/***/ function(module, exports) {

	'use strict';
	
	exports.__esModule = true;
	exports['default'] = bindActionCreators;
	function bindActionCreator(actionCreator, dispatch) {
	  return function () {
	    return dispatch(actionCreator.apply(undefined, arguments));
	  };
	}
	
	/**
	 * Turns an object whose values are action creators, into an object with the
	 * same keys, but with every function wrapped into a `dispatch` call so they
	 * may be invoked directly. This is just a convenience method, as you can call
	 * `store.dispatch(MyActionCreators.doSomething())` yourself just fine.
	 *
	 * For convenience, you can also pass a single function as the first argument,
	 * and get a function in return.
	 *
	 * @param {Function|Object} actionCreators An object whose values are action
	 * creator functions. One handy way to obtain it is to use ES6 `import * as`
	 * syntax. You may also pass a single function.
	 *
	 * @param {Function} dispatch The `dispatch` function available on your Redux
	 * store.
	 *
	 * @returns {Function|Object} The object mimicking the original object, but with
	 * every action creator wrapped into the `dispatch` call. If you passed a
	 * function as `actionCreators`, the return value will also be a single
	 * function.
	 */
	function bindActionCreators(actionCreators, dispatch) {
	  if (typeof actionCreators === 'function') {
	    return bindActionCreator(actionCreators, dispatch);
	  }
	
	  if (typeof actionCreators !== 'object' || actionCreators === null) {
	    throw new Error('bindActionCreators expected an object or a function, instead received ' + (actionCreators === null ? 'null' : typeof actionCreators) + '. ' + 'Did you write "import ActionCreators from" instead of "import * as ActionCreators from"?');
	  }
	
	  var keys = Object.keys(actionCreators);
	  var boundActionCreators = {};
	  for (var i = 0; i < keys.length; i++) {
	    var key = keys[i];
	    var actionCreator = actionCreators[key];
	    if (typeof actionCreator === 'function') {
	      boundActionCreators[key] = bindActionCreator(actionCreator, dispatch);
	    }
	  }
	  return boundActionCreators;
	}

/***/ },

/***/ 677:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	
	exports.__esModule = true;
	
	var _extends = Object.assign || function (target) { for (var i = 1; i < arguments.length; i++) { var source = arguments[i]; for (var key in source) { if (Object.prototype.hasOwnProperty.call(source, key)) { target[key] = source[key]; } } } return target; };
	
	exports['default'] = applyMiddleware;
	
	var _compose = __webpack_require__(678);
	
	var _compose2 = _interopRequireDefault(_compose);
	
	function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { 'default': obj }; }
	
	/**
	 * Creates a store enhancer that applies middleware to the dispatch method
	 * of the Redux store. This is handy for a variety of tasks, such as expressing
	 * asynchronous actions in a concise manner, or logging every action payload.
	 *
	 * See `redux-thunk` package as an example of the Redux middleware.
	 *
	 * Because middleware is potentially asynchronous, this should be the first
	 * store enhancer in the composition chain.
	 *
	 * Note that each middleware will be given the `dispatch` and `getState` functions
	 * as named arguments.
	 *
	 * @param {...Function} middlewares The middleware chain to be applied.
	 * @returns {Function} A store enhancer applying the middleware.
	 */
	function applyMiddleware() {
	  for (var _len = arguments.length, middlewares = Array(_len), _key = 0; _key < _len; _key++) {
	    middlewares[_key] = arguments[_key];
	  }
	
	  return function (createStore) {
	    return function (reducer, preloadedState, enhancer) {
	      var store = createStore(reducer, preloadedState, enhancer);
	      var _dispatch = store.dispatch;
	      var chain = [];
	
	      var middlewareAPI = {
	        getState: store.getState,
	        dispatch: function dispatch(action) {
	          return _dispatch(action);
	        }
	      };
	      chain = middlewares.map(function (middleware) {
	        return middleware(middlewareAPI);
	      });
	      _dispatch = _compose2['default'].apply(undefined, chain)(store.dispatch);
	
	      return _extends({}, store, {
	        dispatch: _dispatch
	      });
	    };
	  };
	}

/***/ },

/***/ 678:
/***/ function(module, exports) {

	"use strict";
	
	exports.__esModule = true;
	exports["default"] = compose;
	/**
	 * Composes single-argument functions from right to left. The rightmost
	 * function can take multiple arguments as it provides the signature for
	 * the resulting composite function.
	 *
	 * @param {...Function} funcs The functions to compose.
	 * @returns {Function} A function obtained by composing the argument functions
	 * from right to left. For example, compose(f, g, h) is identical to doing
	 * (...args) => f(g(h(...args))).
	 */
	
	function compose() {
	  for (var _len = arguments.length, funcs = Array(_len), _key = 0; _key < _len; _key++) {
	    funcs[_key] = arguments[_key];
	  }
	
	  if (funcs.length === 0) {
	    return function (arg) {
	      return arg;
	    };
	  }
	
	  if (funcs.length === 1) {
	    return funcs[0];
	  }
	
	  var last = funcs[funcs.length - 1];
	  var rest = funcs.slice(0, -1);
	  return function () {
	    return rest.reduceRight(function (composed, f) {
	      return f(composed);
	    }, last.apply(undefined, arguments));
	  };
	}

/***/ },

/***/ 679:
/***/ function(module, exports) {

	/**
	 * Copyright 2015, Yahoo! Inc.
	 * Copyrights licensed under the New BSD License. See the accompanying LICENSE file for terms.
	 */
	'use strict';
	
	var REACT_STATICS = {
	    childContextTypes: true,
	    contextTypes: true,
	    defaultProps: true,
	    displayName: true,
	    getDefaultProps: true,
	    mixins: true,
	    propTypes: true,
	    type: true
	};
	
	var KNOWN_STATICS = {
	    name: true,
	    length: true,
	    prototype: true,
	    caller: true,
	    arguments: true,
	    arity: true
	};
	
	var isGetOwnPropertySymbolsAvailable = typeof Object.getOwnPropertySymbols === 'function';
	
	module.exports = function hoistNonReactStatics(targetComponent, sourceComponent, customStatics) {
	    if (typeof sourceComponent !== 'string') { // don't hoist over string (html) components
	        var keys = Object.getOwnPropertyNames(sourceComponent);
	
	        /* istanbul ignore else */
	        if (isGetOwnPropertySymbolsAvailable) {
	            keys = keys.concat(Object.getOwnPropertySymbols(sourceComponent));
	        }
	
	        for (var i = 0; i < keys.length; ++i) {
	            if (!REACT_STATICS[keys[i]] && !KNOWN_STATICS[keys[i]] && (!customStatics || !customStatics[keys[i]])) {
	                try {
	                    targetComponent[keys[i]] = sourceComponent[keys[i]];
	                } catch (error) {
	
	                }
	            }
	        }
	    }
	
	    return targetComponent;
	};


/***/ },

/***/ 680:
/***/ function(module, exports, __webpack_require__) {

	/**
	 * Copyright 2013-2015, Facebook, Inc.
	 * All rights reserved.
	 *
	 * This source code is licensed under the BSD-style license found in the
	 * LICENSE file in the root directory of this source tree. An additional grant
	 * of patent rights can be found in the PATENTS file in the same directory.
	 */
	
	'use strict';
	
	/**
	 * Use invariant() to assert state which your program assumes to be true.
	 *
	 * Provide sprintf-style format (only %s is supported) and arguments
	 * to provide information about what broke and what you were
	 * expecting.
	 *
	 * The invariant message will be stripped in production, but the invariant
	 * will remain to ensure logic does not differ in production.
	 */
	
	var invariant = function(condition, format, a, b, c, d, e, f) {
	  if (true) {
	    if (format === undefined) {
	      throw new Error('invariant requires an error message argument');
	    }
	  }
	
	  if (!condition) {
	    var error;
	    if (format === undefined) {
	      error = new Error(
	        'Minified exception occurred; use the non-minified dev environment ' +
	        'for the full error message and additional helpful warnings.'
	      );
	    } else {
	      var args = [a, b, c, d, e, f];
	      var argIndex = 0;
	      error = new Error(
	        format.replace(/%s/g, function() { return args[argIndex++]; })
	      );
	      error.name = 'Invariant Violation';
	    }
	
	    error.framesToPop = 1; // we don't care about invariant's own frame
	    throw error;
	  }
	};
	
	module.exports = invariant;


/***/ },

/***/ 681:
/***/ function(module, exports, __webpack_require__) {

	var __WEBPACK_AMD_DEFINE_ARRAY__, __WEBPACK_AMD_DEFINE_RESULT__;/*!
	  Copyright (c) 2016 Jed Watson.
	  Licensed under the MIT License (MIT), see
	  http://jedwatson.github.io/classnames
	*/
	/* global define */
	
	(function () {
		'use strict';
	
		var hasOwn = {}.hasOwnProperty;
	
		function classNames () {
			var classes = [];
	
			for (var i = 0; i < arguments.length; i++) {
				var arg = arguments[i];
				if (!arg) continue;
	
				var argType = typeof arg;
	
				if (argType === 'string' || argType === 'number') {
					classes.push(arg);
				} else if (Array.isArray(arg)) {
					classes.push(classNames.apply(null, arg));
				} else if (argType === 'object') {
					for (var key in arg) {
						if (hasOwn.call(arg, key) && arg[key]) {
							classes.push(key);
						}
					}
				}
			}
	
			return classes.join(' ');
		}
	
		if (typeof module !== 'undefined' && module.exports) {
			module.exports = classNames;
		} else if (true) {
			// register as 'classnames', consistent with npm package name
			!(__WEBPACK_AMD_DEFINE_ARRAY__ = [], __WEBPACK_AMD_DEFINE_RESULT__ = function () {
				return classNames;
			}.apply(exports, __WEBPACK_AMD_DEFINE_ARRAY__), __WEBPACK_AMD_DEFINE_RESULT__ !== undefined && (module.exports = __WEBPACK_AMD_DEFINE_RESULT__));
		} else {
			window.classNames = classNames;
		}
	}());


/***/ },

/***/ 682:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	
	// TODO: doc all the things... and cleanly.
	// Apply reducer & action on sub-state through a given path.
	
	// TODO: specific lodash dependencies
	var isFunction = __webpack_require__(313);
	var lodashGetIn = __webpack_require__(394);
	var lodashSetIn = __webpack_require__(683);
	var lodashMerge = __webpack_require__(686);
	var lodashMergeWith = __webpack_require__(725);
	var lodashIsArray = __webpack_require__(317);
	var lodashIsString = __webpack_require__(726);
	// TODO: npm install npm.im/warning
	
	// sentinel value
	var NOT_SET = {};
	
	// NOTE: needed b/c
	// _.merge([1,2,3],[undefined]) => [1,2,3]
	// _.merge({a: void 0}, {b: void 0}) => {a: void 0}
	//
	// Above are undesired merging behaviours.
	var customMerge = function customMerge(_oldValue, newValue, key, destObject) {
	    if (newValue === void 0) {
	        // TODO: necessary?
	        //var key = _.isArray(destObject) ? Number(key) : key;
	        // NOTE: this is a side-effect
	        destObject[key] = newValue;
	    }
	};
	
	// if path points to root
	var __isRoot = function __isRoot(path) {
	    // see: https://github.com/lodash/lodash/issues/2638
	    return lodashIsArray(path) && path.length <= 0 || lodashIsString(path) && path.trim().length <= 0;
	};
	
	var __getIn = function __getIn(rootData, path) {
	
	    if (__isRoot(path)) {
	        return rootData;
	    }
	
	    return lodashGetIn(rootData, path);
	};
	
	// NOTE:
	// - should return new object.
	// - should NOT edit rootData in-place
	var __setIn = function __setIn(rootData, path, newValue) {
	
	    var isArray = lodashIsArray(rootData);
	
	    var patch = __isRoot(path) ? newValue : lodashSetIn(isArray ? [] : {}, path, newValue);
	
	    // NOTE: the following will not work: {...state, ...patch};
	
	    return lodashMergeWith(isArray ? [] : {}, rootData, patch, customMerge);
	};
	
	var treeReducer = function treeReducer(state, action) {
	
	    if (true) {
	        if (!action.meta) {
	            console.log(action);
	            // TODO: improve error
	            throw Error('no meta in action');
	        }
	
	        if (!action.meta.__redux_tree) {
	            // TODO: improve error
	            throw Error('reduceIn/reduceInWith not used');
	        }
	    }
	
	    var _action$meta$__redux_ = action.meta.__redux_tree;
	    var reducer = _action$meta$__redux_.reducer;
	    var path = _action$meta$__redux_.path;
	    var getIn = _action$meta$__redux_.getIn;
	    var setIn = _action$meta$__redux_.setIn;
	
	
	    if (true) {
	
	        if (!reducer) {
	            // TODO: improve error
	            throw Error('no reducer');
	        }
	
	        if (!path) {
	            // TODO: improve error
	            throw Error('no path');
	        }
	    }
	
	    var oldValue = getIn(state, path);
	    var newValue = reducer(oldValue, action);
	    var newRoot = setIn(state, path, newValue);
	
	    return newRoot;
	};
	
	// reducer factory
	var makeReducer = function makeReducer() {
	    var _ref = arguments.length <= 0 || arguments[0] === undefined ? {} : arguments[0];
	
	    var _ref$reducer = _ref.reducer;
	    var fallbackReducer = _ref$reducer === undefined ? NOT_SET : _ref$reducer;
	
	
	    var afterInit = false;
	
	    // const fallbackReducer = reducer;
	    var fallbackReducerIsFunction = isFunction(fallbackReducer);
	
	    return function () {
	        var state = arguments.length <= 0 || arguments[0] === undefined ? NOT_SET : arguments[0];
	        var action = arguments[1];
	
	
	        if (true) {
	            if (state === NOT_SET) {
	                // TODO: improve error
	                throw Error('no initial state given');
	            }
	        }
	
	        if (!afterInit) {
	            afterInit = true;
	            return state;
	        }
	
	        if (fallbackReducerIsFunction) {
	            if (!action.meta || action.meta && action.meta.reducer && isFunction(action.meta.reducer)) {
	                return fallbackReducer(state, action);
	            }
	        }
	
	        return treeReducer(state, action);
	    };
	};
	
	// redux-tree action creator/transformer that adds the following to the given action
	//
	// action = {
	//     // ...
	//     meta: {
	//         __redux_tree: {
	//             path: path,
	//             reducer: reducer,
	//             getIn: getIn,
	//             setIn: setIn
	//         }
	//     }
	// };
	//
	// NOTE: given action must be flux standard action (FSA) compliant.
	//
	// The returned action is flux standard action (FSA) compliant.
	//
	var reduceIn = function reduceIn(reducer, path, action) {
	    var getIn = arguments.length <= 3 || arguments[3] === undefined ? __getIn : arguments[3];
	    var setIn = arguments.length <= 4 || arguments[4] === undefined ? __setIn : arguments[4];
	    var shouldPollute = arguments.length <= 5 || arguments[5] === undefined ? false : arguments[5];
	
	
	    // NOTE: shouldPollute allows pollution of action object (in-place overwriting)
	
	    var patch = {
	        meta: {
	            __redux_tree: {
	                path: path, // array
	                reducer: reducer, // redux compatible reducer
	                getIn: getIn,
	                setIn: setIn
	            }
	        }
	    };
	
	    return shouldPollute ? lodashMerge(action, patch) : lodashMerge({}, action, patch);
	};
	
	// reduceIn factory to cache given getIn, setIn, shouldPollute = false
	//
	// Be able to customize getIn and setIn.
	//
	// type getIn = (rootData, path) => valueAtPath
	// type setIn = (rootData, path, value) => rootData
	//
	// Useful for Immutable.js objects
	var reduceInWith = function reduceInWith() {
	    var getIn = arguments.length <= 0 || arguments[0] === undefined ? __getIn : arguments[0];
	    var setIn = arguments.length <= 1 || arguments[1] === undefined ? __setIn : arguments[1];
	    var shouldPollute = arguments.length <= 2 || arguments[2] === undefined ? false : arguments[2];
	
	    return function (reducer, path, action) {
	        return reduceIn(reducer, path, action, getIn, setIn, shouldPollute);
	    };
	};
	
	module.exports = {
	
	    // export reducer
	    makeReducer: makeReducer,
	
	    // reducer application
	    reduceIn: reduceIn,
	    reduceInWith: reduceInWith
	};

/***/ },

/***/ 683:
/***/ function(module, exports, __webpack_require__) {

	var baseSet = __webpack_require__(684);
	
	/**
	 * Sets the value at `path` of `object`. If a portion of `path` doesn't exist,
	 * it's created. Arrays are created for missing index properties while objects
	 * are created for all other missing properties. Use `_.setWith` to customize
	 * `path` creation.
	 *
	 * **Note:** This method mutates `object`.
	 *
	 * @static
	 * @memberOf _
	 * @since 3.7.0
	 * @category Object
	 * @param {Object} object The object to modify.
	 * @param {Array|string} path The path of the property to set.
	 * @param {*} value The value to set.
	 * @returns {Object} Returns `object`.
	 * @example
	 *
	 * var object = { 'a': [{ 'b': { 'c': 3 } }] };
	 *
	 * _.set(object, 'a[0].b.c', 4);
	 * console.log(object.a[0].b.c);
	 * // => 4
	 *
	 * _.set(object, ['x', '0', 'y', 'z'], 5);
	 * console.log(object.x[0].y.z);
	 * // => 5
	 */
	function set(object, path, value) {
	  return object == null ? object : baseSet(object, path, value);
	}
	
	module.exports = set;


/***/ },

/***/ 684:
/***/ function(module, exports, __webpack_require__) {

	var assignValue = __webpack_require__(685),
	    castPath = __webpack_require__(396),
	    isIndex = __webpack_require__(318),
	    isKey = __webpack_require__(402),
	    isObject = __webpack_require__(314),
	    toKey = __webpack_require__(403);
	
	/**
	 * The base implementation of `_.set`.
	 *
	 * @private
	 * @param {Object} object The object to modify.
	 * @param {Array|string} path The path of the property to set.
	 * @param {*} value The value to set.
	 * @param {Function} [customizer] The function to customize path creation.
	 * @returns {Object} Returns `object`.
	 */
	function baseSet(object, path, value, customizer) {
	  if (!isObject(object)) {
	    return object;
	  }
	  path = isKey(path, object) ? [path] : castPath(path);
	
	  var index = -1,
	      length = path.length,
	      lastIndex = length - 1,
	      nested = object;
	
	  while (nested != null && ++index < length) {
	    var key = toKey(path[index]),
	        newValue = value;
	
	    if (index != lastIndex) {
	      var objValue = nested[key];
	      newValue = customizer ? customizer(objValue, key, nested) : undefined;
	      if (newValue === undefined) {
	        newValue = isObject(objValue)
	          ? objValue
	          : (isIndex(path[index + 1]) ? [] : {});
	      }
	    }
	    assignValue(nested, key, newValue);
	    nested = nested[key];
	  }
	  return object;
	}
	
	module.exports = baseSet;


/***/ },

/***/ 685:
/***/ function(module, exports, __webpack_require__) {

	var eq = __webpack_require__(332);
	
	/** Used for built-in method references. */
	var objectProto = Object.prototype;
	
	/** Used to check objects for own properties. */
	var hasOwnProperty = objectProto.hasOwnProperty;
	
	/**
	 * Assigns `value` to `key` of `object` if the existing value is not equivalent
	 * using [`SameValueZero`](http://ecma-international.org/ecma-262/7.0/#sec-samevaluezero)
	 * for equality comparisons.
	 *
	 * @private
	 * @param {Object} object The object to modify.
	 * @param {string} key The key of the property to assign.
	 * @param {*} value The value to assign.
	 */
	function assignValue(object, key, value) {
	  var objValue = object[key];
	  if (!(hasOwnProperty.call(object, key) && eq(objValue, value)) ||
	      (value === undefined && !(key in object))) {
	    object[key] = value;
	  }
	}
	
	module.exports = assignValue;


/***/ },

/***/ 686:
/***/ function(module, exports, __webpack_require__) {

	var baseMerge = __webpack_require__(687),
	    createAssigner = __webpack_require__(721);
	
	/**
	 * This method is like `_.assign` except that it recursively merges own and
	 * inherited enumerable string keyed properties of source objects into the
	 * destination object. Source properties that resolve to `undefined` are
	 * skipped if a destination value exists. Array and plain object properties
	 * are merged recursively. Other objects and value types are overridden by
	 * assignment. Source objects are applied from left to right. Subsequent
	 * sources overwrite property assignments of previous sources.
	 *
	 * **Note:** This method mutates `object`.
	 *
	 * @static
	 * @memberOf _
	 * @since 0.5.0
	 * @category Object
	 * @param {Object} object The destination object.
	 * @param {...Object} [sources] The source objects.
	 * @returns {Object} Returns `object`.
	 * @example
	 *
	 * var object = {
	 *   'a': [{ 'b': 2 }, { 'd': 4 }]
	 * };
	 *
	 * var other = {
	 *   'a': [{ 'c': 3 }, { 'e': 5 }]
	 * };
	 *
	 * _.merge(object, other);
	 * // => { 'a': [{ 'b': 2, 'c': 3 }, { 'd': 4, 'e': 5 }] }
	 */
	var merge = createAssigner(function(object, source, srcIndex) {
	  baseMerge(object, source, srcIndex);
	});
	
	module.exports = merge;


/***/ },

/***/ 687:
/***/ function(module, exports, __webpack_require__) {

	var Stack = __webpack_require__(327),
	    arrayEach = __webpack_require__(302),
	    assignMergeValue = __webpack_require__(688),
	    baseKeysIn = __webpack_require__(689),
	    baseMergeDeep = __webpack_require__(691),
	    isArray = __webpack_require__(317),
	    isObject = __webpack_require__(314),
	    isTypedArray = __webpack_require__(385);
	
	/**
	 * The base implementation of `_.merge` without support for multiple sources.
	 *
	 * @private
	 * @param {Object} object The destination object.
	 * @param {Object} source The source object.
	 * @param {number} srcIndex The index of `source`.
	 * @param {Function} [customizer] The function to customize merged values.
	 * @param {Object} [stack] Tracks traversed source values and their merged
	 *  counterparts.
	 */
	function baseMerge(object, source, srcIndex, customizer, stack) {
	  if (object === source) {
	    return;
	  }
	  if (!(isArray(source) || isTypedArray(source))) {
	    var props = baseKeysIn(source);
	  }
	  arrayEach(props || source, function(srcValue, key) {
	    if (props) {
	      key = srcValue;
	      srcValue = source[key];
	    }
	    if (isObject(srcValue)) {
	      stack || (stack = new Stack);
	      baseMergeDeep(object, source, key, srcIndex, baseMerge, customizer, stack);
	    }
	    else {
	      var newValue = customizer
	        ? customizer(object[key], srcValue, (key + ''), object, source, stack)
	        : undefined;
	
	      if (newValue === undefined) {
	        newValue = srcValue;
	      }
	      assignMergeValue(object, key, newValue);
	    }
	  });
	}
	
	module.exports = baseMerge;


/***/ },

/***/ 688:
/***/ function(module, exports, __webpack_require__) {

	var eq = __webpack_require__(332);
	
	/**
	 * This function is like `assignValue` except that it doesn't assign
	 * `undefined` values.
	 *
	 * @private
	 * @param {Object} object The object to modify.
	 * @param {string} key The key of the property to assign.
	 * @param {*} value The value to assign.
	 */
	function assignMergeValue(object, key, value) {
	  if ((value !== undefined && !eq(object[key], value)) ||
	      (typeof key == 'number' && value === undefined && !(key in object))) {
	    object[key] = value;
	  }
	}
	
	module.exports = assignMergeValue;


/***/ },

/***/ 689:
/***/ function(module, exports, __webpack_require__) {

	var isObject = __webpack_require__(314),
	    isPrototype = __webpack_require__(320),
	    nativeKeysIn = __webpack_require__(690);
	
	/** Used for built-in method references. */
	var objectProto = Object.prototype;
	
	/** Used to check objects for own properties. */
	var hasOwnProperty = objectProto.hasOwnProperty;
	
	/**
	 * The base implementation of `_.keysIn` which doesn't treat sparse arrays as dense.
	 *
	 * @private
	 * @param {Object} object The object to query.
	 * @returns {Array} Returns the array of property names.
	 */
	function baseKeysIn(object) {
	  if (!isObject(object)) {
	    return nativeKeysIn(object);
	  }
	  var isProto = isPrototype(object),
	      result = [];
	
	  for (var key in object) {
	    if (!(key == 'constructor' && (isProto || !hasOwnProperty.call(object, key)))) {
	      result.push(key);
	    }
	  }
	  return result;
	}
	
	module.exports = baseKeysIn;


/***/ },

/***/ 690:
/***/ function(module, exports) {

	/**
	 * This function is like
	 * [`Object.keys`](http://ecma-international.org/ecma-262/7.0/#sec-object.keys)
	 * except that it includes inherited enumerable properties.
	 *
	 * @private
	 * @param {Object} object The object to query.
	 * @returns {Array} Returns the array of property names.
	 */
	function nativeKeysIn(object) {
	  var result = [];
	  if (object != null) {
	    for (var key in Object(object)) {
	      result.push(key);
	    }
	  }
	  return result;
	}
	
	module.exports = nativeKeysIn;


/***/ },

/***/ 691:
/***/ function(module, exports, __webpack_require__) {

	var assignMergeValue = __webpack_require__(688),
	    baseClone = __webpack_require__(692),
	    copyArray = __webpack_require__(696),
	    isArguments = __webpack_require__(310),
	    isArray = __webpack_require__(317),
	    isArrayLikeObject = __webpack_require__(311),
	    isFunction = __webpack_require__(313),
	    isObject = __webpack_require__(314),
	    isPlainObject = __webpack_require__(669),
	    isTypedArray = __webpack_require__(385),
	    toPlainObject = __webpack_require__(719);
	
	/**
	 * A specialized version of `baseMerge` for arrays and objects which performs
	 * deep merges and tracks traversed objects enabling objects with circular
	 * references to be merged.
	 *
	 * @private
	 * @param {Object} object The destination object.
	 * @param {Object} source The source object.
	 * @param {string} key The key of the value to merge.
	 * @param {number} srcIndex The index of `source`.
	 * @param {Function} mergeFunc The function to merge values.
	 * @param {Function} [customizer] The function to customize assigned values.
	 * @param {Object} [stack] Tracks traversed source values and their merged
	 *  counterparts.
	 */
	function baseMergeDeep(object, source, key, srcIndex, mergeFunc, customizer, stack) {
	  var objValue = object[key],
	      srcValue = source[key],
	      stacked = stack.get(srcValue);
	
	  if (stacked) {
	    assignMergeValue(object, key, stacked);
	    return;
	  }
	  var newValue = customizer
	    ? customizer(objValue, srcValue, (key + ''), object, source, stack)
	    : undefined;
	
	  var isCommon = newValue === undefined;
	
	  if (isCommon) {
	    newValue = srcValue;
	    if (isArray(srcValue) || isTypedArray(srcValue)) {
	      if (isArray(objValue)) {
	        newValue = objValue;
	      }
	      else if (isArrayLikeObject(objValue)) {
	        newValue = copyArray(objValue);
	      }
	      else {
	        isCommon = false;
	        newValue = baseClone(srcValue, true);
	      }
	    }
	    else if (isPlainObject(srcValue) || isArguments(srcValue)) {
	      if (isArguments(objValue)) {
	        newValue = toPlainObject(objValue);
	      }
	      else if (!isObject(objValue) || (srcIndex && isFunction(objValue))) {
	        isCommon = false;
	        newValue = baseClone(srcValue, true);
	      }
	      else {
	        newValue = objValue;
	      }
	    }
	    else {
	      isCommon = false;
	    }
	  }
	  if (isCommon) {
	    // Recursively merge objects and arrays (susceptible to call stack limits).
	    stack.set(srcValue, newValue);
	    mergeFunc(newValue, srcValue, srcIndex, customizer, stack);
	    stack['delete'](srcValue);
	  }
	  assignMergeValue(object, key, newValue);
	}
	
	module.exports = baseMergeDeep;


/***/ },

/***/ 692:
/***/ function(module, exports, __webpack_require__) {

	var Stack = __webpack_require__(327),
	    arrayEach = __webpack_require__(302),
	    assignValue = __webpack_require__(685),
	    baseAssign = __webpack_require__(693),
	    cloneBuffer = __webpack_require__(695),
	    copyArray = __webpack_require__(696),
	    copySymbols = __webpack_require__(697),
	    getAllKeys = __webpack_require__(700),
	    getTag = __webpack_require__(379),
	    initCloneArray = __webpack_require__(703),
	    initCloneByTag = __webpack_require__(704),
	    initCloneObject = __webpack_require__(715),
	    isArray = __webpack_require__(317),
	    isBuffer = __webpack_require__(717),
	    isHostObject = __webpack_require__(344),
	    isObject = __webpack_require__(314),
	    keys = __webpack_require__(307);
	
	/** `Object#toString` result references. */
	var argsTag = '[object Arguments]',
	    arrayTag = '[object Array]',
	    boolTag = '[object Boolean]',
	    dateTag = '[object Date]',
	    errorTag = '[object Error]',
	    funcTag = '[object Function]',
	    genTag = '[object GeneratorFunction]',
	    mapTag = '[object Map]',
	    numberTag = '[object Number]',
	    objectTag = '[object Object]',
	    regexpTag = '[object RegExp]',
	    setTag = '[object Set]',
	    stringTag = '[object String]',
	    symbolTag = '[object Symbol]',
	    weakMapTag = '[object WeakMap]';
	
	var arrayBufferTag = '[object ArrayBuffer]',
	    dataViewTag = '[object DataView]',
	    float32Tag = '[object Float32Array]',
	    float64Tag = '[object Float64Array]',
	    int8Tag = '[object Int8Array]',
	    int16Tag = '[object Int16Array]',
	    int32Tag = '[object Int32Array]',
	    uint8Tag = '[object Uint8Array]',
	    uint8ClampedTag = '[object Uint8ClampedArray]',
	    uint16Tag = '[object Uint16Array]',
	    uint32Tag = '[object Uint32Array]';
	
	/** Used to identify `toStringTag` values supported by `_.clone`. */
	var cloneableTags = {};
	cloneableTags[argsTag] = cloneableTags[arrayTag] =
	cloneableTags[arrayBufferTag] = cloneableTags[dataViewTag] =
	cloneableTags[boolTag] = cloneableTags[dateTag] =
	cloneableTags[float32Tag] = cloneableTags[float64Tag] =
	cloneableTags[int8Tag] = cloneableTags[int16Tag] =
	cloneableTags[int32Tag] = cloneableTags[mapTag] =
	cloneableTags[numberTag] = cloneableTags[objectTag] =
	cloneableTags[regexpTag] = cloneableTags[setTag] =
	cloneableTags[stringTag] = cloneableTags[symbolTag] =
	cloneableTags[uint8Tag] = cloneableTags[uint8ClampedTag] =
	cloneableTags[uint16Tag] = cloneableTags[uint32Tag] = true;
	cloneableTags[errorTag] = cloneableTags[funcTag] =
	cloneableTags[weakMapTag] = false;
	
	/**
	 * The base implementation of `_.clone` and `_.cloneDeep` which tracks
	 * traversed objects.
	 *
	 * @private
	 * @param {*} value The value to clone.
	 * @param {boolean} [isDeep] Specify a deep clone.
	 * @param {boolean} [isFull] Specify a clone including symbols.
	 * @param {Function} [customizer] The function to customize cloning.
	 * @param {string} [key] The key of `value`.
	 * @param {Object} [object] The parent object of `value`.
	 * @param {Object} [stack] Tracks traversed objects and their clone counterparts.
	 * @returns {*} Returns the cloned value.
	 */
	function baseClone(value, isDeep, isFull, customizer, key, object, stack) {
	  var result;
	  if (customizer) {
	    result = object ? customizer(value, key, object, stack) : customizer(value);
	  }
	  if (result !== undefined) {
	    return result;
	  }
	  if (!isObject(value)) {
	    return value;
	  }
	  var isArr = isArray(value);
	  if (isArr) {
	    result = initCloneArray(value);
	    if (!isDeep) {
	      return copyArray(value, result);
	    }
	  } else {
	    var tag = getTag(value),
	        isFunc = tag == funcTag || tag == genTag;
	
	    if (isBuffer(value)) {
	      return cloneBuffer(value, isDeep);
	    }
	    if (tag == objectTag || tag == argsTag || (isFunc && !object)) {
	      if (isHostObject(value)) {
	        return object ? value : {};
	      }
	      result = initCloneObject(isFunc ? {} : value);
	      if (!isDeep) {
	        return copySymbols(value, baseAssign(result, value));
	      }
	    } else {
	      if (!cloneableTags[tag]) {
	        return object ? value : {};
	      }
	      result = initCloneByTag(value, tag, baseClone, isDeep);
	    }
	  }
	  // Check for circular references and return its corresponding clone.
	  stack || (stack = new Stack);
	  var stacked = stack.get(value);
	  if (stacked) {
	    return stacked;
	  }
	  stack.set(value, result);
	
	  if (!isArr) {
	    var props = isFull ? getAllKeys(value) : keys(value);
	  }
	  arrayEach(props || value, function(subValue, key) {
	    if (props) {
	      key = subValue;
	      subValue = value[key];
	    }
	    // Recursively populate clone (susceptible to call stack limits).
	    assignValue(result, key, baseClone(subValue, isDeep, isFull, customizer, key, value, stack));
	  });
	  return result;
	}
	
	module.exports = baseClone;


/***/ },

/***/ 693:
/***/ function(module, exports, __webpack_require__) {

	var copyObject = __webpack_require__(694),
	    keys = __webpack_require__(307);
	
	/**
	 * The base implementation of `_.assign` without support for multiple sources
	 * or `customizer` functions.
	 *
	 * @private
	 * @param {Object} object The destination object.
	 * @param {Object} source The source object.
	 * @returns {Object} Returns `object`.
	 */
	function baseAssign(object, source) {
	  return object && copyObject(source, keys(source), object);
	}
	
	module.exports = baseAssign;


/***/ },

/***/ 694:
/***/ function(module, exports, __webpack_require__) {

	var assignValue = __webpack_require__(685);
	
	/**
	 * Copies properties of `source` to `object`.
	 *
	 * @private
	 * @param {Object} source The object to copy properties from.
	 * @param {Array} props The property identifiers to copy.
	 * @param {Object} [object={}] The object to copy properties to.
	 * @param {Function} [customizer] The function to customize copied values.
	 * @returns {Object} Returns `object`.
	 */
	function copyObject(source, props, object, customizer) {
	  object || (object = {});
	
	  var index = -1,
	      length = props.length;
	
	  while (++index < length) {
	    var key = props[index];
	
	    var newValue = customizer
	      ? customizer(object[key], source[key], key, object, source)
	      : undefined;
	
	    assignValue(object, key, newValue === undefined ? source[key] : newValue);
	  }
	  return object;
	}
	
	module.exports = copyObject;


/***/ },

/***/ 695:
/***/ function(module, exports) {

	/**
	 * Creates a clone of  `buffer`.
	 *
	 * @private
	 * @param {Buffer} buffer The buffer to clone.
	 * @param {boolean} [isDeep] Specify a deep clone.
	 * @returns {Buffer} Returns the cloned buffer.
	 */
	function cloneBuffer(buffer, isDeep) {
	  if (isDeep) {
	    return buffer.slice();
	  }
	  var result = new buffer.constructor(buffer.length);
	  buffer.copy(result);
	  return result;
	}
	
	module.exports = cloneBuffer;


/***/ },

/***/ 696:
/***/ function(module, exports) {

	/**
	 * Copies the values of `source` to `array`.
	 *
	 * @private
	 * @param {Array} source The array to copy values from.
	 * @param {Array} [array=[]] The array to copy values to.
	 * @returns {Array} Returns `array`.
	 */
	function copyArray(source, array) {
	  var index = -1,
	      length = source.length;
	
	  array || (array = Array(length));
	  while (++index < length) {
	    array[index] = source[index];
	  }
	  return array;
	}
	
	module.exports = copyArray;


/***/ },

/***/ 697:
/***/ function(module, exports, __webpack_require__) {

	var copyObject = __webpack_require__(694),
	    getSymbols = __webpack_require__(698);
	
	/**
	 * Copies own symbol properties of `source` to `object`.
	 *
	 * @private
	 * @param {Object} source The object to copy symbols from.
	 * @param {Object} [object={}] The object to copy symbols to.
	 * @returns {Object} Returns `object`.
	 */
	function copySymbols(source, object) {
	  return copyObject(source, getSymbols(source), object);
	}
	
	module.exports = copySymbols;


/***/ },

/***/ 698:
/***/ function(module, exports, __webpack_require__) {

	var overArg = __webpack_require__(322),
	    stubArray = __webpack_require__(699);
	
	/* Built-in method references for those with the same name as other `lodash` methods. */
	var nativeGetSymbols = Object.getOwnPropertySymbols;
	
	/**
	 * Creates an array of the own enumerable symbol properties of `object`.
	 *
	 * @private
	 * @param {Object} object The object to query.
	 * @returns {Array} Returns the array of symbols.
	 */
	var getSymbols = nativeGetSymbols ? overArg(nativeGetSymbols, Object) : stubArray;
	
	module.exports = getSymbols;


/***/ },

/***/ 699:
/***/ function(module, exports) {

	/**
	 * This method returns a new empty array.
	 *
	 * @static
	 * @memberOf _
	 * @since 4.13.0
	 * @category Util
	 * @returns {Array} Returns the new empty array.
	 * @example
	 *
	 * var arrays = _.times(2, _.stubArray);
	 *
	 * console.log(arrays);
	 * // => [[], []]
	 *
	 * console.log(arrays[0] === arrays[1]);
	 * // => false
	 */
	function stubArray() {
	  return [];
	}
	
	module.exports = stubArray;


/***/ },

/***/ 700:
/***/ function(module, exports, __webpack_require__) {

	var baseGetAllKeys = __webpack_require__(701),
	    getSymbols = __webpack_require__(698),
	    keys = __webpack_require__(307);
	
	/**
	 * Creates an array of own enumerable property names and symbols of `object`.
	 *
	 * @private
	 * @param {Object} object The object to query.
	 * @returns {Array} Returns the array of property names and symbols.
	 */
	function getAllKeys(object) {
	  return baseGetAllKeys(object, keys, getSymbols);
	}
	
	module.exports = getAllKeys;


/***/ },

/***/ 701:
/***/ function(module, exports, __webpack_require__) {

	var arrayPush = __webpack_require__(702),
	    isArray = __webpack_require__(317);
	
	/**
	 * The base implementation of `getAllKeys` and `getAllKeysIn` which uses
	 * `keysFunc` and `symbolsFunc` to get the enumerable property names and
	 * symbols of `object`.
	 *
	 * @private
	 * @param {Object} object The object to query.
	 * @param {Function} keysFunc The function to get the keys of `object`.
	 * @param {Function} symbolsFunc The function to get the symbols of `object`.
	 * @returns {Array} Returns the array of property names and symbols.
	 */
	function baseGetAllKeys(object, keysFunc, symbolsFunc) {
	  var result = keysFunc(object);
	  return isArray(object) ? result : arrayPush(result, symbolsFunc(object));
	}
	
	module.exports = baseGetAllKeys;


/***/ },

/***/ 702:
/***/ function(module, exports) {

	/**
	 * Appends the elements of `values` to `array`.
	 *
	 * @private
	 * @param {Array} array The array to modify.
	 * @param {Array} values The values to append.
	 * @returns {Array} Returns `array`.
	 */
	function arrayPush(array, values) {
	  var index = -1,
	      length = values.length,
	      offset = array.length;
	
	  while (++index < length) {
	    array[offset + index] = values[index];
	  }
	  return array;
	}
	
	module.exports = arrayPush;


/***/ },

/***/ 703:
/***/ function(module, exports) {

	/** Used for built-in method references. */
	var objectProto = Object.prototype;
	
	/** Used to check objects for own properties. */
	var hasOwnProperty = objectProto.hasOwnProperty;
	
	/**
	 * Initializes an array clone.
	 *
	 * @private
	 * @param {Array} array The array to clone.
	 * @returns {Array} Returns the initialized clone.
	 */
	function initCloneArray(array) {
	  var length = array.length,
	      result = array.constructor(length);
	
	  // Add properties assigned by `RegExp#exec`.
	  if (length && typeof array[0] == 'string' && hasOwnProperty.call(array, 'index')) {
	    result.index = array.index;
	    result.input = array.input;
	  }
	  return result;
	}
	
	module.exports = initCloneArray;


/***/ },

/***/ 704:
/***/ function(module, exports, __webpack_require__) {

	var cloneArrayBuffer = __webpack_require__(705),
	    cloneDataView = __webpack_require__(706),
	    cloneMap = __webpack_require__(707),
	    cloneRegExp = __webpack_require__(710),
	    cloneSet = __webpack_require__(711),
	    cloneSymbol = __webpack_require__(713),
	    cloneTypedArray = __webpack_require__(714);
	
	/** `Object#toString` result references. */
	var boolTag = '[object Boolean]',
	    dateTag = '[object Date]',
	    mapTag = '[object Map]',
	    numberTag = '[object Number]',
	    regexpTag = '[object RegExp]',
	    setTag = '[object Set]',
	    stringTag = '[object String]',
	    symbolTag = '[object Symbol]';
	
	var arrayBufferTag = '[object ArrayBuffer]',
	    dataViewTag = '[object DataView]',
	    float32Tag = '[object Float32Array]',
	    float64Tag = '[object Float64Array]',
	    int8Tag = '[object Int8Array]',
	    int16Tag = '[object Int16Array]',
	    int32Tag = '[object Int32Array]',
	    uint8Tag = '[object Uint8Array]',
	    uint8ClampedTag = '[object Uint8ClampedArray]',
	    uint16Tag = '[object Uint16Array]',
	    uint32Tag = '[object Uint32Array]';
	
	/**
	 * Initializes an object clone based on its `toStringTag`.
	 *
	 * **Note:** This function only supports cloning values with tags of
	 * `Boolean`, `Date`, `Error`, `Number`, `RegExp`, or `String`.
	 *
	 * @private
	 * @param {Object} object The object to clone.
	 * @param {string} tag The `toStringTag` of the object to clone.
	 * @param {Function} cloneFunc The function to clone values.
	 * @param {boolean} [isDeep] Specify a deep clone.
	 * @returns {Object} Returns the initialized clone.
	 */
	function initCloneByTag(object, tag, cloneFunc, isDeep) {
	  var Ctor = object.constructor;
	  switch (tag) {
	    case arrayBufferTag:
	      return cloneArrayBuffer(object);
	
	    case boolTag:
	    case dateTag:
	      return new Ctor(+object);
	
	    case dataViewTag:
	      return cloneDataView(object, isDeep);
	
	    case float32Tag: case float64Tag:
	    case int8Tag: case int16Tag: case int32Tag:
	    case uint8Tag: case uint8ClampedTag: case uint16Tag: case uint32Tag:
	      return cloneTypedArray(object, isDeep);
	
	    case mapTag:
	      return cloneMap(object, isDeep, cloneFunc);
	
	    case numberTag:
	    case stringTag:
	      return new Ctor(object);
	
	    case regexpTag:
	      return cloneRegExp(object);
	
	    case setTag:
	      return cloneSet(object, isDeep, cloneFunc);
	
	    case symbolTag:
	      return cloneSymbol(object);
	  }
	}
	
	module.exports = initCloneByTag;


/***/ },

/***/ 705:
/***/ function(module, exports, __webpack_require__) {

	var Uint8Array = __webpack_require__(375);
	
	/**
	 * Creates a clone of `arrayBuffer`.
	 *
	 * @private
	 * @param {ArrayBuffer} arrayBuffer The array buffer to clone.
	 * @returns {ArrayBuffer} Returns the cloned array buffer.
	 */
	function cloneArrayBuffer(arrayBuffer) {
	  var result = new arrayBuffer.constructor(arrayBuffer.byteLength);
	  new Uint8Array(result).set(new Uint8Array(arrayBuffer));
	  return result;
	}
	
	module.exports = cloneArrayBuffer;


/***/ },

/***/ 706:
/***/ function(module, exports, __webpack_require__) {

	var cloneArrayBuffer = __webpack_require__(705);
	
	/**
	 * Creates a clone of `dataView`.
	 *
	 * @private
	 * @param {Object} dataView The data view to clone.
	 * @param {boolean} [isDeep] Specify a deep clone.
	 * @returns {Object} Returns the cloned data view.
	 */
	function cloneDataView(dataView, isDeep) {
	  var buffer = isDeep ? cloneArrayBuffer(dataView.buffer) : dataView.buffer;
	  return new dataView.constructor(buffer, dataView.byteOffset, dataView.byteLength);
	}
	
	module.exports = cloneDataView;


/***/ },

/***/ 707:
/***/ function(module, exports, __webpack_require__) {

	var addMapEntry = __webpack_require__(708),
	    arrayReduce = __webpack_require__(709),
	    mapToArray = __webpack_require__(376);
	
	/**
	 * Creates a clone of `map`.
	 *
	 * @private
	 * @param {Object} map The map to clone.
	 * @param {Function} cloneFunc The function to clone values.
	 * @param {boolean} [isDeep] Specify a deep clone.
	 * @returns {Object} Returns the cloned map.
	 */
	function cloneMap(map, isDeep, cloneFunc) {
	  var array = isDeep ? cloneFunc(mapToArray(map), true) : mapToArray(map);
	  return arrayReduce(array, addMapEntry, new map.constructor);
	}
	
	module.exports = cloneMap;


/***/ },

/***/ 708:
/***/ function(module, exports) {

	/**
	 * Adds the key-value `pair` to `map`.
	 *
	 * @private
	 * @param {Object} map The map to modify.
	 * @param {Array} pair The key-value pair to add.
	 * @returns {Object} Returns `map`.
	 */
	function addMapEntry(map, pair) {
	  // Don't return `map.set` because it's not chainable in IE 11.
	  map.set(pair[0], pair[1]);
	  return map;
	}
	
	module.exports = addMapEntry;


/***/ },

/***/ 709:
/***/ function(module, exports) {

	/**
	 * A specialized version of `_.reduce` for arrays without support for
	 * iteratee shorthands.
	 *
	 * @private
	 * @param {Array} [array] The array to iterate over.
	 * @param {Function} iteratee The function invoked per iteration.
	 * @param {*} [accumulator] The initial value.
	 * @param {boolean} [initAccum] Specify using the first element of `array` as
	 *  the initial value.
	 * @returns {*} Returns the accumulated value.
	 */
	function arrayReduce(array, iteratee, accumulator, initAccum) {
	  var index = -1,
	      length = array ? array.length : 0;
	
	  if (initAccum && length) {
	    accumulator = array[++index];
	  }
	  while (++index < length) {
	    accumulator = iteratee(accumulator, array[index], index, array);
	  }
	  return accumulator;
	}
	
	module.exports = arrayReduce;


/***/ },

/***/ 710:
/***/ function(module, exports) {

	/** Used to match `RegExp` flags from their coerced string values. */
	var reFlags = /\w*$/;
	
	/**
	 * Creates a clone of `regexp`.
	 *
	 * @private
	 * @param {Object} regexp The regexp to clone.
	 * @returns {Object} Returns the cloned regexp.
	 */
	function cloneRegExp(regexp) {
	  var result = new regexp.constructor(regexp.source, reFlags.exec(regexp));
	  result.lastIndex = regexp.lastIndex;
	  return result;
	}
	
	module.exports = cloneRegExp;


/***/ },

/***/ 711:
/***/ function(module, exports, __webpack_require__) {

	var addSetEntry = __webpack_require__(712),
	    arrayReduce = __webpack_require__(709),
	    setToArray = __webpack_require__(377);
	
	/**
	 * Creates a clone of `set`.
	 *
	 * @private
	 * @param {Object} set The set to clone.
	 * @param {Function} cloneFunc The function to clone values.
	 * @param {boolean} [isDeep] Specify a deep clone.
	 * @returns {Object} Returns the cloned set.
	 */
	function cloneSet(set, isDeep, cloneFunc) {
	  var array = isDeep ? cloneFunc(setToArray(set), true) : setToArray(set);
	  return arrayReduce(array, addSetEntry, new set.constructor);
	}
	
	module.exports = cloneSet;


/***/ },

/***/ 712:
/***/ function(module, exports) {

	/**
	 * Adds `value` to `set`.
	 *
	 * @private
	 * @param {Object} set The set to modify.
	 * @param {*} value The value to add.
	 * @returns {Object} Returns `set`.
	 */
	function addSetEntry(set, value) {
	  // Don't return `set.add` because it's not chainable in IE 11.
	  set.add(value);
	  return set;
	}
	
	module.exports = addSetEntry;


/***/ },

/***/ 713:
/***/ function(module, exports, __webpack_require__) {

	var Symbol = __webpack_require__(374);
	
	/** Used to convert symbols to primitives and strings. */
	var symbolProto = Symbol ? Symbol.prototype : undefined,
	    symbolValueOf = symbolProto ? symbolProto.valueOf : undefined;
	
	/**
	 * Creates a clone of the `symbol` object.
	 *
	 * @private
	 * @param {Object} symbol The symbol object to clone.
	 * @returns {Object} Returns the cloned symbol object.
	 */
	function cloneSymbol(symbol) {
	  return symbolValueOf ? Object(symbolValueOf.call(symbol)) : {};
	}
	
	module.exports = cloneSymbol;


/***/ },

/***/ 714:
/***/ function(module, exports, __webpack_require__) {

	var cloneArrayBuffer = __webpack_require__(705);
	
	/**
	 * Creates a clone of `typedArray`.
	 *
	 * @private
	 * @param {Object} typedArray The typed array to clone.
	 * @param {boolean} [isDeep] Specify a deep clone.
	 * @returns {Object} Returns the cloned typed array.
	 */
	function cloneTypedArray(typedArray, isDeep) {
	  var buffer = isDeep ? cloneArrayBuffer(typedArray.buffer) : typedArray.buffer;
	  return new typedArray.constructor(buffer, typedArray.byteOffset, typedArray.length);
	}
	
	module.exports = cloneTypedArray;


/***/ },

/***/ 715:
/***/ function(module, exports, __webpack_require__) {

	var baseCreate = __webpack_require__(716),
	    getPrototype = __webpack_require__(670),
	    isPrototype = __webpack_require__(320);
	
	/**
	 * Initializes an object clone.
	 *
	 * @private
	 * @param {Object} object The object to clone.
	 * @returns {Object} Returns the initialized clone.
	 */
	function initCloneObject(object) {
	  return (typeof object.constructor == 'function' && !isPrototype(object))
	    ? baseCreate(getPrototype(object))
	    : {};
	}
	
	module.exports = initCloneObject;


/***/ },

/***/ 716:
/***/ function(module, exports, __webpack_require__) {

	var isObject = __webpack_require__(314);
	
	/** Built-in value references. */
	var objectCreate = Object.create;
	
	/**
	 * The base implementation of `_.create` without support for assigning
	 * properties to the created object.
	 *
	 * @private
	 * @param {Object} prototype The object to inherit from.
	 * @returns {Object} Returns the new object.
	 */
	function baseCreate(proto) {
	  return isObject(proto) ? objectCreate(proto) : {};
	}
	
	module.exports = baseCreate;


/***/ },

/***/ 717:
/***/ function(module, exports, __webpack_require__) {

	/* WEBPACK VAR INJECTION */(function(module) {var root = __webpack_require__(347),
	    stubFalse = __webpack_require__(718);
	
	/** Detect free variable `exports`. */
	var freeExports = typeof exports == 'object' && exports && !exports.nodeType && exports;
	
	/** Detect free variable `module`. */
	var freeModule = freeExports && typeof module == 'object' && module && !module.nodeType && module;
	
	/** Detect the popular CommonJS extension `module.exports`. */
	var moduleExports = freeModule && freeModule.exports === freeExports;
	
	/** Built-in value references. */
	var Buffer = moduleExports ? root.Buffer : undefined;
	
	/* Built-in method references for those with the same name as other `lodash` methods. */
	var nativeIsBuffer = Buffer ? Buffer.isBuffer : undefined;
	
	/**
	 * Checks if `value` is a buffer.
	 *
	 * @static
	 * @memberOf _
	 * @since 4.3.0
	 * @category Lang
	 * @param {*} value The value to check.
	 * @returns {boolean} Returns `true` if `value` is a buffer, else `false`.
	 * @example
	 *
	 * _.isBuffer(new Buffer(2));
	 * // => true
	 *
	 * _.isBuffer(new Uint8Array(2));
	 * // => false
	 */
	var isBuffer = nativeIsBuffer || stubFalse;
	
	module.exports = isBuffer;
	
	/* WEBPACK VAR INJECTION */}.call(exports, __webpack_require__(389)(module)))

/***/ },

/***/ 718:
/***/ function(module, exports) {

	/**
	 * This method returns `false`.
	 *
	 * @static
	 * @memberOf _
	 * @since 4.13.0
	 * @category Util
	 * @returns {boolean} Returns `false`.
	 * @example
	 *
	 * _.times(2, _.stubFalse);
	 * // => [false, false]
	 */
	function stubFalse() {
	  return false;
	}
	
	module.exports = stubFalse;


/***/ },

/***/ 719:
/***/ function(module, exports, __webpack_require__) {

	var copyObject = __webpack_require__(694),
	    keysIn = __webpack_require__(720);
	
	/**
	 * Converts `value` to a plain object flattening inherited enumerable string
	 * keyed properties of `value` to own properties of the plain object.
	 *
	 * @static
	 * @memberOf _
	 * @since 3.0.0
	 * @category Lang
	 * @param {*} value The value to convert.
	 * @returns {Object} Returns the converted plain object.
	 * @example
	 *
	 * function Foo() {
	 *   this.b = 2;
	 * }
	 *
	 * Foo.prototype.c = 3;
	 *
	 * _.assign({ 'a': 1 }, new Foo);
	 * // => { 'a': 1, 'b': 2 }
	 *
	 * _.assign({ 'a': 1 }, _.toPlainObject(new Foo));
	 * // => { 'a': 1, 'b': 2, 'c': 3 }
	 */
	function toPlainObject(value) {
	  return copyObject(value, keysIn(value));
	}
	
	module.exports = toPlainObject;


/***/ },

/***/ 720:
/***/ function(module, exports, __webpack_require__) {

	var arrayLikeKeys = __webpack_require__(308),
	    baseKeysIn = __webpack_require__(689),
	    isArrayLike = __webpack_require__(312);
	
	/**
	 * Creates an array of the own and inherited enumerable property names of `object`.
	 *
	 * **Note:** Non-object values are coerced to objects.
	 *
	 * @static
	 * @memberOf _
	 * @since 3.0.0
	 * @category Object
	 * @param {Object} object The object to query.
	 * @returns {Array} Returns the array of property names.
	 * @example
	 *
	 * function Foo() {
	 *   this.a = 1;
	 *   this.b = 2;
	 * }
	 *
	 * Foo.prototype.c = 3;
	 *
	 * _.keysIn(new Foo);
	 * // => ['a', 'b', 'c'] (iteration order is not guaranteed)
	 */
	function keysIn(object) {
	  return isArrayLike(object) ? arrayLikeKeys(object, true) : baseKeysIn(object);
	}
	
	module.exports = keysIn;


/***/ },

/***/ 721:
/***/ function(module, exports, __webpack_require__) {

	var baseRest = __webpack_require__(722),
	    isIterateeCall = __webpack_require__(724);
	
	/**
	 * Creates a function like `_.assign`.
	 *
	 * @private
	 * @param {Function} assigner The function to assign values.
	 * @returns {Function} Returns the new assigner function.
	 */
	function createAssigner(assigner) {
	  return baseRest(function(object, sources) {
	    var index = -1,
	        length = sources.length,
	        customizer = length > 1 ? sources[length - 1] : undefined,
	        guard = length > 2 ? sources[2] : undefined;
	
	    customizer = (assigner.length > 3 && typeof customizer == 'function')
	      ? (length--, customizer)
	      : undefined;
	
	    if (guard && isIterateeCall(sources[0], sources[1], guard)) {
	      customizer = length < 3 ? undefined : customizer;
	      length = 1;
	    }
	    object = Object(object);
	    while (++index < length) {
	      var source = sources[index];
	      if (source) {
	        assigner(object, source, index, customizer);
	      }
	    }
	    return object;
	  });
	}
	
	module.exports = createAssigner;


/***/ },

/***/ 722:
/***/ function(module, exports, __webpack_require__) {

	var apply = __webpack_require__(723);
	
	/* Built-in method references for those with the same name as other `lodash` methods. */
	var nativeMax = Math.max;
	
	/**
	 * The base implementation of `_.rest` which doesn't validate or coerce arguments.
	 *
	 * @private
	 * @param {Function} func The function to apply a rest parameter to.
	 * @param {number} [start=func.length-1] The start position of the rest parameter.
	 * @returns {Function} Returns the new function.
	 */
	function baseRest(func, start) {
	  start = nativeMax(start === undefined ? (func.length - 1) : start, 0);
	  return function() {
	    var args = arguments,
	        index = -1,
	        length = nativeMax(args.length - start, 0),
	        array = Array(length);
	
	    while (++index < length) {
	      array[index] = args[start + index];
	    }
	    index = -1;
	    var otherArgs = Array(start + 1);
	    while (++index < start) {
	      otherArgs[index] = args[index];
	    }
	    otherArgs[start] = array;
	    return apply(func, this, otherArgs);
	  };
	}
	
	module.exports = baseRest;


/***/ },

/***/ 723:
/***/ function(module, exports) {

	/**
	 * A faster alternative to `Function#apply`, this function invokes `func`
	 * with the `this` binding of `thisArg` and the arguments of `args`.
	 *
	 * @private
	 * @param {Function} func The function to invoke.
	 * @param {*} thisArg The `this` binding of `func`.
	 * @param {Array} args The arguments to invoke `func` with.
	 * @returns {*} Returns the result of `func`.
	 */
	function apply(func, thisArg, args) {
	  switch (args.length) {
	    case 0: return func.call(thisArg);
	    case 1: return func.call(thisArg, args[0]);
	    case 2: return func.call(thisArg, args[0], args[1]);
	    case 3: return func.call(thisArg, args[0], args[1], args[2]);
	  }
	  return func.apply(thisArg, args);
	}
	
	module.exports = apply;


/***/ },

/***/ 724:
/***/ function(module, exports, __webpack_require__) {

	var eq = __webpack_require__(332),
	    isArrayLike = __webpack_require__(312),
	    isIndex = __webpack_require__(318),
	    isObject = __webpack_require__(314);
	
	/**
	 * Checks if the given arguments are from an iteratee call.
	 *
	 * @private
	 * @param {*} value The potential iteratee value argument.
	 * @param {*} index The potential iteratee index or key argument.
	 * @param {*} object The potential iteratee object argument.
	 * @returns {boolean} Returns `true` if the arguments are from an iteratee call,
	 *  else `false`.
	 */
	function isIterateeCall(value, index, object) {
	  if (!isObject(object)) {
	    return false;
	  }
	  var type = typeof index;
	  if (type == 'number'
	        ? (isArrayLike(object) && isIndex(index, object.length))
	        : (type == 'string' && index in object)
	      ) {
	    return eq(object[index], value);
	  }
	  return false;
	}
	
	module.exports = isIterateeCall;


/***/ },

/***/ 725:
/***/ function(module, exports, __webpack_require__) {

	var baseMerge = __webpack_require__(687),
	    createAssigner = __webpack_require__(721);
	
	/**
	 * This method is like `_.merge` except that it accepts `customizer` which
	 * is invoked to produce the merged values of the destination and source
	 * properties. If `customizer` returns `undefined`, merging is handled by the
	 * method instead. The `customizer` is invoked with seven arguments:
	 * (objValue, srcValue, key, object, source, stack).
	 *
	 * **Note:** This method mutates `object`.
	 *
	 * @static
	 * @memberOf _
	 * @since 4.0.0
	 * @category Object
	 * @param {Object} object The destination object.
	 * @param {...Object} sources The source objects.
	 * @param {Function} customizer The function to customize assigned values.
	 * @returns {Object} Returns `object`.
	 * @example
	 *
	 * function customizer(objValue, srcValue) {
	 *   if (_.isArray(objValue)) {
	 *     return objValue.concat(srcValue);
	 *   }
	 * }
	 *
	 * var object = { 'a': [1], 'b': [2] };
	 * var other = { 'a': [3], 'b': [4] };
	 *
	 * _.mergeWith(object, other, customizer);
	 * // => { 'a': [1, 3], 'b': [2, 4] }
	 */
	var mergeWith = createAssigner(function(object, source, srcIndex, customizer) {
	  baseMerge(object, source, srcIndex, customizer);
	});
	
	module.exports = mergeWith;


/***/ },

/***/ 726:
/***/ function(module, exports, __webpack_require__) {

	var isArray = __webpack_require__(317),
	    isObjectLike = __webpack_require__(316);
	
	/** `Object#toString` result references. */
	var stringTag = '[object String]';
	
	/** Used for built-in method references. */
	var objectProto = Object.prototype;
	
	/**
	 * Used to resolve the
	 * [`toStringTag`](http://ecma-international.org/ecma-262/7.0/#sec-object.prototype.tostring)
	 * of values.
	 */
	var objectToString = objectProto.toString;
	
	/**
	 * Checks if `value` is classified as a `String` primitive or object.
	 *
	 * @static
	 * @since 0.1.0
	 * @memberOf _
	 * @category Lang
	 * @param {*} value The value to check.
	 * @returns {boolean} Returns `true` if `value` is a string, else `false`.
	 * @example
	 *
	 * _.isString('abc');
	 * // => true
	 *
	 * _.isString(1);
	 * // => false
	 */
	function isString(value) {
	  return typeof value == 'string' ||
	    (!isArray(value) && isObjectLike(value) && objectToString.call(value) == stringTag);
	}
	
	module.exports = isString;


/***/ },

/***/ 727:
/***/ function(module, exports, __webpack_require__) {

	var __WEBPACK_AMD_DEFINE_RESULT__;(function () {
	  'use strict';
	  
	  function fetchPonyfill(options) {
	    var Promise = options && options.Promise || self.Promise;
	    var XMLHttpRequest = options && options.XMLHttpRequest || self.XMLHttpRequest;
	  
	    return (function () {
	      var self = {};
	
	      (function(self) {
	        'use strict';
	
	        if (self.fetch) {
	          return
	        }
	
	        var support = {
	          searchParams: 'URLSearchParams' in self,
	          iterable: 'Symbol' in self && 'iterator' in Symbol,
	          blob: 'FileReader' in self && 'Blob' in self && (function() {
	            try {
	              new Blob()
	              return true
	            } catch(e) {
	              return false
	            }
	          })(),
	          formData: 'FormData' in self,
	          arrayBuffer: 'ArrayBuffer' in self
	        }
	
	        function normalizeName(name) {
	          if (typeof name !== 'string') {
	            name = String(name)
	          }
	          if (/[^a-z0-9\-#$%&'*+.\^_`|~]/i.test(name)) {
	            throw new TypeError('Invalid character in header field name')
	          }
	          return name.toLowerCase()
	        }
	
	        function normalizeValue(value) {
	          if (typeof value !== 'string') {
	            value = String(value)
	          }
	          return value
	        }
	
	        // Build a destructive iterator for the value list
	        function iteratorFor(items) {
	          var iterator = {
	            next: function() {
	              var value = items.shift()
	              return {done: value === undefined, value: value}
	            }
	          }
	
	          if (support.iterable) {
	            iterator[Symbol.iterator] = function() {
	              return iterator
	            }
	          }
	
	          return iterator
	        }
	
	        function Headers(headers) {
	          this.map = {}
	
	          if (headers instanceof Headers) {
	            headers.forEach(function(value, name) {
	              this.append(name, value)
	            }, this)
	
	          } else if (headers) {
	            Object.getOwnPropertyNames(headers).forEach(function(name) {
	              this.append(name, headers[name])
	            }, this)
	          }
	        }
	
	        Headers.prototype.append = function(name, value) {
	          name = normalizeName(name)
	          value = normalizeValue(value)
	          var list = this.map[name]
	          if (!list) {
	            list = []
	            this.map[name] = list
	          }
	          list.push(value)
	        }
	
	        Headers.prototype['delete'] = function(name) {
	          delete this.map[normalizeName(name)]
	        }
	
	        Headers.prototype.get = function(name) {
	          var values = this.map[normalizeName(name)]
	          return values ? values[0] : null
	        }
	
	        Headers.prototype.getAll = function(name) {
	          return this.map[normalizeName(name)] || []
	        }
	
	        Headers.prototype.has = function(name) {
	          return this.map.hasOwnProperty(normalizeName(name))
	        }
	
	        Headers.prototype.set = function(name, value) {
	          this.map[normalizeName(name)] = [normalizeValue(value)]
	        }
	
	        Headers.prototype.forEach = function(callback, thisArg) {
	          Object.getOwnPropertyNames(this.map).forEach(function(name) {
	            this.map[name].forEach(function(value) {
	              callback.call(thisArg, value, name, this)
	            }, this)
	          }, this)
	        }
	
	        Headers.prototype.keys = function() {
	          var items = []
	          this.forEach(function(value, name) { items.push(name) })
	          return iteratorFor(items)
	        }
	
	        Headers.prototype.values = function() {
	          var items = []
	          this.forEach(function(value) { items.push(value) })
	          return iteratorFor(items)
	        }
	
	        Headers.prototype.entries = function() {
	          var items = []
	          this.forEach(function(value, name) { items.push([name, value]) })
	          return iteratorFor(items)
	        }
	
	        if (support.iterable) {
	          Headers.prototype[Symbol.iterator] = Headers.prototype.entries
	        }
	
	        function consumed(body) {
	          if (body.bodyUsed) {
	            return Promise.reject(new TypeError('Already read'))
	          }
	          body.bodyUsed = true
	        }
	
	        function fileReaderReady(reader) {
	          return new Promise(function(resolve, reject) {
	            reader.onload = function() {
	              resolve(reader.result)
	            }
	            reader.onerror = function() {
	              reject(reader.error)
	            }
	          })
	        }
	
	        function readBlobAsArrayBuffer(blob) {
	          var reader = new FileReader()
	          reader.readAsArrayBuffer(blob)
	          return fileReaderReady(reader)
	        }
	
	        function readBlobAsText(blob) {
	          var reader = new FileReader()
	          reader.readAsText(blob)
	          return fileReaderReady(reader)
	        }
	
	        function Body() {
	          this.bodyUsed = false
	
	          this._initBody = function(body) {
	            this._bodyInit = body
	            if (typeof body === 'string') {
	              this._bodyText = body
	            } else if (support.blob && Blob.prototype.isPrototypeOf(body)) {
	              this._bodyBlob = body
	            } else if (support.formData && FormData.prototype.isPrototypeOf(body)) {
	              this._bodyFormData = body
	            } else if (support.searchParams && URLSearchParams.prototype.isPrototypeOf(body)) {
	              this._bodyText = body.toString()
	            } else if (!body) {
	              this._bodyText = ''
	            } else if (support.arrayBuffer && ArrayBuffer.prototype.isPrototypeOf(body)) {
	              // Only support ArrayBuffers for POST method.
	              // Receiving ArrayBuffers happens via Blobs, instead.
	            } else {
	              throw new Error('unsupported BodyInit type')
	            }
	
	            if (!this.headers.get('content-type')) {
	              if (typeof body === 'string') {
	                this.headers.set('content-type', 'text/plain;charset=UTF-8')
	              } else if (this._bodyBlob && this._bodyBlob.type) {
	                this.headers.set('content-type', this._bodyBlob.type)
	              } else if (support.searchParams && URLSearchParams.prototype.isPrototypeOf(body)) {
	                this.headers.set('content-type', 'application/x-www-form-urlencoded;charset=UTF-8')
	              }
	            }
	          }
	
	          if (support.blob) {
	            this.blob = function() {
	              var rejected = consumed(this)
	              if (rejected) {
	                return rejected
	              }
	
	              if (this._bodyBlob) {
	                return Promise.resolve(this._bodyBlob)
	              } else if (this._bodyFormData) {
	                throw new Error('could not read FormData body as blob')
	              } else {
	                return Promise.resolve(new Blob([this._bodyText]))
	              }
	            }
	
	            this.arrayBuffer = function() {
	              return this.blob().then(readBlobAsArrayBuffer)
	            }
	
	            this.text = function() {
	              var rejected = consumed(this)
	              if (rejected) {
	                return rejected
	              }
	
	              if (this._bodyBlob) {
	                return readBlobAsText(this._bodyBlob)
	              } else if (this._bodyFormData) {
	                throw new Error('could not read FormData body as text')
	              } else {
	                return Promise.resolve(this._bodyText)
	              }
	            }
	          } else {
	            this.text = function() {
	              var rejected = consumed(this)
	              return rejected ? rejected : Promise.resolve(this._bodyText)
	            }
	          }
	
	          if (support.formData) {
	            this.formData = function() {
	              return this.text().then(decode)
	            }
	          }
	
	          this.json = function() {
	            return this.text().then(JSON.parse)
	          }
	
	          return this
	        }
	
	        // HTTP methods whose capitalization should be normalized
	        var methods = ['DELETE', 'GET', 'HEAD', 'OPTIONS', 'POST', 'PUT']
	
	        function normalizeMethod(method) {
	          var upcased = method.toUpperCase()
	          return (methods.indexOf(upcased) > -1) ? upcased : method
	        }
	
	        function Request(input, options) {
	          options = options || {}
	          var body = options.body
	          if (Request.prototype.isPrototypeOf(input)) {
	            if (input.bodyUsed) {
	              throw new TypeError('Already read')
	            }
	            this.url = input.url
	            this.credentials = input.credentials
	            if (!options.headers) {
	              this.headers = new Headers(input.headers)
	            }
	            this.method = input.method
	            this.mode = input.mode
	            if (!body) {
	              body = input._bodyInit
	              input.bodyUsed = true
	            }
	          } else {
	            this.url = input
	          }
	
	          this.credentials = options.credentials || this.credentials || 'omit'
	          if (options.headers || !this.headers) {
	            this.headers = new Headers(options.headers)
	          }
	          this.method = normalizeMethod(options.method || this.method || 'GET')
	          this.mode = options.mode || this.mode || null
	          this.referrer = null
	
	          if ((this.method === 'GET' || this.method === 'HEAD') && body) {
	            throw new TypeError('Body not allowed for GET or HEAD requests')
	          }
	          this._initBody(body)
	        }
	
	        Request.prototype.clone = function() {
	          return new Request(this)
	        }
	
	        function decode(body) {
	          var form = new FormData()
	          body.trim().split('&').forEach(function(bytes) {
	            if (bytes) {
	              var split = bytes.split('=')
	              var name = split.shift().replace(/\+/g, ' ')
	              var value = split.join('=').replace(/\+/g, ' ')
	              form.append(decodeURIComponent(name), decodeURIComponent(value))
	            }
	          })
	          return form
	        }
	
	        function headers(xhr) {
	          var head = new Headers()
	          var pairs = (xhr.getAllResponseHeaders() || '').trim().split('\n')
	          pairs.forEach(function(header) {
	            var split = header.trim().split(':')
	            var key = split.shift().trim()
	            var value = split.join(':').trim()
	            head.append(key, value)
	          })
	          return head
	        }
	
	        Body.call(Request.prototype)
	
	        function Response(bodyInit, options) {
	          if (!options) {
	            options = {}
	          }
	
	          this.type = 'default'
	          this.status = options.status
	          this.ok = this.status >= 200 && this.status < 300
	          this.statusText = options.statusText
	          this.headers = options.headers instanceof Headers ? options.headers : new Headers(options.headers)
	          this.url = options.url || ''
	          this._initBody(bodyInit)
	        }
	
	        Body.call(Response.prototype)
	
	        Response.prototype.clone = function() {
	          return new Response(this._bodyInit, {
	            status: this.status,
	            statusText: this.statusText,
	            headers: new Headers(this.headers),
	            url: this.url
	          })
	        }
	
	        Response.error = function() {
	          var response = new Response(null, {status: 0, statusText: ''})
	          response.type = 'error'
	          return response
	        }
	
	        var redirectStatuses = [301, 302, 303, 307, 308]
	
	        Response.redirect = function(url, status) {
	          if (redirectStatuses.indexOf(status) === -1) {
	            throw new RangeError('Invalid status code')
	          }
	
	          return new Response(null, {status: status, headers: {location: url}})
	        }
	
	        self.Headers = Headers
	        self.Request = Request
	        self.Response = Response
	
	        self.fetch = function(input, init) {
	          return new Promise(function(resolve, reject) {
	            var request
	            if (Request.prototype.isPrototypeOf(input) && !init) {
	              request = input
	            } else {
	              request = new Request(input, init)
	            }
	
	            var xhr = new XMLHttpRequest()
	
	            function responseURL() {
	              if ('responseURL' in xhr) {
	                return xhr.responseURL
	              }
	
	              // Avoid security warnings on getResponseHeader when not allowed by CORS
	              if (/^X-Request-URL:/m.test(xhr.getAllResponseHeaders())) {
	                return xhr.getResponseHeader('X-Request-URL')
	              }
	
	              return
	            }
	
	            xhr.onload = function() {
	              var options = {
	                status: xhr.status,
	                statusText: xhr.statusText,
	                headers: headers(xhr),
	                url: responseURL()
	              }
	              var body = 'response' in xhr ? xhr.response : xhr.responseText
	              resolve(new Response(body, options))
	            }
	
	            xhr.onerror = function() {
	              reject(new TypeError('Network request failed'))
	            }
	
	            xhr.ontimeout = function() {
	              reject(new TypeError('Network request failed'))
	            }
	
	            xhr.open(request.method, request.url, true)
	
	            if (request.credentials === 'include') {
	              xhr.withCredentials = true
	            }
	
	            if ('responseType' in xhr && support.blob) {
	              xhr.responseType = 'blob'
	            }
	
	            request.headers.forEach(function(value, name) {
	              xhr.setRequestHeader(name, value)
	            })
	
	            xhr.send(typeof request._bodyInit === 'undefined' ? null : request._bodyInit)
	          })
	        }
	        self.fetch.polyfill = true
	      })(typeof self !== 'undefined' ? self : this);
	
	
	      return self.fetch;
	    }());
	  }
	
	  if (true) {
	    !(__WEBPACK_AMD_DEFINE_RESULT__ = function () {
	      return fetchPonyfill;
	    }.call(exports, __webpack_require__, exports, module), __WEBPACK_AMD_DEFINE_RESULT__ !== undefined && (module.exports = __WEBPACK_AMD_DEFINE_RESULT__));
	  } else if (typeof exports === 'object') {
	    module.exports = fetchPonyfill;
	  } else {
	    self.fetchPonyfill = fetchPonyfill;
	  }
	}());
	


/***/ },

/***/ 728:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	
	var Promise = __webpack_require__(299);
	
	var transform = function transform(jsonResponse) {
	
	    var error = jsonResponse && jsonResponse.error ? jsonResponse.error : null;
	    var payload = jsonResponse && jsonResponse.payload ? jsonResponse.payload : null;
	
	    return {
	        error: error,
	        payload: payload
	    };
	};
	
	module.exports = function (response) {
	    // NOTE: response generated from github's fetch (https://github.com/github/fetch)
	    // NOTE: response.json() returns a Promise
	
	    // TODO: simplify this
	    return new Promise(function (resolve) {
	        Promise.resolve(response.json()).then(function (jsonResponse) {
	            resolve(transform(jsonResponse));
	        }).catch(function () {
	            resolve(transform());
	        });
	    });
	};

/***/ },

/***/ 729:
/***/ function(module, exports) {

	'use strict';
	
	/* constants */
	
	/*
	enum MarkdownView {
	    Render,
	    Source
	}
	 */
	var MARKDOWN_VIEW = 'MARKDOWN_VIEW'; // key
	var MARKDOWN_VIEW_RENDER = 'MARKDOWN_VIEW_RENDER';
	var MARKDOWN_VIEW_SOURCE = 'MARKDOWN_VIEW_SOURCE';
	
	/* keys */
	// deck props
	var DECK_ID = 'DECK_ID';
	var DECK_NAME = 'DECK_NAME';
	var DECK_DESCRIPTION = 'DECK_DESCRIPTION';
	
	// card props
	var CARD_TITLE = 'CARD_TITLE';
	var CARD_DESCRIPTION = 'CARD_DESCRIPTION';
	var CARD_QUESTION = 'CARD_QUESTION';
	var CARD_ANSWER = 'CARD_ANSWER';
	var CARD_IS_ACTIVE = 'CARD_IS_ACTIVE';
	var CARD_SETTINGS = 'CARD_SETTINGS';
	var CARD_META = 'CARD_META';
	var CARD_ID = 'CARD_ID';
	
	var MARKDOWN_CONTENTS = 'MARKDOWN_CONTENTS';
	var POST_TO = 'POST_TO'; // URL to send POST request
	var DELETE_TO = 'DELETE_TO'; // URL to send DELETE request
	var IS_EDITING = 'IS_EDITING';
	
	var VALUE = 'VALUE';
	
	var IS_CONFIRM_SKIP = 'IS_CONFIRM_SKIP';
	var CURRENT_TAB = 'CURRENT_TAB';
	
	var ERROR = 'ERROR';
	var ERROR_MESSAGE = 'ERROR_MESSAGE';
	
	var CREATED_AT = 'CREATED_AT';
	var UPDATED_AT = 'UPDATED_AT';
	var SEEN_AT = 'SEEN_AT';
	var REVIEWED_AT = 'REVIEWED_AT';
	
	module.exports = {
	
	    MARKDOWN_VIEW: MARKDOWN_VIEW,
	    MARKDOWN_VIEW_RENDER: MARKDOWN_VIEW_RENDER,
	    MARKDOWN_VIEW_SOURCE: MARKDOWN_VIEW_SOURCE,
	
	    // keys
	    DECK_ID: DECK_ID,
	    DECK_NAME: DECK_NAME,
	    DECK_DESCRIPTION: DECK_DESCRIPTION,
	
	    CARD_TITLE: CARD_TITLE,
	    CARD_DESCRIPTION: CARD_DESCRIPTION,
	    CARD_QUESTION: CARD_QUESTION,
	    CARD_ANSWER: CARD_ANSWER,
	    CARD_SETTINGS: CARD_SETTINGS,
	    CARD_IS_ACTIVE: CARD_IS_ACTIVE,
	    CARD_META: CARD_META,
	    CARD_ID: CARD_ID,
	
	    MARKDOWN_CONTENTS: MARKDOWN_CONTENTS,
	    POST_TO: POST_TO,
	    DELETE_TO: DELETE_TO,
	    IS_EDITING: IS_EDITING,
	
	    VALUE: VALUE,
	
	    IS_CONFIRM_SKIP: IS_CONFIRM_SKIP,
	    CURRENT_TAB: CURRENT_TAB,
	
	    ERROR: ERROR,
	    ERROR_MESSAGE: ERROR_MESSAGE,
	
	    CREATED_AT: CREATED_AT,
	    UPDATED_AT: UPDATED_AT,
	    SEEN_AT: SEEN_AT,
	    REVIEWED_AT: REVIEWED_AT
	};

/***/ },

/***/ 730:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	
	var React = __webpack_require__(490);
	var isFunction = __webpack_require__(313);
	
	var ErrorComponent = function ErrorComponent(props) {
	
	    if (!props.error || String(props.error).trim().length <= 0) {
	        return null;
	    }
	
	    if (isFunction(props.onConfirm)) {
	
	        return React.createElement(
	            'div',
	            { className: 'columns' },
	            React.createElement(
	                'div',
	                { className: 'column' },
	                React.createElement(
	                    'div',
	                    { className: 'notification is-danger' },
	                    React.createElement('button', { className: 'delete', onClick: props.onConfirm }),
	                    props.error
	                )
	            )
	        );
	    }
	
	    return React.createElement(
	        'div',
	        { className: 'columns' },
	        React.createElement(
	            'div',
	            { className: 'column' },
	            React.createElement(
	                'div',
	                { className: 'notification is-danger' },
	                props.error
	            )
	        )
	    );
	};
	
	if (true) {
	    ErrorComponent.propTypes = {
	        error: React.PropTypes.string,
	        onConfirm: React.PropTypes.func
	    };
	}
	
	module.exports = ErrorComponent;

/***/ },

/***/ 731:
/***/ function(module, exports) {

	"use strict";
	
	var boolReducer = function boolReducer() {
	    var state = arguments.length <= 0 || arguments[0] === undefined ? false : arguments[0];
	    var action = arguments[1];
	
	    switch (action.type) {
	        case true:
	        case false:
	            state = action.type;
	            break;
	        default:
	            state = false;
	    }
	
	    return state;
	};
	
	module.exports = boolReducer;

/***/ },

/***/ 732:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	
	var _require = __webpack_require__(729);
	
	var ERROR_MESSAGE = _require.ERROR_MESSAGE;
	
	
	var DEFAULT = {
	    type: {}
	};
	
	var errorReducer = function errorReducer(state) {
	    var action = arguments.length <= 1 || arguments[1] === undefined ? DEFAULT : arguments[1];
	
	
	    switch (action.type) {
	        case ERROR_MESSAGE:
	
	            state = {
	                message: action.payload || ''
	            };
	
	            break;
	        default:
	
	            state = {
	                message: ''
	            };
	    }
	
	    return state;
	};
	
	module.exports = errorReducer;

/***/ },

/***/ 733:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	
	var _require = __webpack_require__(682);
	
	var makeReducer = _require.makeReducer;
	
	var _require2 = __webpack_require__(667);
	
	var createStore = _require2.createStore;
	var applyMiddleware = _require2.applyMiddleware;
	
	
	var rehydrateFactory = __webpack_require__(734);
	
	var IDENTITY = function IDENTITY(x) {
	    return x;
	};
	
	module.exports = function () {
	    var initialState = arguments.length <= 0 || arguments[0] === undefined ? void 0 : arguments[0];
	    var fallbackReducer = arguments.length <= 1 || arguments[1] === undefined ? IDENTITY : arguments[1];
	
	
	    if (true) {
	        if (initialState === void 0) {
	            throw Error('expected initialState');
	        }
	    }
	
	    // TODO: refactor to module
	    var middleware = function middleware() {
	
	        if (true) {
	
	            var createLogger = __webpack_require__(735);
	            var logger = createLogger();
	
	            return applyMiddleware(logger);
	        }
	
	        return applyMiddleware();
	    };
	
	    var store = createStore(makeReducer({
	        reducer: rehydrateFactory(fallbackReducer)
	    }), initialState, middleware());
	
	    return store;
	};

/***/ },

/***/ 734:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	
	// reducer for rehydration
	//
	// terminology: http://stackoverflow.com/questions/29824908/what-does-dehydrate-and-rehydrate-stand-for-in-fluxible
	
	var merge = __webpack_require__(686);
	
	// const REHYDRATE = Symbol('REHYDRATE');
	//
	// This is cheaper alternative to Symbol.
	var REHYDRATE = ['REHYDRATE'];
	var HOT_PATH = ['HOT_PATH'];
	
	var IDENTITY = function IDENTITY(x) {
	    return x;
	};
	
	// reducer factory
	module.exports = function () {
	    var fallbackReducer = arguments.length <= 0 || arguments[0] === undefined ? IDENTITY : arguments[0];
	
	
	    var isHotPath = false;
	
	    return function (state, action) {
	
	        if (true) {
	            if (!action.type) {
	                console.error('Action not FSA. Given ' + action);
	            }
	        }
	
	        if (isHotPath) {
	            return fallbackReducer(state, action);
	        }
	
	        switch (action.type) {
	
	            case REHYDRATE:
	
	                // rationale: action.payload may be staler than state
	
	                return merge({}, action.payload, state);
	
	            case HOT_PATH:
	                isHotPath = true;
	                return state;
	
	            default:
	                return fallbackReducer(state, action);
	        }
	
	        // unreachable!();
	    };
	};
	
	// action creators
	
	module.exports.hydrate = function (state) {
	    return {
	        type: REHYDRATE,
	        payload: state
	    };
	};
	
	module.exports.hotpath = function () {
	    return {
	        type: HOT_PATH
	    };
	};

/***/ },

/***/ 735:
/***/ function(module, exports) {

	"use strict";
	
	function _toConsumableArray(arr) { if (Array.isArray(arr)) { for (var i = 0, arr2 = Array(arr.length); i < arr.length; i++) { arr2[i] = arr[i]; } return arr2; } else { return Array.from(arr); } }
	
	function _typeof(obj) { return obj && typeof Symbol !== "undefined" && obj.constructor === Symbol ? "symbol" : typeof obj; }
	
	var repeat = function repeat(str, times) {
	  return new Array(times + 1).join(str);
	};
	var pad = function pad(num, maxLength) {
	  return repeat("0", maxLength - num.toString().length) + num;
	};
	var formatTime = function formatTime(time) {
	  return "@ " + pad(time.getHours(), 2) + ":" + pad(time.getMinutes(), 2) + ":" + pad(time.getSeconds(), 2) + "." + pad(time.getMilliseconds(), 3);
	};
	
	// Use the new performance api to get better precision if available
	var timer = typeof performance !== "undefined" && typeof performance.now === "function" ? performance : Date;
	
	/**
	 * parse the level option of createLogger
	 *
	 * @property {string | function | object} level - console[level]
	 * @property {object} action
	 * @property {array} payload
	 * @property {string} type
	 */
	
	function getLogLevel(level, action, payload, type) {
	  switch (typeof level === "undefined" ? "undefined" : _typeof(level)) {
	    case "object":
	      return typeof level[type] === "function" ? level[type].apply(level, _toConsumableArray(payload)) : level[type];
	    case "function":
	      return level(action);
	    default:
	      return level;
	  }
	}
	
	/**
	 * Creates logger with followed options
	 *
	 * @namespace
	 * @property {object} options - options for logger
	 * @property {string | function | object} options.level - console[level]
	 * @property {boolean} options.duration - print duration of each action?
	 * @property {boolean} options.timestamp - print timestamp with each action?
	 * @property {object} options.colors - custom colors
	 * @property {object} options.logger - implementation of the `console` API
	 * @property {boolean} options.logErrors - should errors in action execution be caught, logged, and re-thrown?
	 * @property {boolean} options.collapsed - is group collapsed?
	 * @property {boolean} options.predicate - condition which resolves logger behavior
	 * @property {function} options.stateTransformer - transform state before print
	 * @property {function} options.actionTransformer - transform action before print
	 * @property {function} options.errorTransformer - transform error before print
	 */
	
	function createLogger() {
	  var options = arguments.length <= 0 || arguments[0] === undefined ? {} : arguments[0];
	  var _options$level = options.level;
	  var level = _options$level === undefined ? "log" : _options$level;
	  var _options$logger = options.logger;
	  var logger = _options$logger === undefined ? console : _options$logger;
	  var _options$logErrors = options.logErrors;
	  var logErrors = _options$logErrors === undefined ? true : _options$logErrors;
	  var collapsed = options.collapsed;
	  var predicate = options.predicate;
	  var _options$duration = options.duration;
	  var duration = _options$duration === undefined ? false : _options$duration;
	  var _options$timestamp = options.timestamp;
	  var timestamp = _options$timestamp === undefined ? true : _options$timestamp;
	  var transformer = options.transformer;
	  var _options$stateTransfo = options.stateTransformer;
	  var // deprecated
	  stateTransformer = _options$stateTransfo === undefined ? function (state) {
	    return state;
	  } : _options$stateTransfo;
	  var _options$actionTransf = options.actionTransformer;
	  var actionTransformer = _options$actionTransf === undefined ? function (actn) {
	    return actn;
	  } : _options$actionTransf;
	  var _options$errorTransfo = options.errorTransformer;
	  var errorTransformer = _options$errorTransfo === undefined ? function (error) {
	    return error;
	  } : _options$errorTransfo;
	  var _options$colors = options.colors;
	  var colors = _options$colors === undefined ? {
	    title: function title() {
	      return "#000000";
	    },
	    prevState: function prevState() {
	      return "#9E9E9E";
	    },
	    action: function action() {
	      return "#03A9F4";
	    },
	    nextState: function nextState() {
	      return "#4CAF50";
	    },
	    error: function error() {
	      return "#F20404";
	    }
	  } : _options$colors;
	
	  // exit if console undefined
	
	  if (typeof logger === "undefined") {
	    return function () {
	      return function (next) {
	        return function (action) {
	          return next(action);
	        };
	      };
	    };
	  }
	
	  if (transformer) {
	    console.error("Option 'transformer' is deprecated, use stateTransformer instead");
	  }
	
	  var logBuffer = [];
	  function printBuffer() {
	    logBuffer.forEach(function (logEntry, key) {
	      var started = logEntry.started;
	      var startedTime = logEntry.startedTime;
	      var action = logEntry.action;
	      var prevState = logEntry.prevState;
	      var error = logEntry.error;
	      var took = logEntry.took;
	      var nextState = logEntry.nextState;
	
	      var nextEntry = logBuffer[key + 1];
	      if (nextEntry) {
	        nextState = nextEntry.prevState;
	        took = nextEntry.started - started;
	      }
	      // message
	      var formattedAction = actionTransformer(action);
	      var isCollapsed = typeof collapsed === "function" ? collapsed(function () {
	        return nextState;
	      }, action) : collapsed;
	
	      var formattedTime = formatTime(startedTime);
	      var titleCSS = colors.title ? "color: " + colors.title(formattedAction) + ";" : null;
	      var title = "action " + (timestamp ? formattedTime : "") + " " + formattedAction.type + " " + (duration ? "(in " + took.toFixed(2) + " ms)" : "");
	
	      // render
	      try {
	        if (isCollapsed) {
	          if (colors.title) logger.groupCollapsed("%c " + title, titleCSS);else logger.groupCollapsed(title);
	        } else {
	          if (colors.title) logger.group("%c " + title, titleCSS);else logger.group(title);
	        }
	      } catch (e) {
	        logger.log(title);
	      }
	
	      var prevStateLevel = getLogLevel(level, formattedAction, [prevState], "prevState");
	      var actionLevel = getLogLevel(level, formattedAction, [formattedAction], "action");
	      var errorLevel = getLogLevel(level, formattedAction, [error, prevState], "error");
	      var nextStateLevel = getLogLevel(level, formattedAction, [nextState], "nextState");
	
	      if (prevStateLevel) {
	        if (colors.prevState) logger[prevStateLevel]("%c prev state", "color: " + colors.prevState(prevState) + "; font-weight: bold", prevState);else logger[prevStateLevel]("prev state", prevState);
	      }
	
	      if (actionLevel) {
	        if (colors.action) logger[actionLevel]("%c action", "color: " + colors.action(formattedAction) + "; font-weight: bold", formattedAction);else logger[actionLevel]("action", formattedAction);
	      }
	
	      if (error && errorLevel) {
	        if (colors.error) logger[errorLevel]("%c error", "color: " + colors.error(error, prevState) + "; font-weight: bold", error);else logger[errorLevel]("error", error);
	      }
	
	      if (nextStateLevel) {
	        if (colors.nextState) logger[nextStateLevel]("%c next state", "color: " + colors.nextState(nextState) + "; font-weight: bold", nextState);else logger[nextStateLevel]("next state", nextState);
	      }
	
	      try {
	        logger.groupEnd();
	      } catch (e) {
	        logger.log("—— log end ——");
	      }
	    });
	    logBuffer.length = 0;
	  }
	
	  return function (_ref) {
	    var getState = _ref.getState;
	    return function (next) {
	      return function (action) {
	        // exit early if predicate function returns false
	        if (typeof predicate === "function" && !predicate(getState, action)) {
	          return next(action);
	        }
	
	        var logEntry = {};
	        logBuffer.push(logEntry);
	
	        logEntry.started = timer.now();
	        logEntry.startedTime = new Date();
	        logEntry.prevState = stateTransformer(getState());
	        logEntry.action = action;
	
	        var returnedValue = undefined;
	        if (logErrors) {
	          try {
	            returnedValue = next(action);
	          } catch (e) {
	            logEntry.error = errorTransformer(e);
	          }
	        } else {
	          returnedValue = next(action);
	        }
	
	        logEntry.took = timer.now() - logEntry.started;
	        logEntry.nextState = stateTransformer(getState());
	
	        printBuffer();
	
	        if (logEntry.error) throw logEntry.error;
	        return returnedValue;
	      };
	    };
	  };
	}
	
	module.exports = createLogger;

/***/ },

/***/ 737:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	
	// helper for client entry files
	
	// TODO: deferred to cdn: https://cdnjs.com/libraries/babel-polyfill
	// require('babel-polyfill');
	
	var ReactDOM = __webpack_require__(522);
	
	var rehydrate = __webpack_require__(734);
	
	module.exports = function (maker, preRenderState, postRenderState, mountTarget) {
	
	    // NOTES:
	    // - preRenderState := inject state before initial render.
	    //                     useful for injecting state that doesn't cause UI change
	    //
	    // - postRenderState := inject state after initial render. this is used for statically compiled react components
	
	    var _maker = maker(preRenderState);
	
	    var component = _maker.component;
	    var store = _maker.store;
	
	
	    if (!postRenderState) {
	        if (true) {
	            var invariant = __webpack_require__(680);
	            invariant(maker.initialState, 'maker.initialState not set');
	        }
	        postRenderState = maker.initialState;
	    }
	
	    var firstRender = false;
	    var afterRender = function afterRender() {
	
	        if (firstRender) {
	            return;
	        }
	
	        firstRender = true;
	
	        store.dispatch(rehydrate.hydrate(postRenderState));
	        store.dispatch(rehydrate.hotpath());
	
	        if (true) {
	            console.log('rehydration finished.');
	        }
	
	        if (maker.afterRender) {
	            maker.afterRender(store);
	        }
	    };
	
	    if (maker.preRender) {
	        maker.preRender();
	    }
	
	    ReactDOM.render(component, mountTarget, afterRender);
	};

/***/ },

/***/ 746:
44,

/***/ 747:
45,

/***/ 821:
/***/ function(module, exports, __webpack_require__) {

	"use strict";
	
	exports.__esModule = true;
	
	var _iterator = __webpack_require__(822);
	
	var _iterator2 = _interopRequireDefault(_iterator);
	
	var _symbol = __webpack_require__(825);
	
	var _symbol2 = _interopRequireDefault(_symbol);
	
	var _typeof = typeof _symbol2.default === "function" && typeof _iterator2.default === "symbol" ? function (obj) { return typeof obj; } : function (obj) { return obj && typeof _symbol2.default === "function" && obj.constructor === _symbol2.default ? "symbol" : typeof obj; };
	
	function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }
	
	exports.default = typeof _symbol2.default === "function" && _typeof(_iterator2.default) === "symbol" ? function (obj) {
	  return typeof obj === "undefined" ? "undefined" : _typeof(obj);
	} : function (obj) {
	  return obj && typeof _symbol2.default === "function" && obj.constructor === _symbol2.default ? "symbol" : typeof obj === "undefined" ? "undefined" : _typeof(obj);
	};

/***/ },

/***/ 822:
/***/ function(module, exports, __webpack_require__) {

	module.exports = { "default": __webpack_require__(823), __esModule: true };

/***/ },

/***/ 823:
/***/ function(module, exports, __webpack_require__) {

	__webpack_require__(465);
	__webpack_require__(434);
	module.exports = __webpack_require__(824).f('iterator');

/***/ },

/***/ 824:
[935, 462],

/***/ 825:
/***/ function(module, exports, __webpack_require__) {

	module.exports = { "default": __webpack_require__(826), __esModule: true };

/***/ },

/***/ 826:
/***/ function(module, exports, __webpack_require__) {

	__webpack_require__(827);
	__webpack_require__(475);
	__webpack_require__(836);
	__webpack_require__(837);
	module.exports = __webpack_require__(418).Symbol;

/***/ },

/***/ 827:
[922, 417, 446, 426, 416, 445, 828, 427, 457, 461, 458, 462, 824, 829, 830, 831, 832, 423, 439, 429, 430, 448, 833, 835, 422, 450, 834, 747, 746, 444, 421],

/***/ 828:
[931, 458, 424, 446, 422, 427],

/***/ 829:
[936, 417, 418, 444, 824, 422],

/***/ 830:
[937, 450, 439],

/***/ 831:
[946, 450, 746, 747],

/***/ 832:
[947, 441],

/***/ 833:
[951, 439, 834],

/***/ 834:
[952, 451, 459],

/***/ 835:
[953, 747, 430, 439, 429, 446, 425, 426],

/***/ 836:
[979, 829],

/***/ 837:
[980, 829],

/***/ 860:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	
	var merge = __webpack_require__(686);
	
	var createStore = __webpack_require__(733);
	
	var IDENTITY = function IDENTITY(x) {
	    return x;
	};
	
	var componentCreator = function componentCreator(initialState, getComponent) {
	    var fallbackReducer = arguments.length <= 2 || arguments[2] === undefined ? IDENTITY : arguments[2];
	
	
	    return function (preRenderState) {
	
	        if (preRenderState) {
	            preRenderState = merge({}, initialState, preRenderState);
	        } else {
	            preRenderState = initialState;
	        }
	
	        var store = createStore(preRenderState, fallbackReducer);
	
	        var component = getComponent(store);
	
	        return {
	            store: store,
	            component: component
	        };
	    };
	};
	
	module.exports = componentCreator;

/***/ },

/***/ 913:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	
	var _defineProperty2 = __webpack_require__(412);
	
	var _defineProperty3 = _interopRequireDefault(_defineProperty2);
	
	var _slicedToArray2 = __webpack_require__(431);
	
	var _slicedToArray3 = _interopRequireDefault(_slicedToArray2);
	
	var _promise = __webpack_require__(473);
	
	var _promise2 = _interopRequireDefault(_promise);
	
	var _typeof2 = __webpack_require__(821);
	
	var _typeof3 = _interopRequireDefault(_typeof2);
	
	var _initialState;
	
	function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }
	
	__webpack_require__(1);
	
	var React = __webpack_require__(490);
	
	var _require = __webpack_require__(660);
	
	var Provider = _require.Provider;
	var connect = _require.connect;
	
	var classnames = __webpack_require__(681);
	var get = __webpack_require__(394);
	
	var _require2 = __webpack_require__(682);
	
	var reduceIn = _require2.reduceIn;
	
	
	var fetch = __webpack_require__(727)({
	    Promise: __webpack_require__(299)
	});
	
	var jsonDecode = __webpack_require__(728);
	
	var _require3 = __webpack_require__(729);
	
	var DELETE_TO = _require3.DELETE_TO;
	var ERROR = _require3.ERROR;
	var ERROR_MESSAGE = _require3.ERROR_MESSAGE;
	
	// TODO: move these constants
	
	var CONFIRM_DELETE = 'CONFIRM_DELETE';
	var SUBMITTING = 'SUBMITTING';
	
	/* react components */
	
	var ErrorComponent = __webpack_require__(730);
	
	var __DeleteCard = function __DeleteCard(props) {
	    var submitting = props.submitting;
	    var confirmDelete = props.confirmDelete;
	    var handleConfirm = props.handleConfirm;
	    var error = props.error;
	    var dispatch = props.dispatch;
	
	
	    if (confirmDelete) {
	        var _ret = function () {
	            var deleteURL = props.deleteURL;
	
	
	            var handleDelete = function handleDelete(event) {
	                event.preventDefault();
	                deleteCard(dispatch, deleteURL, submitting);
	            };
	
	            return {
	                v: React.createElement(
	                    'div',
	                    null,
	                    React.createElement(ErrorComponent, { error: error && error.message || '', onConfirm: confirmError(dispatch) }),
	                    React.createElement(
	                        'div',
	                        { className: 'columns' },
	                        React.createElement(
	                            'div',
	                            { className: 'column' },
	                            React.createElement(
	                                'strong',
	                                null,
	                                'Are you sure you want to delete this card?'
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
	                                { className: 'level' },
	                                React.createElement(
	                                    'div',
	                                    { className: 'level-left' },
	                                    React.createElement(
	                                        'a',
	                                        {
	                                            className: classnames('button is-bold', {
	                                                'is-disabled': submitting,
	                                                'is-loading': submitting
	                                            }),
	                                            onClick: handleConfirm(false)
	                                        },
	                                        'Cancel'
	                                    )
	                                ),
	                                React.createElement(
	                                    'div',
	                                    { className: 'level-right' },
	                                    React.createElement(
	                                        'a',
	                                        {
	                                            className: classnames('button is-danger is-bold', {
	                                                'is-disabled': submitting,
	                                                'is-loading': submitting
	                                            }),
	                                            onClick: handleDelete
	                                        },
	                                        'Delete'
	                                    )
	                                )
	                            )
	                        )
	                    )
	                )
	            };
	        }();
	
	        if ((typeof _ret === 'undefined' ? 'undefined' : (0, _typeof3.default)(_ret)) === "object") return _ret.v;
	    }
	
	    return React.createElement(
	        'div',
	        null,
	        React.createElement(ErrorComponent, { error: error && error.message || '', onConfirm: confirmError(dispatch) }),
	        React.createElement(
	            'div',
	            { className: 'columns' },
	            React.createElement(
	                'div',
	                { className: 'column' },
	                React.createElement(
	                    'a',
	                    {
	                        className: classnames('button is-danger is-bold', {
	                            'is-disabled': submitting,
	                            'is-loading': submitting
	                        }),
	                        onClick: handleConfirm(true)
	                    },
	                    'Delete Card'
	                )
	            )
	        )
	    );
	};
	
	if (true) {
	    __DeleteCard.propTypes = {
	        submitting: React.PropTypes.bool.isRequired,
	        confirmDelete: React.PropTypes.bool.isRequired,
	        handleConfirm: React.PropTypes.func.isRequired,
	        dispatch: React.PropTypes.func.isRequired,
	        deleteURL: React.PropTypes.string.isRequired,
	        error: React.PropTypes.object
	    };
	}
	
	var DeleteCard = connect(
	// mapStateToProps
	function (state) {
	    return {
	        error: state[ERROR],
	        submitting: state[SUBMITTING],
	        confirmDelete: state[CONFIRM_DELETE],
	        deleteURL: state[DELETE_TO]
	    };
	},
	// mapDispatchToProps
	function (dispatch) {
	    return {
	        dispatch: dispatch,
	        handleConfirm: function handleConfirm(bool) {
	            return function (event) {
	                event.preventDefault();
	                dispatch(reduceIn(
	                // reducer
	                boolReducer,
	                // path
	                [CONFIRM_DELETE],
	                // action
	                {
	                    type: bool
	                }));
	            };
	        }
	    };
	})(__DeleteCard);
	
	/* redux action dispatchers */
	// NOTE: FSA compliant
	
	var defaultRESTError = 'Unable to delete card. Please try again.';
	var deleteCard = function deleteCard(dispatch, deleteURL) {
	    var submitting = arguments.length <= 2 || arguments[2] === undefined ? true : arguments[2];
	
	
	    if (submitting) {
	        return;
	    }
	
	    dispatch(reduceIn(
	    // reducer
	    boolReducer,
	    // path
	    [SUBMITTING],
	    // action
	    {
	        type: true
	    }));
	
	    fetch(deleteURL, {
	        method: 'DELETE'
	    }).then(function (response) {
	        return _promise2.default.all([response.status, jsonDecode(response)]);
	    }).then(function (_ref) {
	        var _ref2 = (0, _slicedToArray3.default)(_ref, 2);
	
	        var statusCode = _ref2[0];
	        var jsonResponse = _ref2[1];
	
	
	        dispatch(reduceIn(
	        // reducer
	        boolReducer,
	        // path
	        [SUBMITTING],
	        // action
	        {
	            type: false
	        }));
	
	        switch (statusCode) {
	            case 500: // Internal Server Error
	            case 400:
	                // Bad Request
	
	                dispatch(reduceIn(
	                // reducer
	                errorReducer,
	                // path
	                [ERROR],
	                // action
	                {
	                    type: ERROR_MESSAGE,
	                    payload: get(jsonResponse, ['error'], defaultRESTError)
	                }));
	
	                return;
	                break;
	
	            case 200:
	                // Ok
	
	                if (jsonResponse.payload) {
	
	                    dispatch(reduceIn(
	                    // reducer
	                    errorReducer,
	                    // path
	                    [ERROR],
	                    // action
	                    {
	                        type: ERROR_MESSAGE,
	                        payload: get(jsonResponse, ['error'], '')
	                    }));
	
	                    window.location.href = jsonResponse.payload.redirect_to;
	                } else {
	
	                    dispatch(reduceIn(
	                    // reducer
	                    errorReducer,
	                    // path
	                    [ERROR],
	                    // action
	                    {
	                        type: ERROR_MESSAGE,
	                        payload: get(jsonResponse, ['error'], 'Unable to delete this card.')
	                    }));
	                }
	
	                return;
	                break;
	
	            default:
	                // Unexpected http status code
	                dispatch(reduceIn(
	                // reducer
	                errorReducer,
	                // path
	                [ERROR],
	                // action
	                {
	                    type: ERROR_MESSAGE,
	                    payload: get(jsonResponse, ['error'], defaultRESTError)
	                }));
	        }
	    }).catch(function () /*err*/{
	
	        // any other errors
	        // console.log('err:', err);
	
	        dispatch(reduceIn(
	        // reducer
	        boolReducer,
	        // path
	        [SUBMITTING],
	        // action
	        {
	            type: false
	        }));
	
	        dispatch(reduceIn(
	        // reducer
	        errorReducer,
	        // path
	        [ERROR],
	        // action
	        {
	            type: ERROR_MESSAGE,
	            payload: defaultRESTError
	        }));
	    });
	};
	
	var confirmError = function confirmError(dispatch) {
	    return function (event) {
	        event.preventDefault();
	        dispatch(reduceIn(
	        // reducer
	        errorReducer,
	        // path
	        [ERROR],
	        // action
	        {
	            type: ERROR_MESSAGE,
	            payload: ''
	        }));
	    };
	};
	
	/* redux reducers */
	
	var boolReducer = __webpack_require__(731);
	var errorReducer = __webpack_require__(732);
	
	/* default state */
	
	var initialState = (_initialState = {}, (0, _defineProperty3.default)(_initialState, DELETE_TO, ''), (0, _defineProperty3.default)(_initialState, CONFIRM_DELETE, false), (0, _defineProperty3.default)(_initialState, SUBMITTING, false), (0, _defineProperty3.default)(_initialState, ERROR, errorReducer()), _initialState);
	
	/* exports */
	
	var componentCreator = __webpack_require__(860);
	
	module.exports = componentCreator(initialState, function (store) {
	
	    var component = React.createElement(
	        Provider,
	        { store: store },
	        React.createElement(DeleteCard, null)
	    );
	
	    return component;
	});
	
	module.exports.initialState = initialState;

/***/ }

});
//# sourceMappingURL=deck_card_settings.js.map