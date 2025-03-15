# Bing Wallpaper Downloader

A Rust-based application to download Bing's daily wallpapers.

## Features

- Download the daily Bing wallpaper
- Save wallpapers to a specified directory
- Cross-platform support

## Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/bing_wallpaper_downloader.git
   ```
2. Navigate to the project directory:
   ```sh
   cd bing_wallpaper_downloader
   ```
3. Build the project:
   ```sh
   cargo build --release
   ```

## Usage

Run the application with default settings (saves to "bing_wallpapers" folder):

```sh
./target/release/bing_wallpaper_downloader
```

Specify a custom directory to save wallpapers:

```sh
./target/release/bing_wallpaper_downloader --dir /path/to/directory
```

Or use the short option:

```sh
./target/release/bing_wallpaper_downloader -d /path/to/directory
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- Inspired by Bing's beautiful daily wallpapers.
