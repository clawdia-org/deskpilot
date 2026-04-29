#!/usr/bin/env node

const https = require('https');
const fs = require('fs');
const path = require('path');
const { execFileSync } = require('child_process');

const REPO = 'clawdia-org/deskpilot';
const BINARY_NAME = 'deskpilot';

function getPlatform() {
    switch (process.platform) {
        case 'darwin': return 'apple-darwin';
        case 'linux': return 'unknown-linux-gnu';
        case 'win32': return 'pc-windows-msvc';
        default: throw new Error(`Unsupported platform: ${process.platform}`);
    }
}

function getArch() {
    switch (process.arch) {
        case 'x64': return 'x86_64';
        case 'arm64': return 'aarch64';
        default: throw new Error(`Unsupported architecture: ${process.arch}`);
    }
}

function getVersion() {
    // Use the npm package version to ensure reproducible installs
    const pkg = require('../package.json');
    return `v${pkg.version}`;
}

function downloadFile(url, dest) {
    return new Promise((resolve, reject) => {
        const request = (targetUrl) => {
            https.get(targetUrl, { headers: { 'User-Agent': 'deskpilot-npm-installer' } }, (res) => {
                if (res.statusCode === 301 || res.statusCode === 302) {
                    // Consume and discard the redirect response body before following
                    res.resume();
                    return request(res.headers.location);
                }
                if (res.statusCode !== 200) {
                    res.resume();
                    return reject(new Error(`HTTP ${res.statusCode} downloading ${targetUrl}`));
                }
                const file = fs.createWriteStream(dest);
                res.pipe(file);
                file.on('finish', () => file.close(resolve));
                file.on('error', (err) => {
                    fs.unlink(dest, () => {});
                    reject(err);
                });
            }).on('error', reject);
        };
        request(url);
    });
}

function extract(archivePath, destDir) {
    if (archivePath.endsWith('.zip')) {
        if (process.platform === 'win32') {
            execFileSync('powershell', [
                '-NoProfile', '-Command',
                `Expand-Archive -Path "${archivePath}" -DestinationPath "${destDir}" -Force`
            ]);
        } else {
            execFileSync('unzip', ['-o', archivePath, '-d', destDir]);
        }
    } else {
        execFileSync('tar', ['-xzf', archivePath, '-C', destDir]);
    }
}

async function main() {
    const platform = getPlatform();
    const arch = getArch();
    const target = `${arch}-${platform}`;

    console.log(`Installing deskpilot for ${target}...`);

    const version = getVersion();
    console.log(`Version: ${version}`);

    const ext = process.platform === 'win32' ? 'zip' : 'tar.gz';
    const artifactName = `${BINARY_NAME}-${target}.${ext}`;
    const downloadUrl = `https://github.com/${REPO}/releases/download/${version}/${artifactName}`;

    const tmpDir = path.join(__dirname, '..', 'tmp');
    const binariesDir = path.join(__dirname, '..', 'binaries', process.platform, process.arch);

    fs.mkdirSync(tmpDir, { recursive: true });
    fs.mkdirSync(binariesDir, { recursive: true });

    const archivePath = path.join(tmpDir, artifactName);

    console.log(`Downloading ${downloadUrl}...`);
    await downloadFile(downloadUrl, archivePath);

    console.log('Extracting...');
    extract(archivePath, tmpDir);

    const binaryName = process.platform === 'win32' ? `${BINARY_NAME}.exe` : BINARY_NAME;
    const extractedBinary = path.join(tmpDir, binaryName);
    const destBinary = path.join(binariesDir, binaryName);

    fs.copyFileSync(extractedBinary, destBinary);
    fs.chmodSync(destBinary, 0o755);

    // Cleanup
    fs.rmSync(tmpDir, { recursive: true, force: true });

    console.log('Installation complete!');
}

main().catch(err => {
    console.error('Installation failed:', err.message);
    process.exit(1);
});
