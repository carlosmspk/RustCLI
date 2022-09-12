pub fn string_from_event(event: &crossterm::event::Event) -> String {
    match event {
        crossterm::event::Event::FocusGained => String::from("Focus Gained"),
        crossterm::event::Event::FocusLost => String::from("Focus Gained"),
        crossterm::event::Event::Key(key) => match key.code {
            crossterm::event::KeyCode::Backspace => String::from("Backspace"),
            crossterm::event::KeyCode::Enter => String::from("Enter"),
            crossterm::event::KeyCode::Left => String::from("Left"),
            crossterm::event::KeyCode::Right => String::from("Right"),
            crossterm::event::KeyCode::Up => String::from("Up"),
            crossterm::event::KeyCode::Down => String::from("Down"),
            crossterm::event::KeyCode::Home => String::from("Home"),
            crossterm::event::KeyCode::End => String::from("End"),
            crossterm::event::KeyCode::PageUp => String::from("PageUp"),
            crossterm::event::KeyCode::PageDown => String::from("PageDown"),
            crossterm::event::KeyCode::Tab => String::from("Tab"),
            crossterm::event::KeyCode::BackTab => String::from("BackTab"),
            crossterm::event::KeyCode::Delete => String::from("Delete"),
            crossterm::event::KeyCode::Insert => String::from("Insert"),
            crossterm::event::KeyCode::F(f_key) => format!("F{}", f_key),
            crossterm::event::KeyCode::Char(char_key) => if char_key == ' ' {String::from("Spacebar")} else { format!("{}", char_key) },
            crossterm::event::KeyCode::Null => String::from("Null"),
            crossterm::event::KeyCode::Esc => String::from("Esc"),
            crossterm::event::KeyCode::CapsLock => String::from("CapsLock"),
            crossterm::event::KeyCode::ScrollLock => String::from("ScrollLock"),
            crossterm::event::KeyCode::NumLock => String::from("NumLock"),
            crossterm::event::KeyCode::PrintScreen => String::from("PrintScreen"),
            crossterm::event::KeyCode::Pause => String::from("Pause"),
            crossterm::event::KeyCode::Menu => String::from("Menu"),
            crossterm::event::KeyCode::KeypadBegin => String::from("KeypadBegin"),
            crossterm::event::KeyCode::Media(_) => String::from("Media"),
            crossterm::event::KeyCode::Modifier(_) => String::from("Modifier"),
        },
        crossterm::event::Event::Mouse(mouse_event) => {
            let (x, y) = (mouse_event.row, mouse_event.column);
            match mouse_event.kind {
                crossterm::event::MouseEventKind::Down(button_down) => match button_down {
                    crossterm::event::MouseButton::Left => {
                        format!("Pressed Left Button Down at ({}, {})", x, y)
                    }
                    crossterm::event::MouseButton::Right => {
                        format!("Pressed Right Button Down at ({}, {})", x, y)
                    }
                    crossterm::event::MouseButton::Middle => {
                        format!("Pressed Middle Button Down at ({}, {})", x, y)
                    }
                },
                crossterm::event::MouseEventKind::Up(button_up) => match button_up {
                    crossterm::event::MouseButton::Left => {
                        format!("Pressed Left Button Up at ({}, {})", x, y)
                    }
                    crossterm::event::MouseButton::Right => {
                        format!("Pressed Right Button Up at ({}, {})", x, y)
                    }
                    crossterm::event::MouseButton::Middle => {
                        format!("Pressed Middle Button Up at ({}, {})", x, y)
                    }
                },
                crossterm::event::MouseEventKind::Drag(button_dragged) => match button_dragged {
                    crossterm::event::MouseButton::Left => {
                        format!("Dragged Left Button ({}, {})", x, y)
                    }
                    crossterm::event::MouseButton::Right => {
                        format!("Dragged Right Button at ({}, {})", x, y)
                    }
                    crossterm::event::MouseButton::Middle => {
                        format!("Dragged Middle Button at ({}, {})", x, y)
                    }
                },
                crossterm::event::MouseEventKind::Moved => format!("Moved Mouse at ({}, {})", x, y),
                crossterm::event::MouseEventKind::ScrollDown => {
                    format!("Scrolled at ({}, {})", x, y)
                }
                crossterm::event::MouseEventKind::ScrollUp => {
                    format!("Scrolled Up at ({}, {})", x, y)
                }
            }
        }
        crossterm::event::Event::Paste(pasted_string) => {
            format!("Paste:\n\t{}", pasted_string)
        }
        crossterm::event::Event::Resize(x, y) => format!("({}, {})", x, y),
    }
}
