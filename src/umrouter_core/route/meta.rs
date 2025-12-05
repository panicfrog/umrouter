use crate::umrouter_core::types::{
    LifecycleEvent, PresentationMode, RouteId, RuntimeKind, StackId,
};

/// 参数 schema 的配置占位结构。
///
/// 你可以决定是：
/// - "一份 schema 内部分 path/query/body 三块"
/// 或
/// - "三份 schema 分开存"
///
/// 这里仅作为占位，具体结构可后续细化。
#[derive(Debug, Clone)]
pub struct ParamSchemaSpec {
    /// schema 的标识（例如 JSON Schema 的 id 或 registry key）。
    pub schema_id: Option<String>,

    /// 是否区分 path/query/body 的子 schema。
    pub has_sub_schemas: bool,
}

/// Hook 声明。
///
/// 声明路由需要监听哪些生命周期事件和自定义 hook。
#[derive(Debug, Clone)]
pub struct HookSpec {
    /// 启用哪些生命周期事件。
    pub enabled_lifecycles: Vec<LifecycleEvent>,

    /// 业务自定义 hook key 列表。
    pub custom_hooks: Vec<String>,
}

/// 动画/展示偏好。
#[derive(Debug, Clone)]
pub struct TransitionSpec {
    /// 页面展示模式：push / modal / sheet 等。
    pub presentation: PresentationMode,

    /// 动画关键字（供 adapter 解析）。
    ///
    /// 例如："fade", "slide_up", "none" 等。
    pub animation: Option<String>,

    /// 是否允许侧滑返回等手势。
    pub gesture_back_enabled: bool,
}

/// 路由类型：
/// - StackRoute：常规 push/pop/replace 的页面路由
/// - MultiStackRoute：会对多个栈进行协调的高阶路由（切 tab / reset 多栈等）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteKind {
    StackRoute,
    MultiStackRoute,
}

/// 单条业务路由的元信息。
///
/// 描述"这条路是什么"、"由谁渲染"、"默认挂在哪个栈"、
/// 以及与参数、动画等相关的配置。
///
/// 注意：中间件不再在路由级别静态绑定，
/// 而是由各中间件的 matcher 动态决定是否生效。
#[derive(Debug, Clone)]
pub struct RouteMeta {
    /// 唯一路由 ID，用于在 RouteStore / RouterState / StackFrame 中引用。
    pub id: RouteId,

    /// 路由的路径模式，用于 URL 匹配。
    ///
    /// 例如：
    /// - "/home"
    /// - "/orders/:orderId/detail"
    /// - "/auth/profile"
    pub path: String,

    /// 人类可读的路由名，例如：
    /// - "auth.profile"
    /// - "home.index"
    pub name: String,

    /// 页面由哪个 runtime 渲染（Native / RN / Flutter / WebView …）。
    pub runtime: RuntimeKind,

    /// 建议挂载到哪个业务栈，例如 "home" / "trade" / "auth"。
    pub preferred_stack: StackId,

    /// 路由类型：普通栈路由 / 多栈操作路由。
    pub route_kind: RouteKind,

    /// 参数校验相关的 schema 配置（path/query/body）。
    pub param_schema: ParamSchemaSpec,

    /// Hook 声明。
    pub hook_spec: HookSpec,

    /// 动画与展示偏好（push / modal / 手势返回等）。
    pub transition_spec: TransitionSpec,

    /// 业务标签（例如 "auth-required" / "public" / "admin-only"）。
    ///
    /// 中间件可以通过 matcher 根据标签来决定是否生效。
    pub tags: Vec<String>,
}
