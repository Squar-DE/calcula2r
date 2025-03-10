use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, Grid, Label};
use std::cell::RefCell;
use std::rc::Rc;
use evalexpr::eval;

fn main() {
    let app = Application::builder()
        .application_id("com.example.Calculator")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Calculator")
            .resizable(false) // Prevent resizing
            .build();

        let grid = Grid::new();
        grid.set_column_spacing(15); // Add more spacing between columns
        grid.set_row_spacing(15); // Add more spacing between rows
        grid.set_margin_top(20);
        grid.set_margin_bottom(20);
        grid.set_margin_start(20);
        grid.set_margin_end(20);
        
        let label = Label::new(Some("0"));
        label.set_margin_bottom(15); // Add gap between label and buttons
        label.set_xalign(1.0);
        label.set_hexpand(true);
        label.set_markup("<span font='24'><b>0</b></span>"); // Make label larger and bold
        
        let expression = Rc::new(RefCell::new(String::new()));
        grid.attach(&label, 0, 0, 4, 1);

        let buttons = vec![
            ("7", 1, 0), ("8", 1, 1), ("9", 1, 2), ("/", 1, 3),
            ("4", 2, 0), ("5", 2, 1), ("6", 2, 2), ("*", 2, 3),
            ("1", 3, 0), ("2", 3, 1), ("3", 3, 2), ("-", 3, 3),
            ("0", 4, 1), ("=", 4, 0), ("C", 4, 2), ("+", 4, 3),
        ];

        for (label_text, row, col) in buttons {
            let button = Button::with_label(label_text);
            button.set_size_request(60, 60); // Make buttons larger
            let label_clone = label.clone();
            let expression_clone = Rc::clone(&expression);

            button.connect_clicked(move |_| {
                let mut exp = expression_clone.borrow_mut();
                if label_text == "=" {
                    if exp.as_str() == "" {
                        return;
                    }
                    let result = eval_expression(&exp);
                    *exp = result;
                } else if label_text == "C" {
                    *exp = String::new();
                } else {
                    if exp.as_str() == "0" {
                        *exp = label_text.to_string();
                    } else {
                        exp.push_str(label_text);
                    }
                }
                label_clone.set_markup(&format!("<span font='24'><b>{}</b></span>", if exp.is_empty() { "0" } else { &exp }));
            });

            grid.attach(&button, col, row, 1, 1);
        }

        window.set_child(Some(&grid));
        window.show();
    });

    app.run();
}

fn eval_expression(expr: &str) -> String {
    eval(expr).map(|res| res.to_string()).unwrap_or_else(|_| "Error".to_string())
}
