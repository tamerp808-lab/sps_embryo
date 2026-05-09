use std::time::Duration;

/// وصف أداة (وكيل) مع صلاحياتها
pub struct ToolSpec {
    pub name: &'static str,
    pub allowed: bool,
    pub timeout: Duration,
    pub requires_sandbox: bool,
    pub max_retries: u32,
}

/// سجل الأدوات – يتحكم فيمن يستطيع فعل ماذا
pub struct ToolRegistry;

impl ToolRegistry {
    /// يُعيد مواصفات الأداة المطلوبة
    pub fn get_spec(tool_name: &str) -> Option<ToolSpec> {
        match tool_name {
            "voice" => Some(ToolSpec {
                name: "voice",
                allowed: true,
                timeout: Duration::from_secs(10),
                requires_sandbox: false,
                max_retries: 1,
            }),
            "mobile" => Some(ToolSpec {
                name: "mobile",
                allowed: true,
                timeout: Duration::from_secs(15),
                requires_sandbox: false,
                max_retries: 0,
            }),
            "browser" => Some(ToolSpec {
                name: "browser",
                allowed: true,
                timeout: Duration::from_secs(30),
                requires_sandbox: true,
                max_retries: 2,
            }),
            "coder" => Some(ToolSpec {
                name: "coder",
                allowed: true,
                timeout: Duration::from_secs(20),
                requires_sandbox: true,
                max_retries: 2,
            }),
            "vision" => Some(ToolSpec {
                name: "vision",
                allowed: true,
                timeout: Duration::from_secs(25),
                requires_sandbox: true,
                max_retries: 1,
            }),
            "calendar" => Some(ToolSpec {
                name: "calendar",
                allowed: true,
                timeout: Duration::from_secs(5),
                requires_sandbox: false,
                max_retries: 0,
            }),
            "gmail" => Some(ToolSpec {
                name: "gmail",
                allowed: true,
                timeout: Duration::from_secs(10),
                requires_sandbox: false,
                max_retries: 1,
            }),
            "repair" => Some(ToolSpec {
                name: "repair",
                allowed: true,
                timeout: Duration::from_secs(60),
                requires_sandbox: true,
                max_retries: 3,
            }),
            _ => None,
        }
    }

    /// هل يُسمح للأداة بالعمل؟
    pub fn is_allowed(tool_name: &str) -> bool {
        Self::get_spec(tool_name)
            .map(|s| s.allowed)
            .unwrap_or(false)
    }
}
