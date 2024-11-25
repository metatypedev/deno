// deno-fmt-ignore-file
// deno-lint-ignore-file

// Copyright Joyent and Node contributors. All rights reserved. MIT license.
// Taken from Node 20.11.1
// This file is automatically generated by `tests/node_compat/runner/setup.ts`. Do not modify this file manually.

'use strict';
const common = require('../common');
const fs = require('fs');

const tmpdir = require('../common/tmpdir');
tmpdir.refresh();

{
  const file = tmpdir.resolve('write-end-test0.txt');
  const stream = fs.createWriteStream(file, {
    fs: {
      open: common.mustCall(fs.open),
      write: common.mustCallAtLeast(fs.write, 1),
      close: common.mustCall(fs.close),
    }
  });
  stream.end('asd');
  stream.on('close', common.mustCall());
}


{
  const file = tmpdir.resolve('write-end-test1.txt');
  const stream = fs.createWriteStream(file, {
    fs: {
      open: common.mustCall(fs.open),
      write: fs.write,
      writev: common.mustCallAtLeast(fs.writev, 1),
      close: common.mustCall(fs.close),
    }
  });
  stream.write('asd');
  stream.write('asd');
  stream.write('asd');
  stream.end();
  stream.on('close', common.mustCall());
}
