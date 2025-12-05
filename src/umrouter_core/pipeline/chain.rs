use crate::umrouter_core::types::MiddlewareId;

/// 某一次路由解析之后，对应的一条"中间件链配置"。
///
/// - pre_rw_chain：在核心中间件之前执行（业务读写）
/// - core_chain：核心中间件（参数校验 / hook）
/// - post_ro_chain：在核心之后执行（业务只读）
///
/// 此结构通常会附加在 ResolvedRoute / NavContext 上，
/// 供 pipeline 执行阶段直接使用。
#[derive(Debug, Clone)]
pub struct ResolvedMiddlewareChain {
    /// 业务前置读写中间件 id 链（合并各个 Scope 得来的）。
    pub pre_rw_chain: Vec<MiddlewareId>,

    /// 核心中间件链（RouterCore 内建，不通过 Scope 配置）。
    pub core_chain: Vec<MiddlewareId>,

    /// 业务后置只读中间件 id 链。
    pub post_ro_chain: Vec<MiddlewareId>,
}

