use std::collections::HashMap;

use super::chain::ResolvedMiddlewareChain;
use crate::umrouter_core::route::ResolvedRoute;
use crate::umrouter_core::state::RouterState;
use crate::umrouter_core::types::CanonicalParams;

/// 简单的扩展数据容器（可以用 AnyMap / typemap 等更复杂实现）。
#[derive(Debug, Default)]
pub struct ExtensionsMap {
    // 占位：你可以替换成真正的类型安全容器。
    pub inner: HashMap<String, String>,
}

/// 中间件执行时看到的上下文。
///
/// 实际字段可以根据需要再细化，这里只保留关键关联点。
#[derive(Debug)]
pub struct MiddlewareContext<'a> {
    /// 这次导航解析出的路由信息。
    pub resolved_route: &'a ResolvedRoute<'a>,

    /// 当前（或即将）生效的 RouterState 快照。
    ///
    /// - 只读中间件只能观察
    /// - 读写中间件可以通过受控 API 提交变更请求（实际实现中可不直接给 &mut）
    pub router_state: &'a RouterState,

    /// 解析后的参数（path/query/body merge 后的 canonical params）。
    pub params: &'a mut CanonicalParams,

    /// 解析得到的中间件链配置，按阶段拆分。
    pub middleware_chain: &'a ResolvedMiddlewareChain,

    /// 用于存放中间件之间传递的扩展数据（例如 auth 结果 / 实验分组）。
    pub extensions: ExtensionsMap,
}

/// 一次导航解析完成后，进入 pipeline 之前的"完整上下文描述"。
///
/// 包含：
/// - 路由解析结果（ResolvedRoute）
/// - 完整参数（合并了 path/query/body）
/// - 解析好的中间件链
#[derive(Debug)]
pub struct NavResolution<'a> {
    /// 路由解析结果。
    pub route: ResolvedRoute<'a>,

    /// 规范化参数（path/query/body merge 后的结果）。
    pub canonical_params: CanonicalParams,

    /// 中间件调用链配置（pre_rw / core / post_ro）。
    pub middleware_chain: ResolvedMiddlewareChain,
}

