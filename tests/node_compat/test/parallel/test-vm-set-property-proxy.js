// deno-fmt-ignore-file
// deno-lint-ignore-file

// Copyright Joyent and Node contributors. All rights reserved. MIT license.
// Taken from Node 20.11.1
// This file is automatically generated by `tests/node_compat/runner/setup.ts`. Do not modify this file manually.

'use strict';
const common = require('../common');
const assert = require('assert');
const vm = require('vm');

// Regression test for https://github.com/nodejs/node/issues/34606

const handler = {
  getOwnPropertyDescriptor: common.mustCallAtLeast(() => {
    return {};
  })
};

const proxy = new Proxy({}, handler);
assert.throws(() => vm.runInNewContext('p = 6', proxy),
              /getOwnPropertyDescriptor/);
