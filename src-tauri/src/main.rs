#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::sync::Mutex;
use std::sync::Arc;

fn generate_random_string(size: usize) -> String {
  let charset: &str = "abcdefghijklmnopqrstuvwxyz";
  return random_string::generate(size, charset);
}

fn random_string_arr(size: usize) -> Vec<String>{
  let mut string_arr = vec![];
  for _ in 0..1000 {
    string_arr.push(generate_random_string(size));
  }
  return string_arr;
}

fn main() {
  tauri::Builder::default()
  .on_page_load(move |app, _ev| {
    let app_wrapped = Arc::new(Mutex::new(app));
    match app_wrapped.lock() {
        Ok(app) => {
            let app_moved = app_wrapped.clone();
            app.listen("1kB", move |_request| {
              for message in random_string_arr(1000).iter() {
                app_moved.lock().unwrap().emit("1kB", message);
              }
            });
        },
        Err(err) => {
            panic!("{}", err);
        }
    };
    match app_wrapped.lock() {
      Ok(app) => {
          let app_moved = app_wrapped.clone();
          app.listen("10kB", move |_request| {
            for message in random_string_arr(10000).iter() {
              app_moved.lock().unwrap().emit("10kB", message);
            }
          });
      },
      Err(err) => {
          panic!("{}", err);
      }
    };
    match app_wrapped.lock() {
      Ok(app) => {
          let app_moved = app_wrapped.clone();
          app.listen("100kB", move |_request| {
            for message in random_string_arr(100000).iter() {
              app_moved.lock().unwrap().emit("100kB", message);
            }
          });
      },
      Err(err) => {
          panic!("{}", err);
      }
    };
    match app_wrapped.lock() {
      Ok(app) => {
          let app_moved = app_wrapped.clone();
          app.listen("1MB", move |_request| {
            for message in random_string_arr(1000000).iter() {
              app_moved.lock().unwrap().emit("1MB", message);
            }
          });
      },
      Err(err) => {
          panic!("{}", err);
      }
    };
  })
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}
