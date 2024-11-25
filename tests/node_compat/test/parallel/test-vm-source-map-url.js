// deno-fmt-ignore-file
// deno-lint-ignore-file

// Copyright Joyent and Node contributors. All rights reserved. MIT license.
// Taken from Node 20.11.1
// This file is automatically generated by `tests/node_compat/runner/setup.ts`. Do not modify this file manually.

'use strict';

require('../common');
const assert = require('assert');
const vm = require('vm');

function checkSourceMapUrl(source, expectedSourceMapURL) {
  const script = new vm.Script(source);
  assert.strictEqual(script.sourceMapURL, expectedSourceMapURL);
}

// No magic comment
checkSourceMapUrl(`
function myFunc() {}
`, undefined);

// Malformed magic comment
checkSourceMapUrl(`
function myFunc() {}
// sourceMappingURL=sourcemap.json
`, undefined);

// Expected magic comment
checkSourceMapUrl(`
function myFunc() {}
//# sourceMappingURL=sourcemap.json
`, 'sourcemap.json');
