use crate::umrouter_core::types::{MiddlewareId, MiddlewareScopeId};

/// 一条"作用域"规则：
///
/// - 由一个或多个路径 pattern 组成（保存在 ScopeStore 里）
/// - 告诉 RouterCore：
///   在这些 path 下，哪些中间件应该参与 pipeline。
#[derive(Debug, Clone)]
pub struct MiddlewareScope {
    pub id: MiddlewareScopeId,

    /// 在"前置读写阶段"的中间件列表。
    ///
    /// 例如：
    ///     - auth_guard（拦截未登录）
    ///     - rate_limit_guard（限流）
    pub pre_rw: Vec<MiddlewareId>,

    /// 在"后置只读阶段"的中间件列表。
    ///
    /// 例如：
    ///     - logging 中间件
    ///     - 埋点上报中间件
    pub post_ro: Vec<MiddlewareId>,
}

