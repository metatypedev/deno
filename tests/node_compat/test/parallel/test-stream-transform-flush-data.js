// deno-fmt-ignore-file
// deno-lint-ignore-file

// Copyright Joyent and Node contributors. All rights reserved. MIT license.
// Taken from Node 20.11.1
// This file is automatically generated by `tests/node_compat/runner/setup.ts`. Do not modify this file manually.

'use strict';

require('../common');

const assert = require('assert');
const Transform = require('stream').Transform;


const expected = 'asdf';


function _transform(d, e, n) {
  n();
}

function _flush(n) {
  n(null, expected);
}

const t = new Transform({
  transform: _transform,
  flush: _flush
});

t.end(Buffer.from('blerg'));
t.on('data', (data) => {
  assert.strictEqual(data.toString(), expected);
});
