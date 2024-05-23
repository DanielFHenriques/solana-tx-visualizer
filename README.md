# Solana Transaction Visualizer

This is a CLI which listens for USDC transactions in the Solana blockhain.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Setup and Installation](#setup-and-installation)
- [Running Tests](#running-tests)

## Prerequisites

Before you begin, ensure you have met the following requirements:
- **Docker**: The project uses DevContainers, which require Docker to run.
- **Visual Studio Code**: Recommended for using DevContainers with its excellent support via the Remote - Containers extension.
- **Remote - Containers Extension**: Install this VS Code extension to easily manage and use DevContainers.

## Setup and Installation

Follow these steps to get your development environment running:

1. **Clone the Repository**
   ```bash
   git clone https://github.com/DanielFHenriques/solana-tx-visualizer.git
   cd solana-tx-visualizer

2. **Open in Visual Studio Code**
    - Open VS Code and navigate to the cloned project folder.
    - VS Code might prompt you to reopen the project in a container. If so, click "Reopen in Container".
    - If not prompted, open the command palette (Ctrl+Shift+P or Cmd+Shift+P on Mac) and select "Remote-Containers: Open Folder in Container".

3. **Install Dependencies**
    - The container should automatically install all required dependencies defined in the Dockerfile or the devcontainer.json.

    - To manually build and start the development container, use:
    ```
    Remote-Containers: Rebuild and Reopen in Container

4. **Build the Project**
    - Within the DevContainer, run the following command to compile the project:
    ```bash
    cargo build

## Running Tests
To run tests, execute the following command in the terminal inside your DevContainer:
```bash
cargo test
```
