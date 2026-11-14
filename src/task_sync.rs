//! Synchronization primitives used by the task scheduler.

use std::sync;

use parking_lot::MutexGuard;

/// A condition variable for threads that wait while holding the global task lock
/// but are not workers (e.g. the thread submitting frames).
///
/// [`parking_lot::Condvar`] requeues a notified waiter onto the mutex
/// if the notifier holds it, and the mutex is then usually handed directly
/// to that waiter on unlock ("eventual fairness").
/// The global task lock would thus be owned by a thread that is still asleep,
/// stalling every worker looking for a task until that thread gets scheduled.
/// This condvar instead wakes the waiter directly,
/// which then competes for the lock like everybody else.
#[derive(Default)]
pub struct TaskCondvar {
    lock: sync::Mutex<()>,
    cond: sync::Condvar,
}

impl TaskCondvar {
    pub const fn new() -> Self {
        Self {
            lock: sync::Mutex::new(()),
            cond: sync::Condvar::new(),
        }
    }

    #[inline]
    pub fn notify_one(&self) {
        // Taking the lock orders us w.r.t. a waiter
        // that is between unlocking the outer mutex and going to sleep.
        drop(self.lock.lock());
        self.cond.notify_one();
    }

    #[inline]
    pub fn wait<T>(&self, guard: &mut MutexGuard<'_, T>) {
        // Poisoning is irrelevant as the inner mutex guards no data.
        let inner = self
            .lock
            .lock()
            .unwrap_or_else(sync::PoisonError::into_inner);
        MutexGuard::unlocked(guard, || {
            drop(
                self.cond
                    .wait(inner)
                    .unwrap_or_else(sync::PoisonError::into_inner),
            );
        });
    }
}
