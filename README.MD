# Kifi - Command Line Interface (CLI) Program

**Kifi** is a Rust-based command-line interface (CLI) program that provides a simple way to manage and track changes in your projects and repositories. It offers four main commands to streamline your version control workflow.

## Installation

Before using Kifi, you need to build and install it. Here are the steps to get started:

### Prerequisites

Before building Kifi, make sure you have Rust installed on your system. You can install Rust by following the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

### Build Kifi

1. Clone the Kifi repository:

   ```shell
   git clone https://github.com/Vaurkhorov/kifi
   ```

2. Change into the Kifi directory:

   ```shell
   cd kifi
   ```

3. Build Kifi using Cargo (Rust's package manager):

   ```shell
   cargo build --release
   ```

   This command will compile Kifi and create a release version in the "target/release" directory.

4. To verify that the build was successful, run the following command:

   ```shell
   ./target/release/kifi --version
   ```

   You should see the version information for Kifi.

### Adding Kifi to PATH

To use Kifi conveniently, you should add it to your system's PATH. The following instructions show how to do this:

#### Windows

1. Search for "Environment Variables" in the Start menu and select "Edit the system environment variables."

2. In the "System Properties" window, click the "Environment Variables" button.

3. Under "User variables" or "System variables," find the "Path" variable and click "Edit."

4. Click "New" and add the path to the Kifi executable, typically located in the "target/release" directory.

5. Click "OK" to save the changes.

6. Close any open command prompts and open a new one to use Kifi.

## Usage

Kifi provides the following commands:

### 1. `kifi init`

This command initializes a new repository in the current directory. It sets up a Kifi repository to start tracking files.

```shell
kifi init
```

### 2. `kifi track <file_name>`

Use this command to start tracking a specific file. Replace `file_name` with the name of the file you want to track.

```shell
kifi track file_name
```

### 3. `kifi preview`

Generate diffs between the current and already pushed versions of tracked files. This helps you review changes before committing.

```shell
kifi preview
```

### 4. `kifi klick`

This command takes a 'snapshot', making a commit to the repository.

```shell
kifi klick
```

## License

This software is distributed under the GNU AGPLv3 license. See the [LICENSE](LICENSE) file for more details.
