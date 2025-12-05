use std::sync::Arc;

use crate::umrouter_core::route::RouteMeta;
use crate::umrouter_core::types::{CanonicalParams, MiddlewareId, RuntimeKind, StackId};

/// 中间件执行阶段。
///
/// 决定中间件在 pipeline 中的执行时机。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MiddlewarePhase {
    /// 前置读写阶段：在核心处理之前执行，可以修改请求/上下文。
    ///
    /// 典型用途：
    /// - 认证/授权检查
    /// - 参数预处理
    /// - 请求拦截/重定向
    PreRW,

    /// 核心阶段：RouterCore 内建的处理。
    ///
    /// 典型用途：
    /// - 参数校验
    /// - Hook 调用
    Core,

    /// 后置只读阶段：在核心处理之后执行，只能观察不能修改。
    ///
    /// 典型用途：
    /// - 日志记录
    /// - 埋点上报
    /// - 性能监控
    PostRO,
}

/// 中间件的访问模式。
///
/// 用于在 pipeline 中做权限控制。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessMode {
    /// 只读：只能观察上下文，不能修改。
    ReadOnly,

    /// 读写：允许修改上下文/请求。
    ReadWrite,
}

//
// ========== Matcher 相关 ==========
//

/// 匹配上下文：Matcher 执行时可以访问的信息。
#[derive(Debug)]
pub struct MatchContext<'a> {
    /// 路由元信息（包含 path, name, tags, runtime 等）。
    pub route: &'a RouteMeta,

    /// 目标栈。
    pub target_stack: &'a StackId,

    /// 当前请求的 runtime。
    pub runtime: RuntimeKind,

    /// 解析后的参数。
    pub params: &'a CanonicalParams,
}

/// Matcher trait：决定中间件是否对当前请求生效。
///
/// 不同实现方式（Rust/FFI/WASM/JS）各自实现这个 trait。
pub trait Matcher: Send + Sync {
    /// 判断是否匹配。
    fn matches(&self, ctx: &MatchContext) -> bool;

    /// 可选：返回一个描述性名称，用于调试。
    fn name(&self) -> &str {
        "unnamed_matcher"
    }
}

//
// ========== Executor 相关 ==========
//

/// 中间件执行结果。
#[derive(Debug, Clone)]
pub enum MiddlewareResult {
    /// 继续执行后续中间件。
    Continue,

    /// 中止执行，不再执行后续中间件。
    ///
    /// 可以携带一个原因说明。
    Abort { reason: String },

    /// 重定向到另一个路由。
    Redirect { target: String },
}

/// 执行上下文：Executor 执行时可以访问和修改的信息。
#[derive(Debug)]
pub struct ExecuteContext<'a> {
    /// 路由元信息。
    pub route: &'a RouteMeta,

    /// 目标栈。
    pub target_stack: &'a StackId,

    /// 当前请求的 runtime。
    pub runtime: RuntimeKind,

    /// 解析后的参数（可修改，如果是 ReadWrite 模式）。
    pub params: &'a mut CanonicalParams,

    /// 扩展数据：中间件之间传递的数据。
    pub extensions: &'a mut Extensions,
}

/// 扩展数据容器。
///
/// 用于中间件之间传递数据。
/// TODO: 后续可以替换为类型安全的 TypeMap。
#[derive(Debug, Default)]
pub struct Extensions {
    data: std::collections::HashMap<String, String>,
}

impl Extensions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.data.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(|s| s.as_str())
    }
}

/// Executor trait：中间件的具体执行逻辑。
///
/// 不同实现方式（Rust/FFI/WASM/JS）各自实现这个 trait。
pub trait Executor: Send + Sync {
    /// 执行中间件逻辑。
    fn execute(&self, ctx: &mut ExecuteContext) -> MiddlewareResult;

    /// 可选：返回一个描述性名称，用于调试。
    fn name(&self) -> &str {
        "unnamed_executor"
    }
}

//
// ========== Middleware 定义 ==========
//

/// 完整的中间件定义。
///
/// 一个中间件由以下部分组成：
/// - Matcher：决定是否对当前请求生效
/// - Executor：具体的执行逻辑
/// - Phase：在 pipeline 的哪个阶段执行
/// - Priority：同阶段内的执行顺序
pub struct Middleware {
    /// 中间件唯一标识。
    pub id: MiddlewareId,

    /// 匹配器：决定这个中间件是否对当前请求生效。
    pub matcher: Arc<dyn Matcher>,

    /// 执行器：具体的中间件逻辑。
    pub executor: Arc<dyn Executor>,

    /// 执行阶段：PreRW / Core / PostRO。
    pub phase: MiddlewarePhase,

    /// 访问模式：只读 / 读写。
    pub access_mode: AccessMode,

    /// 优先级：同阶段内的执行顺序。
    ///
    /// 数值越小优先级越高，越先执行。
    pub priority: i32,

    /// 业务标签（用于分类、调试等）。
    pub tags: Vec<String>,
}

impl std::fmt::Debug for Middleware {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Middleware")
            .field("id", &self.id)
            .field("matcher", &self.matcher.name())
            .field("executor", &self.executor.name())
            .field("phase", &self.phase)
            .field("access_mode", &self.access_mode)
            .field("priority", &self.priority)
            .field("tags", &self.tags)
            .finish()
    }
}

impl Clone for Middleware {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            matcher: Arc::clone(&self.matcher),
            executor: Arc::clone(&self.executor),
            phase: self.phase,
            access_mode: self.access_mode,
            priority: self.priority,
            tags: self.tags.clone(),
        }
    }
}

//
// ========== 便捷的 Rust 实现 ==========
//

/// 用闭包创建 Matcher 的便捷结构。
pub struct FnMatcher<F> {
    name: String,
    func: F,
}

impl<F> FnMatcher<F>
where
    F: Fn(&MatchContext) -> bool + Send + Sync,
{
    pub fn new(name: impl Into<String>, func: F) -> Self {
        Self {
            name: name.into(),
            func,
        }
    }
}

impl<F> Matcher for FnMatcher<F>
where
    F: Fn(&MatchContext) -> bool + Send + Sync,
{
    fn matches(&self, ctx: &MatchContext) -> bool {
        (self.func)(ctx)
    }

    fn name(&self) -> &str {
        &self.name
    }
}

/// 用闭包创建 Executor 的便捷结构。
pub struct FnExecutor<F> {
    name: String,
    func: F,
}

impl<F> FnExecutor<F>
where
    F: Fn(&mut ExecuteContext) -> MiddlewareResult + Send + Sync,
{
    pub fn new(name: impl Into<String>, func: F) -> Self {
        Self {
            name: name.into(),
            func,
        }
    }
}

impl<F> Executor for FnExecutor<F>
where
    F: Fn(&mut ExecuteContext) -> MiddlewareResult + Send + Sync,
{
    fn execute(&self, ctx: &mut ExecuteContext) -> MiddlewareResult {
        (self.func)(ctx)
    }

    fn name(&self) -> &str {
        &self.name
    }
}

//
// ========== 匹配所有的 Matcher ==========
//

/// 匹配所有请求的 Matcher。
pub struct AlwaysMatcher;

impl Matcher for AlwaysMatcher {
    fn matches(&self, _ctx: &MatchContext) -> bool {
        true
    }

    fn name(&self) -> &str {
        "always"
    }
}
