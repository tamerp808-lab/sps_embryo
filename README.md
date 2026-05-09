# SPS Embryo - Deterministic Cognitive Operating System

## 🚀 التشغيل السريع
1. تأكد من تثبيت Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. شغّل: `cargo run`
3. افتح `http://localhost:9101/dashboard`

## 🧠 الوكلاء
- **VoiceAgent**: نطق واستماع.
- **MobileAgent**: فتح تطبيقات، لقطات شاشة، كتابة نصوص.
- **BrowserAgent**: تصفح وبحث.
- **CoderAgent**: كتابة ومراجعة وتصحيح الأكواد.
- **VisionAgent**: تحليل الصور.
- **CalendarAgent**: إنشاء أحداث.
- **GmailAgent**: إرسال بريد.

## 🔧 أوامر API
- `/execute` - تنفيذ مهمة.
- `/goal/set` - إضافة هدف.
- `/memory/search` - بحث في الذاكرة.
- `/admin/compact` - ضغط السجلات.
- `/admin/verify_chain` - التحقق من سلامة السلسلة.

## 🔒 الأمان
قم بتعيين `SPS_API_KEY` في المتغيرات لتفعيل المصادقة.
