use crate::capabilities::{Capability, CapabilityToken};

pub struct AutoCapability;

impl AutoCapability {
    /// يتحقق من أن الرمز يملك الصلاحية المطلوبة
    pub fn validate_request(token: &CapabilityToken, required: Capability) -> Result<(), String> {
        if token.has(&required) {
            Ok(())
        } else {
            Err(format!("Missing capability {:?}", required))
        }
    }

    /// يتحقق من عدة صلاحيات دفعة واحدة
    pub fn validate_all(token: &CapabilityToken, required: &[Capability]) -> Result<(), String> {
        for cap in required {
            if !token.has(cap) {
                return Err(format!("Missing capability {:?}", cap));
            }
        }
        Ok(())
    }
}
