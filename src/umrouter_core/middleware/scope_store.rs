use std::collections::HashMap;

use matchit::Router as MatchitRouter;

use super::scope::MiddlewareScope;
use crate::umrouter_core::types::MiddlewareScopeId;

/// 用于 path -> MiddlewareScopeId 的匹配路由器。
///
/// 与"页面路由"的 matchit::Router<RouteId> 不同，
/// 这里专门用来匹配横切 concerns（auth / logging / 实验等）的作用域。
#[derive(Debug, Default)]
pub struct MiddlewareScopeRouter {
    /// path pattern -> scope id
    ///
    /// 例如：
    ///     "/auth/**"     -> scope_auth_required
    ///     "/auth/login"  -> scope_guest_only
    ///     "/public/**"   -> scope_public
    pub router: MatchitRouter<MiddlewareScopeId>,
}

/// 中间件作用域的集中管理结构。
///
/// - `scopes` 保存 scope 的具体配置（pre_rw / post_ro）
/// - `router` 用于根据 path 匹配到一个或多个 scope id
#[derive(Debug, Default)]
pub struct MiddlewareScopeStore {
    /// scope id -> scope 配置
    pub scopes: HashMap<MiddlewareScopeId, MiddlewareScope>,

    /// 用于根据 path 匹配 scope id 的 matchit router。
    pub router: MiddlewareScopeRouter,
}

