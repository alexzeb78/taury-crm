use tauri::api::process::{Command, CommandEvent};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct SidecarManager {
    child: Arc<Mutex<Option<tauri::api::process::CommandChild>>>,
}

impl Drop for SidecarManager {
    fn drop(&mut self) {
        // Tuer le processus Python quand l'app se ferme
        let child = self.child.clone();
        tokio::spawn(async move {
            let mut child_lock = child.lock().await;
            if let Some(child_process) = child_lock.take() {
                let _ = child_process.kill();
                println!("🛑 Document Generator arrêté");
            }
        });
    }
}

impl SidecarManager {
    pub fn new() -> Self {
        Self {
            child: Arc::new(Mutex::new(None)),
        }
    }
    
    pub async fn start(&self) -> Result<(), String> {
        // Lancer le sidecar Python
        let (mut rx, child) = Command::new_sidecar("doc-generator")
            .map_err(|e| format!("Failed to create sidecar command: {}", e))?
            .spawn()
            .map_err(|e| format!("Failed to spawn sidecar: {}", e))?;
        
        // Stocker le child process
        let mut child_lock = self.child.lock().await;
        *child_lock = Some(child);
        drop(child_lock);
        
        // Logger les events du sidecar (en async)
        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                match event {
                    CommandEvent::Stdout(line) => {
                        println!("[Doc Generator] {}", line);
                    }
                    CommandEvent::Stderr(line) => {
                        eprintln!("[Doc Generator ERROR] {}", line);
                    }
                    CommandEvent::Error(err) => {
                        eprintln!("[Doc Generator FATAL] {}", err);
                    }
                    CommandEvent::Terminated(payload) => {
                        println!("[Doc Generator] Terminated with code: {:?}", payload.code);
                        break;
                    }
                    _ => {}
                }
            }
        });
        
        // Attendre un peu que l'API démarre
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        
        // Vérifier que l'API répond
        match reqwest::get("http://127.0.0.1:8001/health").await {
            Ok(resp) if resp.status().is_success() => {
                println!("✅ Document Generator API démarrée");
                Ok(())
            }
            _ => {
                println!("⚠️ Document Generator démarré mais pas encore prêt");
                Ok(())
            }
        }
    }
}

