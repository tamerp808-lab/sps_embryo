use crate::llm_chain;
use crate::memory_system::{LongTermMemory, ShortTermMemory, VectorMemory};

pub struct BrainAgent;

impl BrainAgent {
    pub fn process(
        prompt: &str,
        short_mem: &mut ShortTermMemory,
        long_mem: &LongTermMemory,
        vec_mem: &VectorMemory,
    ) -> String {
        // 1. أضف إلى الذاكرة قصيرة المدى
        short_mem.add("user", prompt);

        // 2. اجمع السياق
        let short_context = short_mem.formatted();
        let long_memories = long_mem.retrieve_top(5);
        let long_context: String = long_memories
            .iter()
            .map(|(r, c)| format!("[Memory] {}: {}", r, c))
            .collect::<Vec<_>>()
            .join("\n");

        // 3. بحث دلالي (إن أمكن)
        // هنا نستخدم تضمينًا بسيطًا للاستعلام للبحث في vec_mem
        let query_emb = crate::llm_chain::embed_sync(prompt);
        let similar = vec_mem.search(&query_emb, 3);
        let similar_context = similar.join("\n");

        // 4. بناء prompt كامل
        let full_prompt = format!(
            "You are the central brain of an AI OS. Use the following context to answer.\n\
             Recent conversation:\n{}\n\n\
             Important memories:\n{}\n\n\
             Semantically similar:\n{}\n\n\
             User: {}\nAssistant:",
            short_context, long_context, similar_context, prompt
        );

        // 5. صنف الطلب
        let classification = llm_chain::chat_sync(&format!(
            "Conversation:\n{}\n\nClassify this request into ONE word: \
             [CHAT, BUILD, OPEN, EMAIL, SPEAK, SEARCH, CALENDAR, SCREENSHOT, CODE, PLAN]. \
             Output ONLY the word.\nRequest: \"{}\"\nWord:",
            short_context, prompt
        ));

        let reply = match classification.trim().to_uppercase().as_str() {
            "BUILD" => format!("✅ بناء: {}", llm_chain::chat_sync(&full_prompt)),
            "OPEN" => format!("📱 فتح: {}", llm_chain::chat_sync(&full_prompt)),
            "EMAIL" => format!("📧 بريد: {}", llm_chain::chat_sync(&full_prompt)),
            "SPEAK" => format!("🔊 نطق: {}", llm_chain::chat_sync(&full_prompt)),
            "SEARCH" => format!("🔍 بحث: {}", llm_chain::chat_sync(&full_prompt)),
            "CALENDAR" => format!("📅 تقويم: {}", llm_chain::chat_sync(&full_prompt)),
            "SCREENSHOT" => "📸 جاري أخذ لقطة شاشة...".to_string(),
            "CODE" => format!("💻 كود: {}", llm_chain::chat_sync(&full_prompt)),
            "PLAN" => format!("📋 خطة:\n{}", llm_chain::chat_sync(&full_prompt)),
            _ => llm_chain::chat_sync(&full_prompt), // CHAT
        };

        // 6. خزّن في الذاكرة طويلة المدى إذا كان مهمًا
        long_mem.store("assistant", &reply, None, 0.3).ok();
        // أضف أيضًا إلى vector memory (للبحث الدلالي)
        // (في الإصدار الكامل سنضيف التضمين هنا)

        // 7. أضف الرد إلى الذاكرة قصيرة المدى
        short_mem.add("assistant", &reply);

        reply
    }
}
