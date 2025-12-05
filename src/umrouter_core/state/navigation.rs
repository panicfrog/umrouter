use std::collections::HashMap;

use crate::umrouter_core::types::{CanonicalParams, RouteId, RuntimeKind, StackId};

//
// ========== 导航状态模型：StackFrame / StackState / RouterState / Transition ==========
//

/// 一次具体的"页面实例"，对应 UI 上看到的一页内容。
///
/// - 绑定一个 RouteId（指向 RouteMeta）
/// - 指明由哪个 runtime 渲染
/// - 可以携带参数快照、打开时间等信息
#[derive(Debug, Clone)]
pub struct StackFrame {
    /// 对应的业务路由标识，用于在 RouteStore 中定位 RouteMeta。
    pub route_id: RouteId,

    /// 渲染这个页面实例所使用的 runtime 类型。
    pub runtime: RuntimeKind,

    /// 打开此页面时的参数快照（可选）。
    ///
    /// 建议存的是已 merge 的 CanonicalParams，便于调试 / 回放。
    pub params_snapshot: Option<CanonicalParams>,

    /// 打开时间（毫秒时间戳，可选），用于埋点 / 调试。
    pub opened_at_millis: Option<u64>,

    /// 扩展标签（如 "auth-flow-step-1" / "experiment-A"）。
    pub tags: Vec<String>,
}

/// 一条"业务导航栈"的状态。
///
/// 通过类型设计保证 invariant：栈至少有一个 root frame。
///
/// - `root_frame` 永远存在，是栈底页面
/// - `additional_frames` 是 root 之上的页面列表（可以为空）
///
/// 栈的创建必须伴随 root frame 的创建；
/// Pop root frame 时要么 no-op，要么通过显式 MultiStack 操作关闭整个栈。
#[derive(Debug, Clone)]
pub struct StackState {
    /// 栈的逻辑标识，用于区分不同业务栈（tab/flow）。
    pub id: StackId,

    /// 栈底页面（root frame），永远存在。
    pub root_frame: StackFrame,

    /// root 之上的页面帧列表（从底到顶）。
    ///
    /// - 如果为空，则当前栈只有 root_frame
    /// - additional_frames.last() 是栈顶（如果非空）
    pub additional_frames: Vec<StackFrame>,
}

impl StackState {
    /// 获取所有 frames（包括 root），从底到顶。
    pub fn all_frames(&self) -> impl Iterator<Item = &StackFrame> {
        std::iter::once(&self.root_frame).chain(self.additional_frames.iter())
    }

    /// 获取栈顶 frame。
    pub fn top_frame(&self) -> &StackFrame {
        self.additional_frames.last().unwrap_or(&self.root_frame)
    }

    /// 获取栈深度（总 frame 数）。
    pub fn depth(&self) -> usize {
        1 + self.additional_frames.len()
    }
}

/// 表示参与当前可见 UI 的 runtime 层级信息。
///
/// v1 可以先作为占位结构不使用，仅为未来扩展预留。
#[derive(Debug, Clone)]
pub struct RuntimeLayer {
    /// 具体 runtime 类型。
    pub runtime: RuntimeKind,

    /// runtime 实例标识（某些 runtime 可能存在多个并行实例，例如多 Flutter engine）。
    pub instance_id: Option<String>,

    /// 此层是否位于最顶层（例如 RN 叠在 Flutter 之上）。
    pub is_top: bool,
}

/// 整个 App 当前导航状态的"世界快照"。
///
/// RouterCore 在每次处理导航请求前后，都会拿到一个 RouterState：
/// - before: 旧世界
/// - after:  新世界
///
/// StateMachine 的职责就是从 before 计算出 after，
/// 并生成一个 Transition 描述这次发生的变化。
#[derive(Debug, Clone)]
pub struct RouterState {
    /// 所有业务导航栈：StackId -> StackState。
    ///
    /// 由于 StackState 类型保证了至少有一个 frame，
    /// 所以不存在空栈。
    pub stacks: HashMap<StackId, StackState>,

    /// 当前处于前台的栈。
    ///
    /// - 切换底部 tab 本质上就是修改这个字段；
    /// - 栈本身的 frames 通常被保留，只是 active 的栈变了。
    pub active_stack: StackId,

    /// 当前可见的 runtime 实例层级信息（可选扩展）。
    pub runtime_layers: Vec<RuntimeLayer>,
}

/// 导航变更的"操作类型"描述。
///
/// 状态机会根据当前 RouterState + RouteMeta + NavAction 请求，
/// 计算出一个或多个 TransitionKind，然后据此生成新的 RouterState。
#[derive(Debug, Clone)]
pub enum TransitionKind {
    /// 在指定栈上执行 Push。
    Push {
        target_stack: StackId,
        route_id: RouteId,
    },

    /// 在指定栈上执行 Pop（从栈顶移除一个 frame）。
    ///
    /// NOTE: 由于 StackState 类型保证了 root frame 永远存在，
    ///       Pop 只会移除 additional_frames 中的元素。
    ///       如果 additional_frames 为空，Pop 为 no-op。
    Pop { target_stack: StackId },

    /// 在指定栈上执行 Replace（用新的 route 替换栈顶）。
    Replace {
        target_stack: StackId,
        route_id: RouteId,
    },

    /// 将指定栈重置为一个新的 root route。
    ResetStack {
        target_stack: StackId,
        new_root: RouteId,
    },

    /// 切换当前前台栈。
    ///
    /// - 如果目标栈不存在，状态机可决定是否自动创建并 push 其默认 root route。
    SwitchActiveStack {
        target_stack: StackId,
        /// 可选：如果目标栈不存在，使用哪个 route 作为新 root。
        ensure_root: Option<RouteId>,
    },

    /// 多栈操作（预留用于以后更复杂的场景）。
    ///
    /// 比如同时重置多个栈、按照某个策略批量迁移栈内容等。
    MultiStackOperation {
        /// 业务定义的多栈行为 key / 类型，用于在状态机中做具体解释。
        behavior_key: String,
    },
}

/// 栈顶页面的轻量快照，用于调试和 command 生成参考。
#[derive(Debug, Clone)]
pub struct StackTopSnapshot {
    pub route_id: RouteId,
    pub runtime: RuntimeKind,
}

/// 为了避免在 Transition 中完整拷贝 RouterState，
/// 提供一个轻量的状态摘要结构，用于日志、调试和命令生成参考。
#[derive(Debug, Clone)]
pub struct StateSummary {
    /// 当前前台栈。
    pub active_stack: StackId,

    /// 各栈的栈顶 route 概览（如果存在）。
    pub stack_tops: HashMap<StackId, Option<StackTopSnapshot>>,
}

/// 一次完整导航操作导致的"状态变更描述"。
///
/// Transition 是状态机的输出：
/// - 描述从哪个 RouterState 到哪个 RouterState（摘要）
/// - 过程中做了哪些 TransitionKind（可能不止一个）
/// - 哪些栈受到影响
/// - runtime 层级是否发生变化
#[derive(Debug, Clone)]
pub struct Transition {
    /// 导航操作开始前的状态快照摘要。
    pub from_state_summary: StateSummary,

    /// 导航操作结束后的状态快照摘要。
    pub to_state_summary: StateSummary,

    /// 这次导航包含的原子变更步骤列表。
    ///
    /// 例如一个复杂的操作可能先 Reset 再 Push，再 SwitchActiveStack。
    pub steps: Vec<TransitionKind>,
}
