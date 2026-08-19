use wesl::Wesl;

fn main() {
    Wesl::new("src/shaders").build_artifact(&"package::main".parse().unwrap(), "shader");
}
