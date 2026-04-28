#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');

const binaryPath = path.join(__dirname, '..', 'binaries', process.platform, process.arch, 'deskpilot' + (process.platform === 'win32' ? '.exe' : ''));

const child = spawn(binaryPath, process.argv.slice(2), {
    stdio: 'inherit',
    env: process.env
});

child.on('exit', (code) => {
    process.exit(code || 0);
});
