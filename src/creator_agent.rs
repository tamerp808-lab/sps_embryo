use crate::outbox::{EffectOutbox, EffectRequest, EffectType};
use std::fs;
use std::path::PathBuf;

pub struct CreatorAgent;

impl CreatorAgent {
    pub fn create_video(_desc: &str, outbox: &mut EffectOutbox) {
        let path = format!("/tmp/sps_video_{}.mp4", uuid::Uuid::now_v7());
        let cmd = format!("ffmpeg -f lavfi -i color=c=black:s=1280x720:d=5 -an {} -y 2>/dev/null && echo {} || echo 'FAILED'", path, path);
        outbox.enqueue(EffectRequest {
            id: uuid::Uuid::now_v7().as_u64_pair().0,
            tick: 0,
            effect_type: EffectType::Subprocess,
            payload: cmd,
        });
    }
    pub fn add_audio(video_path: &str, audio_path: &str, outbox: &mut EffectOutbox) {
        let output = format!("/tmp/sps_av_{}.mp4", uuid::Uuid::now_v7());
        let cmd = format!("ffmpeg -i {} -i {} -c:v copy -c:a aac -shortest {} -y 2>/dev/null && echo '{}' || echo 'FAILED'", video_path, audio_path, output, output);
        outbox.enqueue(EffectRequest {
            id: uuid::Uuid::now_v7().as_u64_pair().0,
            tick: 0,
            effect_type: EffectType::Subprocess,
            payload: cmd,
        });
    }
    pub fn add_text_overlay(video_path: &str, text: &str, outbox: &mut EffectOutbox) {
        let output = format!("/tmp/sps_text_{}.mp4", uuid::Uuid::now_v7());
        let cmd = format!("ffmpeg -i {} -vf \"drawtext=text='{}':fontsize=24:fontcolor=white:x=(w-text_w)/2:y=10\" -codec:a copy {} -y 2>/dev/null && echo '{}' || echo 'FAILED'", video_path, text, output, output);
        outbox.enqueue(EffectRequest {
            id: uuid::Uuid::now_v7().as_u64_pair().0,
            tick: 0,
            effect_type: EffectType::Subprocess,
            payload: cmd,
        });
    }
    pub fn build_app(spec: &str, outbox: &mut EffectOutbox) {
        let dir = format!("/tmp/sps_app_{}", uuid::Uuid::now_v7());
        let cmd = format!("mkdir -p '{}' && echo '<!DOCTYPE html><html><body><h1>{}</h1></body></html>' > '{}/index.html' && echo '{}'", dir, spec, dir, dir);
        outbox.enqueue(EffectRequest {
            id: uuid::Uuid::now_v7().as_u64_pair().0,
            tick: 0,
            effect_type: EffectType::Subprocess,
            payload: cmd,
        });
    }
    pub fn deploy_app(app_dir: &str, server_url: &str, outbox: &mut EffectOutbox) {
        let cmd = format!(
            "curl -X POST {} -F 'app=@{}' 2>/dev/null || echo 'FAILED'",
            server_url, app_dir
        );
        outbox.enqueue(EffectRequest {
            id: uuid::Uuid::now_v7().as_u64_pair().0,
            tick: 0,
            effect_type: EffectType::Subprocess,
            payload: cmd,
        });
    }

    /// بناء موقع ويب بسيط وإرجاع معرفه
    pub fn build_site(spec: &str) -> Result<String, String> {
        let id = uuid::Uuid::now_v7().to_string();
        let dir_path = PathBuf::from("/tmp/sps_sites").join(&id);
        fs::create_dir_all(&dir_path).map_err(|e| e.to_string())?;

        // إنشاء صفحة index.html بسيطة تعتمد على المواصفات
        let html = format!(
            "<!DOCTYPE html>\n<html lang=\"ar\">\n<head><meta charset=\"UTF-8\"><title>{spec}</title></head>\n<body style=\"background:#0a0a0f;color:#e0eeff;font-family:sans-serif;padding:20px;\">\n<h1>{spec}</h1>\n<p>تم إنشاء هذا الموقع بواسطة SPS Embryo.</p>\n</body>\n</html>",
            spec = spec
        );
        fs::write(dir_path.join("index.html"), &html).map_err(|e| e.to_string())?;

        Ok(id)
    }

    /// ضغط مجلد الموقع وإرجاع مسار الأرشيف
    pub fn zip_site(site_id: &str) -> Result<PathBuf, String> {
        let dir_path = PathBuf::from("/tmp/sps_sites").join(site_id);
        if !dir_path.exists() {
            return Err("Site not found".into());
        }
        let zip_path = PathBuf::from("/tmp").join(format!("site_{}.zip", site_id));
        let status = std::process::Command::new("zip")
            .args(["-r", &zip_path.to_string_lossy(), "."])
            .current_dir(&dir_path)
            .status()
            .map_err(|e| e.to_string())?;
        if status.success() {
            Ok(zip_path)
        } else {
            Err("zip command failed".into())
        }
    }
}
