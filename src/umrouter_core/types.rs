use std::collections::BTreeMap;

use serde_json::Value;

/// 业务路由的内部标识。
///
/// 实际实现中可以是新类型包裹的 usize/u32。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RouteId(pub u32);

/// 业务导航栈的标识（按业务域 / tab / flow 划分）。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StackId(pub String);

/// 运行时类型：页面由谁渲染。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuntimeKind {
    Native,
    ReactNative,
    Flutter,
    // WebView,
    // 未来可以扩展其他 runtime 类型
}

/// 规范化参数：
///
/// 代表 path/query/body merge 后的统一参数视图。
/// 使用 serde_json::Value 支持复杂的参数结构（数组、嵌套对象等）。
#[derive(Debug, Clone, Default)]
pub struct CanonicalParams {
    pub map: BTreeMap<String, Value>,
}

/// 业务中间件的唯一标识（逻辑层面用的 ID）。
///
/// 比如：
/// - "auth_guard"
/// - "guest_only_guard"
/// - "trace_request"
/// - "ab_experiment_v1"
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MiddlewareId(pub String);

/// 页面生命周期事件类型。
///
/// 用于声明路由需要监听哪些生命周期事件。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LifecycleEvent {
    /// 页面出现（可见）
    OnAppear,
    /// 页面消失（不可见）
    OnDisappear,
    /// 页面即将离开（可用于拦截返回）
    OnBeforeLeave,
    /// 页面已经离开
    OnAfterLeave,
    /// 页面获得焦点
    OnFocus,
    /// 页面失去焦点
    OnBlur,
}

/// 页面展示模式。
///
/// 决定页面如何呈现给用户。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PresentationMode {
    /// 标准 push 入栈
    Push,
    /// 模态弹窗（全屏）
    Modal,
    /// 底部弹出（半屏）
    Sheet,
    /// 替换当前页面（无动画）
    Replace,
    /// 自定义展示（由 adapter 解释）
    Custom,
}
