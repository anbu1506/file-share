# File Share

## Overview

FileShare is a multithreaded and asynchronous file-sharing application written in Rust that utilizes pure TCP for seamless file transfers between computers within a local network. The application supports both Windows and Linux platforms, ensuring a consistent experience for users across different operating systems.

## Features

- **Cross-Platform Compatibility:** Works seamlessly on both Windows and Linux operating systems.
- **Fast and Efficient:** Utilizes Rust's performance advantages with a multithreaded and asynchronous approach for quick and efficient file transfers.
- **User-Friendly Interface:** Simple command-line interface for easy usage.
- **Local Network Sharing:** Share files securely within your local network, without the need for external servers, using pure TCP.

## Quick Start

1. **Download the Binary:**
   - [Download the latest release](https://github.com/anbu1506/file-share/releases/latest)

2. **Run the Application:**
   ```bash
   # To send a file . note: the files can be choosen using the file dialog that appears when you run this application
   ./share -s 

   # To receive a file, use the  port (8080) or any other port number
   ./share -r (port)
