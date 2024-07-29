pub use charbs_macros::ScheduleLabel;

use std::{
  any::{Any, TypeId},
  cell::{Ref, RefCell, RefMut},
  collections::HashMap,
  marker::PhantomData,
  ops::{Deref, DerefMut},
};

/// A container structure that assembles generic resources to compose a state.
///
/// Resources can be virtually anything that a structure needs to store as part
/// of the state of a larger structure.
///
/// This is intended to be used in conjunction with a [`Scheduler`] as input
/// for the dependencies its schedule' handlers require. Together they allow
/// the creation of complex and independent systems that can easily co-exist.
#[derive(Debug, Default)]
pub struct State {
  resources: HashMap<TypeId, RefCell<Box<dyn Any>>>,
}

impl State {
  /// Adds a new generic resource to the state container.
  ///
  /// # Arguments
  ///
  /// * `resource` - The resource of type `R` to be added.
  pub fn add_resource<R: 'static>(&mut self, resource: R) {
    let key = TypeId::of::<R>();
    let value = RefCell::new(Box::new(resource));

    self.resources.insert(key, value);
  }

  /// Merges another state into this one.
  ///
  /// # Arguments
  ///
  /// * `state` - The [`State`] to be merged into this one.
  pub fn merge(&mut self, state: &mut Self) {
    for (key, value) in state.drain() {
      self.resources.insert(key, value);
    }
  }

  /// Returns a generic resource from the state container.
  ///
  /// Requested resource must be wrapped with a [`Res`] or [`ResMut`] to get a
  /// read-only reference or one with mutability.
  ///
  /// # Arguments
  ///
  /// * `->` - A read-only or mutable reference to the requested resource.
  pub fn get<R: HandlerParam + 'static>(&self) -> <R as HandlerParam>::Item<'_> {
    R::retrieve(&self.resources)
  }

  /// Returns a read-only reference to all the resources within the state
  /// container.
  pub fn all(&self) -> &HashMap<TypeId, RefCell<Box<dyn Any>>> {
    &self.resources
  }

  /// Returns a mutable reference to all the resources within the state
  /// container.
  pub fn all_mut(&mut self) -> &mut HashMap<TypeId, RefCell<Box<dyn Any>>> {
    &mut self.resources
  }

  /// Drains all resources from the state container and returns them as a vector
  /// containing a tuple with the type ID and the resource.
  ///
  /// Useful to easily clear the state container and re-use the resources
  /// somewhere else.
  ///
  /// # Arguments
  ///
  /// * `->` - A vector of tuples containing the type ID and the resource.
  pub fn drain(&mut self) -> Vec<(TypeId, RefCell<Box<dyn Any>>)> {
    self.resources.drain().collect()
  }
}

/// A trait that represents a handler.
///
/// This should only be implemented for [`HandlerFunction`].
pub trait Handler {
  /// Executes the handler.
  ///
  /// # Arguments
  ///
  /// * `resources` - A mutable reference to a [`HashMap`] of resources.
  fn run(&mut self, resources: &mut HashMap<TypeId, RefCell<Box<dyn Any>>>);
}

/// A trait that represents a valid parameter a generic handler function can
/// be injected with. By default, two structures implement this:
///
/// [`Res`]: Access a read-only reference to a resource.
/// [`ResMut`]: Access a mutable reference to a resource.
pub trait HandlerParam {
  /// Provides a copy of the struct with a new lifetime.
  type Item<'new>;

  /// Retrieves a reference to a resource and wraps it in a new [`Self::Item`].
  ///
  /// # Arguments
  ///
  /// * `resources` - A reference to the injectable resources instance.
  ///
  /// * `->` - A reference to the injectable resource.
  fn retrieve(resources: &HashMap<TypeId, RefCell<Box<dyn Any>>>) -> Self::Item<'_>;
}

/// A structure representing the actual handler function that will be executed
/// with injected resources.
pub struct HandlerFunction<Input, F> {
  f: F,
  marker: PhantomData<fn() -> Input>,
}

/// The scheduler and state structures are heavily based on this article:
/// https://promethia-27.github.io/dependency_injection_like_bevy_from_scratch
///
/// A macro to implement the [`Handler`] trait for a [`HandlerFunction`].
/// It essentially defines and calls the inner function of a [`HandlerFunction`]
/// with the generic parameters that are passed to the macro.
macro_rules! impl_handler {
  ($($p:ident),*) => {
    #[allow(unused, non_snake_case)]
    impl<F, $($p: HandlerParam),*> Handler for HandlerFunction<($($p,)*), F>
      where
        for<'a, 'b> &'a mut F: FnMut($($p),*) +
          FnMut($(<$p as HandlerParam>::Item<'b>),*)
    {
      fn run(&mut self, resources: &mut HashMap<TypeId, RefCell<Box<dyn Any>>>) {
        #[allow(clippy::too_many_arguments)]
        fn call_inner<$($p),*>(mut f: impl FnMut($($p),*), $($p: $p),*) {
          f($($p),*)
        }

        $(let $p= $p::retrieve(resources);)*

        call_inner(&mut self.f, $($p),*)
      }
    }
  }
}

// Allow for 10 parameters
impl_handler!();
impl_handler!(T1);
impl_handler!(T1, T2);
impl_handler!(T1, T2, T3);
impl_handler!(T1, T2, T3, T4);
impl_handler!(T1, T2, T3, T4, T5);
impl_handler!(T1, T2, T3, T4, T5, T6);
impl_handler!(T1, T2, T3, T4, T5, T6, T7);
impl_handler!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_handler!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_handler!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);

/// A trait that represents a generic function that can be transformed into a
/// [`Handler`] that can inject parameters into it.
pub trait IntoHandler<Input> {
  type Handler: Handler;

  /// Transforms the generic function into a [`Handler`].
  ///
  /// # Arguments
  ///
  /// * `->` - A [`Handler`] ready to be injected with resources.
  fn into_handler(self) -> Self::Handler;
}

/// A macro to implement the [`IntoHandler`] trait for a generic function
/// allowing any function with [`HandlerParam`]s to be converted in to a
/// [`HandlerFunction`] to be injected with dependencies.
macro_rules! impl_into_handler{
  ($($p:ident),*) => {
    impl<F, $($p: HandlerParam),*> IntoHandler<($($p,)*)> for F
      where
        for<'a, 'b> &'a mut F: FnMut($($p),*) +
          FnMut($(<$p as HandlerParam>::Item<'b>),*)
    {
      type Handler = HandlerFunction<($($p,)*), Self>;

      fn into_handler(self) -> Self::Handler {
        HandlerFunction {
          f: self,
          marker: Default::default(),
        }
      }
    }
  }
}

// Allow for 10 parameters
impl_into_handler!();
impl_into_handler!(T1);
impl_into_handler!(T1, T2);
impl_into_handler!(T1, T2, T3);
impl_into_handler!(T1, T2, T3, T4);
impl_into_handler!(T1, T2, T3, T4, T5);
impl_into_handler!(T1, T2, T3, T4, T5, T6);
impl_into_handler!(T1, T2, T3, T4, T5, T6, T7);
impl_into_handler!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_into_handler!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_into_handler!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);

/// A struct that represents a read-only reference to a resource.
pub struct Res<'a, T: 'static> {
  value: Ref<'a, Box<dyn Any>>,
  _marker: PhantomData<&'a T>,
}

/// Allow access to a read-only reference to the underlying resource.
impl<T: 'static> Deref for Res<'_, T> {
  type Target = T;

  fn deref(&self) -> &T {
    self.value.downcast_ref().unwrap()
  }
}

impl<'res, T: 'static> HandlerParam for Res<'res, T> {
  /// Provides a copy of the struct with a new lifetime.
  type Item<'new> = Res<'new, T>;

  fn retrieve(resources: &HashMap<TypeId, RefCell<Box<dyn Any>>>) -> Self::Item<'_> {
    Res {
      value: resources
        .get(&TypeId::of::<T>())
        .expect("Cannot find resource.")
        .borrow(),
      _marker: PhantomData,
    }
  }
}

/// A struct that represents a mutable reference to a resource.
pub struct ResMut<'a, T: 'static> {
  value: RefMut<'a, Box<dyn Any>>,
  _marker: PhantomData<&'a mut T>,
}

/// Allow access to a read-only reference to the underlying resource.
impl<T: 'static> Deref for ResMut<'_, T> {
  type Target = T;

  fn deref(&self) -> &T {
    self.value.downcast_ref().unwrap()
  }
}

/// Allow access to a mutable reference to the underlying resource.
impl<T: 'static> DerefMut for ResMut<'_, T> {
  fn deref_mut(&mut self) -> &mut T {
    self.value.downcast_mut().unwrap()
  }
}

impl<'res, T: 'static> HandlerParam for ResMut<'res, T> {
  /// Provides a copy of the struct with a new lifetime.
  type Item<'new> = ResMut<'new, T>;

  fn retrieve(resources: &HashMap<TypeId, RefCell<Box<dyn Any>>>) -> Self::Item<'_> {
    ResMut {
      value: resources.get(&TypeId::of::<T>()).unwrap().borrow_mut(),
      _marker: PhantomData,
    }
  }
}

/// A structure that allows the storage of [`Handler`] functions to be executed
/// with dynamically injected resources.
///
/// This is intended to be used in conjunction with a [`Scheduler`] as a
/// mechanism to execute specific functions ([`Handler`]s) at the specific
/// moment.
///
/// [`Schedule`]s are agnostic of the [`State`] used to execute the [`Handler`]
/// functions. This is why any function that executes a [`Handler`]s will
/// require a [`State`] to be specified.
#[derive(Default)]
pub(crate) struct Schedule {
  handlers: Vec<Box<dyn Handler>>,
}

impl Schedule {
  /// Executes all [`Handler`]s that have been added to the [`Schedule`] and
  /// allow them to use specific resources from a [`State`].
  ///
  /// # Arguments
  ///
  /// * `state` - A mutable reference to a [`State`].
  pub fn run(&mut self, state: &mut State) {
    let resources = state.all_mut();

    // Run the handlers in order.
    for handler in self.handlers.iter_mut() {
      handler.run(resources);
    }
  }

  /// Adds a new [`Handler`] to the [`Schedule`].
  ///
  /// # Arguments
  ///
  /// * `handler` - The [`Handler`] to be added.
  pub fn add_handler<I, S: Handler + 'static>(
    &mut self,
    handler: impl IntoHandler<I, Handler = S>,
  ) {
    self.handlers.push(Box::new(handler.into_handler()));
  }
}

/// A trait used for defining labels for [`Schedule`]s.
///
/// This trait is mainly used by the [`ScheduleLabel`] macro to help quickly
/// create scheduling labels.
pub trait ScheduleLabel {}

/// A structure used for creating different [`Schedule`]s with [`Handler`]s
/// effectively allowing for a system to be executed in a real-time manner.
#[derive(Default)]
pub struct Scheduler {
  schedules: HashMap<TypeId, Schedule>,
}

impl Scheduler {
  /// Executes a [`Schedule`] with a specific [`State`] based on the given
  /// [`ScheduleLabel`].
  ///
  /// The label parameter exists for consistency with the
  /// [`Scheduler::add_handler`] method.
  ///
  /// # Arguments
  ///
  /// * `label` - The label used to find the [`Schedule`] which will be
  ///   executed.
  /// * `state` - The [`State`] to be used by the [`Schedule`]s.
  #[allow(unused_variables)]
  pub fn run<R: ScheduleLabel + 'static>(&mut self, label: R, state: &mut State) {
    let key = TypeId::of::<R>();

    if let Some(schedule) = self.schedules.get_mut(&key) {
      schedule.run(state);
    }
  }

  /// Adds a [`Handler`] ƒunction to the specified [`Schedule`] using a
  /// [`ScheduleLabel`].
  ///
  /// The label parameter exists because it cannot live as a template generic.
  /// That would require having to specify I & S for [`Handler`] functions
  /// which is not doable.
  ///
  /// # Arguments
  ///
  /// * `label` - The label used to find the [`Schedule`] onto which the handler
  ///   should be added.
  /// * `handler` - The handler to be added to the [`Schedule`].
  #[allow(unused_variables)]
  pub fn add_handler<R: ScheduleLabel + 'static, I, S: Handler + 'static>(
    &mut self,
    label: R,
    handler: impl IntoHandler<I, Handler = S>,
  ) {
    let key = TypeId::of::<R>();

    if let Some(schedule) = self.schedules.get_mut(&key) {
      schedule.add_handler(handler);
    } else {
      let mut schedule = Schedule::default();
      schedule.add_handler(handler);

      self.schedules.insert(key, schedule);
    }
  }
}
