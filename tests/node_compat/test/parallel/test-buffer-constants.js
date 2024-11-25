// deno-fmt-ignore-file
// deno-lint-ignore-file

// Copyright Joyent and Node contributors. All rights reserved. MIT license.
// Taken from Node 20.11.1
// This file is automatically generated by `tests/node_compat/runner/setup.ts`. Do not modify this file manually.

'use strict';
require('../common');
const assert = require('assert');

const { kMaxLength, kStringMaxLength } = require('buffer');
const { MAX_LENGTH, MAX_STRING_LENGTH } = require('buffer').constants;

assert.strictEqual(typeof MAX_LENGTH, 'number');
assert.strictEqual(typeof MAX_STRING_LENGTH, 'number');
assert(MAX_STRING_LENGTH <= MAX_LENGTH);
assert.throws(() => ' '.repeat(MAX_STRING_LENGTH + 1),
              /^RangeError: Invalid string length$/);

' '.repeat(MAX_STRING_LENGTH); // Should not throw.

// Legacy values match:
assert.strictEqual(kMaxLength, MAX_LENGTH);
assert.strictEqual(kStringMaxLength, MAX_STRING_LENGTH);
