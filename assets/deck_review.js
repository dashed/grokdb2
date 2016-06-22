webpackJsonp([0],{

/***/ 0:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	
	var client = __webpack_require__(1);
	
	var maker = __webpack_require__(515);
	
	// TODO: debug
	var initialState = __webpack_require__(552).merge({}, maker.initialState);
	initialState.TAB_QUESTION.CARD_CONTENTS = 'question';
	window.__INITIAL_STATE__ = initialState;
	
	client(maker, document.getElementById('deck-review-container'));

/***/ },

/***/ 1:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	
	// helper for client entry files
	
	__webpack_require__(2);
	
	var ReactDOM = __webpack_require__(300);
	
	var rehydrate = __webpack_require__(460);
	
	module.exports = function (maker, mountTarget) {
	
	    var initialState = window.__INITIAL_STATE__;
	
	    var _maker = maker();
	
	    var component = _maker.component;
	    var store = _maker.store;
	
	
	    var firstRender = false;
	    var afterRender = function afterRender() {
	
	        if (firstRender) {
	            return;
	        }
	        firstRender = true;
	
	        store.dispatch(rehydrate.hydrate(initialState));
	
	        console.log('finished render lol');
	    };
	
	    ReactDOM.render(component, mountTarget, afterRender);
	};

/***/ },

/***/ 460:
/***/ function(module, exports, __webpack_require__) {

	/* WEBPACK VAR INJECTION */(function(process) {'use strict';
	
	var _symbol = __webpack_require__(461);
	
	var _symbol2 = _interopRequireDefault(_symbol);
	
	function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }
	
	// reducer for rehydration
	//
	// terminology: http://stackoverflow.com/questions/29824908/what-does-dehydrate-and-rehydrate-stand-for-in-fluxible
	
	var REHYDRATE = (0, _symbol2.default)('REHYDRATE');
	
	// reducer
	module.exports = function (state, action) {
	
	    if (process.env.NODE_ENV !== 'production') {
	        if (!action.type) {
	            console.error('Action not FSA. Given ' + action);
	        }
	    }
	
	    switch (action.type) {
	
	        case REHYDRATE:
	            return action.payload;
	
	        default:
	            return state;
	    }
	
	    // unreachable!();
	};
	
	// action creator
	module.exports.hydrate = function (state) {
	    return {
	        type: REHYDRATE,
	        payload: state
	    };
	};
	/* WEBPACK VAR INJECTION */}.call(exports, __webpack_require__(296)))

/***/ },

/***/ 461:
/***/ function(module, exports, __webpack_require__) {

	module.exports = { "default": __webpack_require__(462), __esModule: true };

/***/ },

/***/ 462:
/***/ function(module, exports, __webpack_require__) {

	__webpack_require__(463);
	__webpack_require__(512);
	__webpack_require__(513);
	__webpack_require__(514);
	module.exports = __webpack_require__(469).Symbol;

/***/ },

/***/ 463:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	// ECMAScript 6 symbols shim
	var global         = __webpack_require__(464)
	  , has            = __webpack_require__(465)
	  , DESCRIPTORS    = __webpack_require__(466)
	  , $export        = __webpack_require__(468)
	  , redefine       = __webpack_require__(480)
	  , META           = __webpack_require__(481).KEY
	  , $fails         = __webpack_require__(467)
	  , shared         = __webpack_require__(483)
	  , setToStringTag = __webpack_require__(484)
	  , uid            = __webpack_require__(482)
	  , wks            = __webpack_require__(485)
	  , wksExt         = __webpack_require__(486)
	  , wksDefine      = __webpack_require__(487)
	  , keyOf          = __webpack_require__(489)
	  , enumKeys       = __webpack_require__(502)
	  , isArray        = __webpack_require__(505)
	  , anObject       = __webpack_require__(474)
	  , toIObject      = __webpack_require__(492)
	  , toPrimitive    = __webpack_require__(478)
	  , createDesc     = __webpack_require__(479)
	  , _create        = __webpack_require__(506)
	  , gOPNExt        = __webpack_require__(509)
	  , $GOPD          = __webpack_require__(511)
	  , $DP            = __webpack_require__(473)
	  , $keys          = __webpack_require__(490)
	  , gOPD           = $GOPD.f
	  , dP             = $DP.f
	  , gOPN           = gOPNExt.f
	  , $Symbol        = global.Symbol
	  , $JSON          = global.JSON
	  , _stringify     = $JSON && $JSON.stringify
	  , PROTOTYPE      = 'prototype'
	  , HIDDEN         = wks('_hidden')
	  , TO_PRIMITIVE   = wks('toPrimitive')
	  , isEnum         = {}.propertyIsEnumerable
	  , SymbolRegistry = shared('symbol-registry')
	  , AllSymbols     = shared('symbols')
	  , OPSymbols      = shared('op-symbols')
	  , ObjectProto    = Object[PROTOTYPE]
	  , USE_NATIVE     = typeof $Symbol == 'function'
	  , QObject        = global.QObject;
	// Don't use setters in Qt Script, https://github.com/zloirock/core-js/issues/173
	var setter = !QObject || !QObject[PROTOTYPE] || !QObject[PROTOTYPE].findChild;
	
	// fallback for old Android, https://code.google.com/p/v8/issues/detail?id=687
	var setSymbolDesc = DESCRIPTORS && $fails(function(){
	  return _create(dP({}, 'a', {
	    get: function(){ return dP(this, 'a', {value: 7}).a; }
	  })).a != 7;
	}) ? function(it, key, D){
	  var protoDesc = gOPD(ObjectProto, key);
	  if(protoDesc)delete ObjectProto[key];
	  dP(it, key, D);
	  if(protoDesc && it !== ObjectProto)dP(ObjectProto, key, protoDesc);
	} : dP;
	
	var wrap = function(tag){
	  var sym = AllSymbols[tag] = _create($Symbol[PROTOTYPE]);
	  sym._k = tag;
	  return sym;
	};
	
	var isSymbol = USE_NATIVE && typeof $Symbol.iterator == 'symbol' ? function(it){
	  return typeof it == 'symbol';
	} : function(it){
	  return it instanceof $Symbol;
	};
	
	var $defineProperty = function defineProperty(it, key, D){
	  if(it === ObjectProto)$defineProperty(OPSymbols, key, D);
	  anObject(it);
	  key = toPrimitive(key, true);
	  anObject(D);
	  if(has(AllSymbols, key)){
	    if(!D.enumerable){
	      if(!has(it, HIDDEN))dP(it, HIDDEN, createDesc(1, {}));
	      it[HIDDEN][key] = true;
	    } else {
	      if(has(it, HIDDEN) && it[HIDDEN][key])it[HIDDEN][key] = false;
	      D = _create(D, {enumerable: createDesc(0, false)});
	    } return setSymbolDesc(it, key, D);
	  } return dP(it, key, D);
	};
	var $defineProperties = function defineProperties(it, P){
	  anObject(it);
	  var keys = enumKeys(P = toIObject(P))
	    , i    = 0
	    , l = keys.length
	    , key;
	  while(l > i)$defineProperty(it, key = keys[i++], P[key]);
	  return it;
	};
	var $create = function create(it, P){
	  return P === undefined ? _create(it) : $defineProperties(_create(it), P);
	};
	var $propertyIsEnumerable = function propertyIsEnumerable(key){
	  var E = isEnum.call(this, key = toPrimitive(key, true));
	  if(this === ObjectProto && has(AllSymbols, key) && !has(OPSymbols, key))return false;
	  return E || !has(this, key) || !has(AllSymbols, key) || has(this, HIDDEN) && this[HIDDEN][key] ? E : true;
	};
	var $getOwnPropertyDescriptor = function getOwnPropertyDescriptor(it, key){
	  it  = toIObject(it);
	  key = toPrimitive(key, true);
	  if(it === ObjectProto && has(AllSymbols, key) && !has(OPSymbols, key))return;
	  var D = gOPD(it, key);
	  if(D && has(AllSymbols, key) && !(has(it, HIDDEN) && it[HIDDEN][key]))D.enumerable = true;
	  return D;
	};
	var $getOwnPropertyNames = function getOwnPropertyNames(it){
	  var names  = gOPN(toIObject(it))
	    , result = []
	    , i      = 0
	    , key;
	  while(names.length > i){
	    if(!has(AllSymbols, key = names[i++]) && key != HIDDEN && key != META)result.push(key);
	  } return result;
	};
	var $getOwnPropertySymbols = function getOwnPropertySymbols(it){
	  var IS_OP  = it === ObjectProto
	    , names  = gOPN(IS_OP ? OPSymbols : toIObject(it))
	    , result = []
	    , i      = 0
	    , key;
	  while(names.length > i){
	    if(has(AllSymbols, key = names[i++]) && (IS_OP ? has(ObjectProto, key) : true))result.push(AllSymbols[key]);
	  } return result;
	};
	
	// 19.4.1.1 Symbol([description])
	if(!USE_NATIVE){
	  $Symbol = function Symbol(){
	    if(this instanceof $Symbol)throw TypeError('Symbol is not a constructor!');
	    var tag = uid(arguments.length > 0 ? arguments[0] : undefined);
	    var $set = function(value){
	      if(this === ObjectProto)$set.call(OPSymbols, value);
	      if(has(this, HIDDEN) && has(this[HIDDEN], tag))this[HIDDEN][tag] = false;
	      setSymbolDesc(this, tag, createDesc(1, value));
	    };
	    if(DESCRIPTORS && setter)setSymbolDesc(ObjectProto, tag, {configurable: true, set: $set});
	    return wrap(tag);
	  };
	  redefine($Symbol[PROTOTYPE], 'toString', function toString(){
	    return this._k;
	  });
	
	  $GOPD.f = $getOwnPropertyDescriptor;
	  $DP.f   = $defineProperty;
	  __webpack_require__(510).f = gOPNExt.f = $getOwnPropertyNames;
	  __webpack_require__(504).f  = $propertyIsEnumerable;
	  __webpack_require__(503).f = $getOwnPropertySymbols;
	
	  if(DESCRIPTORS && !__webpack_require__(488)){
	    redefine(ObjectProto, 'propertyIsEnumerable', $propertyIsEnumerable, true);
	  }
	
	  wksExt.f = function(name){
	    return wrap(wks(name));
	  }
	}
	
	$export($export.G + $export.W + $export.F * !USE_NATIVE, {Symbol: $Symbol});
	
	for(var symbols = (
	  // 19.4.2.2, 19.4.2.3, 19.4.2.4, 19.4.2.6, 19.4.2.8, 19.4.2.9, 19.4.2.10, 19.4.2.11, 19.4.2.12, 19.4.2.13, 19.4.2.14
	  'hasInstance,isConcatSpreadable,iterator,match,replace,search,species,split,toPrimitive,toStringTag,unscopables'
	).split(','), i = 0; symbols.length > i; )wks(symbols[i++]);
	
	for(var symbols = $keys(wks.store), i = 0; symbols.length > i; )wksDefine(symbols[i++]);
	
	$export($export.S + $export.F * !USE_NATIVE, 'Symbol', {
	  // 19.4.2.1 Symbol.for(key)
	  'for': function(key){
	    return has(SymbolRegistry, key += '')
	      ? SymbolRegistry[key]
	      : SymbolRegistry[key] = $Symbol(key);
	  },
	  // 19.4.2.5 Symbol.keyFor(sym)
	  keyFor: function keyFor(key){
	    if(isSymbol(key))return keyOf(SymbolRegistry, key);
	    throw TypeError(key + ' is not a symbol!');
	  },
	  useSetter: function(){ setter = true; },
	  useSimple: function(){ setter = false; }
	});
	
	$export($export.S + $export.F * !USE_NATIVE, 'Object', {
	  // 19.1.2.2 Object.create(O [, Properties])
	  create: $create,
	  // 19.1.2.4 Object.defineProperty(O, P, Attributes)
	  defineProperty: $defineProperty,
	  // 19.1.2.3 Object.defineProperties(O, Properties)
	  defineProperties: $defineProperties,
	  // 19.1.2.6 Object.getOwnPropertyDescriptor(O, P)
	  getOwnPropertyDescriptor: $getOwnPropertyDescriptor,
	  // 19.1.2.7 Object.getOwnPropertyNames(O)
	  getOwnPropertyNames: $getOwnPropertyNames,
	  // 19.1.2.8 Object.getOwnPropertySymbols(O)
	  getOwnPropertySymbols: $getOwnPropertySymbols
	});
	
	// 24.3.2 JSON.stringify(value [, replacer [, space]])
	$JSON && $export($export.S + $export.F * (!USE_NATIVE || $fails(function(){
	  var S = $Symbol();
	  // MS Edge converts symbol values to JSON as {}
	  // WebKit converts symbol values to JSON as null
	  // V8 throws on boxed symbols
	  return _stringify([S]) != '[null]' || _stringify({a: S}) != '{}' || _stringify(Object(S)) != '{}';
	})), 'JSON', {
	  stringify: function stringify(it){
	    if(it === undefined || isSymbol(it))return; // IE8 returns string on undefined
	    var args = [it]
	      , i    = 1
	      , replacer, $replacer;
	    while(arguments.length > i)args.push(arguments[i++]);
	    replacer = args[1];
	    if(typeof replacer == 'function')$replacer = replacer;
	    if($replacer || !isArray(replacer))replacer = function(key, value){
	      if($replacer)value = $replacer.call(this, key, value);
	      if(!isSymbol(value))return value;
	    };
	    args[1] = replacer;
	    return _stringify.apply($JSON, args);
	  }
	});
	
	// 19.4.3.4 Symbol.prototype[@@toPrimitive](hint)
	$Symbol[PROTOTYPE][TO_PRIMITIVE] || __webpack_require__(472)($Symbol[PROTOTYPE], TO_PRIMITIVE, $Symbol[PROTOTYPE].valueOf);
	// 19.4.3.5 Symbol.prototype[@@toStringTag]
	setToStringTag($Symbol, 'Symbol');
	// 20.2.1.9 Math[@@toStringTag]
	setToStringTag(Math, 'Math', true);
	// 24.3.3 JSON[@@toStringTag]
	setToStringTag(global.JSON, 'JSON', true);

/***/ },

/***/ 464:
/***/ function(module, exports) {

	// https://github.com/zloirock/core-js/issues/86#issuecomment-115759028
	var global = module.exports = typeof window != 'undefined' && window.Math == Math
	  ? window : typeof self != 'undefined' && self.Math == Math ? self : Function('return this')();
	if(typeof __g == 'number')__g = global; // eslint-disable-line no-undef

/***/ },

/***/ 465:
/***/ function(module, exports) {

	var hasOwnProperty = {}.hasOwnProperty;
	module.exports = function(it, key){
	  return hasOwnProperty.call(it, key);
	};

/***/ },

/***/ 466:
/***/ function(module, exports, __webpack_require__) {

	// Thank's IE8 for his funny defineProperty
	module.exports = !__webpack_require__(467)(function(){
	  return Object.defineProperty({}, 'a', {get: function(){ return 7; }}).a != 7;
	});

/***/ },

/***/ 467:
/***/ function(module, exports) {

	module.exports = function(exec){
	  try {
	    return !!exec();
	  } catch(e){
	    return true;
	  }
	};

/***/ },

/***/ 468:
/***/ function(module, exports, __webpack_require__) {

	var global    = __webpack_require__(464)
	  , core      = __webpack_require__(469)
	  , ctx       = __webpack_require__(470)
	  , hide      = __webpack_require__(472)
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

/***/ 469:
/***/ function(module, exports) {

	var core = module.exports = {version: '2.4.0'};
	if(typeof __e == 'number')__e = core; // eslint-disable-line no-undef

/***/ },

/***/ 470:
/***/ function(module, exports, __webpack_require__) {

	// optional / simple context binding
	var aFunction = __webpack_require__(471);
	module.exports = function(fn, that, length){
	  aFunction(fn);
	  if(that === undefined)return fn;
	  switch(length){
	    case 1: return function(a){
	      return fn.call(that, a);
	    };
	    case 2: return function(a, b){
	      return fn.call(that, a, b);
	    };
	    case 3: return function(a, b, c){
	      return fn.call(that, a, b, c);
	    };
	  }
	  return function(/* ...args */){
	    return fn.apply(that, arguments);
	  };
	};

/***/ },

/***/ 471:
/***/ function(module, exports) {

	module.exports = function(it){
	  if(typeof it != 'function')throw TypeError(it + ' is not a function!');
	  return it;
	};

/***/ },

/***/ 472:
/***/ function(module, exports, __webpack_require__) {

	var dP         = __webpack_require__(473)
	  , createDesc = __webpack_require__(479);
	module.exports = __webpack_require__(466) ? function(object, key, value){
	  return dP.f(object, key, createDesc(1, value));
	} : function(object, key, value){
	  object[key] = value;
	  return object;
	};

/***/ },

/***/ 473:
/***/ function(module, exports, __webpack_require__) {

	var anObject       = __webpack_require__(474)
	  , IE8_DOM_DEFINE = __webpack_require__(476)
	  , toPrimitive    = __webpack_require__(478)
	  , dP             = Object.defineProperty;
	
	exports.f = __webpack_require__(466) ? Object.defineProperty : function defineProperty(O, P, Attributes){
	  anObject(O);
	  P = toPrimitive(P, true);
	  anObject(Attributes);
	  if(IE8_DOM_DEFINE)try {
	    return dP(O, P, Attributes);
	  } catch(e){ /* empty */ }
	  if('get' in Attributes || 'set' in Attributes)throw TypeError('Accessors not supported!');
	  if('value' in Attributes)O[P] = Attributes.value;
	  return O;
	};

/***/ },

/***/ 474:
/***/ function(module, exports, __webpack_require__) {

	var isObject = __webpack_require__(475);
	module.exports = function(it){
	  if(!isObject(it))throw TypeError(it + ' is not an object!');
	  return it;
	};

/***/ },

/***/ 475:
/***/ function(module, exports) {

	module.exports = function(it){
	  return typeof it === 'object' ? it !== null : typeof it === 'function';
	};

/***/ },

/***/ 476:
/***/ function(module, exports, __webpack_require__) {

	module.exports = !__webpack_require__(466) && !__webpack_require__(467)(function(){
	  return Object.defineProperty(__webpack_require__(477)('div'), 'a', {get: function(){ return 7; }}).a != 7;
	});

/***/ },

/***/ 477:
/***/ function(module, exports, __webpack_require__) {

	var isObject = __webpack_require__(475)
	  , document = __webpack_require__(464).document
	  // in old IE typeof document.createElement is 'object'
	  , is = isObject(document) && isObject(document.createElement);
	module.exports = function(it){
	  return is ? document.createElement(it) : {};
	};

/***/ },

/***/ 478:
/***/ function(module, exports, __webpack_require__) {

	// 7.1.1 ToPrimitive(input [, PreferredType])
	var isObject = __webpack_require__(475);
	// instead of the ES6 spec version, we didn't implement @@toPrimitive case
	// and the second argument - flag - preferred type is a string
	module.exports = function(it, S){
	  if(!isObject(it))return it;
	  var fn, val;
	  if(S && typeof (fn = it.toString) == 'function' && !isObject(val = fn.call(it)))return val;
	  if(typeof (fn = it.valueOf) == 'function' && !isObject(val = fn.call(it)))return val;
	  if(!S && typeof (fn = it.toString) == 'function' && !isObject(val = fn.call(it)))return val;
	  throw TypeError("Can't convert object to primitive value");
	};

/***/ },

/***/ 479:
/***/ function(module, exports) {

	module.exports = function(bitmap, value){
	  return {
	    enumerable  : !(bitmap & 1),
	    configurable: !(bitmap & 2),
	    writable    : !(bitmap & 4),
	    value       : value
	  };
	};

/***/ },

/***/ 480:
/***/ function(module, exports, __webpack_require__) {

	module.exports = __webpack_require__(472);

/***/ },

/***/ 481:
/***/ function(module, exports, __webpack_require__) {

	var META     = __webpack_require__(482)('meta')
	  , isObject = __webpack_require__(475)
	  , has      = __webpack_require__(465)
	  , setDesc  = __webpack_require__(473).f
	  , id       = 0;
	var isExtensible = Object.isExtensible || function(){
	  return true;
	};
	var FREEZE = !__webpack_require__(467)(function(){
	  return isExtensible(Object.preventExtensions({}));
	});
	var setMeta = function(it){
	  setDesc(it, META, {value: {
	    i: 'O' + ++id, // object ID
	    w: {}          // weak collections IDs
	  }});
	};
	var fastKey = function(it, create){
	  // return primitive with prefix
	  if(!isObject(it))return typeof it == 'symbol' ? it : (typeof it == 'string' ? 'S' : 'P') + it;
	  if(!has(it, META)){
	    // can't set metadata to uncaught frozen object
	    if(!isExtensible(it))return 'F';
	    // not necessary to add metadata
	    if(!create)return 'E';
	    // add missing metadata
	    setMeta(it);
	  // return object ID
	  } return it[META].i;
	};
	var getWeak = function(it, create){
	  if(!has(it, META)){
	    // can't set metadata to uncaught frozen object
	    if(!isExtensible(it))return true;
	    // not necessary to add metadata
	    if(!create)return false;
	    // add missing metadata
	    setMeta(it);
	  // return hash weak collections IDs
	  } return it[META].w;
	};
	// add metadata on freeze-family methods calling
	var onFreeze = function(it){
	  if(FREEZE && meta.NEED && isExtensible(it) && !has(it, META))setMeta(it);
	  return it;
	};
	var meta = module.exports = {
	  KEY:      META,
	  NEED:     false,
	  fastKey:  fastKey,
	  getWeak:  getWeak,
	  onFreeze: onFreeze
	};

/***/ },

/***/ 482:
/***/ function(module, exports) {

	var id = 0
	  , px = Math.random();
	module.exports = function(key){
	  return 'Symbol('.concat(key === undefined ? '' : key, ')_', (++id + px).toString(36));
	};

/***/ },

/***/ 483:
/***/ function(module, exports, __webpack_require__) {

	var global = __webpack_require__(464)
	  , SHARED = '__core-js_shared__'
	  , store  = global[SHARED] || (global[SHARED] = {});
	module.exports = function(key){
	  return store[key] || (store[key] = {});
	};

/***/ },

/***/ 484:
/***/ function(module, exports, __webpack_require__) {

	var def = __webpack_require__(473).f
	  , has = __webpack_require__(465)
	  , TAG = __webpack_require__(485)('toStringTag');
	
	module.exports = function(it, tag, stat){
	  if(it && !has(it = stat ? it : it.prototype, TAG))def(it, TAG, {configurable: true, value: tag});
	};

/***/ },

/***/ 485:
/***/ function(module, exports, __webpack_require__) {

	var store      = __webpack_require__(483)('wks')
	  , uid        = __webpack_require__(482)
	  , Symbol     = __webpack_require__(464).Symbol
	  , USE_SYMBOL = typeof Symbol == 'function';
	
	var $exports = module.exports = function(name){
	  return store[name] || (store[name] =
	    USE_SYMBOL && Symbol[name] || (USE_SYMBOL ? Symbol : uid)('Symbol.' + name));
	};
	
	$exports.store = store;

/***/ },

/***/ 486:
/***/ function(module, exports, __webpack_require__) {

	exports.f = __webpack_require__(485);

/***/ },

/***/ 487:
/***/ function(module, exports, __webpack_require__) {

	var global         = __webpack_require__(464)
	  , core           = __webpack_require__(469)
	  , LIBRARY        = __webpack_require__(488)
	  , wksExt         = __webpack_require__(486)
	  , defineProperty = __webpack_require__(473).f;
	module.exports = function(name){
	  var $Symbol = core.Symbol || (core.Symbol = LIBRARY ? {} : global.Symbol || {});
	  if(name.charAt(0) != '_' && !(name in $Symbol))defineProperty($Symbol, name, {value: wksExt.f(name)});
	};

/***/ },

/***/ 488:
/***/ function(module, exports) {

	module.exports = true;

/***/ },

/***/ 489:
/***/ function(module, exports, __webpack_require__) {

	var getKeys   = __webpack_require__(490)
	  , toIObject = __webpack_require__(492);
	module.exports = function(object, el){
	  var O      = toIObject(object)
	    , keys   = getKeys(O)
	    , length = keys.length
	    , index  = 0
	    , key;
	  while(length > index)if(O[key = keys[index++]] === el)return key;
	};

/***/ },

/***/ 490:
/***/ function(module, exports, __webpack_require__) {

	// 19.1.2.14 / 15.2.3.14 Object.keys(O)
	var $keys       = __webpack_require__(491)
	  , enumBugKeys = __webpack_require__(501);
	
	module.exports = Object.keys || function keys(O){
	  return $keys(O, enumBugKeys);
	};

/***/ },

/***/ 491:
/***/ function(module, exports, __webpack_require__) {

	var has          = __webpack_require__(465)
	  , toIObject    = __webpack_require__(492)
	  , arrayIndexOf = __webpack_require__(496)(false)
	  , IE_PROTO     = __webpack_require__(500)('IE_PROTO');
	
	module.exports = function(object, names){
	  var O      = toIObject(object)
	    , i      = 0
	    , result = []
	    , key;
	  for(key in O)if(key != IE_PROTO)has(O, key) && result.push(key);
	  // Don't enum bug & hidden keys
	  while(names.length > i)if(has(O, key = names[i++])){
	    ~arrayIndexOf(result, key) || result.push(key);
	  }
	  return result;
	};

/***/ },

/***/ 492:
/***/ function(module, exports, __webpack_require__) {

	// to indexed object, toObject with fallback for non-array-like ES3 strings
	var IObject = __webpack_require__(493)
	  , defined = __webpack_require__(495);
	module.exports = function(it){
	  return IObject(defined(it));
	};

/***/ },

/***/ 493:
/***/ function(module, exports, __webpack_require__) {

	// fallback for non-array-like ES3 and non-enumerable old V8 strings
	var cof = __webpack_require__(494);
	module.exports = Object('z').propertyIsEnumerable(0) ? Object : function(it){
	  return cof(it) == 'String' ? it.split('') : Object(it);
	};

/***/ },

/***/ 494:
/***/ function(module, exports) {

	var toString = {}.toString;
	
	module.exports = function(it){
	  return toString.call(it).slice(8, -1);
	};

/***/ },

/***/ 495:
/***/ function(module, exports) {

	// 7.2.1 RequireObjectCoercible(argument)
	module.exports = function(it){
	  if(it == undefined)throw TypeError("Can't call method on  " + it);
	  return it;
	};

/***/ },

/***/ 496:
/***/ function(module, exports, __webpack_require__) {

	// false -> Array#indexOf
	// true  -> Array#includes
	var toIObject = __webpack_require__(492)
	  , toLength  = __webpack_require__(497)
	  , toIndex   = __webpack_require__(499);
	module.exports = function(IS_INCLUDES){
	  return function($this, el, fromIndex){
	    var O      = toIObject($this)
	      , length = toLength(O.length)
	      , index  = toIndex(fromIndex, length)
	      , value;
	    // Array#includes uses SameValueZero equality algorithm
	    if(IS_INCLUDES && el != el)while(length > index){
	      value = O[index++];
	      if(value != value)return true;
	    // Array#toIndex ignores holes, Array#includes - not
	    } else for(;length > index; index++)if(IS_INCLUDES || index in O){
	      if(O[index] === el)return IS_INCLUDES || index || 0;
	    } return !IS_INCLUDES && -1;
	  };
	};

/***/ },

/***/ 497:
/***/ function(module, exports, __webpack_require__) {

	// 7.1.15 ToLength
	var toInteger = __webpack_require__(498)
	  , min       = Math.min;
	module.exports = function(it){
	  return it > 0 ? min(toInteger(it), 0x1fffffffffffff) : 0; // pow(2, 53) - 1 == 9007199254740991
	};

/***/ },

/***/ 498:
/***/ function(module, exports) {

	// 7.1.4 ToInteger
	var ceil  = Math.ceil
	  , floor = Math.floor;
	module.exports = function(it){
	  return isNaN(it = +it) ? 0 : (it > 0 ? floor : ceil)(it);
	};

/***/ },

/***/ 499:
/***/ function(module, exports, __webpack_require__) {

	var toInteger = __webpack_require__(498)
	  , max       = Math.max
	  , min       = Math.min;
	module.exports = function(index, length){
	  index = toInteger(index);
	  return index < 0 ? max(index + length, 0) : min(index, length);
	};

/***/ },

/***/ 500:
/***/ function(module, exports, __webpack_require__) {

	var shared = __webpack_require__(483)('keys')
	  , uid    = __webpack_require__(482);
	module.exports = function(key){
	  return shared[key] || (shared[key] = uid(key));
	};

/***/ },

/***/ 501:
/***/ function(module, exports) {

	// IE 8- don't enum bug keys
	module.exports = (
	  'constructor,hasOwnProperty,isPrototypeOf,propertyIsEnumerable,toLocaleString,toString,valueOf'
	).split(',');

/***/ },

/***/ 502:
/***/ function(module, exports, __webpack_require__) {

	// all enumerable object keys, includes symbols
	var getKeys = __webpack_require__(490)
	  , gOPS    = __webpack_require__(503)
	  , pIE     = __webpack_require__(504);
	module.exports = function(it){
	  var result     = getKeys(it)
	    , getSymbols = gOPS.f;
	  if(getSymbols){
	    var symbols = getSymbols(it)
	      , isEnum  = pIE.f
	      , i       = 0
	      , key;
	    while(symbols.length > i)if(isEnum.call(it, key = symbols[i++]))result.push(key);
	  } return result;
	};

/***/ },

/***/ 503:
/***/ function(module, exports) {

	exports.f = Object.getOwnPropertySymbols;

/***/ },

/***/ 504:
/***/ function(module, exports) {

	exports.f = {}.propertyIsEnumerable;

/***/ },

/***/ 505:
/***/ function(module, exports, __webpack_require__) {

	// 7.2.2 IsArray(argument)
	var cof = __webpack_require__(494);
	module.exports = Array.isArray || function isArray(arg){
	  return cof(arg) == 'Array';
	};

/***/ },

/***/ 506:
/***/ function(module, exports, __webpack_require__) {

	// 19.1.2.2 / 15.2.3.5 Object.create(O [, Properties])
	var anObject    = __webpack_require__(474)
	  , dPs         = __webpack_require__(507)
	  , enumBugKeys = __webpack_require__(501)
	  , IE_PROTO    = __webpack_require__(500)('IE_PROTO')
	  , Empty       = function(){ /* empty */ }
	  , PROTOTYPE   = 'prototype';
	
	// Create object with fake `null` prototype: use iframe Object with cleared prototype
	var createDict = function(){
	  // Thrash, waste and sodomy: IE GC bug
	  var iframe = __webpack_require__(477)('iframe')
	    , i      = enumBugKeys.length
	    , gt     = '>'
	    , iframeDocument;
	  iframe.style.display = 'none';
	  __webpack_require__(508).appendChild(iframe);
	  iframe.src = 'javascript:'; // eslint-disable-line no-script-url
	  // createDict = iframe.contentWindow.Object;
	  // html.removeChild(iframe);
	  iframeDocument = iframe.contentWindow.document;
	  iframeDocument.open();
	  iframeDocument.write('<script>document.F=Object</script' + gt);
	  iframeDocument.close();
	  createDict = iframeDocument.F;
	  while(i--)delete createDict[PROTOTYPE][enumBugKeys[i]];
	  return createDict();
	};
	
	module.exports = Object.create || function create(O, Properties){
	  var result;
	  if(O !== null){
	    Empty[PROTOTYPE] = anObject(O);
	    result = new Empty;
	    Empty[PROTOTYPE] = null;
	    // add "__proto__" for Object.getPrototypeOf polyfill
	    result[IE_PROTO] = O;
	  } else result = createDict();
	  return Properties === undefined ? result : dPs(result, Properties);
	};

/***/ },

/***/ 507:
/***/ function(module, exports, __webpack_require__) {

	var dP       = __webpack_require__(473)
	  , anObject = __webpack_require__(474)
	  , getKeys  = __webpack_require__(490);
	
	module.exports = __webpack_require__(466) ? Object.defineProperties : function defineProperties(O, Properties){
	  anObject(O);
	  var keys   = getKeys(Properties)
	    , length = keys.length
	    , i = 0
	    , P;
	  while(length > i)dP.f(O, P = keys[i++], Properties[P]);
	  return O;
	};

/***/ },

/***/ 508:
/***/ function(module, exports, __webpack_require__) {

	module.exports = __webpack_require__(464).document && document.documentElement;

/***/ },

/***/ 509:
/***/ function(module, exports, __webpack_require__) {

	// fallback for IE11 buggy Object.getOwnPropertyNames with iframe and window
	var toIObject = __webpack_require__(492)
	  , gOPN      = __webpack_require__(510).f
	  , toString  = {}.toString;
	
	var windowNames = typeof window == 'object' && window && Object.getOwnPropertyNames
	  ? Object.getOwnPropertyNames(window) : [];
	
	var getWindowNames = function(it){
	  try {
	    return gOPN(it);
	  } catch(e){
	    return windowNames.slice();
	  }
	};
	
	module.exports.f = function getOwnPropertyNames(it){
	  return windowNames && toString.call(it) == '[object Window]' ? getWindowNames(it) : gOPN(toIObject(it));
	};


/***/ },

/***/ 510:
/***/ function(module, exports, __webpack_require__) {

	// 19.1.2.7 / 15.2.3.4 Object.getOwnPropertyNames(O)
	var $keys      = __webpack_require__(491)
	  , hiddenKeys = __webpack_require__(501).concat('length', 'prototype');
	
	exports.f = Object.getOwnPropertyNames || function getOwnPropertyNames(O){
	  return $keys(O, hiddenKeys);
	};

/***/ },

/***/ 511:
/***/ function(module, exports, __webpack_require__) {

	var pIE            = __webpack_require__(504)
	  , createDesc     = __webpack_require__(479)
	  , toIObject      = __webpack_require__(492)
	  , toPrimitive    = __webpack_require__(478)
	  , has            = __webpack_require__(465)
	  , IE8_DOM_DEFINE = __webpack_require__(476)
	  , gOPD           = Object.getOwnPropertyDescriptor;
	
	exports.f = __webpack_require__(466) ? gOPD : function getOwnPropertyDescriptor(O, P){
	  O = toIObject(O);
	  P = toPrimitive(P, true);
	  if(IE8_DOM_DEFINE)try {
	    return gOPD(O, P);
	  } catch(e){ /* empty */ }
	  if(has(O, P))return createDesc(!pIE.f.call(O, P), O[P]);
	};

/***/ },

/***/ 512:
/***/ function(module, exports) {



/***/ },

/***/ 513:
/***/ function(module, exports, __webpack_require__) {

	__webpack_require__(487)('asyncIterator');

/***/ },

/***/ 514:
/***/ function(module, exports, __webpack_require__) {

	__webpack_require__(487)('observable');

/***/ },

/***/ 515:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	
	var _defineProperty2 = __webpack_require__(516);
	
	var _defineProperty3 = _interopRequireDefault(_defineProperty2);
	
	var _TAB_QUESTION, _TAB_ANSWER, _TAB_DESCRIPTION, _initialState;
	
	function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }
	
	var React = __webpack_require__(520);
	
	var _require = __webpack_require__(526);
	
	var Provider = _require.Provider;
	var connect = _require.connect;
	
	var _require2 = __webpack_require__(533);
	
	var createStore = _require2.createStore;
	
	var classnames = __webpack_require__(548);
	var TextareaAutosize = __webpack_require__(549).default;
	
	var _require3 = __webpack_require__(554);
	
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
	
	var _require4 = __webpack_require__(551);
	
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
	
	var CardReviewTabsComponent = connect(
	// mapStateToProps
	function (state) {
	    return (0, _defineProperty3.default)({}, TAB, state[TAB]);
	})(__CardReviewTabsComponent);
	
	var __RenderSourceComponent = function __RenderSourceComponent(props) {
	
	    // NOTE: switchTab(dispatch: dispatch, next_view: MARKDOWN_VIEW);
	
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
	                    href: '#render',
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
	                    href: '#source',
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
	
	var CustomScoreButtonComponent = function CustomScoreButtonComponent(props) {
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
	
	var rehydrate = __webpack_require__(460);
	
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

/***/ },

/***/ 516:
/***/ function(module, exports, __webpack_require__) {

	"use strict";
	
	exports.__esModule = true;
	
	var _defineProperty = __webpack_require__(517);
	
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

/***/ 517:
/***/ function(module, exports, __webpack_require__) {

	module.exports = { "default": __webpack_require__(518), __esModule: true };

/***/ },

/***/ 518:
/***/ function(module, exports, __webpack_require__) {

	__webpack_require__(519);
	var $Object = __webpack_require__(469).Object;
	module.exports = function defineProperty(it, key, desc){
	  return $Object.defineProperty(it, key, desc);
	};

/***/ },

/***/ 519:
/***/ function(module, exports, __webpack_require__) {

	var $export = __webpack_require__(468);
	// 19.1.2.4 / 15.2.3.6 Object.defineProperty(O, P, Attributes)
	$export($export.S + $export.F * !__webpack_require__(466), 'Object', {defineProperty: __webpack_require__(473).f});

/***/ },

/***/ 551:
/***/ function(module, exports, __webpack_require__) {

	/* WEBPACK VAR INJECTION */(function(process) {'use strict';
	
	// TODO: doc all the things... and cleanly.
	
	// TODO: specific lodash dependencies
	var _ = __webpack_require__(552);
	var isFunction = _.isFunction;
	// TODO: npm install npm.im/warning
	
	// sentinel value
	var NOT_SET = {};
	
	var __getIn = function __getIn(rootData, path) {
	    // TODO: check path is array
	    return _.get(rootData, path);
	};
	
	var __setIn = function __setIn(rootData, path, newValue) {
	    // TODO: check path is array
	    var patch = _.set({}, path, newValue);
	    // NOTE: the following will not work: {...state, ...patch};
	    return _.merge({}, rootData, patch);
	};
	
	var treeReducer = function treeReducer(state, action) {
	
	    if (process.env.NODE_ENV !== 'production') {
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
	
	
	    if (process.env.NODE_ENV !== 'production') {
	
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
	
	
	        if (process.env.NODE_ENV !== 'production') {
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
	
	    return shouldPollute ? _.merge(action, patch) : _.merge({}, action, patch);
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
	/* WEBPACK VAR INJECTION */}.call(exports, __webpack_require__(296)))

/***/ },

/***/ 554:
/***/ function(module, exports) {

	'use strict';
	
	/* constants */
	
	/*
	enum Tab {
	    Question,
	    Answer,
	    Description
	}
	 */
	var TAB = 'TAB'; // key
	var TAB_QUESTION = 'TAB_QUESTION';
	var TAB_ANSWER = 'TAB_ANSWER';
	var TAB_DESCRIPTION = 'TAB_DESCRIPTION';
	
	/*
	enum MarkdownView {
	    Render,
	    Source
	}
	 */
	var MARKDOWN_VIEW = 'MARKDOWN_VIEW'; // key
	var MARKDOWN_VIEW_RENDER = 'MARKDOWN_VIEW_RENDER';
	var MARKDOWN_VIEW_SOURCE = 'MARKDOWN_VIEW_SOURCE';
	
	// key
	var CARD_CONTENTS = 'CARD_CONTENTS';
	
	/*
	// UI state machine
	enum CardPerformanceControl {
	    Initial, // empty ui
	    DefaultChoices,
	    CustomScore,
	}
	 */
	var CARD_PERF_CONTROL_VIEW = 'CARD_PERF_CONTROL_VIEW'; // key
	var CARD_PERF_CONTROL__INITIAL = 'CARD_PERF_CONTROL__INITIAL'; // empty UI
	var CARD_PERF_CONTROL__DEFAULT_CHOICES = 'CARD_PERF_CONTROL__DEFAULT_CHOICES';
	
	/*
	enum SkipCardView {
	    Initial, // not confirming skip
	    Confirm
	}
	 */
	var SKIPCARD_VIEW = 'SKIPCARD_VIEW';
	var SKIPCARD_INITIAL = 'SKIPCARD_INITIAL';
	var SKIPCARD_CONFIRM = 'SKIPCARD_CONFIRM';
	
	module.exports = {
	
	    TAB: TAB,
	    TAB_QUESTION: TAB_QUESTION,
	    TAB_ANSWER: TAB_ANSWER,
	    TAB_DESCRIPTION: TAB_DESCRIPTION,
	
	    MARKDOWN_VIEW: MARKDOWN_VIEW,
	    MARKDOWN_VIEW_RENDER: MARKDOWN_VIEW_RENDER,
	    MARKDOWN_VIEW_SOURCE: MARKDOWN_VIEW_SOURCE,
	
	    CARD_CONTENTS: CARD_CONTENTS,
	
	    CARD_PERF_CONTROL_VIEW: CARD_PERF_CONTROL_VIEW,
	    CARD_PERF_CONTROL__INITIAL: CARD_PERF_CONTROL__INITIAL,
	    CARD_PERF_CONTROL__DEFAULT_CHOICES: CARD_PERF_CONTROL__DEFAULT_CHOICES,
	
	    SKIPCARD_VIEW: SKIPCARD_VIEW,
	    SKIPCARD_INITIAL: SKIPCARD_INITIAL,
	    SKIPCARD_CONFIRM: SKIPCARD_CONFIRM
	
	};

/***/ }

});
//# sourceMappingURL=deck_review.js.map