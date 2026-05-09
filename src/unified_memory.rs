use crate::conversation_memory::ConversationMemory;
use crate::memory_system::{LongTermMemory, ShortTermMemory, VectorMemory};
use std::sync::{Arc, Mutex};

/// الرسم البياني الموحد للذاكرة – يجميع كل أنواع الذاكرة في كيان واحد
pub struct UnifiedMemory {
    pub short_term: Arc<Mutex<ShortTermMemory>>,
    pub long_term: Arc<LongTermMemory>,
    pub vector: Arc<Mutex<VectorMemory>>,
    pub conversation: Arc<ConversationMemory>,
}

impl UnifiedMemory {
    pub fn new(
        short_term: Arc<Mutex<ShortTermMemory>>,
        long_term: Arc<LongTermMemory>,
        vector: Arc<Mutex<VectorMemory>>,
        conversation: Arc<ConversationMemory>,
    ) -> Self {
        Self {
            short_term,
            long_term,
            vector,
            conversation,
        }
    }

    /// يُسجل معلومة جديدة في جميع طبقات الذاكرة
    pub fn store(&self, role: &str, content: &str) {
        // ذاكرة قصيرة المدى
        if let Ok(mut stm) = self.short_term.lock() {
            stm.add(role, content);
        }

        // ذاكرة طويلة المدى
        self.long_term.store(role, content, None, 0.5).ok();

        // ذاكرة المتجهات (اختياري)
        if let Ok(mut vm) = self.vector.lock() {
            vm.insert(content.to_string(), vec![]); // سيُحسَّن لاحقاً باستخدام تضمين فعلي
        }

        // ذاكرة المحادثة
        self.conversation.add_message(role, content).ok();
    }

    /// يسترجع كل الذكريات ذات الصلة (للحلقة المعرفية)
    pub fn retrieve_context(&self) -> String {
        let stm = self
            .short_term
            .lock()
            .map(|s| s.formatted())
            .unwrap_or_default();
        let ltm: String = self
            .long_term
            .retrieve_top(5)
            .iter()
            .map(|(r, c)| format!("[{}] {}", r, c))
            .collect::<Vec<_>>()
            .join("\n");
        format!("STM:\n{}\n\nLTM:\n{}", stm, ltm)
    }
}
