use std::{
  collections::VecDeque,
  future::Future,
  pin::Pin,
  sync::{Arc, Mutex},
  task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
  thread,
  time::{Duration, Instant},
};

type BoxFuture = Pin<Box<dyn Future<Output = ()> + Send>>;

pub struct MiniRuntime {
  tasks: VecDeque<Task>,
}

struct Task {
  future: BoxFuture,
}

impl MiniRuntime {
  pub fn new() -> Self {
      Self {
          tasks: VecDeque::new(),
      }
  }

  pub fn spawn<F>(&mut self, future: F)
  where
      F: Future<Output = ()> + Send + 'static,
  {
      self.tasks.push_back(Task {
          future: Box::pin(future),
      });
  }

  pub fn block_on(&mut self, fut: impl Future<Output = ()> + Send + 'static) {
      self.spawn(fut);

      while !self.tasks.is_empty() {
          let mut remaining = VecDeque::new();

          while let Some(mut task) = self.tasks.pop_front() {
              let waker = dummy_waker();
              let mut ctx = Context::from_waker(&waker);
              match task.future.as_mut().poll(&mut ctx) {
                  Poll::Pending => remaining.push_back(task),
                  Poll::Ready(()) => {}
              }
          }

          self.tasks = remaining;

          // Prevent busy loop
          thread::sleep(Duration::from_millis(10));
      }
  }
}

// === TimerFuture ===

pub struct TimerFuture {
  shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
  completed: bool,
  waker: Option<Waker>,
}

impl TimerFuture {
  pub fn new(duration: Duration) -> Self {
      let shared_state = Arc::new(Mutex::new(SharedState {
          completed: false,
          waker: None,
      }));

      let thread_shared_state = shared_state.clone();
      thread::spawn(move || {
          thread::sleep(duration);
          let mut state = thread_shared_state.lock().unwrap();
          state.completed = true;
          if let Some(waker) = state.waker.take() {
              waker.wake();
          }
      });

      TimerFuture { shared_state }
  }
}

impl Future for TimerFuture {
  type Output = ();

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
      let mut state = self.shared_state.lock().unwrap();

      if state.completed {
          Poll::Ready(())
      } else {
          state.waker = Some(cx.waker().clone());
          Poll::Pending
      }
  }
}

// === Dummy Waker ===

fn dummy_raw_waker() -> RawWaker {
  fn no_op(_: *const ()) {}
  fn clone(_: *const ()) -> RawWaker {
      dummy_raw_waker()
  }

  let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);
  RawWaker::new(std::ptr::null(), vtable)
}

fn dummy_waker() -> Waker {
  unsafe { Waker::from_raw(dummy_raw_waker()) }
}
