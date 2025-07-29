fn main() {
    slint_build::compile("ui/background.slint").expect("Slint build failed");
    slint_build::compile("ui/menu.slint").expect("Slint build failed");
}
