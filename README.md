# Bundler API

LÖVEBrew Bundler API simplifies creating homebrew with LÖVEPotion. It provides REST endpoints for converting assets and building homebrew executable files for the Nintendo 3DS, Switch and Wii U. These are utilized by the [bundler frontend](https://github.com/lovebrew/bundler) to create an easy-to-use homebrew development tool.

## Deployment

### Prerequisites

- [Rust](https://www.rust-lang.org/)
- [devkitPro pacman](https://devkitpro.org/wiki/devkitPro_pacman) and the following packages:
  - `tex3ds` for 3DS asset conversion
  - `3dstools` for building 3DSX binaries
  - `switch-tools` for building NRO binaries
  - `wut-tools` for building WUHB binaries

### Installation

1. Clone the repository:

   ```bash
   git clone <repository-url>
   cd bundler-api
   ```

2. Run the server:
   ```bash
   cargo run
   ```

## Contributing

Contributions are welcome! Please submit a pull request or file an issue if you have suggestions or bug reports.
