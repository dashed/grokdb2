const React = require('react');
const TextareaAutosize = require('react-textarea-autosize').default;

const MarkdownSource = function(props) {

    return (<TextareaAutosize style={props.style}
        key='textarea'
        useCacheForDOMMeasurements
        minRows={6}
        maxRows={10}
        className='form-input'
        // id="deck_source"
        placeholder={props.placeholder}
        // onChange={this.onSourceChange}
        value={props.contents}
        readOnly={!props.editable}
    />);
};

MarkdownSource.defaultProps = {
    placeholder: '',
    style: {}
};

if(process.env.NODE_ENV !== 'production') {
    MarkdownSource.propTypes = {
        style: React.PropTypes.object,
        contents: React.PropTypes.string,
        editable: React.PropTypes.bool,
        placeholder: React.PropTypes.string,
    };
}

module.exports = MarkdownSource;
