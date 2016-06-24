const React = require('react');
const TextareaAutosize = require('react-textarea-autosize').default;
const has = require('lodash/has');

const MarkdownSource = function(props) {

    const contents = has(props.assignProps, 'value') ?
        (props.assignProps.value || '') : (props.contents || '')

    return (<TextareaAutosize

        style={props.style}
        // key='textarea'
        useCacheForDOMMeasurements
        minRows={6}
        maxRows={10}
        className='form-input'
        // id="deck_source"
        placeholder={props.placeholder}
        // onChange={this.onSourceChange}
        {...props.assignProps}
        value={contents}
    />);
};

MarkdownSource.defaultProps = {
    placeholder: '',
    style: {},
    assignProps: {},
    contents: ''
};

if(process.env.NODE_ENV !== 'production') {
    MarkdownSource.propTypes = {
        style: React.PropTypes.object.isRequired,
        contents: React.PropTypes.string.isRequired,
        editable: React.PropTypes.bool.isRequired,
        placeholder: React.PropTypes.string.isRequired,
        assignProps: React.PropTypes.object,
    };
}

module.exports = MarkdownSource;
