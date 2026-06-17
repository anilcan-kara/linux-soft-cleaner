#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const os = require('os');
const { spawnSync } = require('child_process');
const https = require('https');

const VERSION = '0.1.0';
const REPO = 'anilcan-kara/linux-soft-cleaner';

function getTarget() {
    const platform = os.platform();
    const arch = os.arch();

    if (platform === 'linux') {
        if (arch === 'x64') return 'x86_64-unknown-linux-musl';
        if (arch === 'arm64') return 'aarch64-unknown-linux-musl';
    } else if (platform === 'darwin') {
        if (arch === 'x64') return 'x86_64-apple-darwin';
        if (arch === 'arm64') return 'aarch64-apple-darwin';
    }
    return null;
}

function downloadFile(url, destPath) {
    return new Promise((resolve, reject) => {
        const file = fs.createWriteStream(destPath);
        
        const request = (targetUrl) => {
            https.get(targetUrl, (response) => {
                if (response.statusCode === 301 || response.statusCode === 302) {
                    // Handle redirects
                    request(response.headers.location);
                    return;
                }
                
                if (response.statusCode !== 200) {
                    reject(new Error(`Failed to download binary: HTTP ${response.statusCode}`));
                    return;
                }

                response.pipe(file);
                
                file.on('finish', () => {
                    file.close();
                    resolve();
                });
            }).on('error', (err) => {
                fs.unlink(destPath, () => {});
                reject(err);
            });
        };

        request(url);
    });
}

function main() {
    const target = getTarget();
    if (!target) {
        console.error(`Error: Unsupported platform/architecture (${os.platform()}/${os.arch()}).`);
        console.error("linux-soft-cleaner only supports 64-bit Linux and macOS.");
        process.exit(1);
    }

    const homeDir = os.homedir();
    const installDir = path.join(homeDir, '.linux-soft-cleaner', `v${VERSION}`);
    const binName = 'linux-soft-cleaner';
    const binPath = path.join(installDir, binName);

    if (fs.existsSync(binPath)) {
        // Run the binary directly
        runBinary(binPath);
        return;
    }

    // Binary does not exist, download it
    console.log(`linux-soft-cleaner binary not found locally. Downloading for ${target}...`);
    
    if (!fs.existsSync(installDir)) {
        fs.mkdirSync(installDir, { recursive: true });
    }

    const archiveName = `linux-soft-cleaner-${target}.tar.gz`;
    const archivePath = path.join(installDir, archiveName);
    const downloadUrl = `https://github.com/${REPO}/releases/download/v${VERSION}/${archiveName}`;

    downloadFile(downloadUrl, archivePath)
        .then(() => {
            // Extract the tar.gz file
            console.log("Extracting archive...");
            const tarResult = spawnSync('tar', ['-xzf', archivePath, '-C', installDir]);
            if (tarResult.status !== 0) {
                throw new Error(`Failed to extract tar archive: ${tarResult.stderr.toString()}`);
            }

            // Cleanup archive file
            try {
                fs.unlinkSync(archivePath);
            } catch (e) {}

            // Ensure executable permissions
            try {
                fs.chmodSync(binPath, '755');
            } catch (e) {}

            console.log("Download and extraction complete.\n");
            runBinary(binPath);
        })
        .catch((err) => {
            console.error(`Error: Failed to install binary: ${err.message}`);
            // Cleanup folders on failure
            try {
                fs.unlinkSync(archivePath);
            } catch (e) {}
            process.exit(1);
        });
}

function runBinary(binaryPath) {
    const args = process.argv.slice(2);
    const result = spawnSync(binaryPath, args, { stdio: 'inherit' });
    process.exit(result.status ?? 0);
}

main();
