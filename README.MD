# kifi - Version Control

**kifi** is a Rust-based command-line interface (CLI) program that provides a simple way to manage and track changes in your projects and repositories. It offers four main commands to streamline your version control workflow.

## Installation

Before using kifi, you need to build and install it. Here are the steps to get started:

### Prerequisites

Before building kifi, make sure you have Rust installed on your system. You can install Rust by following the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

### Install kifi

1. Clone the kifi repository:

   ```shell
   git clone https://github.com/Vaurkhorov/kifi
   ```

2. Install kifi using Cargo (Rust's package manager):

   ```shell
   cargo install --path kifi
   ```

   This command will compile kifi and create a release version in the "target/release" directory.

3. To verify that the build was successful, run the following command:

   ```shell
   ./target/release/kifi --version
   ```

   You should see the version information for kifi.
   If you don't, verify whether Cargo's bin is present in the environment path variable.

## Usage

kifi provides the following commands:

### 1. `kifi init`

This command initializes a new repository in the current directory. It sets up a kifi repository to start tracking files.

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

Licensed under either of
- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.
