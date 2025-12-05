use std::collections::HashMap;

use super::descriptor::MiddlewareDescriptor;
use crate::umrouter_core::types::MiddlewareId;

/// 全局中间件注册表：
///
/// - 按 id 索引到具体描述；
/// - RouterCore 通过它知道某个 MiddlewareId 背后是哪种实现。
#[derive(Debug, Default)]
pub struct MiddlewareRegistry {
    pub descriptors: HashMap<MiddlewareId, MiddlewareDescriptor>,
}

