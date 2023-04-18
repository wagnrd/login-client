use slint::slint;

slint! {
    import { VerticalBox, HorizontalBox } from "std-widgets.slint";

    export component MainWindow inherits Window {
        title: "Test App";
        width: 200px;
        height: 200px;

        Text {
            text: "Test App";
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    MainWindow::new()
        .unwrap()
        .run()
}
