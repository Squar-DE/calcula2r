use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, Grid, Label};
use std::cell::RefCell;
use std::rc::Rc;
use evalexpr::{eval_with_context, ContextWithMutableFunctions, HashMapContext, Value, EvalexprError, Function, ValueType};

fn main() {
    let app = Application::builder()
        .application_id("com.SquarDE.Calcula2r")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Calculator")
            .resizable(false)
            .build();

        let grid = Grid::new();
        grid.set_column_spacing(10);
        grid.set_row_spacing(10);
        grid.set_margin_top(20);
        grid.set_margin_bottom(20);
        grid.set_margin_start(20);
        grid.set_margin_end(20);
        
        // Stylish display label
        let label = Label::new(Some("0"));
        label.add_css_class("display");
        label.set_xalign(1.0);
        label.set_hexpand(true);
        
        // Enhanced CSS styling
        let css_provider = gtk4::CssProvider::new();
        css_provider.load_from_data("
            .display {
                background-color: #2d2d2d;
                color: #ffffff;
                border-radius: 8px;
                padding: 20px;
                font-size: 24px;
                font-weight: bold;
                margin-bottom: 20px;
            }
            
            button {
                background-color: #4a4a4a;
                color: white;
                border-radius: 8px;
                font-size: 18px;
                transition: all 0.2s;
            }
            
            button:hover {
                background-color: #5a5a5a;
            }
            
            .operator {
                background-color: #ff9500;
            }
            
            .operator:hover {
                background-color: #ffaa33;
            }
            
            .equals {
                background-color: #007AFF;
            }
            
            .equals:hover {
                background-color: #0a84ff;
            }
            
            .clear {
                background-color: #ff3b30;
            }
            
            .clear:hover {
                background-color: #ff453a;
            }
        ");

        let display = gtk4::gdk::Display::default().expect("Could not get default display");
        gtk4::style_context_add_provider_for_display(
            &display,
            &css_provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let expression = Rc::new(RefCell::new(String::new()));
        grid.attach(&label, 0, 0, 4, 1);

        let buttons = vec![
            ("7", 1, 0), ("8", 1, 1), ("9", 1, 2), ("÷", 1, 3),
            ("4", 2, 0), ("5", 2, 1), ("6", 2, 2), ("×", 2, 3),
            ("1", 3, 0), ("2", 3, 1), ("3", 3, 2), ("-", 3, 3),
            ("0", 4, 0), (".", 4, 1), ("C", 4, 2), ("+", 4, 3),
            ("=", 5, 0), ("√", 5, 1), ("^", 5, 2), ("%", 5, 3),
        ];

        for (label_text, row, col) in buttons {
            let button = Button::with_label(label_text);
            button.set_size_request(75, 75);
            
            // Add style classes based on button type
            match label_text {
                "+" | "-" | "×" | "÷" | "^" | "%" | "√" => button.add_css_class("operator"),
                "=" => button.add_css_class("equals"),
                "C" => button.add_css_class("clear"),
                _ => {}
            }

            let label_clone = label.clone();
            let expression_clone = Rc::clone(&expression);

            button.connect_clicked(move |_| {
                let mut exp = expression_clone.borrow_mut();
                if label_text == "=" {
                    let result = eval_expression(&exp);
                    *exp = result;
                } else if label_text == "C" {
                    *exp = String::new();
                } else {
                    // Modified condition to handle empty state and "0"
                    if exp.is_empty() || exp.as_str() == "0" || exp.as_str() == "Error" {
                        *exp = match label_text {
                            "√" => "sqrt(".to_string(),
                            "%" => "0.01".to_string(),
                            _ => label_text.to_string(),
                        };
                    } else {
                        match label_text {
                            "÷" => exp.push('/'),
                            "×" => exp.push('*'),
                            "√" => {
                                if exp.ends_with(|c: char| c.is_numeric() || c == ')') {
                                    exp.push_str("*sqrt(");
                                } else {
                                    exp.push_str("sqrt(");
                                }
                            },
                            "%" => {
                                if let Some(last_char) = exp.chars().last() {
                                    if last_char.is_numeric() {
                                        exp.push_str("*0.01");
                                    } else {
                                        exp.push_str("0.01");
                                    }
                                }
                            },
                            "^" => exp.push('^'),
                            _ => exp.push_str(label_text),
                        }
                    }
                }
                
                // Update display without trimming trailing zeros
                let display_text = if exp.is_empty() { "0" } else { &exp };
                label_clone.set_markup(&format!("<span font='24'><b>{}</b></span>", display_text));
            });

            grid.attach(&button, col, row, 1, 1);
        }

        window.set_child(Some(&grid));
        window.present();
    });

    app.run();
}

fn eval_expression(expr: &str) -> String {
    let mut context = HashMapContext::new();
    
    // Corrected sqrt function implementation
    context.set_function(
        "sqrt".into(),
        Function::new(|arg: &Value| {
            match arg {
                Value::Float(f) => Ok(Value::Float(f.sqrt())),
                Value::Int(i) => Ok(Value::Float((*i as f64).sqrt())),
                other => Err(EvalexprError::type_error(
                    other.clone(),
                    vec![ValueType::Float, ValueType::Int]
                )),
            }
        })
    );

    let mut formatted_expr = expr.to_string();
    // Add missing closing parentheses
    let open_count = formatted_expr.matches('(').count();
    let close_count = formatted_expr.matches(')').count();
    formatted_expr.push_str(&")".repeat(open_count - close_count));

    eval_with_context(&formatted_expr, &context)
        .map(|v| {
            let s = format!("{:.8}", v);
            s.trim_end_matches('0')
                .trim_end_matches('.')
                .to_string()
        })
        .unwrap_or_else(|_| "Error".into())
}
