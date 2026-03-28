/// Marker trait for future effect execution backends.
/// The runtime decides whether an effect may run; the handler will later
/// define how an approved effect is carried out.
pub trait EffectHandler {}
