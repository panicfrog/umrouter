use std::collections::BTreeMap;

use super::meta::RouteMeta;
use crate::umrouter_core::types::RouteId;

/// 基于 path/name 解析得到的「路由解析结果」。
///
/// - 绑定了具体的 RouteMeta
/// - 含 path 参数（例如 ":orderId" -> "123"）
///
/// 之后会在 pipeline 中配合 query/body 组成 CanonicalParams。
#[derive(Debug)]
pub struct ResolvedRoute<'a> {
    /// 路由 ID。
    pub id: RouteId,

    /// 路由元信息引用。
    pub meta: &'a RouteMeta,

    /// path 参数（":id" / ":orderId" 等）。
    pub path_params: BTreeMap<String, String>,
}

