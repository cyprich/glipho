fn main() {
    let path = "ui/app.slint";
    let result = slint_build::compile(path);

    if let Err(e) = result {
        panic!("Failed to compile {}\n{}", path, e)
    }
}
