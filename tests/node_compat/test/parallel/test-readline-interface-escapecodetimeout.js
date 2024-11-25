// deno-fmt-ignore-file
// deno-lint-ignore-file

// Copyright Joyent and Node contributors. All rights reserved. MIT license.
// Taken from Node 20.11.1
// This file is automatically generated by `tests/node_compat/runner/setup.ts`. Do not modify this file manually.

'use strict';
require('../common');

// This test ensures that the escapeCodeTimeout option set correctly

const assert = require('assert');
const readline = require('readline');
const EventEmitter = require('events').EventEmitter;

class FakeInput extends EventEmitter {
  resume() {}
  pause() {}
  write() {}
  end() {}
}

{
  const fi = new FakeInput();
  const rli = new readline.Interface({
    input: fi,
    output: fi,
    escapeCodeTimeout: 50
  });
  assert.strictEqual(rli.escapeCodeTimeout, 50);
  rli.close();
}

[
  null,
  {},
  NaN,
  '50',
].forEach((invalidInput) => {
  assert.throws(() => {
    const fi = new FakeInput();
    const rli = new readline.Interface({
      input: fi,
      output: fi,
      escapeCodeTimeout: invalidInput
    });
    rli.close();
  }, {
    name: 'TypeError',
    code: 'ERR_INVALID_ARG_VALUE'
  });
});
