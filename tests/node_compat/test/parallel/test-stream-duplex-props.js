// deno-fmt-ignore-file
// deno-lint-ignore-file

// Copyright Joyent and Node contributors. All rights reserved. MIT license.
// Taken from Node 20.11.1
// This file is automatically generated by `tests/node_compat/runner/setup.ts`. Do not modify this file manually.

'use strict';

require('../common');
const assert = require('assert');
const { Duplex } = require('stream');

{
  const d = new Duplex({
    objectMode: true,
    highWaterMark: 100
  });

  assert.strictEqual(d.writableObjectMode, true);
  assert.strictEqual(d.writableHighWaterMark, 100);
  assert.strictEqual(d.readableObjectMode, true);
  assert.strictEqual(d.readableHighWaterMark, 100);
}

{
  const d = new Duplex({
    readableObjectMode: false,
    readableHighWaterMark: 10,
    writableObjectMode: true,
    writableHighWaterMark: 100
  });

  assert.strictEqual(d.writableObjectMode, true);
  assert.strictEqual(d.writableHighWaterMark, 100);
  assert.strictEqual(d.readableObjectMode, false);
  assert.strictEqual(d.readableHighWaterMark, 10);
}
