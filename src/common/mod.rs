pub(crate) mod components;

use leptos::*;

pub fn create_resource_slice<T, R, O, S>(
    signal: Resource<R, T>,
    getter: impl Fn(&Option<T>) -> O + Clone + Copy + 'static,
    setter: impl Fn(&mut Option<T>, S) + Clone + Copy + 'static,
) -> (Signal<O>, SignalSetter<S>)
    where
        O: PartialEq,
        T: Clone,
        R: Clone
{
    (
        create_resource_read_slice(signal, getter),
        create_resource_write_slice(signal, setter),
    )
}

pub fn create_resource_read_slice<T, R, O>(
    signal: Resource<R, T>,
    getter: impl Fn(&Option<T>) -> O + Clone + Copy + 'static,
) -> Signal<O>
    where
        O: PartialEq,
        T: Clone,
        R: Clone
{
    create_memo(move |_| signal.with(getter)).into()
}

pub fn create_resource_write_slice<T, R, O>(
    signal: Resource<R, T>,
    setter: impl Fn(&mut Option<T>, O) + Clone + Copy + 'static,

) -> SignalSetter<O>
    where
        T: Clone,
        R: Clone
{
    let setter = move |value| signal.update(|x| setter(x, value));
    setter.into_signal_setter()
}