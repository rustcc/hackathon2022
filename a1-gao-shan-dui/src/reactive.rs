use slotmap::{new_key_type, SecondaryMap, SlotMap};
use std::{
    any::Any,
    cell::{Cell, RefCell},
    collections::HashSet,
    marker::PhantomData,
    rc::Rc,
};

thread_local! {
    static RT: Runtime = Runtime::default();
}

/// 响应系统的运行时，是各原语间进行沟通的桥梁。
#[derive(Default)]
struct Runtime {
    /// 观察者，即系统中当前运行的 [`Effect`]，[`Signal`] 通过检查该字段来更新
    /// 自己的订阅者。
    observer: Cell<Option<EffectId>>,
    /// 作用域，记录了系统中所有存活的 [`Scope`]。
    scopes: RefCell<SlotMap<ScopeId, RawScope>>,
    /// 信号及其上下文，记录系统中所有存活的 [`Signal`]。一个 [`Signal`] 的创建
    /// 与释放由其所在的 [`Scope`] 进行管理，即当一个 [`Scope`] 被释放的时候，
    /// 其内部所有的 [`Signal`] 都应该被释放。
    signals: RefCell<SlotMap<SignalId, RawSignal>>,
    signal_contexts: RefCell<SecondaryMap<SignalId, SignalContext>>,
    /// 副作用及其上下文，记录系统中所有存活的 [`Effect`]。与 [`Signal`] 类似，
    /// 它的存活受其所在作用域的管理。
    effects: RefCell<SlotMap<EffectId, RawEffect>>,
    effect_contexts: RefCell<SecondaryMap<EffectId, EffectContext>>,
}

new_key_type! {
    struct ScopeId;
    struct SignalId;
    struct EffectId;
}

/// 作用域，管理响应原语的核心机制。使用者可以创建一个作用域来引入响应系统。
#[derive(Clone, Copy)]
pub struct Scope {
    id: ScopeId,
}

#[derive(Default)]
struct RawScope {
    /// 当前 [`Scope`] 分配的 [`Signal`] 与 [`Effect`]。这里亦可以用来管理，
    /// 例如 on_cleanup 等生命周期 hook。
    cleanups: Vec<Cleanup>,
}

enum Cleanup {
    Child(ScopeId),
    Signal(SignalId),
    Effect(EffectId),
}

impl ScopeId {
    fn try_dispose(&self) -> Result<(), ()> {
        RT.with(|rt| {
            let raw = { rt.scopes.borrow_mut().get_mut(*self).map(std::mem::take) };
            if let Some(raw) = raw {
                // 大多数情况下，后创建的原语，例如 Effect，Signal 等，只会依赖于比它
                // 先创建的原语，因此最后创建的需要最先被释放。
                for cleanup in raw.cleanups.into_iter().rev() {
                    match cleanup {
                        Cleanup::Child(id) => {
                            // 可能被手动释放。
                            id.try_dispose().ok();
                        }
                        Cleanup::Signal(id) => {
                            rt.signals.borrow_mut().remove(id).unwrap();
                            rt.signal_contexts.borrow_mut().remove(id).unwrap();
                        }
                        Cleanup::Effect(id) => {
                            let raw = rt.effects.borrow_mut().remove(id).unwrap();
                            if Rc::strong_count(&raw) != 1 {
                                panic!("试图释放一个正在运行中的 Effect");
                            }
                            rt.effect_contexts.borrow_mut().remove(id).unwrap();
                        }
                    }
                }
                rt.scopes.borrow_mut().remove(*self).unwrap();
                Ok(())
            } else {
                Err(())
            }
        })
    }

    fn on_cleanup(&self, cleanup: Cleanup) {
        RT.with(|rt| {
            rt.scopes
                .borrow_mut()
                .get_mut(*self)
                .unwrap_or_else(|| panic!("试图访问一个已经释放的 Scope"))
                .cleanups
                .push(cleanup);
        });
    }
}

/// 当 [`ScopeDisposer`] 被 drop 时，自动释放其管理的 [`Scope`]，如果其被 [`std::mem::forget`]，
/// 那么该作用域将永远不会被释放。
pub struct ScopeDisposer(Scope);

impl ScopeDisposer {
    fn new(parent: Option<ScopeId>) -> Self {
        RT.with(|rt| {
            let id = { rt.scopes.borrow_mut().insert(Default::default()) };
            if let Some(parent) = parent {
                parent.on_cleanup(Cleanup::Child(id));
            }
            Self(Scope { id })
        })
    }
}

impl Drop for ScopeDisposer {
    fn drop(&mut self) {
        self.0.id.try_dispose().ok();
    }
}

/// 信号，触发响应系统的核心机制。响应系统通过跟踪对 [`Signal`] 的读写来确定各原
/// 语间的依赖关系，并通知相关依赖对 [`Signal`] 的更新作出反应。
pub struct Signal<T: 'static> {
    id: SignalId,
    marker: PhantomData<T>,
}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Signal<T> {}

/// [`Signal`] 内部储存的变量。
type RawSignal = Box<dyn Any>;

#[derive(Default)]
struct SignalContext {
    /// [`Signal`] 所影响的全部 [`Effect`]，写入 [`Signal`] 时被触发。
    subscribers: HashSet<EffectId>,
    // 这里我们也可以使用 `IndexMap` 来保留 [`Effect`] 订阅的顺序，以便于
    // 使嵌套 [`Effect`] 中最内层的可以最先被触发。
}

impl<T> Signal<T> {
    /// 读取此 [`Signal`] 储存的变量，并追踪其所在的 [`Effect`]，即如果在一个
    /// [`Effect`] 中执行，则下一次写入会触发该 [`Effect`]。
    pub fn get(&self) -> T
    where
        T: Clone,
    {
        self.track();
        RT.with(|rt| {
            rt.signals
                .borrow()
                .get(self.id)
                .unwrap_or_else(|| panic!("试图读取一个已经被释放的 Signal"))
                .downcast_ref::<T>()
                // 每个线程对应一个独立的运行时，如果一个 Signal 被发送到了其它线程，
                // 其 ID 对应的内存单元可能与当前记录的类型不匹配。
                .unwrap_or_else(|| panic!("Signal 类型不匹配"))
                .clone()
        })
    }

    /// 通知当前 [`Runtime::observer`] 来依赖此 [`Signal`]。
    fn track(&self) {
        RT.with(|rt| {
            if let Some(id) = rt.observer.get() {
                // 在这里我们通知 Effect 来依赖自己，而不是主动去订阅 Effect，如果
                // 在同一个 Effect 中进行读写，这样可以避免该 Effect 被递归调用。
                rt.effect_contexts
                    .borrow_mut()
                    .get_mut(id)
                    .unwrap_or_else(|| unreachable!("一个运行中的 Effect 肯定存在于运行时中"))
                    .depenencies
                    .insert(self.id);
            }
        })
    }

    fn write<U>(&self, f: impl FnOnce(&mut T) -> U) -> U {
        RT.with(|rt| {
            let mut signals = rt.signals.borrow_mut();
            let t = signals
                .get_mut(self.id)
                .unwrap_or_else(|| panic!("试图写入一个已经被释放的 Signal"))
                .downcast_mut::<T>()
                .unwrap_or_else(|| panic!("Signal 类型不匹配"));
            f(t)
        })
    }

    /// 写入此 [`Signal`]，并触发受其影响的 [`Effect`]。
    pub fn set(&self, val: T) {
        self.write(|t| *t = val);
        self.trigger();
    }

    /// 根据以前的值来更新此 [`Signal`]。
    pub fn update(&self, f: impl FnOnce(&T) -> T) {
        self.write(|t| *t = f(t));
        self.trigger();
    }

    /// 触发受此 [`Signal`] 影响的全部 [`Effect`]。
    fn trigger(&self) {
        RT.with(|rt| {
            let subscribers = {
                // 在一个独立的代码块中可变借用，防止运行 Effect 时 rt.signal_contexts
                // 被同时可变借用。
                rt.signal_contexts
                    .borrow_mut()
                    .get_mut(self.id)
                    .unwrap_or_else(|| panic!("试图访问一个已经被释放的作用域"))
                    // 清空当前 subscribers，每次 Effect 被触发都需要重新跟踪依赖关系。
                    .subscribers
                    .drain()
                    .collect::<Vec<_>>()
            };
            for id in subscribers {
                // 子作用域的中 Signal 可能已经被释放了，没必要执行。
                id.try_run().ok();
            }
        });
    }
}

/// 副作用，系统中响应的核心机制。一个 [`Effect`] 可以依赖于多个 [`Signal`]，并在它们被写入时
/// 作出反应。
#[derive(Clone, Copy)]
pub struct Effect {
    id: EffectId,
}

impl Effect {
    pub fn run(&self) {
        self.id
            .try_run()
            .unwrap_or_else(|_| panic!("试图运行一个已经被释放的 Effect"));
    }
}

/// [`Effect`] 内部储存的实际作出反应的函数。
type RawEffect = Rc<RefCell<dyn FnMut()>>;

#[derive(Default)]
struct EffectContext {
    /// [`Effect`] 所依赖的全部 [`Signal`]。
    depenencies: HashSet<SignalId>,
}

impl EffectId {
    // 当此 [`Effect`] 未被释放时，执行该 [`Effect`] 并重新追踪依赖关系，否则返回 Err.
    fn try_run(&self) -> Result<(), ()> {
        RT.with(|rt| {
            let raw = {
                // 获取 Effect 的一个副本，在 Effect 运行时创建 Effect 时，这样能避免
                // rt.effects 被同时可变借用。
                rt.effects.borrow().get(*self).cloned()
            };
            if let Some(raw) = raw {
                // 1) 清空依赖关系。
                self.clear_dependencies();

                // 2) 将当前 Effect 设为观察者。
                let prev_observer = rt.observer.take();
                rt.observer.set(Some(*self));

                // 3) 执行 Effect 函数。
                raw.borrow_mut()();
                drop(raw);

                // 4) 更新依赖关系。
                self.subscriber_dependencies();

                // 5) 恢复原先的观察者。
                rt.observer.set(prev_observer);

                Ok(())
            } else {
                Err(())
            }
        })
    }

    fn clear_dependencies(&self) {
        RT.with(|rt| {
            let mut effect_contexts = rt.effect_contexts.borrow_mut();
            let depenencies = effect_contexts
                .get_mut(*self)
                .unwrap_or_else(|| unreachable!("运行中的 Effect 不可能被释放"));
            let mut signal_contexts = rt.signal_contexts.borrow_mut();
            for id in depenencies.depenencies.drain() {
                // 子作用域的中 Signal 可能已经被释放了，没必要更新。
                if let Some(ctx) = signal_contexts.get_mut(id) {
                    // 通知 Signal 取消订阅自己，因为 Effect 执行之后，该 Signal 可能
                    // 不再被当前 Effect 依赖。
                    ctx.subscribers.remove(self);
                }
            }
        })
    }

    fn subscriber_dependencies(&self) {
        RT.with(|rt| {
            let effect_contexts = rt.effect_contexts.borrow();
            let depenencies = effect_contexts
                .get(*self)
                .unwrap_or_else(|| unreachable!("运行中的 Effect 不可能被释放"));
            let mut signal_contexts = rt.signal_contexts.borrow_mut();
            for id in depenencies.depenencies.iter() {
                if let Some(ctx) = signal_contexts.get_mut(*id) {
                    // Effect 执行之后，内部所有被跟踪的 Singal 都应该存在于该 Effect
                    // 对应的 depenencies 中，我们需要订阅这些 Signal 以便其被写
                    // 入时自己能够被触发。
                    ctx.subscribers.insert(*self);
                }
            }
        })
    }
}

/// 调用 `op`，其中所有的操作，例如 [`Signal::get`]，均不会被所在的 [`Effect`] 追踪。
pub fn untrack<U>(op: impl FnOnce() -> U) -> U {
    RT.with(|rt| {
        let prev_observer = rt.observer.take();
        let output = op();
        rt.observer.set(prev_observer);
        output
    })
}

/// 创建一个根 [`Scope`]，返回 `f` 的返回值及 [`ScopeDisposer`] 用来释放该作用域。
pub fn create_root<U>(f: impl FnOnce(Scope) -> U) -> (U, ScopeDisposer) {
    let disposer = ScopeDisposer::new(None);
    let output = f(disposer.0);
    (output, disposer)
}

impl Scope {
    /// 创建一个子 [`Scope`]，当前 [`Scope`] 被释放时，该子 [`Scope`] 也会被释放，亦可以
    /// 通过返回的 [`ScopeDisposer`] 手动释放。
    pub fn create_child<U>(&self, f: impl FnOnce(Scope) -> U) -> (U, ScopeDisposer) {
        let disposer = ScopeDisposer::new(Some(self.id));
        let output = f(disposer.0);
        (output, disposer)
    }

    /// 创建一个 [`Signal`].
    pub fn create_signal<T>(&self, init: T) -> Signal<T> {
        let id = RT.with(|rt| {
            let id = rt.signals.borrow_mut().insert(Box::new(init));
            rt.signal_contexts
                .borrow_mut()
                .insert(id, Default::default());
            id
        });
        self.id.on_cleanup(Cleanup::Signal(id));
        Signal {
            id,
            marker: PhantomData,
        }
    }

    /// 创建一个 [`Effect`].
    pub fn create_effect(&self, f: impl 'static + FnMut()) {
        let id = RT.with(|rt| {
            let id = rt.effects.borrow_mut().insert(Rc::new(RefCell::new(f)));
            rt.effect_contexts
                .borrow_mut()
                .insert(id, Default::default());
            id
        });
        self.id.on_cleanup(Cleanup::Effect(id));
        // 第一次执行来追踪最初的依赖关系。
        id.try_run().unwrap();
    }

    /// 创建一个 [`Signal`]，其跟踪 `f` 的返回值，每当其返回新值时，该 [`Signal`]
    /// 会随之更新。
    pub fn create_memo<T>(&self, mut f: impl 'static + FnMut() -> T) -> Signal<T> {
        let memo = Rc::new(Cell::new(None::<Signal<T>>));
        let cx = *self;
        cx.create_effect({
            let memo = memo.clone();
            move || {
                let new_val = f();
                if let Some(signal) = memo.get() {
                    signal.set(new_val);
                } else {
                    let signal = cx.create_signal(new_val);
                    memo.set(Some(signal));
                }
            }
        });
        // memo 应该随 Effect 执行而有了初值
        memo.get().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn signal() {
        create_root(|cx| {
            let state = cx.create_signal(0);
            assert_eq!(state.get(), 0);
            state.update(|x| *x + 1);
            assert_eq!(state.get(), 1);
        });
    }

    #[test]
    fn memo() {
        create_root(|cx| {
            let state = cx.create_signal(1);
            let double = cx.create_memo(move || state.get() * 2);
            assert_eq!(state.get(), 1);
            assert_eq!(double.get(), 2);
            state.update(|x| *x + 1);
            assert_eq!(state.get(), 2);
            assert_eq!(double.get(), 4);
        });
    }

    #[test]
    fn effect() {
        create_root(|cx| {
            let trigger = cx.create_signal(());
            let counter = Rc::new(Cell::new(0));
            cx.create_effect({
                let counter = counter.clone();
                move || {
                    trigger.get();
                    counter.set(counter.get() + 1);
                }
            });
            assert_eq!(counter.get(), 1);
            trigger.set(());
            assert_eq!(counter.get(), 2);
        });
    }

    #[test]
    fn disposer() {
        let assert_count_of_singals =
            |count| RT.with(|rt| assert_eq!(rt.signals.borrow().len(), count));

        create_root(|cx| {
            cx.create_signal(());
            assert_count_of_singals(1);
            cx.create_child(|cx| {
                cx.create_signal(());
                assert_count_of_singals(2);
            });
            assert_count_of_singals(1);
        });
        assert_count_of_singals(0);
    }
}
