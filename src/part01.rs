pub fn run() {
    println!(
        "
        Use this command to install rust on linux:
            $ curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh

        Check rust version
            $ rustc --version

        Start a new project
            $ cargo new hello-rust

        Compile and run rust program
            $ cargo run
        "
    );

    println!("Hello, World!");
}
