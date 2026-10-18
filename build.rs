mod test;

fn main() {
    let config = slint_build::CompilerConfiguration::new()
        .with_style("fluent-dark".into());
    slint_build::compile_with_config("src/gui/main.slint", config).unwrap();

    #[cfg(windows)]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("src/gui/image/icon.ico");
        res.compile().unwrap();
    }


//     slint_build::compile("src/gui/main.slint").unwrap();
}

