use serde::{Deserialize, Serialize};
use std::io::Write;
use std::process::{Child, Command};
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
pub struct MpvStatus {
    pub is_installed: bool,
    pub is_running: bool,
    pub path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MpvLaunchOptions {
    pub file_path: String,
    pub subtitle_size: Option<u32>,
    pub audio_track: Option<String>,
    pub subtitle_track: Option<String>,
    pub start_time: Option<f64>,
}

pub struct MpvManager {
    child: Mutex<Option<Child>>,
}

impl MpvManager {
    pub fn new() -> Self {
        Self {
            child: Mutex::new(None),
        }
    }

    pub fn find_mpv_binary() -> Option<String> {
        let candidates = vec![
            "mpv".to_string(),
            "/opt/homebrew/bin/mpv".to_string(),
            "/usr/local/bin/mpv".to_string(),
            "/usr/bin/mpv".to_string(),
            "C:\\Program Files\\mpv\\mpv.exe".to_string(),
            "C:\\mpv\\mpv.exe".to_string(),
        ];

        for c in candidates {
            if let Ok(output) = Command::new(&c).arg("--version").output() {
                if output.status.success() {
                    return Some(c);
                }
            }
        }
        None
    }

    pub fn is_installed() -> bool {
        Self::find_mpv_binary().is_some()
    }

    pub fn is_running(&self) -> bool {
        let mut lock = self.child.lock().unwrap();
        if let Some(child) = lock.as_mut() {
            match child.try_wait() {
                Ok(None) => true,
                _ => {
                    *lock = None;
                    false
                }
            }
        } else {
            false
        }
    }

    pub fn kill_current(&self) {
        let mut lock = self.child.lock().unwrap();
        if let Some(mut child) = lock.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }

    pub fn play(&self, opts: MpvLaunchOptions) -> Result<(), String> {
        self.kill_current();

        let mpv_bin = Self::find_mpv_binary()
            .ok_or_else(|| "MPV není nainstalováno na tomto počítači.".to_string())?;

        let mut cmd = Command::new(&mpv_bin);
        cmd.arg(&opts.file_path);

        // IPC socket pro obousměrné ovládání
        #[cfg(unix)]
        let socket_path = "/tmp/movieapp_mpv.sock";
        #[cfg(windows)]
        let socket_path = r"\\.\pipe\movieapp_mpv";

        cmd.arg(format!("--input-ipc-server={}", socket_path));

        // Nastavení velikosti titulků
        if let Some(size) = opts.subtitle_size {
            cmd.arg(format!("--sub-font-size={}", size));
        }

        // Nastavení výchozího audia
        if let Some(audio) = opts.audio_track {
            if audio != "default" {
                cmd.arg(format!("--alang={}", audio.to_lowercase()));
            }
        }

        // Nastavení výchozích titulků
        if let Some(sub) = opts.subtitle_track {
            if sub == "off" {
                cmd.arg("--sid=no");
            } else if sub != "default" {
                cmd.arg(format!("--slang={}", sub.to_lowercase()));
            }
        }

        // Čas spuštění
        if let Some(t) = opts.start_time {
            if t > 0.0 {
                cmd.arg(format!("--start={}", t));
            }
        }

        // Vzhled a integrace
        cmd.arg("--title=MovieApp Player - MPV");
        cmd.arg("--keep-open=yes");

        let child = cmd.spawn().map_err(|e| format!("Chyba při spouštění MPV: {}", e))?;

        let mut lock = self.child.lock().unwrap();
        *lock = Some(child);

        Ok(())
    }

    pub fn send_ipc_command(&self, json_cmd: &str) -> Result<(), String> {
        #[cfg(unix)]
        {
            use std::os::unix::net::UnixStream;
            let socket_path = "/tmp/movieapp_mpv.sock";
            let mut stream = UnixStream::connect(socket_path)
                .map_err(|e| format!("Nepodařilo se připojit k MPV IPC socketu: {}", e))?;
            stream
                .write_all(format!("{}\n", json_cmd).as_bytes())
                .map_err(|e| format!("Chyba zápisu do socketu: {}", e))?;
            Ok(())
        }
        #[cfg(windows)]
        {
            Err("Windows named pipes not implemented yet".to_string())
        }
    }
}
