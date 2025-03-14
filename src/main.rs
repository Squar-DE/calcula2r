use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button, Grid, Label};
use std::cell::RefCell;
use std::rc::Rc;
use evalexpr::{eval_with_context, ContextWithMutableFunctions, HashMapContext, Value, EvalexprError, Function, ValueType};
use regex::Regex;

fn main() {
    let app = Application::builder()
        .application_id("com.example.Calculator")
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

        let label = Label::new(Some("0"));
        label.add_css_class("display");
        label.set_xalign(1.0);
        label.set_hexpand(true);

        let css_provider = gtk4::CssProvider::new();
        css_provider.load_from_data("
            .display {
                background-color: #1a1a1a;
                color: #ffffff;
                border-radius: 8px;
                padding: 20px;
                font-size: 28px;
                font-weight: bold;
                margin-bottom: 20px;
            }
            
            number {
                background-color: #333333;
                color: white;
                border-radius: 8px;
                font-size: 20px;
                transition: all 0.2s;
            }
            button {
                font-size: 25px;
            }

            button:hover {
                filter: brightness(85%);
            }
            
            .operator {
                background-color: #ff9500;
            }
            
            .equals {
                background-color: #007AFF;
            }
            
            .clear {
                background-color: #ff3b30;
            }
            
            .paren {
                background-color: #76949f;
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
            ("C", 4, 0), ("0", 4, 1), (".", 4, 2), ("+", 4, 3), 
            ("%", 5, 0), ("√", 5, 1), ("^", 5, 2), ("=", 5, 3),
            ("(", 6, 1), (")", 6, 2),
        ];

        for (label_text, row, col) in buttons {
            let button = Button::with_label(label_text);
            button.set_size_request(75, 75);
            
            match label_text {
                "+" | "-" | "×" | "÷" | "^" | "%" | "√" => button.add_css_class("operator"),
                "=" => button.add_css_class("equals"),
                "C" => button.add_css_class("clear"),
                "(" | ")" => button.add_css_class("paren"),
                _ => {},
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
                    let should_replace = exp.is_empty() || exp.as_str() == "0" || exp.as_str() == "Error";

                    if should_replace {
                        *exp = match label_text {
                            "√" => "sqrt(".to_string(),
                            "%" => "0.01".to_string(),
                            "." => "0.".to_string(),
                            "(" | ")" => label_text.to_string(),
                            _ => label_text.to_string(),
                        };
                    } else {
                        match label_text {
                            "÷" => exp.push('/'),
                            "×" => exp.push('*'),
                            "^" => exp.push('^'),
                            "%" => exp.push('%'),
                            "√" => {
                                if exp.ends_with(|c: char| c.is_numeric() || c == ')') {
                                    exp.push_str("*sqrt(");
                                } else {
                                    exp.push_str("sqrt(");
                                }
                            },
                            "." => {
                                let last_number = exp.split(|c: char| !c.is_numeric() && c != '.')
                                    .last()
                                    .unwrap_or("");
                                if !last_number.contains('.') {
                                    exp.push('.');
                                }
                            },
                            "(" => {
                                if exp.ends_with(|c: char| c.is_numeric() || c == ')') {
                                    exp.push_str("*(");
                                } else {
                                    exp.push('(');
                                }
                            },
                            ")" => {
                                let open = exp.matches('(').count();
                                let close = exp.matches(')').count();
                                if open > close {
                                    exp.push(')');
                                }
                            },
                            _ => exp.push_str(label_text),
                        }
                    }
                }
                
                let display_text = exp
                    .replace("*", "×")  
                    .replace("/", "÷")
                    .replace("sqrt(", "√(")
                    .replace("×(", "(");

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

    // Handle the Result from set_function
    context.set_function(
        "sqrt".into(),
        Function::new(|arg: &Value| match arg {
            Value::Float(f) => Ok(Value::Float(f.sqrt())),
            Value::Int(i) => Ok(Value::Float((*i as f64).sqrt())),
            other => Err(EvalexprError::type_error(
                other.clone(),
                vec![ValueType::Float, ValueType::Int]
            )),
        })
    ).unwrap(); // Add unwrap here to handle the Result

    let processed_expr = expr
        .replace("%", "*0.01")
        .replace("^", "**"); // Convert power operator

    // Improved division handling using float conversion
    let re = Regex::new(r"(\d+)\s*/\s*(-?\d+)").unwrap();
    let processed_expr = re.replace_all(&processed_expr, |caps: &regex::Captures| {
        format!("({}.0)/{}", &caps[1], &caps[2]) // Convert to float division
    }).to_string();

    // Balance parentheses
    let open_count = processed_expr.matches('(').count();
    let close_count = processed_expr.matches(')').count();
    let balanced_expr = if open_count > close_count {
        format!("{}{}", processed_expr, ")".repeat(open_count - close_count))
    } else {
        processed_expr
    };

    match eval_with_context(&balanced_expr, &context) {
        Ok(value) => {
            let string_value = value.to_string();
            if let Ok(float_value) = string_value.parse::<f64>() {
                if float_value.fract() == 0.0 {
                    format!("{:.0}", float_value)
                } else {
                    let formatted = format!("{:.8}", float_value);
                    formatted
                        .trim_end_matches('0')
                        .trim_end_matches('.')
                        .to_string()
                }
            } else {
                string_value
            }
        }
        Err(e) => {
            eprintln!("Evaluation error: {:?}", e);
            "Error".to_string()
        }
    }
}
