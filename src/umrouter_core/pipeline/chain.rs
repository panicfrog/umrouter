use crate::umrouter_core::types::MiddlewareId;

/// 某一次路由解析之后，对应的一条"中间件链配置"。
///
/// 通过遍历所有中间件的 matcher，收集匹配的中间件，
/// 按 phase 分组并按 priority 排序后得到。
///
/// - pre_rw_chain：在核心中间件之前执行（业务读写）
/// - core_chain：核心中间件（参数校验 / hook）
/// - post_ro_chain：在核心之后执行（业务只读）
#[derive(Debug, Clone)]
pub struct ResolvedMiddlewareChain {
    /// 前置读写中间件 id 链（按 priority 排序）。
    pub pre_rw_chain: Vec<MiddlewareId>,

    /// 核心中间件链（RouterCore 内建）。
    pub core_chain: Vec<MiddlewareId>,

    /// 后置只读中间件 id 链（按 priority 排序）。
    pub post_ro_chain: Vec<MiddlewareId>,
}
