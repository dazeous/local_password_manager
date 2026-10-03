# Password Manager

A simple cross-platform password manager built with [Tauri](https://tauri.app/), React, and Rust. It securely encrypts stored credentials using AES-GCM and keeps them in a local JSON file.

## Features

- Save credentials (app name, username, password)
- Search for saved credentials
- Delete stored credentials
- AES-GCM encryption for passwords
- Local storage in an encrypted/encoded form
- Cross-platform desktop app

## Tech Stack

- Frontend: React + Vite
- Backend: Rust + Tauri
- Crypto: aes-gcm, rand, base64

## Prerequisites

- [Node.js](https://nodejs.org/) (v18+ recommended)
- [Rust](https://www.rust-lang.org/tools/install)
- [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) for your OS

## Setup

1. Clone the repository
2. Create a .env file in the src-tauri directory with:
   - FILE_NAME=passwords.json
   - MASTER_KEY=your-32-byte-secret-key-here
3. npm install

## Development

npm run tauri dev

## Build

npm run tauri build

## Environment Variables

| Variable   | Description                                  | Required |
| ---------- | -------------------------------------------- | -------- |
| FILE_NAME  | Name of the local JSON file to store entries | Yes      |
| MASTER_KEY | Secret key used for AES-GCM encryption       | Yes      |

## Security Notes

- Passwords are encrypted with AES-GCM; nonce is prepended and base64-encoded.
- .env is not committed. Keep your master key secure.
- Local-first - data stays on your machine.

## License

GPL v3.0
