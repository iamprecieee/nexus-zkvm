# The Nexus zkVM

<div align="left">
    <a href="https://t.me/nexus_zkvm">
        <img src="https://img.shields.io/endpoint?color=neon&logo=telegram&label=chat&url=https%3A%2F%2Fmogyo.ro%2Fquart-apis%2Ftgmembercount%3Fchat_id%3Dnexus_zkvm"/></a>
    <a href="https://github.com/nexus-xyz/nexus-zkvm/graphs/contributors">
        <img src="https://img.shields.io/github/contributors/nexus-xyz/nexus-zkvm.svg"></a>
    <a href="https://twitter.com/NexusLabsHQ">
        <img src="https://img.shields.io/badge/Twitter-black?logo=x&logoColor=white"/></a>
    <a href="https://nexus.xyz">
        <img src="https://img.shields.io/static/v1?label=Stage&message=Alpha&color=2BB4AB"/></a>
    <a href="https://github.com/nexus-xyz/nexus-zkvm/blob/main/LICENSE-MIT">
        <img src="https://img.shields.io/badge/license-MIT-blue"/></a>
    <a href="https://github.com/nexus-xyz/nexus-zkvm/blob/main/LICENSE-APACHE">
        <img src="https://img.shields.io/badge/license-APACHE-blue"/></a>
</div>

<p align="center">
  <p align="center">
   <img width="100%" src="assets/nexus_docs-header.png" alt="Logo">
  </p>
</p>

The Nexus zkVM is a modular, extensible, open-source, and highly-parallelized zkVM, designed to run at *a trillion CPU cycles proved per second* given enough machine power.

## Folding schemes

If you're interested in our implementation of folding schemes, check the [`nexus-nova`](./nova/) crate.

## Quick Start (This has been tailored for windows OS)

### Install CMake

- First, download and install the Cmake Windows x64 Installer from [CMake](https://github.com/Kitware/CMake/releases/download/v3.30.0-rc3/cmake-3.30.0-rc3-windows-x86_64.msi)

### Install Build Tools for Visual Studio

- Download and install the Microsoft C++ Build Tools Installer from [Visual Studio Installer](https://aka.ms/vs/17/release/vs_BuildTools.exe)
- Next, run the Visual Studio Installer app.
- In the list of available workloads on the installer's window, select and install ***Visual Studio Build Tools 2022***`.
- When the installation's complete, you will see the ***VS Build Tools 2022*** in the *"**Installed**"* section. Click on *"**Modify**"* and select ***Desktop development with C++***. At the bottom of the window, select *"**Download all, then install**"* and click on *"**Close**"*. Wait for that installation to complete. 

### Install Rust

- Download and install Rust from [Rust](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)

### Add necessary variables to your PATH

- Open VSCode (or any code editor of your choice) and create a new folder (e.g. nexus-project).
- Select `nexus-project` as your current directory:
    ```shell
        cd nexus-project
    ```

- Run each code line below to set your env variables in PATH (Please **verify** the following paths to `Cmake\bin`, `.cargo\bin`, and `link.exe` on your PC):
    ```shell
        $env:Path += ";C:\Program Files\CMake\bin"

        $env:Path += ";C:\Users\iamprecieee\.cargo\bin"

        $env:Path += ";C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\14.40.33807\bin\Hostx64\x64\link.exe"
    ```
    #### **NB: REPLACE *"iamprecieee"* WITH YOUR PC USERNAME. ALSO, REPLACE *"14.40.33807"* WITH YOUR MSVC VERSION.**

- Verify Rust installation using:

    ```shell
    rustc --version
    ```

### Install the Nexus zkVM

- With the RISC-V target:

    ```shell
    rustup target add riscv32i-unknown-none-elf
    ```

- Then, install the Nexus zkVM:

    ```shell
    cargo install --git https://github.com/nexus-xyz/nexus-zkvm nexus-tools --tag 'v1.0.0'
    ```

- Verify the installation:

    ```shell
    cargo nexus --help
    ```

- This should print the available CLI commands.

### Create a new Nexus project

    ```shell
    cargo nexus new nexus-project
    ```

- Navigate to project directory:

    ```shell
    cd nexus-project
    cd src
    ```

### Edit the Main File

- As an example, you can change the content of `./src/main.rs` to:

    ```rust
    #![no_std]
    #![no_main]

    fn fib(n: u32) -> u32 {
        match n {
            0 => 0,
            1 => 1,
            _ => fib(n - 1) + fib(n - 2),
        }
    }

    #[nexus_rt::main]
    fn main() {
        let n = 7;
        let result = fib(n);
        assert_eq!(result, 13);
    }
    ```

### Run your program

    ```bash
    cargo nexus run
    ```

- This command should run successfully. To print the full step-by-step execution trace on the NVM, run:

    ```bash
    cargo nexus run -v
    ```

### Prove your program

- Generate a proof for your Rust program using the Nexus zkVM.

    ```shell
    cargo nexus prove
    ```

- This command will save the proof to `./nexus-proof`.

### Verify your proof

- Finally, load and verify the proof:

    ```shell
    cargo nexus verify
    ```

## Learn More

Run `cargo nexus --help` to see all the available commands.

Also check out the documentation at [docs.nexus.xyz](https://docs.nexus.xyz), or join our [Telegram](https://t.me/nexus_zkvm) chat to discuss!

Nexus is committed to open-source. All of our code is dual licensed under MIT and Apache licenses. We encourage and appreciate contributions.
