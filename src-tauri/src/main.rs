// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted by Tauri", name)
}
#[tauri::command]
fn n_count(seq: &str) -> String {
    let newseq = seq.to_uppercase();
    let count_a = newseq.matches('A').count();
    let count_t = newseq.matches('T').count();
    let count_g = newseq.matches('G').count();
    let count_c = newseq.matches('C').count();
    format!("Count of A is {}! Count of T is {}! Count of G is {}! Count of C is {}!", count_a, count_t, count_g, count_c)
}

#[tauri::command]
fn complementary(seq: &str) -> String {
    let mut complementary_sequence = String::new();

    for base in seq.chars() {
        let complementary_base = match base {
            'A' => 'T',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            'U' => 'T', 
            _ => base,
        };
        complementary_sequence.push(complementary_base);
    }
    
    complementary_sequence 
}

#[tauri::command]
fn gc(seq: &str) -> String {
    let newseq = seq.to_uppercase();
    let count_a = newseq.matches('A').count();
    let count_t = newseq.matches('T').count();
    let count_g = newseq.matches('G').count();
    let count_c = newseq.matches('C').count();
    let total=count_a+count_c+count_g+count_t;
    let gc= ((count_c+count_g)/total)*100;
    format!("GC content in the sequnce is {}! %",gc)
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,n_count,complementary,gc])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
