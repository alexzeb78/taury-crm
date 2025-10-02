// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;
mod commands;
mod db;
mod sidecar;

use commands::{
    auth_commands::*, company_commands::*, customer_commands::*, document_commands::*, 
    proposal_commands::*, invoice_commands::*, document_generator::generate_proposal_word, sync_commands::*,
};
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            println!("🚀 Starting Taury CRM application...");
            
            let app_dir = app.path_resolver()
                .app_data_dir()
                .expect("Failed to get app data directory");

            println!("📁 App data directory: {}", app_dir.display());

            // Create app directory if it doesn't exist
            if !app_dir.exists() {
                println!("📁 Creating app directory...");
                std::fs::create_dir_all(&app_dir)
                    .expect("Failed to create app data directory");
                println!("✅ App directory created");
            } else {
                println!("✅ App directory already exists");
            }

            // Initialize database
            tauri::async_runtime::block_on(async move {
                println!("🔧 Initializing database...");
                let pool = db::init_database(app_dir)
                    .await
                    .map_err(|e| {
                        eprintln!("❌ Database initialization failed: {}", e);
                        e
                    })
                    .expect("Failed to initialize database");

                app.manage(pool);
                println!("✅ Database pool managed");
                
                // Lancer le sidecar Python pour génération de documents
                println!("🐍 Starting document generator sidecar...");
                let sidecar_manager = sidecar::SidecarManager::new();
                if let Err(e) = sidecar_manager.start().await {
                    eprintln!("⚠️ Failed to start document generator: {}", e);
                    eprintln!("   Word generation will not be available.");
                } else {
                    println!("✅ Document generator started successfully");
                }
                app.manage(sidecar_manager);
            });

            println!("✅ Application setup completed");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            // Auth commands
            register,
            login,
            logout,
            // Company commands
            get_companies,
            get_company,
            create_company,
            update_company,
            delete_company,
            // Proposal commands
            get_proposals,
            get_proposal,
            create_proposal,
            update_proposal,
            delete_proposal,
            calculate_product_price,
            test_get_proposals,
            delete_proposal_product,
            // Invoice commands
            create_invoice_from_proposal,
            get_all_invoices,
            get_invoice_by_id,
            get_invoice_by_proposal_id,
            update_invoice,
            update_invoice_status,
            delete_invoice,
            get_invoices_by_status,
            get_invoices_by_company,
            generate_invoice_excel,
            // Customer commands
            get_customers,
            get_customer,
            create_customer,
            update_customer,
            delete_customer,
            // Document commands
            get_documents,
            get_customer_documents,
            create_document,
            delete_document,
            // Document generation
            generate_proposal_word,
            // Sync commands
            get_sync_status,
            sync_with_server,
            force_sync_cleanup,
            get_server_url,
            set_server_url,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
