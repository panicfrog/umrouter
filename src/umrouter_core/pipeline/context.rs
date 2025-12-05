use super::chain::ResolvedMiddlewareChain;
use crate::umrouter_core::middleware::Extensions;
use crate::umrouter_core::route::ResolvedRoute;
use crate::umrouter_core::types::CanonicalParams;

/// 一次导航解析完成后，进入 pipeline 之前的"完整上下文描述"。
///
/// 包含：
/// - 路由解析结果（ResolvedRoute）
/// - 完整参数（合并了 path/query/body）
/// - 解析好的中间件链
/// - 扩展数据容器
#[derive(Debug)]
pub struct NavResolution<'a> {
    /// 路由解析结果。
    pub route: ResolvedRoute<'a>,

    /// 规范化参数（path/query/body merge 后的结果）。
    pub canonical_params: CanonicalParams,

    /// 中间件调用链配置（pre_rw / core / post_ro）。
    pub middleware_chain: ResolvedMiddlewareChain,

    /// 扩展数据容器（中间件之间传递数据）。
    pub extensions: Extensions,
}
