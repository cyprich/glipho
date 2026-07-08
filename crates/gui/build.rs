fn main() {
    #[rustfmt::skip]
    let build_files = [
        "ui/app.slint",
        "ui/counter.slint"
    ];

    for f in build_files {
        if let Err(e) = slint_build::compile(f) {
            panic!("Failed to compile {f}\n{e}")
        }
    }
}
