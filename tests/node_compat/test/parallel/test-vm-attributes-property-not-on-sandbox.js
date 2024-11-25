// deno-fmt-ignore-file
// deno-lint-ignore-file

// Copyright Joyent and Node contributors. All rights reserved. MIT license.
// Taken from Node 20.11.1
// This file is automatically generated by `tests/node_compat/runner/setup.ts`. Do not modify this file manually.

'use strict';
require('../common');
const assert = require('assert');
const vm = require('vm');

// Assert that accessor descriptors are not flattened on the sandbox.
// Issue: https://github.com/nodejs/node/issues/2734
const sandbox = {};
vm.createContext(sandbox);
const code = `Object.defineProperty(
               this,
               'foo',
               { get: function() {return 17} }
             );
             var desc = Object.getOwnPropertyDescriptor(this, 'foo');`;

vm.runInContext(code, sandbox);
assert.strictEqual(typeof sandbox.desc.get, 'function');
