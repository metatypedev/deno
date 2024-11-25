// deno-fmt-ignore-file
// deno-lint-ignore-file

// Copyright Joyent and Node contributors. All rights reserved. MIT license.
// Taken from Node 20.11.1
// This file is automatically generated by `tests/node_compat/runner/setup.ts`. Do not modify this file manually.

'use strict';

require('../common');
const assert = require('assert');
const { Writable } = require('stream');

{
  const writable = new Writable({
    write() {
    }
  });
  assert.strictEqual(writable.writableAborted, false);
  writable.destroy();
  assert.strictEqual(writable.writableAborted, true);
}

{
  const writable = new Writable({
    write() {
    }
  });
  assert.strictEqual(writable.writableAborted, false);
  writable.end();
  writable.destroy();
  assert.strictEqual(writable.writableAborted, true);
}
