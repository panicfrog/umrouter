use std::collections::HashMap;

use super::types::{Middleware, MiddlewarePhase};
use crate::umrouter_core::types::MiddlewareId;

/// 中间件注册表。
///
/// 集中管理所有注册的中间件。
#[derive(Default)]
pub struct MiddlewareRegistry {
    /// 所有注册的中间件：id -> Middleware
    middlewares: HashMap<MiddlewareId, Middleware>,
}

impl std::fmt::Debug for MiddlewareRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MiddlewareRegistry")
            .field("count", &self.middlewares.len())
            .field("ids", &self.middlewares.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl MiddlewareRegistry {
    /// 创建空的注册表。
    pub fn new() -> Self {
        Self::default()
    }

    /// 注册一个中间件。
    pub fn register(&mut self, middleware: Middleware) {
        self.middlewares.insert(middleware.id.clone(), middleware);
    }

    /// 根据 ID 获取中间件。
    pub fn get(&self, id: &MiddlewareId) -> Option<&Middleware> {
        self.middlewares.get(id)
    }

    /// 获取所有中间件。
    pub fn all(&self) -> impl Iterator<Item = &Middleware> {
        self.middlewares.values()
    }

    /// 按阶段获取中间件。
    pub fn by_phase(&self, phase: MiddlewarePhase) -> impl Iterator<Item = &Middleware> {
        self.middlewares.values().filter(move |m| m.phase == phase)
    }

    /// 获取中间件数量。
    pub fn len(&self) -> usize {
        self.middlewares.len()
    }

    /// 是否为空。
    pub fn is_empty(&self) -> bool {
        self.middlewares.is_empty()
    }
}
