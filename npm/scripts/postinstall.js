#!/usr/bin/env node

const https = require('https');
const fs = require('fs');
const path = require('path');
const zlib = require('zlib');

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

async function getLatestVersion() {
    return new Promise((resolve, reject) => {
        https.get(`https://api.github.com/repos/${REPO}/releases/latest`, {
            headers: { 'User-Agent': 'deskpilot-npm-installer' }
        }, (res) => {
            let data = '';
            res.on('data', chunk => data += chunk);
            res.on('end', () => {
                try {
                    const json = JSON.parse(data);
                    resolve(json.tag_name);
                } catch (e) {
                    reject(e);
                }
            });
        }).on('error', reject);
    });
}

async function downloadFile(url, dest) {
    return new Promise((resolve, reject) => {
        const file = fs.createWriteStream(dest);
        https.get(url, {
            headers: { 'User-Agent': 'deskpilot-npm-installer' }
        }, (res) => {
            if (res.statusCode === 302 || res.statusCode === 301) {
                downloadFile(res.headers.location, dest).then(resolve).catch(reject);
                return;
            }
            res.pipe(file);
            file.on('finish', () => {
                file.close();
                resolve();
            });
        }).on('error', (err) => {
            fs.unlink(dest, () => {});
            reject(err);
        });
    });
}

async function extractTarGz(src, dest) {
    return new Promise((resolve, reject) => {
        const gunzip = zlib.createGunzip();
        const extract = require('tar').extract({ cwd: dest });
        
        fs.createReadStream(src)
            .pipe(gunzip)
            .pipe(extract)
            .on('finish', resolve)
            .on('error', reject);
    });
}

async function extractZip(src, dest) {
    const AdmZip = require('adm-zip');
    const zip = new AdmZip(src);
    zip.extractAllTo(dest, true);
}

async function main() {
    const platform = getPlatform();
    const arch = getArch();
    const target = `${arch}-${platform}`;
    
    console.log(`Installing deskpilot for ${target}...`);
    
    const version = await getLatestVersion();
    console.log(`Latest version: ${version}`);
    
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
    if (ext === 'tar.gz') {
        await extractTarGz(archivePath, tmpDir);
    } else {
        await extractZip(archivePath, tmpDir);
    }
    
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
