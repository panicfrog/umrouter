use std::collections::HashMap;

use matchit::Router as MatchitRouter;

use super::meta::RouteMeta;
use crate::umrouter_core::types::RouteId;

/// 路由表存储结构：
///
/// - 所有 RouteMeta（Vec）
/// - name -> RouteId 索引
/// - path -> RouteId 匹配（基于 matchit）
#[derive(Debug)]
pub struct RouteStore {
    /// 所有路由的元信息，索引下标就是内部 RouteId 的值。
    pub metas: Vec<RouteMeta>,

    /// name-based 索引："auth.profile" -> RouteId
    pub name_index: HashMap<String, RouteId>,

    /// path-based 索引，使用 matchit 做底层结构。
    ///
    /// 例如：
    ///     "/auth/profile" -> RouteId(1)
    ///     "/orders/:orderId/detail" -> RouteId(2)
    pub path_router: MatchitRouter<RouteId>,
}

