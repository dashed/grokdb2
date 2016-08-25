#!/usr/bin/env bash
#
# generate static react components for injection in rust template library

DEST_OUTPUT='../src/react_components'

mkdir -p "${DEST_OUTPUT}"

# components

# NODE_PATH=./src/ node generate_component.js 'components/deck_review.js' > "${DEST_OUTPUT}/deck_review"
NODE_PATH=./src/ node generate_component.js 'components/new_deck.js' > "${DEST_OUTPUT}/new_deck"
NODE_PATH=./src/ node generate_component.js 'components/deck_description.js' > "${DEST_OUTPUT}/deck_description"
