#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use arboard::Clipboard;
use rdev::{listen, simulate, Event, EventType, Key, SimulateError};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{env, thread, time};
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenuItem};

#[tauri::command]
fn set_var(key: String, val: String) -> () {
    env::set_var(key, val);
}

fn convert(text: String) -> String {
    let forward_map: HashMap<char, char> = HashMap::from([
        ('@', '"'),
        ('#', '№'),
        ('$', ';'),
        ('^', ':'),
        ('&', '?'),
        ('Q', 'Й'),
        ('W', 'Ц'),
        ('E', 'У'),
        ('R', 'К'),
        ('T', 'Е'),
        ('Y', 'Н'),
        ('U', 'Г'),
        ('I', 'Ш'),
        ('O', 'Щ'),
        ('P', 'З'),
        ('{', 'Х'),
        ('}', 'Ъ'),
        ('|', '/'),
        ('A', 'Ф'),
        ('S', 'Ы'),
        ('D', 'В'),
        ('F', 'А'),
        ('G', 'П'),
        ('H', 'Р'),
        ('J', 'О'),
        ('K', 'Л'),
        ('L', 'Д'),
        (':', 'Ж'),
        ('"', 'Э'),
        ('Z', 'Я'),
        ('X', 'Ч'),
        ('C', 'С'),
        ('V', 'М'),
        ('B', 'И'),
        ('N', 'Т'),
        ('M', 'Ь'),
        ('<', 'Б'),
        ('>', 'Ю'),
        ('?', ','),
        ('~', 'Ё'),
        ('q', 'й'),
        ('w', 'ц'),
        ('e', 'у'),
        ('r', 'к'),
        ('t', 'е'),
        ('y', 'н'),
        ('u', 'г'),
        ('i', 'ш'),
        ('o', 'щ'),
        ('p', 'з'),
        ('[', 'х'),
        (']', 'ъ'),
        ('a', 'ф'),
        ('s', 'ы'),
        ('d', 'в'),
        ('f', 'а'),
        ('g', 'п'),
        ('h', 'р'),
        ('j', 'о'),
        ('k', 'л'),
        ('l', 'д'),
        (';', 'ж'),
        ('\'', 'э'),
        ('z', 'я'),
        ('x', 'ч'),
        ('c', 'с'),
        ('v', 'м'),
        ('b', 'и'),
        ('n', 'т'),
        (',', 'б'),
        ('.', 'ю'),
        ('/', '.'),
        ('`', 'ё'),
    ]);

    let reverse_map: HashMap<char, char> = forward_map.iter().map(|(&k, &v)| (v, k)).collect();

    let (russian_count, english_count) = text.chars().fold((0, 0), |(r, e), c| {
        if forward_map.contains_key(&c) {
            (r + 1, e)
        } else if reverse_map.contains_key(&c) {
            (r, e + 1)
        } else {
            (r, e)
        }
    });

    let map_to_use = if russian_count > english_count {
        &forward_map
    } else {
        &reverse_map
    };

    text.chars()
        .map(|c| *map_to_use.get(&c).unwrap_or(&c))
        .collect()
}

fn main() {
    env::set_var("activation", "C");
    env::set_var("closetotray", "true");

    let state = Arc::new(Mutex::new(State {
        alt_pressed: false,
        meta_left_pressed: false,
        alpha_pressed: false,
    }));

    let state_clone = Arc::clone(&state);

    thread::spawn(move || {
        if let Err(error) = listen(move |event| {
            handle_event(event, state_clone.clone());
        }) {
            println!("rdev error: {:?}", error);
            #[cfg(target_os = "macos")]
            println!("На macOS необходимо разрешить доступ к специальным возможностям: Системные настройки → Конфиденциальность и безопасность → Универсальный доступ");
        }
    });

    let tray_menu = SystemTray::new()
        .with_menu(
            tauri::SystemTrayMenu::new()
                .add_item(CustomMenuItem::new("open", "Настройки"))
                .add_native_item(SystemTrayMenuItem::Separator)
                .add_item(CustomMenuItem::new("quit", "Выход")),
        )
        .with_tooltip("LangSwitcher");

    tauri::Builder::default()
        .system_tray(tray_menu)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                let w = app.get_window("main").unwrap();
                w.show().unwrap();
                w.set_focus().unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "open" => {
                    let w = app.get_window("main").unwrap();
                    w.show().unwrap();
                    w.set_focus().unwrap();
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => match env::var("closetotray") {
                Ok(val) => {
                    if val == "true" {
                        event.window().hide().unwrap();
                        api.prevent_close();
                    }
                }
                Err(err) => println!("Error: {}", err),
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![set_var])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

struct State {
    alt_pressed: bool,
    meta_left_pressed: bool,
    alpha_pressed: bool,
}

fn handle_event(event: Event, state: Arc<Mutex<State>>) {
    let mut state = state.lock().unwrap();
    match event.event_type {
        EventType::KeyPress(key) => {
            if key == Key::Alt {
                state.alt_pressed = true;
            } else if key == Key::MetaLeft {
                state.meta_left_pressed = true;
            } else {
                match env::var("activation") {
                    Ok(val) => {
                        if (val == "C" && key == Key::KeyC)
                            || (val == "S" && key == Key::KeyS)
                            || (val == "L" && key == Key::KeyL)
                        {
                            state.alpha_pressed = true;
                        }
                    }
                    Err(err) => println!("Error: {}", err),
                }
            }
            if state.alt_pressed && state.meta_left_pressed && state.alpha_pressed {
                thread::spawn(move || {
                    fn send(event_type: &EventType) {
                        let delay = time::Duration::from_millis(20);
                        match simulate(event_type) {
                            Ok(()) => (),
                            Err(SimulateError) => {
                                println!("Could not send {:?}", event_type);
                            }
                        }
                        thread::sleep(delay);
                    }

                    send(&EventType::KeyRelease(Key::Alt));
                    send(&EventType::KeyRelease(Key::MetaLeft));
                    match env::var("activation") {
                        Ok(val) => {
                            if val == "C" {
                                send(&EventType::KeyRelease(Key::KeyC));
                            } else if val == "S" {
                                send(&EventType::KeyRelease(Key::KeyS));
                            } else if val == "L" {
                                send(&EventType::KeyRelease(Key::KeyL));
                            }
                        }
                        Err(err) => println!("Error: {}", err),
                    }

                    // На macOS копирование — Cmd+C, на Windows/Linux — Ctrl+C
                    #[cfg(target_os = "macos")]
                    let copy_mod = Key::MetaLeft;
                    #[cfg(not(target_os = "macos"))]
                    let copy_mod = Key::ControlLeft;

                    let mut clipboard = match Clipboard::new() {
                        Ok(c) => c,
                        Err(e) => {
                            println!("Clipboard init error: {:?}", e);
                            return;
                        }
                    };

                    let old_val = clipboard.get_text().unwrap_or_default();

                    send(&EventType::KeyPress(copy_mod));
                    send(&EventType::KeyPress(Key::KeyC));
                    send(&EventType::KeyRelease(copy_mod));
                    send(&EventType::KeyRelease(Key::KeyC));

                    // Ждём, пока буфер обмена обновится после копирования
                    thread::sleep(time::Duration::from_millis(100));

                    let new_val = clipboard.get_text().unwrap_or_default();
                    if let Err(e) = clipboard.set_text(convert(new_val)) {
                        println!("Clipboard set error: {:?}", e);
                        return;
                    }

                    send(&EventType::KeyPress(copy_mod));
                    send(&EventType::KeyPress(Key::KeyV));
                    send(&EventType::KeyRelease(copy_mod));
                    send(&EventType::KeyRelease(Key::KeyV));

                    if let Err(e) = clipboard.set_text(old_val) {
                        println!("Clipboard restore error: {:?}", e);
                    }
                });
            }
        }
        EventType::KeyRelease(key) => {
            if key == Key::Alt {
                state.alt_pressed = false;
            } else if key == Key::MetaLeft {
                state.meta_left_pressed = false;
            } else {
                match env::var("activation") {
                    Ok(val) => {
                        if (val == "C" && key == Key::KeyC)
                            || (val == "S" && key == Key::KeyS)
                            || (val == "L" && key == Key::KeyL)
                        {
                            state.alpha_pressed = false;
                        }
                    }
                    Err(err) => println!("Error: {}", err),
                }
            }
        }
        _ => {}
    }
}
