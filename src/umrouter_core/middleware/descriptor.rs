use crate::umrouter_core::types::MiddlewareId;

/// 中间件的实现方式。
///
/// 统一抽象成几种来源：
/// - 内置 Rust
/// - 业务 Rust
/// - Wasm 模块
/// - Js 脚本（或其他动态脚本）
///
/// 实现细节交给 adapter / runtime 去解释。
#[derive(Debug, Clone)]
pub enum MiddlewareImplKind {
    /// 核心内置中间件（参数校验 / hook 调用等）。
    BuiltIn,

    /// 业务方通过 Rust 扩展提供的中间件。
    BusinessRust,

    /// Wasm 模块中导出的中间件实现。
    WasmModule {
        module_id: String,
        entry_func: String,
    },

    /// Js/TS 脚本中导出的中间件实现（例如动态下发）。
    JsScript {
        script_id: String,
        entry_func: String,
    },
    // 未来可以继续扩展其它形式。
}

/// 中间件的访问级别。
///
/// 用于在 pipeline 中做分级控制。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MiddlewareAccessMode {
    /// 只读：只能观察上下文，不能修改。
    ReadOnly,

    /// 读写：允许修改上下文 / 请求。
    ReadWrite,

    /// 受保护的读写：仅核心系统中间件可用，
    /// 比如参数校验 / hook 调用。
    ProtectedReadWrite,
}

/// 描述一个"中间件实现"的元数据。
///
/// RouterCore 本身只需要知道：
/// - 它的逻辑 id 是什么（MiddlewareId）
/// - 它是哪种实现形态（BuiltIn / Wasm / Js …）
///
/// 真正怎么调用（函数指针 / FFI / Wasm Runtime 调用）
/// 交给运行时环境或 adapter 处理。
#[derive(Debug, Clone)]
pub struct MiddlewareDescriptor {
    pub id: MiddlewareId,
    pub impl_kind: MiddlewareImplKind,
    pub access_mode: MiddlewareAccessMode,
    pub tags: Vec<String>,
}

