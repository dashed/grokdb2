/*

- Find all elements with mathjax_inline class.
- Render mathjax in those elements.

*/
const React = require('react');
const ReactDOM = require('react-dom');

const forEach = require('lodash/forEach');

const MathJaxRenderInline = require('components/dumb/mathjax_inline');

forEach(document.getElementsByClassName('mathjax_inline_pre'), function(elem) {

    const textContent = elem.childNodes[0].nodeValue;

    setTimeout(function(){
        ReactDOM.render(
            <MathJaxRenderInline contents={textContent} />,
            elem,
        );
    }, 0);

});
