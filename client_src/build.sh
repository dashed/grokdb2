#!/usr/bin/env bash

DEST_OUTPUT='../src/react_components'

mkdir -p "${DEST_OUTPUT}"

# components

NODE_PATH=./src/ node generate_component.js 'components/deck_review.js' > "${DEST_OUTPUT}/deck_review"